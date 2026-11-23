//! Offline comparison of an imported catalogue with committed Formal Logos metadata.
//!
//! A changed upstream record is not a broken catalogue link: the pinned Git
//! revision remains a valid, independently inspectable historical reference.
use crate::formal_logos::{Record, Repository};
use crate::import::{self, WorkGroups};
use crate::registry::{Author, Catalog, Edition, PassageReference, SourceReference, Work};
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Serialize)]
pub struct Delta {
    pub added: Vec<String>,
    pub changed: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: usize,
}

impl Delta {
    fn between<T: PartialEq>(before: &BTreeMap<String, T>, after: &BTreeMap<String, T>) -> Self {
        let mut result = Self {
            added: Vec::new(),
            changed: Vec::new(),
            removed: Vec::new(),
            unchanged: 0,
        };
        for (key, value) in after {
            match before.get(key) {
                None => result.added.push(key.clone()),
                Some(old) if old == value => result.unchanged += 1,
                Some(_) => result.changed.push(key.clone()),
            }
        }
        for key in before.keys() {
            if !after.contains_key(key) {
                result.removed.push(key.clone());
            }
        }
        result
    }

    fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.changed.is_empty() || !self.removed.is_empty()
    }
}

#[derive(Debug, Serialize)]
pub struct Inventory {
    pub author_attributions: usize,
    pub works: usize,
    pub source_records: usize,
    pub witness_editions: usize,
    pub passages: usize,
    pub sources_without_passages: usize,
}

impl Inventory {
    fn of(catalog: &Catalog) -> Self {
        let sources: BTreeSet<_> = catalog
            .works
            .iter()
            .flat_map(|work| work.formal_logos_sources.iter().map(|source| source.id.as_str()))
            .collect();
        let referenced: BTreeSet<_> = catalog
            .editions
            .iter()
            .filter_map(|edition| edition.formal_logos.as_ref().map(|r| r.source_id.as_str()))
            .collect();
        Self {
            author_attributions: catalog.authors.len(),
            works: catalog.works.len(),
            source_records: sources.len(),
            witness_editions: catalog.editions.len(),
            passages: catalog
                .editions
                .iter()
                .filter_map(|edition| edition.formal_logos.as_ref())
                .map(|reference| reference.passages.len())
                .sum(),
            sources_without_passages: sources.difference(&referenced).count(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UpstreamChanges {
    /// Full source records, including edition_urns and non-projected notes.
    pub source_records: Delta,
    pub passage_manifests: Delta,
    /// Manifest path is the key: changing a witness identity is a revision, not a rename.
    pub witness_editions: Delta,
    /// Passage ID is the key; selectors, locators and prefixes are compared.
    pub passages: Delta,
    pub works: Delta,
    pub source_attributions: Delta,
    /// Canonical claim provenance fields only; philosophical content is not copied.
    pub claim_attributions: Delta,
}

impl UpstreamChanges {
    fn has_changes(&self) -> bool {
        [
            &self.source_records,
            &self.passage_manifests,
            &self.witness_editions,
            &self.passages,
            &self.works,
            &self.source_attributions,
            &self.claim_attributions,
        ]
        .iter()
        .any(|diff| diff.has_changes())
    }
}

#[derive(Debug, Serialize)]
pub struct PinnedCatalogueMismatches {
    pub source_references: Delta,
    pub works: Delta,
    pub witness_editions: Delta,
    pub passages: Delta,
    pub author_attributions: Delta,
}

impl PinnedCatalogueMismatches {
    fn has_changes(&self) -> bool {
        [
            &self.source_references,
            &self.works,
            &self.witness_editions,
            &self.passages,
            &self.author_attributions,
        ]
        .iter()
        .any(|diff| diff.has_changes())
    }
}

#[derive(Debug, Serialize)]
pub struct FreshnessReport {
    pub pinned_revision: String,
    pub target_revision: String,
    pub revision_differs: bool,
    /// Newer/different metadata is not in itself a broken historical reference.
    pub stale: bool,
    /// A mismatch against the *pinned* projection is distinct from staleness.
    pub broken_references: bool,
    pub pinned_inventory: Inventory,
    pub target_inventory: Inventory,
    pub upstream_changes: UpstreamChanges,
    pub pinned_catalogue_mismatches: PinnedCatalogueMismatches,
}

struct Snapshot {
    projected: Catalog,
    sources: BTreeMap<String, Value>,
    manifests: BTreeMap<String, Value>,
    source_attributions: BTreeMap<String, Value>,
    claim_attributions: BTreeMap<String, Value>,
}

fn id(record: &Record) -> Result<&str, String> {
    record
        .value
        .get("id")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{}: missing id", record.path))
}

fn records_by_id(records: &[Record]) -> Result<BTreeMap<String, Value>, String> {
    let mut mapped = BTreeMap::new();
    for record in records {
        let value = serde_json::json!({"path": record.path, "record": record.value});
        if mapped.insert(id(record)?.to_owned(), value).is_some() {
            return Err(format!("{}: duplicate record id", record.path));
        }
    }
    Ok(mapped)
}

fn records_by_path(records: &[Record]) -> BTreeMap<String, Value> {
    records
        .iter()
        .map(|record| (record.path.clone(), record.value.clone()))
        .collect()
}

fn snapshot(repository: &Repository, groups: &WorkGroups) -> Result<Snapshot, String> {
    let sources = repository.records("sources")?;
    let manifests = repository.records("passages")?;
    let claims = repository.records("claims")?;
    let (projected, _) = import::build_records(
        &repository.revision,
        sources.clone(),
        manifests.clone(),
        groups,
    )?;
    let mut source_attributions = BTreeMap::new();
    for source in &sources {
        source_attributions.insert(
            id(source)?.to_owned(),
            source
                .value
                .get("author")
                .cloned()
                .ok_or_else(|| format!("{}: missing author", source.path))?,
        );
    }
    let mut claim_attributions = BTreeMap::new();
    for claim in &claims {
        let mut fields = Map::new();
        fields.insert("path".into(), Value::String(claim.path.clone()));
        for key in [
            "source_id",
            "locator",
            "passage_id",
            "basis",
            "witness",
            "attribution",
        ] {
            if let Some(value) = claim.value.get(key) {
                fields.insert(key.into(), value.clone());
            }
        }
        if claim_attributions
            .insert(id(claim)?.to_owned(), Value::Object(fields))
            .is_some()
        {
            return Err(format!("{}: duplicate claim id", claim.path));
        }
    }
    Ok(Snapshot {
        projected,
        sources: records_by_id(&sources)?,
        manifests: records_by_path(&manifests),
        source_attributions,
        claim_attributions,
    })
}

fn sources(catalog: &Catalog) -> BTreeMap<String, SourceReference> {
    catalog
        .works
        .iter()
        .flat_map(|work| work.formal_logos_sources.iter())
        .map(|source| (source.id.clone(), source.clone()))
        .collect()
}

fn works(catalog: &Catalog) -> BTreeMap<String, Work> {
    catalog
        .works
        .iter()
        .filter(|work| !work.formal_logos_sources.is_empty())
        .map(|work| (work.id.clone(), work.clone()))
        .collect()
}

fn editions(catalog: &Catalog) -> Result<BTreeMap<String, Edition>, String> {
    let mut result = BTreeMap::new();
    for edition in &catalog.editions {
        if let Some(reference) = &edition.formal_logos
            && result
                .insert(reference.manifest_path.clone(), edition.clone())
                .is_some()
        {
            return Err(format!(
                "duplicate referenced witness manifest: {}",
                reference.manifest_path
            ));
        }
    }
    Ok(result)
}

fn passages(catalog: &Catalog) -> BTreeMap<String, PassageReference> {
    catalog
        .editions
        .iter()
        .filter_map(|edition| edition.formal_logos.as_ref())
        .flat_map(|reference| reference.passages.iter())
        .map(|passage| (passage.id.clone(), passage.clone()))
        .collect()
}

fn authors(catalog: &Catalog) -> BTreeMap<String, Author> {
    catalog
        .authors
        .iter()
        .map(|author| (author.id.clone(), author.clone()))
        .collect()
}

/// Read only local Git objects. Does not mutate the catalogue, sidecars or worktree.
pub fn compare(
    catalog: &Catalog,
    pinned: &Repository,
    target: &Repository,
    groups: &WorkGroups,
) -> Result<FreshnessReport, String> {
    let baseline = snapshot(pinned, groups)?;
    let latest = snapshot(target, groups)?;
    let expected_authors = authors(&baseline.projected);
    let actual_authors = authors(catalog)
        .into_iter()
        .filter(|(id, _)| expected_authors.contains_key(id))
        .collect();
    let pinned_catalogue_mismatches = PinnedCatalogueMismatches {
        source_references: Delta::between(&sources(&baseline.projected), &sources(catalog)),
        works: Delta::between(&works(&baseline.projected), &works(catalog)),
        witness_editions: Delta::between(&editions(&baseline.projected)?, &editions(catalog)?),
        passages: Delta::between(&passages(&baseline.projected), &passages(catalog)),
        author_attributions: Delta::between(&expected_authors, &actual_authors),
    };
    let upstream_changes = UpstreamChanges {
        source_records: Delta::between(&baseline.sources, &latest.sources),
        passage_manifests: Delta::between(&baseline.manifests, &latest.manifests),
        witness_editions: Delta::between(
            &editions(&baseline.projected)?,
            &editions(&latest.projected)?,
        ),
        passages: Delta::between(&passages(&baseline.projected), &passages(&latest.projected)),
        works: Delta::between(&works(&baseline.projected), &works(&latest.projected)),
        source_attributions: Delta::between(
            &baseline.source_attributions,
            &latest.source_attributions,
        ),
        claim_attributions: Delta::between(
            &baseline.claim_attributions,
            &latest.claim_attributions,
        ),
    };
    let revision_differs = pinned.revision != target.revision;
    Ok(FreshnessReport {
        pinned_revision: pinned.revision.clone(),
        target_revision: target.revision.clone(),
        revision_differs,
        stale: revision_differs || upstream_changes.has_changes(),
        broken_references: pinned_catalogue_mismatches.has_changes(),
        pinned_inventory: Inventory::of(&baseline.projected),
        target_inventory: Inventory::of(&latest.projected),
        upstream_changes,
        pinned_catalogue_mismatches,
    })
}
