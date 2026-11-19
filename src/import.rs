//! Import bibliography and witness references from committed Formal Logos metadata.
//!
//! This never acquires source bytes or copies a philosophical research database.
use crate::formal_logos::{Record, Repository};
use crate::hash;
use crate::registry::{
    self, Author, Catalog, CorpusBinding, Edition, EditionReference, PassageReference,
    SourceReference, Work,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const REPOSITORY: &str = "moritzbrantner/formal-logos";
const SELECTORS: &[&str] = &[
    "cts_path",
    "tei_xml_id",
    "html_selector",
    "pdf_pages",
    "raw_text",
    "start_seconds",
    "end_seconds",
    "pages",
    "section",
    "segment_count",
    "span_sha256",
    "text_sha256",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkGroups {
    schema_version: u32,
    groups: Vec<WorkGroup>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkGroup {
    id: String,
    title: String,
    source_ids: Vec<String>,
    reason: String,
}

impl Default for WorkGroups {
    fn default() -> Self {
        Self {
            schema_version: 1,
            groups: Vec::new(),
        }
    }
}

pub fn read_groups(path: Option<&Path>) -> Result<WorkGroups, String> {
    match path {
        Some(path) => {
            serde_json::from_slice(&fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?)
                .map_err(|e| format!("{}: {e}", path.display()))
        }
        None => Ok(WorkGroups::default()),
    }
}

#[derive(Debug, Serialize)]
pub struct ImportSummary {
    pub repository: String,
    pub revision: String,
    pub author_attributions: usize,
    pub works: usize,
    pub source_records: usize,
    pub witness_editions: usize,
    pub passages: usize,
    pub sources_without_passages: usize,
    pub source_bytes_acquired: usize,
}

fn text<'a>(value: &'a Value, key: &str, path: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| format!("{path}: missing or invalid {key}"))
}

fn strings(value: Option<&Value>, key: &str, path: &str) -> Result<Vec<String>, String> {
    let values = value
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{path}: invalid {key}"))?;
    values
        .iter()
        .map(|v| {
            v.as_str()
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .ok_or_else(|| format!("{path}: invalid {key} entry"))
        })
        .collect()
}

fn grouping<'a>(
    groups: &'a WorkGroups,
    source_ids: &BTreeSet<String>,
) -> Result<BTreeMap<String, &'a WorkGroup>, String> {
    if groups.schema_version != 1 {
        return Err("unsupported work grouping schema".into());
    }
    let mut result = BTreeMap::new();
    let mut group_ids = BTreeSet::new();
    for group in &groups.groups {
        if group.title.trim().is_empty()
            || group.reason.trim().is_empty()
            || group.source_ids.len() < 2
        {
            return Err(format!(
                "group {} needs a title, reason, and at least two sources",
                group.id
            ));
        }
        if !group_ids.insert(&group.id) {
            return Err(format!("duplicate group id: {}", group.id));
        }
        for source in &group.source_ids {
            if !source_ids.contains(source) {
                return Err(format!(
                    "group {} references missing source {source}",
                    group.id
                ));
            }
            if result.insert(source.clone(), group).is_some() {
                return Err(format!("source {source} appears in multiple groups"));
            }
        }
    }
    for source in source_ids {
        if !result.contains_key(source) && group_ids.contains(&format!("fl-work-{source}")) {
            return Err(format!(
                "a work group collides with ungrouped source {source}"
            ));
        }
    }
    Ok(result)
}

/// Build a derived catalogue in memory; no files or sources are fetched.
fn build(repository: &Repository, groups: &WorkGroups) -> Result<(Catalog, ImportSummary), String> {
    let sources = repository.records("sources")?;
    let manifests = repository.records("passages")?;
    build_records(&repository.revision, sources, manifests, groups)
}

fn build_records(
    revision: &str,
    sources: Vec<Record>,
    manifests: Vec<Record>,
    groups: &WorkGroups,
) -> Result<(Catalog, ImportSummary), String> {
    if sources.is_empty() {
        return Err("Formal Logos source inventory is empty".into());
    }
    let mut source_ids = BTreeSet::new();
    for source in &sources {
        let id = text(&source.value, "id", &source.path)?;
        if !source_ids.insert(id.to_owned()) {
            return Err(format!("duplicate Formal Logos source: {id}"));
        }
    }
    let grouped = grouping(groups, &source_ids)?;
    let mut authors = BTreeMap::new();
    let mut works: BTreeMap<String, Work> = BTreeMap::new();
    let mut source_works = BTreeMap::new();
    for source in &sources {
        let value = &source.value;
        let id = text(value, "id", &source.path)?;
        let name = text(value, "author", &source.path)?;
        // Preserve the complete attribution string, including joint/institutional names.
        let author_id = format!("fl-author-{}", hash(name.as_bytes()));
        authors.entry(author_id.clone()).or_insert_with(|| Author {
            id: author_id.clone(),
            name: name.to_owned(),
        });
        let title = text(value, "work", &source.path)?;
        let (work_id, work_title) = match grouped.get(id) {
            Some(group) => (group.id.clone(), group.title.clone()),
            None => (format!("fl-work-{id}"), title.to_owned()),
        };
        let work = works.entry(work_id.clone()).or_insert_with(|| Work {
            id: work_id.clone(),
            title: work_title,
            author_ids: Vec::new(),
            formal_logos_sources: Vec::new(),
        });
        if !work.author_ids.contains(&author_id) {
            work.author_ids.push(author_id);
        }
        work.formal_logos_sources.push(SourceReference {
            id: id.to_owned(),
            title: title.to_owned(),
            path: source.path.clone(),
            kind: text(value, "kind", &source.path)?.to_owned(),
            work_urn: text(value, "work_urn", &source.path)?.to_owned(),
            catalog_url: text(value, "catalog_url", &source.path)?.to_owned(),
            citation_system: text(value, "citation_system", &source.path)?.to_owned(),
        });
        source_works.insert(id.to_owned(), work_id);
    }
    let mut editions = Vec::new();
    let mut passage_ids = BTreeSet::new();
    let mut witnessed = BTreeSet::new();
    for manifest in manifests {
        let snapshot = manifest
            .value
            .get("snapshot")
            .filter(|v| v.is_object())
            .ok_or_else(|| format!("{}: missing snapshot", manifest.path))?;
        let source_id = text(snapshot, "source_id", &manifest.path)?;
        let work_id = source_works
            .get(source_id)
            .ok_or_else(|| format!("{}: unknown source {source_id}", manifest.path))?;
        let passages = manifest
            .value
            .get("passages")
            .and_then(Value::as_array)
            .filter(|v| !v.is_empty())
            .ok_or_else(|| format!("{}: no passages", manifest.path))?;
        let mut references = Vec::new();
        for passage in passages {
            let id = text(passage, "id", &manifest.path)?;
            if !passage_ids.insert(id.to_owned()) {
                return Err(format!("duplicate passage: {id}"));
            }
            let selectors = SELECTORS
                .iter()
                .filter_map(|key| passage.get(*key).map(|v| ((*key).to_owned(), v.clone())))
                .collect();
            references.push(PassageReference {
                id: id.to_owned(),
                label: text(passage, "label", &manifest.path)?.to_owned(),
                locator: text(passage, "locator", &manifest.path)?.to_owned(),
                claim_locator_prefixes: strings(
                    passage.get("claim_locator_prefixes"),
                    "claim_locator_prefixes",
                    &manifest.path,
                )?,
                selectors,
            });
        }
        references.sort_by(|a, b| a.id.cmp(&b.id));
        let format = match snapshot.get("kind").and_then(Value::as_str) {
            None => "tei",
            Some("media_span") => "media",
            Some("bibliographic_span") => "bibliographic",
            Some("web_snapshot") => {
                let formats: BTreeSet<&str> = passages
                    .iter()
                    .map(|p| {
                        if p.get("pdf_pages").is_some() {
                            "pdf"
                        } else if p.get("raw_text").is_some() {
                            "text"
                        } else {
                            "html"
                        }
                    })
                    .collect();
                if formats.len() != 1 {
                    return Err(format!("{}: mixed witness formats", manifest.path));
                }
                formats.into_iter().next().ok_or("missing witness format")?
            }
            Some(kind) => {
                return Err(format!(
                    "{}: unsupported witness kind {kind}",
                    manifest.path
                ));
            }
        };
        let reference = EditionReference {
            manifest_path: manifest.path.clone(),
            source_id: source_id.to_owned(),
            snapshot: snapshot.clone(),
            passages: references,
        };
        let mut edition = Edition {
            id: String::new(),
            work_id: work_id.clone(),
            label: snapshot
                .get("edition_urn")
                .and_then(Value::as_str)
                .or_else(|| {
                    snapshot
                        .get("translation")
                        .and_then(|t| t.get("edition"))
                        .and_then(Value::as_str)
                })
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    format!(
                        "{} — {}",
                        works[work_id].title,
                        manifest
                            .path
                            .trim_start_matches("corpus/passages/")
                            .trim_end_matches(".json")
                    )
                }),
            language: text(snapshot, "language", &manifest.path)?.to_owned(),
            license: snapshot
                .get("license")
                .and_then(Value::as_str)
                .unwrap_or("Rights not recorded in upstream witness metadata; not assessed here")
                .to_owned(),
            source_url: snapshot
                .get("source_url")
                .and_then(Value::as_str)
                .map(str::to_owned),
            format: format.to_owned(),
            sha256: None,
            formal_logos: Some(reference),
        };
        // An edition reference is immutable, including its upstream witness and selectors.
        edition.id = format!(
            "fl-edition-{}",
            hash(&serde_json::to_vec(&edition).map_err(|e| e.to_string())?)
        );
        editions.push(edition);
        witnessed.insert(source_id.to_owned());
    }
    for work in works.values_mut() {
        work.author_ids.sort();
        work.formal_logos_sources.sort_by(|a, b| a.id.cmp(&b.id));
    }
    editions.sort_by(|a, b| a.id.cmp(&b.id));
    let catalog = Catalog {
        schema_version: 2,
        authors: authors.into_values().collect(),
        works: works.into_values().collect(),
        editions,
        formal_logos: Some(CorpusBinding {
            repository: REPOSITORY.to_owned(),
            revision: revision.to_owned(),
        }),
    };
    catalog.validate()?;
    let summary = ImportSummary {
        repository: REPOSITORY.to_owned(),
        revision: revision.to_owned(),
        author_attributions: catalog.authors.len(),
        works: catalog.works.len(),
        source_records: source_ids.len(),
        witness_editions: catalog.editions.len(),
        passages: passage_ids.len(),
        sources_without_passages: source_ids.difference(&witnessed).count(),
        source_bytes_acquired: 0,
    };
    Ok((catalog, summary))
}

pub fn import(
    repository: &Repository,
    groups: &WorkGroups,
    catalog_path: &Path,
) -> Result<ImportSummary, String> {
    let (mut imported, summary) = build(repository, groups)?;
    registry::update_catalog(catalog_path, |current| {
        // Only replace this derived projection. Local PDF records remain owned locally.
        for work in &current.works {
            if work.formal_logos_sources.is_empty() {
                if imported.works.iter().any(|w| w.id == work.id) {
                    return Err(format!("import conflicts with local work {}", work.id));
                }
                imported.works.push(work.clone());
            }
        }
        for edition in &current.editions {
            if edition.formal_logos.is_none() {
                if !imported.works.iter().any(|w| w.id == edition.work_id) {
                    return Err(format!(
                        "import would orphan local edition {}; retain its work before refreshing",
                        edition.id
                    ));
                }
                if imported.editions.iter().any(|e| e.id == edition.id) {
                    return Err(format!(
                        "import conflicts with local edition {}",
                        edition.id
                    ));
                }
                imported.editions.push(edition.clone());
            }
        }
        let author_ids: BTreeSet<String> = imported
            .works
            .iter()
            .flat_map(|w| w.author_ids.clone())
            .collect();
        let previously_imported_authors: BTreeSet<&String> = current
            .works
            .iter()
            .filter(|w| !w.formal_logos_sources.is_empty())
            .flat_map(|w| w.author_ids.iter())
            .collect();
        for author in &current.authors {
            if !previously_imported_authors.contains(&author.id) || author_ids.contains(&author.id)
            {
                match imported.authors.iter().find(|a| a.id == author.id) {
                    Some(existing) if existing != author => {
                        return Err(format!("import conflicts with local author {}", author.id));
                    }
                    Some(_) => {}
                    None => imported.authors.push(author.clone()),
                }
            }
        }
        imported.authors.sort_by(|a, b| a.id.cmp(&b.id));
        imported.works.sort_by(|a, b| a.id.cmp(&b.id));
        imported.editions.sort_by(|a, b| a.id.cmp(&b.id));
        *current = imported;
        Ok(())
    })?;
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn source(id: &str, author: &str, title: &str) -> Record {
        Record {
            path: format!("corpus/sources/{id}.json"),
            value: json!({"id":id,"label":title,"kind":"primary_text","author":author,"work":title,"work_urn":format!("urn:example:{id}"),"catalog_url":"https://example.org/shared-bibliography","citation_system":"section"}),
        }
    }

    fn manifest(source: &str, name: &str, translated: bool) -> Record {
        let mut snapshot = json!({"source_id":source,"kind":"web_snapshot","source_url":"https://example.org/text","retrieved_at":"2026-11-19T00:00:00Z","sha256":format!("sha256:{}","a".repeat(64)),"language":"eng","encoding":"utf-8","license":"Synthetic fixture"});
        if translated {
            snapshot["translation"] =
                json!({"translator":"Example Translator","year":1900,"edition":"Translation"});
        }
        Record {
            path: format!("corpus/passages/{name}.json"),
            value: json!({"snapshot":snapshot,"passages":[{"id":name,"label":"Section 1","locator":"§1","claim_locator_prefixes":["§1"],"html_selector":{"element_id":"section-1"},"text_sha256":format!("sha256:{}","b".repeat(64))}]}),
        }
    }

    #[test]
    fn shared_urls_and_joint_attribution_do_not_merge_works() {
        let (catalog, summary) = build_records(
            &"a".repeat(40),
            vec![
                source("one", "First and Second", "One"),
                source("two", "Institute", "Two"),
            ],
            vec![],
            &WorkGroups::default(),
        )
        .unwrap();
        assert_eq!(summary.source_records, 2);
        assert_eq!(summary.sources_without_passages, 2);
        assert_eq!(catalog.works.len(), 2);
        assert!(catalog.authors.iter().any(|a| a.name == "First and Second"));
    }

    #[test]
    fn reviewed_work_group_preserves_translation_witnesses_and_source_ids() {
        let groups:WorkGroups=serde_json::from_value(json!({"schema_version":1,"groups":[{"id":"grouped","title":"One work","source_ids":["one","two"],"reason":"Original and translation of one explicitly identified work"}]})).unwrap();
        let (catalog, summary) = build_records(
            &"a".repeat(40),
            vec![
                source("one", "Author", "Original"),
                source("two", "Author", "Translation"),
            ],
            vec![
                manifest("one", "original", false),
                manifest("two", "translation", true),
            ],
            &groups,
        )
        .unwrap();
        assert_eq!(catalog.works.len(), 1);
        assert_eq!(catalog.works[0].formal_logos_sources.len(), 2);
        assert_eq!(catalog.editions.len(), 2);
        assert_eq!(summary.source_bytes_acquired, 0);
        assert!(catalog.editions.iter().all(|e| e.sha256.is_none()));
        assert_ne!(catalog.editions[0].id, catalog.editions[1].id);
        assert_eq!(
            catalog.editions[0].formal_logos.as_ref().unwrap().passages[0].selectors["html_selector"],
            json!({"element_id":"section-1"})
        );
    }

    #[test]
    fn rejects_unknown_group_sources_and_manifest_sources() {
        let groups:WorkGroups=serde_json::from_value(json!({"schema_version":1,"groups":[{"id":"grouped","title":"Work","source_ids":["one","missing"],"reason":"Example"}]})).unwrap();
        assert!(
            build_records(
                &"a".repeat(40),
                vec![source("one", "Author", "Work")],
                vec![],
                &groups
            )
            .unwrap_err()
            .contains("missing source")
        );
        assert!(
            build_records(
                &"a".repeat(40),
                vec![source("one", "Author", "Work")],
                vec![manifest("unknown", "passage", false)],
                &WorkGroups::default()
            )
            .unwrap_err()
            .contains("unknown source")
        );
    }
}
