use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn run(program: &str, arguments: &[&str], cwd: &Path) -> Output {
    Command::new(program)
        .args(arguments)
        .current_dir(cwd)
        .output()
        .unwrap()
}

fn success(program: &str, arguments: &[&str], cwd: &Path) -> String {
    let output = run(program, arguments, cwd);
    assert!(
        output.status.success(),
        "{} {:?}: {}",
        program,
        arguments,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn cli(arguments: &[&str], cwd: &Path) -> Value {
    serde_json::from_str(&success(
        env!("CARGO_BIN_EXE_formal-logos-sources"),
        arguments,
        cwd,
    ))
    .unwrap()
}

fn write(repo: &Path, name: &str, value: &Value) {
    let path = repo.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}

fn commit(repo: &Path) -> String {
    success("git", &["add", "-A"], repo);
    success(
        "git",
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.org",
            "commit",
            "-qm",
            "fixture update",
        ],
        repo,
    );
    success("git", &["rev-parse", "HEAD"], repo)
        .trim()
        .to_owned()
}

fn source(id: &str) -> Value {
    json!({
        "id":id, "label":id, "author":"A. Example", "work":id,
        "kind":"primary_text", "work_urn":format!("urn:fixture:{id}"),
        "catalog_url":"https://example.org/catalogue", "citation_system":"section"
    })
}

fn manifest(source_id: &str, id: &str, hash: char) -> Value {
    json!({
        "snapshot": {
            "kind":"web_snapshot", "source_id":source_id,
            "source_url":"https://example.org/text", "language":"eng",
            "encoding":"utf-8", "license":"Synthetic fixture",
            "retrieved_at":"2026-11-23T00:00:00Z",
            "sha256":format!("sha256:{}", "a".repeat(64))
        },
        "passages":[{
            "id":id, "label":"Section one", "locator":"§1",
            "claim_locator_prefixes":["§1"], "html_selector":{"element_id":"one"},
            "text_sha256":format!("sha256:{}", hash.to_string().repeat(64))
        }]
    })
}

fn fixture() -> (tempfile::TempDir, PathBuf, String) {
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path().join("upstream");
    fs::create_dir_all(&repo).unwrap();
    success("git", &["init", "-q", "-b", "main"], &repo);
    write(&repo, "corpus/sources/one.json", &source("source-one"));
    write(&repo, "corpus/sources/two.json", &source("source-two"));
    write(
        &repo,
        "corpus/passages/one.json",
        &manifest("source-one", "passage-one", 'b'),
    );
    fs::write(
        dir.path().join("groups.json"),
        br#"{"schema_version":1,"groups":[]}"#,
    )
    .unwrap();
    let first = commit(&repo);
    cli(
        &[
            "import",
            "--formal-logos",
            "upstream",
            "--revision",
            &first,
            "--work-groups",
            "groups.json",
        ],
        dir.path(),
    );
    (dir, repo, first)
}

fn freshness(dir: &Path) -> Value {
    cli(
        &[
            "freshness",
            "--formal-logos",
            "upstream",
            "--revision",
            "main",
            "--work-groups",
            "groups.json",
        ],
        dir,
    )
}

#[test]
fn freshness_distinguishes_revision_drift_from_broken_pinned_references() {
    let (dir, repo, first) = fixture();
    let catalog = dir.path().join("registry/catalog.json");
    let baseline = fs::read(&catalog).unwrap();
    let before = freshness(dir.path());
    assert_eq!(before["pinned_revision"], first);
    assert_eq!(before["target_revision"], first);
    assert_eq!(before["stale"], false);
    assert_eq!(before["broken_references"], false);
    assert_eq!(before["pinned_inventory"]["sources_without_passages"], 1);

    let original: Value = serde_json::from_slice(&baseline).unwrap();
    let edition = original["editions"][0]["id"].as_str().unwrap().to_owned();
    let mut modified = manifest("source-one", "passage-one", 'c');
    modified["passages"][0]["locator"] = "§1 revised".into();
    write(&repo, "corpus/passages/one.json", &modified);
    write(
        &repo,
        "corpus/passages/two.json",
        &manifest("source-two", "passage-two", 'b'),
    );
    let mut revised_source = source("source-one");
    revised_source["notes"] = "Updated editorial note; does not change its work identity".into();
    revised_source["edition_urns"] = json!(["urn:fixture:edition-one"]);
    write(&repo, "corpus/sources/one.json", &revised_source);
    write(
        &repo,
        "corpus/claims/claim.json",
        &json!({
            "id":"claim-one", "source_id":"source-one", "locator":"§1 reasoning",
            "attribution":"Synthetic attribution note"
        }),
    );
    let second = commit(&repo);
    let diff = freshness(dir.path());
    assert_eq!(diff["pinned_revision"], first);
    assert_eq!(diff["target_revision"], second);
    assert_eq!(diff["stale"], true);
    assert_eq!(diff["broken_references"], false);
    assert_eq!(
        diff["upstream_changes"]["source_records"]["changed"],
        json!(["source-one"])
    );
    assert_eq!(
        diff["upstream_changes"]["passage_manifests"]["added"],
        json!(["corpus/passages/two.json"])
    );
    assert_eq!(
        diff["upstream_changes"]["passage_manifests"]["changed"],
        json!(["corpus/passages/one.json"])
    );
    assert_eq!(
        diff["upstream_changes"]["witness_editions"]["changed"],
        json!(["corpus/passages/one.json"])
    );
    assert_eq!(
        diff["upstream_changes"]["passages"]["changed"],
        json!(["passage-one"])
    );
    assert_eq!(
        diff["upstream_changes"]["passages"]["added"],
        json!(["passage-two"])
    );
    assert_eq!(
        diff["upstream_changes"]["claim_attributions"]["added"],
        json!(["claim-one"])
    );
    assert_eq!(diff["pinned_inventory"]["witness_editions"], 1);
    assert_eq!(diff["target_inventory"]["witness_editions"], 2);
    assert_eq!(diff["target_inventory"]["sources_without_passages"], 0);
    assert_eq!(fs::read(&catalog).unwrap(), baseline);
    assert_eq!(cli(&["check"], dir.path())["referenced_editions"], 1);

    // A local acquisition and independent curated records are not importer-owned.
    fs::write(dir.path().join("local.pdf"), b"%PDF-synthetic").unwrap();
    cli(
        &[
            "add",
            "local.pdf",
            "--work-id",
            "local-work",
            "--title",
            "Locally catalogued work",
            "--edition-id",
            "local-edition",
            "--edition-label",
            "Local copy",
            "--language",
            "eng",
            "--license",
            "Synthetic fixture",
        ],
        dir.path(),
    );
    fs::create_dir_all(dir.path().join("registry")).unwrap();
    let sidecar = dir.path().join("registry/author-research.json");
    fs::write(&sidecar, br#"{"schema_version":1,"profiles":[]}"#).unwrap();
    let sidecar_before = fs::read(&sidecar).unwrap();
    cli(
        &[
            "import",
            "--formal-logos",
            "upstream",
            "--revision",
            &second,
            "--work-groups",
            "groups.json",
        ],
        dir.path(),
    );
    let refreshed: Value = serde_json::from_slice(&fs::read(&catalog).unwrap()).unwrap();
    assert!(
        refreshed["editions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e["id"] == "local-edition")
    );
    assert!(
        !refreshed["editions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e["id"] == edition)
    );
    assert_eq!(refreshed["formal_logos"]["revision"], second);
    assert_eq!(fs::read(&sidecar).unwrap(), sidecar_before);
    assert_eq!(
        cli(
            &["check", "--research", "registry/author-research.json"],
            dir.path()
        )["referenced_editions"],
        2
    );
    let checked = freshness(dir.path());
    assert_eq!(checked["stale"], false);
    assert_eq!(checked["broken_references"], false);
    let imported_once = fs::read(&catalog).unwrap();
    cli(
        &[
            "import",
            "--formal-logos",
            "upstream",
            "--revision",
            &second,
            "--work-groups",
            "groups.json",
        ],
        dir.path(),
    );
    assert_eq!(fs::read(&catalog).unwrap(), imported_once);

    // Removal is reported as staleness, not an invalid pinned Git URL.
    fs::remove_file(repo.join("corpus/sources/two.json")).unwrap();
    fs::remove_file(repo.join("corpus/passages/two.json")).unwrap();
    commit(&repo);
    let removed = freshness(dir.path());
    assert_eq!(
        removed["upstream_changes"]["source_records"]["removed"],
        json!(["source-two"])
    );
    assert_eq!(
        removed["upstream_changes"]["passages"]["removed"],
        json!(["passage-two"])
    );
    assert_eq!(removed["broken_references"], false);
}

#[test]
fn pinned_mismatch_is_reported_even_without_upstream_changes() {
    let (dir, _repo, _) = fixture();
    let path = dir.path().join("registry/catalog.json");
    let mut catalog: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    catalog["editions"][0]["formal_logos"]["passages"][0]["locator"] = "inaccurate locator".into();
    fs::write(&path, serde_json::to_vec_pretty(&catalog).unwrap()).unwrap();
    let report = freshness(dir.path());
    assert_eq!(report["stale"], false);
    assert_eq!(report["broken_references"], true);
    assert_eq!(
        report["pinned_catalogue_mismatches"]["passages"]["changed"],
        json!(["passage-one"])
    );
}

#[test]
fn missing_local_git_blob_fails_offline_without_mutating_catalogue() {
    let (dir, repo, _) = fixture();
    let catalog = dir.path().join("registry/catalog.json");
    let original = fs::read(&catalog).unwrap();
    let listing = success(
        "git",
        &["ls-tree", "HEAD", "--", "corpus/sources/one.json"],
        &repo,
    );
    let blob = listing.split_whitespace().nth(2).unwrap();
    fs::remove_file(repo.join(".git/objects").join(&blob[..2]).join(&blob[2..])).unwrap();
    let output = run(
        env!("CARGO_BIN_EXE_formal-logos-sources"),
        &[
            "freshness",
            "--formal-logos",
            "upstream",
            "--work-groups",
            "groups.json",
        ],
        dir.path(),
    );
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unavailable committed Git blob"));
    assert_eq!(fs::read(&catalog).unwrap(), original);
}

#[test]
fn connections_keep_historical_revision_links_after_main_moves() {
    let (dir, repo, first) = fixture();
    // No claims in this fixture, so the connection view contains only the pinned passage.
    let first_report = cli(
        &[
            "connections",
            "--formal-logos",
            "upstream",
            "--source",
            "source-one",
        ],
        dir.path(),
    );
    assert_eq!(first_report["revision"], first);
    assert!(
        first_report["passages"][0]["passage"]["url"]
            .as_str()
            .unwrap()
            .contains(&first)
    );
    let changed = manifest("source-one", "passage-one", 'd');
    write(&repo, "corpus/passages/one.json", &changed);
    commit(&repo);
    let after = cli(
        &[
            "connections",
            "--formal-logos",
            "upstream",
            "--source",
            "source-one",
        ],
        dir.path(),
    );
    assert_eq!(after, first_report);
}
