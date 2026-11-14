//! Git-trackable bibliographic metadata with local content-addressed original files.
use crate::hash;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Work {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edition {
    pub id: String,
    pub work_id: String,
    pub label: String,
    pub language: String,
    pub license: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    pub format: String,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub works: Vec<Work>,
    pub editions: Vec<Edition>,
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            schema_version: 1,
            works: Vec::new(),
            editions: Vec::new(),
        }
    }
}

impl Catalog {
    fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported catalog schema: {}",
                self.schema_version
            ));
        }
        let mut work_ids = HashSet::new();
        let mut edition_ids = HashSet::new();
        for work in &self.works {
            validate_id(&work.id)?;
            nonempty("work title", &work.title)?;
            if !work_ids.insert(&work.id) {
                return Err(format!("duplicate work id: {}", work.id));
            }
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
            if edition.format != "pdf" {
                return Err(format!(
                    "edition {} has unsupported format {}",
                    edition.id, edition.format
                ));
            }
            if !valid_sha256(&edition.sha256) {
                return Err(format!("edition {} has invalid sha256", edition.id));
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

fn valid_sha256(sha: &str) -> bool {
    sha.len() == 64
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

pub fn object_path(store: &Path, sha: &str) -> PathBuf {
    store
        .join("objects")
        .join("sha256")
        .join(&sha[..2])
        .join(sha)
}

fn checked_object(store: &Path, sha: &str) -> Result<Vec<u8>, String> {
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
    let sha = hash(pdf);
    let edition = Edition {
        id: fields.edition_id.to_owned(),
        work_id: fields.work_id.to_owned(),
        label: fields.label.to_owned(),
        language: fields.language.to_owned(),
        license: fields.license.to_owned(),
        source_url: fields.source_url.map(str::to_owned),
        format: "pdf".into(),
        sha256: sha.clone(),
    };
    let work = Work {
        id: fields.work_id.to_owned(),
        title: fields.title.to_owned(),
    };
    // Lock only catalog mutations. Readers always see a complete atomic catalog revision.
    let parent = catalog_path.parent().ok_or("catalog has no parent")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let lock_path = parent.join(".catalog.lock");
    let lock = OpenOptions::new()
        .create(true)
        .append(true)
        .open(lock_path)
        .map_err(|e| e.to_string())?;
    lock.lock().map_err(|e| e.to_string())?;
    let mut catalog = read_catalog(catalog_path)?;
    if let Some(existing) = catalog.works.iter().find(|item| item.id == work.id) {
        if existing != &work {
            return Err(format!("work {} already has different metadata", work.id));
        }
    } else {
        catalog.works.push(work);
    }
    if let Some(existing) = catalog.editions.iter().find(|item| item.id == edition.id) {
        if existing != &edition {
            return Err(format!(
                "edition {} is immutable; use a new edition id",
                edition.id
            ));
        }
    } else {
        catalog.editions.push(edition.clone());
    }
    catalog.works.sort_by(|a, b| a.id.cmp(&b.id));
    catalog.editions.sort_by(|a, b| a.id.cmp(&b.id));
    catalog.validate()?;

    let object = object_path(store, &sha);
    if object.exists() {
        checked_object(store, &sha)?;
    } else if let Err(error) = atomic_write(&object, pdf, true) {
        // Another process may have stored the identical bytes; never overwrite an object.
        if object.exists() {
            checked_object(store, &sha)?;
        } else {
            return Err(error);
        }
    }
    let serialized = serde_json::to_vec_pretty(&catalog).map_err(|e| e.to_string())?;
    atomic_write(
        catalog_path,
        &[serialized.as_slice(), b"\n"].concat(),
        false,
    )?;
    drop(lock);
    Ok(edition)
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
    let bytes = checked_object(store, &edition.sha256)?;
    Ok((edition, bytes))
}

pub fn verify(catalog_path: &Path, store: &Path) -> Result<usize, String> {
    let catalog = read_catalog(catalog_path)?;
    let mut verified = HashSet::new();
    for edition in catalog.editions {
        if verified.insert(edition.sha256.clone()) {
            checked_object(store, &edition.sha256)?;
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
        let object = object_path(&store, &edition.sha256);
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
            br#"{"schema_version":2,"works":[],"editions":[]}"#,
        )
        .unwrap();
        assert!(read_catalog(&catalog).unwrap_err().contains("schema"));
    }
}
