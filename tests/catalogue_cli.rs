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
    let source = json!({"id":"source-one","label":"Synthetic work","kind":"primary_text","author":"First and Second","work":"Synthetic work","work_urn":"doi:10.1234/example","catalog_url":"https://example.org/catalogue","citation_system":"section"});
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
    fs::create_dir_all(repo.join("docs/notes")).unwrap();
    let notes = r#"# Evidence

[§2](https://doi.org/10.1234/Example#page=3) and [§3](https://dx.doi.org/10.1234/example#page=7).
[First version](https://arxiv.org/pdf/1710.05834v1.pdf#page=4).
Second version <https://arxiv.org/abs/1710.05834v2>.
[The recorded edition](https://example.org/text#one).
[The registered work](https://example.org/catalogue).
[Unregistered blog](https://example.org/blog), [issue](https://github.com/example/project/issues/3),
[server](https://mcp.moenarch.com/healthz), [lookalike](https://doi.org.example.org/10.1234/fake).
```sh
curl https://doi.org/10.1234/test
```
"#;
    fs::write(repo.join("docs/notes/proof.md"), notes).unwrap();
    fs::write(
        repo.join("docs/physics.md"),
        "See [the report][ref].\n\n[ref]: https://arxiv.org/abs/1710.05834v2\n",
    )
    .unwrap();
    success("git", &["init", "-q"], &repo);
    success("git", &["add", "corpus", "docs"], &repo);
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
    assert_eq!(summary["research_citations"], 4);
    let cited = |query: &str| -> Value {
        serde_json::from_str(&cli(&["citations", "--source", query], dir.path())).unwrap()
    };
    let paper = cited("https://arxiv.org/abs/1710.05834");
    assert_eq!(paper["formal_logos"]["revision"], revision.trim());
    let arxiv = &paper["citations"][0];
    assert_eq!(arxiv["id"], "arxiv:1710.05834");
    let urls = arxiv["cited_urls"].as_array().unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0]["cited_at"].as_array().unwrap().len(), 2);
    assert_eq!(
        urls[1]["url"],
        "https://arxiv.org/pdf/1710.05834v1.pdf#page=4"
    );
    let doi = cited("doi:10.1234/EXAMPLE");
    let urls = doi["citations"][0]["cited_urls"].as_array().unwrap();
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0]["cited_at"][0]["path"], "docs/notes/proof.md");
    assert_eq!(urls[0]["cited_at"][0]["line"], 3);
    assert_eq!(urls[0]["cited_at"][0]["label"], "§2");
    assert!(urls[0]["work_ids"].is_array());
    assert!(urls[0].get("edition_ids").is_none());
    let by_work = cited("source-one");
    assert_eq!(by_work["citations"].as_array().unwrap().len(), 3);
    let text = cited("https://example.org/text#one");
    let edition_id = text["citations"][0]["cited_urls"][0]["edition_ids"][0]
        .as_str()
        .unwrap();
    assert_eq!(cited(edition_id)["citations"].as_array().unwrap().len(), 1);
    let first = fs::read(&catalogue).unwrap();
    // A dirty checkout must never contaminate records labelled with the committed revision.
    fs::write(repo.join("corpus/sources/one.json"), b"not even JSON").unwrap();
    fs::write(
        repo.join("docs/notes/proof.md"),
        "https://doi.org/10.1234/dirty",
    )
    .unwrap();
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

    // A local HTML page with a cited URL resolves after an explicit offline refresh.
    fs::write(
        dir.path().join("blog.html"),
        "<title>Blog</title><h2>Post</h2><p>Synthetic &amp; local.</p>",
    )
    .unwrap();
    cli(
        &[
            "add-html",
            "blog.html",
            "--work-id",
            "local-work",
            "--title",
            "Local work",
            "--edition-id",
            "local-blog",
            "--edition-label",
            "Local blog page",
            "--language",
            "eng",
            "--license",
            "Synthetic fixture",
            "--encoding",
            "utf-8",
            "--locator",
            "Post",
            "--url",
            "https://example.org/blog",
        ],
        dir.path(),
    );
    assert!(
        cited("local-blog")["citations"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let refreshed: Value = serde_json::from_str(&cli(
        &["refresh-citations", "--formal-logos", "upstream"],
        dir.path(),
    ))
    .unwrap();
    assert_eq!(refreshed["revision"], revision.trim());
    assert_eq!(refreshed["research_citations"], 5);
    let blog = cited("local-blog");
    assert_eq!(
        blog["citations"][0]["cited_urls"][0]["url"],
        "https://example.org/blog"
    );
    let prepared: Value = serde_json::from_str(&cli(
        &[
            "prepare",
            "--edition-id",
            "local-blog",
            "--selector",
            r#"{"heading":"Post"}"#,
            "--text",
        ],
        dir.path(),
    ))
    .unwrap();
    assert_eq!(prepared["text"], "Post\nSynthetic & local.");
    assert_eq!(prepared["locator"], "Post");
    assert_eq!(prepared["remote_witness_verified"], false);
    let check: Value = serde_json::from_str(&cli(&["check"], dir.path())).unwrap();
    assert_eq!(check["local_html_editions"], 1);
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
