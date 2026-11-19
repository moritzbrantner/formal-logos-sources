use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn run(args: &[&str], cwd: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_formal-logos-sources"))
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap()
}

fn succeeds(args: &[&str], cwd: &Path) -> String {
    let result = run(args, cwd);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap()
}

#[test]
fn research_cli_reads_curated_evidence_without_acquiring_or_mutating_sources() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("registry")).unwrap();
    let catalogue = json!({
        "schema_version": 2,
        "authors": [
            {"id": "joint-attribution", "name": "First and Second"},
            {"id": "unprofiled-attribution", "name": "Unclassified writer"}
        ],
        "works": [{"id": "letters", "title": "Letters", "author_ids": ["joint-attribution"]}],
        "editions": []
    });
    let evidence = json!({
        "url": "https://example.org/archive", "title": "Synthetic archive record",
        "checked_on": "2026-11-19", "note": "Fixture records a death in 1716."
    });
    let research = json!({"schema_version": 1, "profiles": [{
        "id": "first", "name": "First Writer", "aliases": ["First"],
        "attributions": [{"attribution_id": "joint-attribution", "relation": "coauthor"}],
        "life": {"status": "deceased", "death_year": 1716, "evidence": [evidence.clone()]},
        "scope_note": "One selected work; bibliography not reconciled.",
        "inventories": [{
            "id": "archive", "title": "Archive inventory", "url": "https://example.org/archive",
            "scope": "Known letters", "publication_status": "ongoing",
            "reconciliation_status": "not_started", "evidence": [evidence.clone()]
        }],
        "retrievals": [{
            "id": "letters-print", "work_id": "letters", "title": "Collected letters",
            "language": "eng", "format": "bibliographic", "url": "https://example.org/letters",
            "coverage": "Selected letters", "access": "catalogue_only",
            "rights": {"status": "not_established", "scope": "No reuse terms located.", "evidence": []},
            "evidence": [evidence], "next_step": "Locate the listed volume in a library."
        }]
    }]});
    let catalog_path = dir.path().join("registry/catalog.json");
    let research_path = dir.path().join("registry/author-research.json");
    let catalog_bytes = serde_json::to_vec(&catalogue).unwrap();
    let research_bytes = serde_json::to_vec(&research).unwrap();
    fs::write(&catalog_path, &catalog_bytes).unwrap();
    fs::write(&research_path, &research_bytes).unwrap();

    let checked: Value = serde_json::from_str(&succeeds(
        &["check", "--research", "registry/author-research.json"],
        dir.path(),
    ))
    .unwrap();
    assert_eq!(checked["research"]["profiles"], 1);
    let result = succeeds(&["research", "--author", "FIRST", "--deceased"], dir.path());
    let report: Value = serde_json::from_str(&result).unwrap();
    assert_eq!(report["total_attribution_strings"], 2);
    assert_eq!(report["mapped_attribution_strings"], 1);
    assert_eq!(report["unprofiled_attribution_strings"], 1);
    assert_eq!(report["selected_profiles"], 1);
    assert_eq!(report["profiles"][0]["profile"]["name"], "First Writer");
    assert_eq!(
        report["profiles"][0]["attributions"][0]["relation"],
        "coauthor"
    );
    assert_eq!(report["profiles"][0]["catalogued_works"], 1);
    assert_eq!(
        report["profiles"][0]["profile"]["retrievals"][0]["rights"]["status"],
        "not_established"
    );
    let markdown = succeeds(
        &["research", "--author", "first", "--format", "markdown"],
        dir.path(),
    );
    assert!(markdown.contains("https://example.org/letters"));
    assert!(markdown.contains("Locate the listed volume in a library."));
    assert!(markdown.contains("1716"));
    assert_eq!(fs::read(&catalog_path).unwrap(), catalog_bytes);
    assert_eq!(fs::read(&research_path).unwrap(), research_bytes);
    assert!(!dir.path().join("local").exists());

    for args in [
        vec!["research", "--author", "unknown"],
        vec!["research", "--research", "missing.json"],
        vec!["check", "--research", "missing.json"],
        vec!["research", "--catalog", "missing.json"],
    ] {
        let failed = run(&args, dir.path());
        assert!(!failed.status.success(), "unexpected success for {args:?}");
        assert!(failed.stdout.is_empty());
    }
}
