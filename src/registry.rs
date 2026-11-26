//! Git-trackable bibliographic metadata with local content-addressed original files.
use crate::hash;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Author {
    pub id: String,
    /// The source's attribution, which may name a person, group, or institution.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReference {
    pub id: String,
    pub path: String,
    pub title: String,
    pub kind: String,
    pub work_urn: String,
    pub catalog_url: String,
    pub citation_system: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Work {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub formal_logos_sources: Vec<SourceReference>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PassageReference {
    pub id: String,
    pub label: String,
    pub locator: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claim_locator_prefixes: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub selectors: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditionReference {
    pub manifest_path: String,
    pub source_id: String,
    /// Upstream metadata only. This is not an acquired or verified local object.
    pub snapshot: serde_json::Value,
    pub passages: Vec<PassageReference>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorpusBinding {
    pub repository: String,
    pub revision: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edition {
    pub id: String,
    pub work_id: String,
    pub label: String,
    /// Conventional citation locator covered by a local edition, e.g. `I q.19`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    pub language: String,
    pub license: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    pub format: String,
    /// Character encoding of a local HTML page, needed to extract passages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formal_logos: Option<EditionReference>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    #[serde(default)]
    pub authors: Vec<Author>,
    pub works: Vec<Work>,
    pub editions: Vec<Edition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub research_citations: Vec<crate::citations::Citation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formal_logos: Option<CorpusBinding>,
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            schema_version: 2,
            authors: Vec::new(),
            works: Vec::new(),
            editions: Vec::new(),
            research_citations: Vec::new(),
            formal_logos: None,
        }
    }
}

impl Catalog {
    pub(crate) fn sort(&mut self) {
        self.authors.sort_by(|a, b| a.id.cmp(&b.id));
        self.works.sort_by(|a, b| a.id.cmp(&b.id));
        self.editions.sort_by(|a, b| a.id.cmp(&b.id));
        // Author order is a byline and is preserved; the importer orders its own works.
        for work in &mut self.works {
            work.formal_logos_sources.sort_by(|a, b| a.id.cmp(&b.id));
        }
        for edition in &mut self.editions {
            if let Some(reference) = &mut edition.formal_logos {
                reference.passages.sort_by(|a, b| a.id.cmp(&b.id));
                for passage in &mut reference.passages {
                    passage.claim_locator_prefixes.sort();
                }
            }
        }
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        crate::citations::validate(self)?;
        if !matches!(self.schema_version, 1 | 2) {
            return Err(format!(
                "unsupported catalog schema: {}",
                self.schema_version
            ));
        }
        if let Some(binding) = &self.formal_logos {
            if binding.repository.split('/').count() != 2
                || binding.repository.split('/').any(|component| {
                    component.is_empty()
                        || matches!(component, "." | "..")
                        || !component.bytes().all(|byte| {
                            byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-')
                        })
                })
            {
                return Err("Formal Logos repository must be owner/name".into());
            }
            if !valid_hex(&binding.revision, 40) {
                return Err("Formal Logos revision must be a full lowercase Git SHA".into());
            }
        }
        let mut author_ids = HashSet::new();
        for author in &self.authors {
            validate_id(&author.id)?;
            nonempty("author name", &author.name)?;
            if !author_ids.insert(&author.id) {
                return Err(format!("duplicate author id: {}", author.id));
            }
        }
        let mut work_ids = HashSet::new();
        let mut source_works = HashMap::new();
        let mut edition_ids = HashSet::new();
        let mut passage_ids = HashSet::new();
        for work in &self.works {
            validate_id(&work.id)?;
            nonempty("work title", &work.title)?;
            if !work_ids.insert(&work.id) {
                return Err(format!("duplicate work id: {}", work.id));
            }
            let mut seen_authors = HashSet::new();
            for author_id in &work.author_ids {
                if !author_ids.contains(author_id) {
                    return Err(format!(
                        "work {} references unknown author {author_id}",
                        work.id
                    ));
                }
                if !seen_authors.insert(author_id) {
                    return Err(format!("work {} repeats author {author_id}", work.id));
                }
            }
            for source in &work.formal_logos_sources {
                validate_id(&source.id)?;
                validate_reference_path(&source.path, "corpus/sources/")?;
                for (name, value) in [
                    ("source title", &source.title),
                    ("source kind", &source.kind),
                    ("work URN", &source.work_urn),
                    ("catalog URL", &source.catalog_url),
                    ("citation system", &source.citation_system),
                ] {
                    nonempty(name, value)?;
                }
                if source_works.insert(&source.id, &work.id).is_some() {
                    return Err(format!("duplicate Formal Logos source id: {}", source.id));
                }
            }
        }
        if !source_works.is_empty() && self.formal_logos.is_none() {
            return Err("Formal Logos source references require a corpus revision binding".into());
        }
        for edition in &self.editions {
            validate_id(&edition.id)?;
            if !edition_ids.insert(&edition.id) {
                return Err(format!("duplicate edition id: {}", edition.id));
            }
            if !work_ids.contains(&edition.work_id) {
                return Err(format!(
                    "edition {} references unknown work {}",
                    edition.id, edition.work_id
                ));
            }
            for (name, value) in [
                ("edition label", &edition.label),
                ("language", &edition.language),
                ("license", &edition.license),
            ] {
                nonempty(name, value)?;
            }
            if !matches!(
                edition.format.as_str(),
                "pdf" | "tei" | "html" | "text" | "media" | "bibliographic"
            ) {
                return Err(format!(
                    "edition {} has unsupported format {}",
                    edition.id, edition.format
                ));
            }
            if let Some(locator) = &edition.locator {
                nonempty("edition locator", locator)?;
            }
            match (&edition.sha256, &edition.formal_logos) {
                (Some(sha), None) => {
                    let encoding_ok = match edition.format.as_str() {
                        "pdf" => edition.encoding.is_none(),
                        "html" => edition
                            .encoding
                            .as_deref()
                            .is_some_and(|e| nonempty("encoding", e).is_ok()),
                        _ => false,
                    };
                    if !encoding_ok || !valid_hex(sha, 64) {
                        return Err(format!(
                            "edition {} requires a PDF, or HTML with its encoding, and a valid sha256",
                            edition.id
                        ));
                    }
                }
                (None, Some(reference)) => {
                    if edition.encoding.is_some() || edition.locator.is_some() {
                        return Err(format!(
                            "referenced edition {} keeps locators and encoding in its upstream metadata",
                            edition.id
                        ));
                    }
                    if source_works.get(&reference.source_id).copied() != Some(&edition.work_id) {
                        return Err(format!(
                            "edition {} references a source outside its work: {}",
                            edition.id, reference.source_id
                        ));
                    }
                    validate_reference_path(&reference.manifest_path, "corpus/passages/")?;
                    validate_snapshot(reference, &edition.format)?;
                    if reference.passages.is_empty() {
                        return Err(format!("edition {} has no passage references", edition.id));
                    }
                    for passage in &reference.passages {
                        validate_id(&passage.id)?;
                        nonempty("passage label", &passage.label)?;
                        nonempty("passage locator", &passage.locator)?;
                        if !passage_ids.insert(&passage.id) {
                            return Err(format!("duplicate passage id: {}", passage.id));
                        }
                        for prefix in &passage.claim_locator_prefixes {
                            if prefix.trim().is_empty() {
                                return Err(format!(
                                    "passage {} has an empty locator prefix",
                                    passage.id
                                ));
                            }
                        }
                        for (key, value) in &passage.selectors {
                            if !matches!(
                                key.as_str(),
                                "cts_path"
                                    | "tei_xml_id"
                                    | "html_selector"
                                    | "pdf_pages"
                                    | "raw_text"
                                    | "start_seconds"
                                    | "end_seconds"
                                    | "pages"
                                    | "section"
                                    | "segment_count"
                                    | "span_sha256"
                                    | "text_sha256"
                            ) || value.is_null()
                            {
                                return Err(format!(
                                    "passage {} has unsupported retrieval metadata: {key}",
                                    passage.id
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "edition {} must identify either a local PDF or a Formal Logos reference",
                        edition.id
                    ));
                }
            }
            if let Some(url) = &edition.source_url
                && !(url.starts_with("https://") || url.starts_with("http://"))
            {
                return Err(format!(
                    "edition {} source_url must be an HTTP(S) URL",
                    edition.id
                ));
            }
        }
        Ok(())
    }
}

fn validate_reference_path(path: &str, prefix: &str) -> Result<(), String> {
    if !path.starts_with(prefix)
        || !path.ends_with(".json")
        || path.contains('\\')
        || path.chars().any(char::is_control)
        || path
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(format!("invalid repository reference path: {path}"));
    }
    Ok(())
}

fn validate_snapshot(reference: &EditionReference, format: &str) -> Result<(), String> {
    let snapshot = reference
        .snapshot
        .as_object()
        .ok_or("snapshot must be an object")?;
    if snapshot
        .get("source_id")
        .and_then(serde_json::Value::as_str)
        != Some(reference.source_id.as_str())
    {
        return Err("snapshot source_id does not match its edition reference".into());
    }
    if snapshot.contains_key("text") || snapshot.contains_key("transcript") {
        return Err("source references must not contain source text or transcripts".into());
    }
    let kind = snapshot.get("kind").and_then(serde_json::Value::as_str);
    let correct_format = match kind {
        None => format == "tei",
        Some("web_snapshot") => matches!(format, "pdf" | "html" | "text"),
        Some("media_span") => format == "media",
        Some("bibliographic_span") => format == "bibliographic",
        Some(other) => return Err(format!("unsupported Formal Logos witness kind: {other}")),
    };
    if !correct_format {
        return Err("edition format does not match its referenced witness kind".into());
    }
    Ok(())
}

fn nonempty(name: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.trim() != value {
        return Err(format!("{name} must be a nonempty, trimmed value"));
    }
    Ok(())
}

fn validate_id(id: &str) -> Result<(), String> {
    if id.len() > 128
        || id.is_empty()
        || !id
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        || !id.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-')
        })
    {
        return Err(format!(
            "invalid id '{id}' (use lowercase ASCII letters, digits, '.', '_' or '-')"
        ));
    }
    Ok(())
}

fn valid_hex(sha: &str, length: usize) -> bool {
    sha.len() == length
        && sha
            .bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

pub fn read_catalog(path: &Path) -> Result<Catalog, String> {
    let catalog = match fs::read(path) {
        Ok(bytes) => {
            serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))?
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Catalog::default(),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    catalog.validate()?;
    Ok(catalog)
}

/// Keep the read/modify/write transaction under the same lock as local PDF imports.
pub fn update_catalog<T>(
    path: &Path,
    update: impl FnOnce(&mut Catalog) -> Result<T, String>,
) -> Result<T, String> {
    let parent = path.parent().ok_or("catalog has no parent")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let lock = OpenOptions::new()
        .create(true)
        .append(true)
        .open(parent.join(".catalog.lock"))
        .map_err(|e| e.to_string())?;
    lock.lock().map_err(|e| e.to_string())?;
    let mut catalog = read_catalog(path)?;
    catalog.sort();
    let previous_editions = catalog.editions.clone();
    let result = update(&mut catalog)?;
    catalog.schema_version = 2;
    catalog.sort();
    catalog.validate()?;
    for previous in previous_editions {
        if let Some(current) = catalog
            .editions
            .iter()
            .find(|edition| edition.id == previous.id)
            && current != &previous
        {
            return Err(format!(
                "edition {} is immutable; use a new edition id",
                previous.id
            ));
        }
    }
    let serialized = serde_json::to_vec_pretty(&catalog).map_err(|e| e.to_string())?;
    atomic_write(path, &[serialized.as_slice(), b"\n"].concat(), false)?;
    drop(lock);
    Ok(result)
}

pub fn object_path(store: &Path, sha: &str) -> PathBuf {
    store
        .join("objects")
        .join("sha256")
        .join(&sha[..2])
        .join(sha)
}

pub(crate) fn checked_object(store: &Path, sha: &str) -> Result<Vec<u8>, String> {
    let path = object_path(store, sha);
    let bytes = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    if hash(&bytes) != sha {
        return Err(format!("object integrity mismatch: {}", path.display()));
    }
    Ok(bytes)
}

fn atomic_write(path: &Path, bytes: &[u8], no_clobber: bool) -> Result<(), String> {
    let parent = path.parent().ok_or("target has no parent")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let mut tmp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    tmp.write_all(bytes).map_err(|e| e.to_string())?;
    tmp.as_file().sync_all().map_err(|e| e.to_string())?;
    if no_clobber {
        tmp.persist_noclobber(path).map_err(|e| e.to_string())?;
    } else {
        tmp.persist(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub struct Add<'a> {
    pub work_id: &'a str,
    pub title: &'a str,
    pub edition_id: &'a str,
    pub label: &'a str,
    pub language: &'a str,
    pub license: &'a str,
    pub source_url: Option<&'a str>,
}

pub fn add(
    catalog_path: &Path,
    store: &Path,
    pdf: &[u8],
    fields: Add<'_>,
) -> Result<Edition, String> {
    if !pdf.starts_with(b"%PDF-") {
        return Err("input is not a PDF".into());
    }
    let mut added = add_local(
        catalog_path,
        store,
        vec![LocalEdition {
            edition: local_edition(&fields, "pdf", None, None, pdf),
            work_title: Some(fields.title),
            bytes: pdf,
        }],
    )?;
    Ok(added.remove(0).0)
}

/// Metadata for one acquired local file. The SHA-256 is computed from `bytes`.
pub fn local_edition(
    fields: &Add<'_>,
    format: &str,
    encoding: Option<&str>,
    locator: Option<&str>,
    bytes: &[u8],
) -> Edition {
    Edition {
        id: fields.edition_id.to_owned(),
        work_id: fields.work_id.to_owned(),
        label: fields.label.to_owned(),
        locator: locator.map(str::to_owned),
        language: fields.language.to_owned(),
        license: fields.license.to_owned(),
        source_url: fields.source_url.map(str::to_owned),
        format: format.to_owned(),
        encoding: encoding.map(str::to_owned),
        sha256: Some(hash(bytes)),
        formal_logos: None,
    }
}

pub struct LocalEdition<'a> {
    pub edition: Edition,
    /// Create the work with this title if it is absent; `None` requires an existing work.
    pub work_title: Option<&'a str>,
    pub bytes: &'a [u8],
}

/// Register local editions and store their bytes in one catalogue transaction.
/// Returns each edition and whether it was newly added (`false`: already identical).
pub fn add_local(
    catalog_path: &Path,
    store: &Path,
    items: Vec<LocalEdition<'_>>,
) -> Result<Vec<(Edition, bool)>, String> {
    update_catalog(catalog_path, |catalog| {
        let mut results = Vec::new();
        for item in &items {
            let edition = &item.edition;
            if edition.sha256.as_deref() != Some(hash(item.bytes).as_str()) {
                return Err(format!(
                    "edition {} does not describe its bytes",
                    edition.id
                ));
            }
            match (
                catalog.works.iter().find(|w| w.id == edition.work_id),
                item.work_title,
            ) {
                (Some(existing), Some(title)) if existing.title != title => {
                    return Err(format!(
                        "work {} already has different metadata",
                        edition.work_id
                    ));
                }
                (Some(_), _) => {}
                (None, Some(title)) => catalog.works.push(Work {
                    id: edition.work_id.clone(),
                    title: title.to_owned(),
                    author_ids: Vec::new(),
                    formal_logos_sources: Vec::new(),
                }),
                (None, None) => return Err(format!("unknown work id: {}", edition.work_id)),
            }
            let added = match catalog.editions.iter().find(|e| e.id == edition.id) {
                Some(existing) if existing != edition => {
                    return Err(format!(
                        "edition {} is immutable; use a new edition id",
                        edition.id
                    ));
                }
                Some(_) => false,
                None => {
                    catalog.editions.push(edition.clone());
                    true
                }
            };
            results.push((edition.clone(), added));
        }
        catalog.validate()?;
        for item in &items {
            let sha = item.edition.sha256.as_deref().unwrap_or_default();
            let object = object_path(store, sha);
            if object.exists() {
                checked_object(store, sha)?;
            } else if let Err(error) = atomic_write(&object, item.bytes, true) {
                // Another process may have stored the identical bytes; never overwrite an object.
                if object.exists() {
                    checked_object(store, sha)?;
                } else {
                    return Err(error);
                }
            }
        }
        Ok(results)
    })
}

pub fn load_edition(
    catalog_path: &Path,
    store: &Path,
    id: &str,
) -> Result<(Edition, Vec<u8>), String> {
    let catalog = read_catalog(catalog_path)?;
    let edition = catalog
        .editions
        .into_iter()
        .find(|e| e.id == id)
        .ok_or_else(|| format!("unknown edition id: {id}"))?;
    let sha = edition.sha256.as_deref().ok_or_else(|| {
        format!(
            "edition {id} is a metadata-only Formal Logos reference; no local file has been acquired"
        )
    })?;
    let bytes = checked_object(store, sha)?;
    Ok((edition, bytes))
}

pub fn verify(catalog_path: &Path, store: &Path) -> Result<usize, String> {
    let catalog = read_catalog(catalog_path)?;
    let mut verified = HashSet::new();
    for edition in catalog.editions {
        if let Some(sha) = edition.sha256
            && verified.insert(sha.clone())
        {
            checked_object(store, &sha)?;
        }
    }
    Ok(verified.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields<'a>(edition_id: &'a str) -> Add<'a> {
        Add {
            work_id: "aristotle-physics",
            title: "Physics",
            edition_id,
            label: "Example first edition",
            language: "grc",
            license: "Synthetic fixture",
            source_url: Some("https://example.org/physics.pdf"),
        }
    }

    fn referenced_catalog() -> Catalog {
        Catalog {
            authors: vec![Author {
                id: "aquinas".into(),
                name: "Thomas Aquinas".into(),
            }],
            works: vec![Work {
                id: "summa".into(),
                title: "Summa theologiae".into(),
                author_ids: vec!["aquinas".into()],
                formal_logos_sources: vec![SourceReference {
                    id: "source-summa".into(),
                    path: "corpus/sources/summa.json".into(),
                    title: "Summa theologiae".into(),
                    kind: "primary_work".into(),
                    work_urn: "urn:example:summa".into(),
                    catalog_url: "https://example.org/summa".into(),
                    citation_system: "Part, question, article".into(),
                }],
            }],
            editions: vec![Edition {
                id: "summa-reference".into(),
                work_id: "summa".into(),
                label: "Synthetic Latin witness reference".into(),
                language: "lat".into(),
                license: "Synthetic metadata".into(),
                source_url: Some("https://example.org/summa.xml".into()),
                format: "tei".into(),
                locator: None,
                encoding: None,
                sha256: None,
                formal_logos: Some(EditionReference {
                    manifest_path: "corpus/passages/summa-q19.json".into(),
                    source_id: "source-summa".into(),
                    snapshot: serde_json::json!({
                        "source_id": "source-summa",
                        "repository": "example/texts",
                        "revision": "1111111111111111111111111111111111111111",
                        "language": "lat"
                    }),
                    passages: vec![PassageReference {
                        id: "passage-q19-a1".into(),
                        label: "I q.19 a.1".into(),
                        locator: "I q.19 a.1".into(),
                        claim_locator_prefixes: vec!["I q.19 a.1 ".into()],
                        selectors: BTreeMap::from([("tei_xml_id".into(), "q19-a1".into())]),
                    }],
                }),
            }],
            formal_logos: Some(CorpusBinding {
                repository: "example/formal-logos".into(),
                revision: "2222222222222222222222222222222222222222".into(),
            }),
            ..Catalog::default()
        }
    }

    #[test]
    fn metadata_only_entries_do_not_claim_local_acquisition() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry/catalog.json");
        let store = dir.path().join("local");
        let catalog = referenced_catalog();
        update_catalog(&path, |current| {
            *current = catalog.clone();
            Ok(())
        })
        .unwrap();
        assert_eq!(read_catalog(&path).unwrap(), catalog);
        assert_eq!(verify(&path, &store).unwrap(), 0);
        assert!(!store.exists());
        assert!(
            load_edition(&path, &store, "summa-reference")
                .unwrap_err()
                .contains("metadata-only")
        );

        add(
            &path,
            &store,
            b"%PDF-fixture",
            Add {
                work_id: "summa",
                title: "Summa theologiae",
                edition_id: "summa-local",
                ..fields("summa-local")
            },
        )
        .unwrap();
        assert_eq!(verify(&path, &store).unwrap(), 1);
        let updated = read_catalog(&path).unwrap();
        assert_eq!(updated.works, catalog.works);
        assert_eq!(updated.authors, catalog.authors);
        assert_eq!(updated.editions.len(), 2);
    }

    #[test]
    fn local_additions_preserve_existing_author_order() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry/catalog.json");
        update_catalog(&path, |catalog| {
            for id in ["second", "first"] {
                catalog.authors.push(Author {
                    id: id.into(),
                    name: id.into(),
                });
            }
            catalog.works.push(Work {
                id: "paper".into(),
                title: "Paper".into(),
                author_ids: vec!["second".into(), "first".into()],
                formal_logos_sources: Vec::new(),
            });
            Ok(())
        })
        .unwrap();
        add(
            &path,
            &dir.path().join("local"),
            b"%PDF-fixture",
            fields("x"),
        )
        .unwrap();
        let catalog = read_catalog(&path).unwrap();
        let paper = catalog.works.iter().find(|w| w.id == "paper").unwrap();
        assert_eq!(paper.author_ids, ["second", "first"]);
    }

    #[test]
    fn html_editions_need_an_encoding_and_pdfs_must_not_have_one() {
        let mut catalog = Catalog::default();
        catalog.works.push(Work {
            id: "work".into(),
            title: "Work".into(),
            author_ids: Vec::new(),
            formal_logos_sources: Vec::new(),
        });
        let html = local_edition(
            &fields("page"),
            "html",
            Some("utf-8"),
            Some("I q.1"),
            b"<p>",
        );
        let html = Edition {
            work_id: "work".into(),
            ..html
        };
        catalog.editions.push(html.clone());
        catalog.validate().unwrap();
        catalog.editions[0].encoding = None;
        assert!(catalog.validate().unwrap_err().contains("encoding"));
        catalog.editions[0] = Edition {
            format: "pdf".into(),
            ..html
        };
        assert!(catalog.validate().unwrap_err().contains("encoding"));
    }

    #[test]
    fn rejects_broken_or_misleading_metadata_references() {
        let catalog = referenced_catalog();
        catalog.validate().unwrap();
        let mut broken = catalog.clone();
        broken.authors.clear();
        assert!(broken.validate().unwrap_err().contains("unknown author"));
        let mut broken = catalog.clone();
        broken.works[0].formal_logos_sources[0].path = "corpus/sources/../secret.json".into();
        assert!(broken.validate().unwrap_err().contains("reference path"));
        let mut broken = catalog.clone();
        broken.formal_logos = None;
        assert!(broken.validate().unwrap_err().contains("revision binding"));
        let mut broken = catalog.clone();
        broken.editions[0].sha256 = Some("0".repeat(64));
        assert!(
            broken
                .validate()
                .unwrap_err()
                .contains("either a local PDF")
        );
        let mut broken = catalog.clone();
        broken.editions[0].formal_logos.as_mut().unwrap().snapshot["kind"] = "new-kind".into();
        assert!(broken.validate().unwrap_err().contains("witness kind"));
        let mut broken = catalog.clone();
        broken.editions[0].formal_logos.as_mut().unwrap().snapshot["text"] = "Copied text".into();
        assert!(broken.validate().unwrap_err().contains("source text"));
        let mut broken = catalog.clone();
        broken.editions[0].formal_logos.as_mut().unwrap().passages[0]
            .selectors
            .insert("quotation".into(), "Copied text".into());
        assert!(
            broken
                .validate()
                .unwrap_err()
                .contains("retrieval metadata")
        );
        let mut broken = catalog;
        let mut duplicate = broken.editions[0].clone();
        duplicate.id = "second-edition".into();
        broken.editions.push(duplicate);
        assert!(broken.validate().unwrap_err().contains("duplicate passage"));
    }

    #[test]
    fn metadata_updates_are_idempotent_and_preserve_immutable_editions() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry/catalog.json");
        update_catalog(&path, |current| {
            *current = referenced_catalog();
            Ok(())
        })
        .unwrap();
        let original = fs::read(&path).unwrap();
        update_catalog(&path, |_| Ok(())).unwrap();
        assert_eq!(fs::read(&path).unwrap(), original);
        assert!(
            update_catalog(&path, |current| {
                current.editions[0].label = "Different identity".into();
                Ok(())
            })
            .unwrap_err()
            .contains("immutable")
        );
        assert_eq!(fs::read(&path).unwrap(), original);
    }

    #[test]
    fn reads_existing_local_catalog_without_rewriting_identity() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry/catalog.json");
        let store = dir.path().join("local");
        let edition = add(&path, &store, b"%PDF-fixture", fields("original")).unwrap();
        let mut legacy: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        legacy["schema_version"] = 1.into();
        legacy.as_object_mut().unwrap().remove("authors");
        fs::write(&path, serde_json::to_vec(&legacy).unwrap()).unwrap();
        assert_eq!(read_catalog(&path).unwrap().editions, vec![edition.clone()]);
        update_catalog(&path, |_| Ok(())).unwrap();
        let updated = read_catalog(&path).unwrap();
        assert_eq!(updated.schema_version, 2);
        assert_eq!(updated.editions, vec![edition]);
        assert_eq!(verify(&path, &store).unwrap(), 1);
    }

    #[test]
    fn simultaneous_updates_retain_both_changes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry/catalog.json");
        let barrier = std::sync::Barrier::new(2);
        std::thread::scope(|scope| {
            for id in ["work-one", "work-two"] {
                let path = &path;
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    update_catalog(path, |catalog| {
                        catalog.works.push(Work {
                            id: id.into(),
                            title: id.into(),
                            author_ids: Vec::new(),
                            formal_logos_sources: Vec::new(),
                        });
                        Ok(())
                    })
                    .unwrap();
                });
            }
        });
        assert_eq!(read_catalog(&path).unwrap().works.len(), 2);
    }

    #[test]
    fn deduplicates_objects_and_rejects_identity_rewriting() {
        let dir = tempfile::tempdir().unwrap();
        let catalog = dir.path().join("registry/catalog.json");
        let store = dir.path().join("local");
        let bytes = b"%PDF-fixture";
        let first = add(&catalog, &store, bytes, fields("physics-1")).unwrap();
        add(&catalog, &store, bytes, fields("physics-1")).unwrap();
        let second = add(&catalog, &store, bytes, fields("physics-2")).unwrap();
        assert_eq!(first.sha256, second.sha256);
        assert_eq!(read_catalog(&catalog).unwrap().editions.len(), 2);
        assert_eq!(verify(&catalog, &store).unwrap(), 1);
        assert_eq!(
            load_edition(&catalog, &store, "physics-1").unwrap().1,
            bytes
        );

        let mut altered = fields("physics-1");
        altered.label = "Different edition";
        assert!(
            add(&catalog, &store, bytes, altered)
                .unwrap_err()
                .contains("immutable")
        );
        assert_eq!(read_catalog(&catalog).unwrap().editions.len(), 2);
        assert!(add(&catalog, &store, b"changed", fields("other")).is_err());
    }

    #[test]
    fn detects_corrupt_or_missing_objects_without_mutating_catalog() {
        let dir = tempfile::tempdir().unwrap();
        let catalog = dir.path().join("registry/catalog.json");
        let store = dir.path().join("local");
        let edition = add(&catalog, &store, b"%PDF-fixture", fields("physics-1")).unwrap();
        let object = object_path(&store, edition.sha256.as_deref().unwrap());
        fs::write(&object, b"corrupted").unwrap();
        assert!(
            verify(&catalog, &store)
                .unwrap_err()
                .contains("integrity mismatch")
        );
        assert!(add(&catalog, &store, b"%PDF-fixture", fields("physics-1")).is_err());
        assert_eq!(read_catalog(&catalog).unwrap().editions.len(), 1);
        fs::remove_file(object).unwrap();
        assert!(verify(&catalog, &store).is_err());
    }

    #[test]
    fn rejects_invalid_catalogs_and_ids() {
        let dir = tempfile::tempdir().unwrap();
        let catalog = dir.path().join("registry/catalog.json");
        let store = dir.path().join("local");
        let bad = Add {
            work_id: "../escape",
            ..fields("one")
        };
        assert!(add(&catalog, &store, b"%PDF-fixture", bad).is_err());
        fs::create_dir_all(catalog.parent().unwrap()).unwrap();
        fs::write(
            &catalog,
            br#"{"schema_version":3,"works":[],"editions":[]}"#,
        )
        .unwrap();
        assert!(read_catalog(&catalog).unwrap_err().contains("schema"));
    }
}
