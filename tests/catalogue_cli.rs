use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn command(program: &str, arguments: &[&str], cwd: &Path) -> Output {
    Command::new(program)
        .args(arguments)
        .current_dir(cwd)
        .output()
        .unwrap()
}

fn success(program: &str, arguments: &[&str], cwd: &Path) -> String {
    let output = command(program, arguments, cwd);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn cli(arguments: &[&str], cwd: &Path) -> String {
    success(env!("CARGO_BIN_EXE_formal-logos-sources"), arguments, cwd)
}

#[test]
fn importing_committed_metadata_is_idempotent_and_preserves_local_pdfs() {
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path().join("upstream");
    fs::create_dir_all(repo.join("corpus/sources")).unwrap();
    fs::create_dir_all(repo.join("corpus/passages")).unwrap();
    let source = json!({"id":"source-one","label":"Synthetic work","kind":"primary_text","author":"First and Second","work":"Synthetic work","work_urn":"urn:synthetic:work","catalog_url":"https://example.org/catalogue","citation_system":"section"});
    fs::write(
        repo.join("corpus/sources/one.json"),
        serde_json::to_vec(&source).unwrap(),
    )
    .unwrap();
    let manifest = json!({
        "snapshot":{"kind":"web_snapshot","source_id":"source-one","source_url":"https://example.org/text","language":"eng","encoding":"utf-8","license":"Synthetic test metadata","retrieved_at":"2026-11-19T00:00:00Z","sha256":format!("sha256:{}","a".repeat(64))},
        "passages":[{"id":"passage-one","label":"Section 1","locator":"§1","claim_locator_prefixes":["§1"],"html_selector":{"element_id":"one"},"text_sha256":format!("sha256:{}","b".repeat(64))}]
    });
    fs::write(
        repo.join("corpus/passages/one.json"),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    success("git", &["init", "-q"], &repo);
    success("git", &["add", "corpus"], &repo);
    success(
        "git",
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.org",
            "commit",
            "-qm",
            "fixture",
        ],
        &repo,
    );
    let revision = success("git", &["rev-parse", "HEAD"], &repo);
    fs::write(
        dir.path().join("groups.json"),
        br#"{"schema_version":1,"groups":[]}"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("local.pdf"),
        b"%PDF-synthetic local fixture",
    )
    .unwrap();
    cli(
        &[
            "add",
            "local.pdf",
            "--work-id",
            "local-work",
            "--title",
            "Local work",
            "--edition-id",
            "local-edition",
            "--edition-label",
            "Local edition",
            "--language",
            "eng",
            "--license",
            "Synthetic fixture",
        ],
        dir.path(),
    );
    let catalogue = dir.path().join("registry/catalog.json");
    let mut local: Value = serde_json::from_slice(&fs::read(&catalogue).unwrap()).unwrap();
    local["authors"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"planned-author","name":"Planned Author"}));
    fs::write(&catalogue, serde_json::to_vec(&local).unwrap()).unwrap();
    let args = [
        "import",
        "--formal-logos",
        "upstream",
        "--revision",
        revision.trim(),
        "--work-groups",
        "groups.json",
    ];
    let summary: Value = serde_json::from_str(&cli(&args, dir.path())).unwrap();
    assert_eq!(summary["source_bytes_acquired"], 0);
    let first = fs::read(&catalogue).unwrap();
    // A dirty checkout must never contaminate records labelled with the committed revision.
    fs::write(repo.join("corpus/sources/one.json"), b"not even JSON").unwrap();
    cli(&args, dir.path());
    assert_eq!(fs::read(&catalogue).unwrap(), first);
    let check: Value = serde_json::from_str(&cli(&["check"], dir.path())).unwrap();
    assert_eq!(check["local_pdf_editions"], 1);
    assert_eq!(check["referenced_editions"], 1);
    let verification: Value = serde_json::from_str(&cli(&["verify"], dir.path())).unwrap();
    assert_eq!(verification["verified_unique_objects"], 1);
    let catalog: Value = serde_json::from_slice(&first).unwrap();
    assert!(
        catalog["authors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|a| a["id"] == "planned-author")
    );
    let referenced_id = catalog["editions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e.get("formal_logos").is_some())
        .unwrap()["id"]
        .as_str()
        .unwrap();
    let refused = command(
        env!("CARGO_BIN_EXE_formal-logos-sources"),
        &["prepare", "--edition-id", referenced_id, "--pages", "1:1"],
        dir.path(),
    );
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("metadata"));
    let author: Value =
        serde_json::from_str(&cli(&["authors", "--name", "First and Second"], dir.path())).unwrap();
    assert_eq!(author["author_attributions"].as_array().unwrap().len(), 1);
    assert_eq!(author["author_attributions"][0]["registered_passages"], 1);
}

#[test]
fn check_fails_when_the_catalogue_is_missing() {
    let dir = tempfile::tempdir().unwrap();
    let result = command(
        env!("CARGO_BIN_EXE_formal-logos-sources"),
        &["check"],
        dir.path(),
    );
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("catalogue does not exist"));
}
