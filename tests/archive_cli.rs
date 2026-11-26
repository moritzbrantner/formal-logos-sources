//! Archive-on-demand against a local HTTP fixture server; no external host is contacted.
use serde_json::{Value, json};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

fn serve() -> (String, Arc<Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let log = Arc::clone(&requests);
    let origin = base.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let target = line.split(' ').nth(1).unwrap_or("").to_owned();
            let mut header = String::new();
            while reader.read_line(&mut header).unwrap() > 2 {
                header.clear();
            }
            log.lock().unwrap().push(target.clone());
            let cdx = format!(
                "/cdx/search/cdx?url=http%3A%2F%2F{}%2Fmissing",
                &origin[7..]
            )
            .replace(':', "%3A");
            let capture = format!("/web/20200101000000id_/{origin}/missing");
            let (status, extra, body) = match target.as_str() {
                "/page" => (
                    "200 OK",
                    "Content-Type: text/html; charset=utf-8\r\n",
                    "<p>live page</p>".to_owned(),
                ),
                "/moved" => (
                    "301 Moved Permanently",
                    "Location: /page\r\n",
                    String::new(),
                ),
                t if t.starts_with(&cdx) => (
                    "200 OK",
                    "Content-Type: application/json\r\n",
                    format!(r#"[["timestamp","original"],["20200101000000","{origin}/missing"]]"#),
                ),
                t if t == capture => (
                    "200 OK",
                    "Content-Type: text/html\r\n",
                    "<p>captured</p>".to_owned(),
                ),
                _ => ("404 Not Found", "", "missing".to_owned()),
            };
            let response = format!(
                "HTTP/1.1 {status}\r\n{extra}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
    });
    (base, requests)
}

fn cli(arguments: &[&str], cwd: &Path) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_formal-logos-sources"))
        .args(arguments)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn cited_urls_are_archived_once_live_or_from_wayback_and_reported_as_local_copies() {
    let (base, requests) = serve();
    let dir = tempfile::tempdir().unwrap();
    let cwd = dir.path();
    fs::create_dir_all(cwd.join("registry")).unwrap();
    let citation = |path: &str| {
        json!({"id": format!("{base}{path}"), "cited_urls": [{"url": format!("{base}{path}#section"),
            "cited_at": [{"path": "docs/notes/example.md", "line": 3}]}]})
    };
    let catalog = json!({"schema_version": 2, "works": [], "editions": [],
        "research_citations": [citation("/missing"), citation("/moved")],
        "formal_logos": {"repository": "example/corpus", "revision": "a".repeat(40)}});
    fs::write(
        cwd.join("registry/catalog.json"),
        serde_json::to_vec(&catalog).unwrap(),
    )
    .unwrap();

    let planned = cli(&["archive", "--dry-run"], cwd);
    assert_eq!(
        planned["targets"],
        json!([format!("{base}/missing"), format!("{base}/moved")])
    );
    assert!(requests.lock().unwrap().is_empty());

    let wayback = ["--wayback", base.as_str(), "--interval-ms", "0"];
    let summary = cli(&[&["archive"][..], &wayback].concat(), cwd);
    assert_eq!(
        (
            summary["archived_live"].as_u64(),
            summary["archived_wayback"].as_u64()
        ),
        (Some(1), Some(1)),
        "{summary}"
    );
    let archive: Value =
        serde_json::from_slice(&fs::read(cwd.join("registry/web-archive.json")).unwrap()).unwrap();
    let entries = archive["entries"].as_array().unwrap();
    let live = &entries[1];
    assert_eq!(live["via"], "live");
    assert_eq!(live["final_url"], format!("{base}/page"));
    assert_eq!(live["content_type"], "text/html; charset=utf-8");
    let sha = live["sha256"].as_str().unwrap();
    assert_eq!(
        fs::read(cwd.join(format!("local/objects/sha256/{}/{sha}", &sha[..2]))).unwrap(),
        b"<p>live page</p>"
    );
    let old = &entries[0];
    assert_eq!(
        (&old["via"], &old["live_status"], &old["wayback_timestamp"]),
        (&json!("wayback"), &json!(404), &json!("20200101000000"))
    );

    let seen = requests.lock().unwrap().len();
    let rerun = cli(&[&["archive"][..], &wayback].concat(), cwd);
    assert_eq!(rerun["already_archived"], 2);
    assert_eq!(requests.lock().unwrap().len(), seen);

    let checked = cli(&["check"], cwd);
    assert_eq!(checked["web_archive"]["entries"], 2);
    let listed = cli(&["citations"], cwd);
    assert_eq!(listed["local_copies"], json!({"web_archive": 2}));
    assert_eq!(
        listed["citations"][1]["cited_urls"][0]["local_copy"]["via"],
        "live"
    );
    let missing = cli(&["citations", "--without-local-copy"], cwd);
    assert_eq!(missing["citations"], json!([]));
}
