mod citations;
mod formal_logos;
mod import;
mod registry;
mod research;

use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn hash(input: &[u8]) -> String {
    let mut encoded = String::with_capacity(64);
    for byte in Sha256::digest(input) {
        write!(&mut encoded, "{byte:02x}").expect("writing to String cannot fail");
    }
    encoded
}

fn inspect(
    bytes: &[u8],
    first: u32,
    last: u32,
    soft_hyphens: bool,
) -> Result<serde_json::Value, String> {
    if first == 0 || last < first {
        return Err("pages must satisfy 1 <= first <= last".into());
    }
    if !bytes.starts_with(b"%PDF-") {
        return Err("input is not a PDF".into());
    }
    let mut child = Command::new("pdftotext")
        .args([
            "-f",
            &first.to_string(),
            "-l",
            &last.to_string(),
            "-enc",
            "UTF-8",
            "-",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Poppler pdftotext unavailable: {e}"))?;
    child
        .stdin
        .take()
        .ok_or("cannot pipe PDF to pdftotext")?
        .write_all(bytes)
        .map_err(|e| format!("PDF input failed: {e}"))?;
    let output = child
        .wait_with_output()
        .map_err(|e| format!("PDF extraction failed: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "PDF extraction failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let raw = String::from_utf8(output.stdout.clone())
        .map_err(|_| "PDF extractor returned invalid UTF-8")?;
    if output.stdout.iter().filter(|&&c| c == 12).count() != (last - first + 1) as usize {
        return Err("selected range exceeds available PDF pages".into());
    }
    // Match Truth Core's selected-page, non-OCR evidence boundary.
    let mut pages: Vec<String> = raw.split('\u{c}').map(ToString::to_string).collect();
    pages.pop(); // Poppler's terminal form feed.
    if pages.iter().any(|p| p.trim().is_empty()) {
        return Err("selected page has no extractable text; OCR is not supported".into());
    }
    let decoded = if soft_hyphens {
        join_soft_hyphens(&raw)
    } else {
        raw
    };
    let normalized = decoded.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return Err("PDF extraction produced empty text".into());
    }
    let excerpt: String = normalized.chars().take(80).collect();
    let incipit = excerpt
        .rsplit_once(' ')
        .map_or(excerpt.as_str(), |(head, _)| head);
    let mut selector = serde_json::json!({"first": first, "last": last});
    if soft_hyphens {
        selector["text_normalization"] = "join_soft_hyphens".into();
    }
    Ok(serde_json::json!({
        "sha256": hash(bytes), "text_sha256": hash(normalized.as_bytes()),
        "pdf_pages": selector, "incipit": incipit, "characters": normalized.chars().count()
    }))
}

fn join_soft_hyphens(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{ad}' {
            while matches!(chars.peek(), Some(' ' | '\t' | '\r' | '\n')) {
                chars.next();
            }
        } else {
            result.push(c);
        }
    }
    result
}

const USAGE: &str = "usage:
  formal-logos-sources inspect FILE --pages FIRST:LAST [--url URL] [--join-soft-hyphens]
  formal-logos-sources add FILE --work-id ID --title TITLE --edition-id ID --edition-label LABEL --language LANG --license TERMS [--url URL] [--catalog PATH] [--store DIR]
  formal-logos-sources list [--catalog PATH]
  formal-logos-sources check [--catalog PATH] [--research FILE]
  formal-logos-sources citations [--source DOI_OR_ARXIV_OR_URL_OR_CATALOG_ID] [--catalog PATH]
  formal-logos-sources authors [--name NAME] [--catalog PATH]
  formal-logos-sources research [--author ID_OR_NAME] [--deceased] [--format json|markdown] [--research FILE] [--catalog PATH]
  formal-logos-sources import --formal-logos DIR [--revision COMMIT] [--work-groups FILE] [--catalog PATH]
  formal-logos-sources connections --formal-logos DIR --source ID [--locator LOCATOR] [--format json|markdown] [--catalog PATH]
  formal-logos-sources verify [--catalog PATH] [--store DIR]
  formal-logos-sources prepare --edition-id ID --pages FIRST:LAST [--join-soft-hyphens] [--catalog PATH] [--store DIR]";

struct Options {
    positional: Vec<String>,
    fields: HashMap<String, String>,
    switches: HashSet<String>,
}

impl Options {
    fn parse(
        args: impl Iterator<Item = String>,
        allowed: &[&str],
        switches: &[&str],
    ) -> Result<Self, String> {
        let mut result = Self {
            positional: Vec::new(),
            fields: HashMap::new(),
            switches: HashSet::new(),
        };
        let mut args = args.peekable();
        while let Some(arg) = args.next() {
            if switches.contains(&arg.as_str()) {
                if !result.switches.insert(arg.clone()) {
                    return Err(format!("duplicate flag: {arg}"));
                }
            } else if allowed.contains(&arg.as_str()) {
                let value = args
                    .next()
                    .ok_or_else(|| format!("missing value for {arg}"))?;
                if value.starts_with("--") || result.fields.insert(arg.clone(), value).is_some() {
                    return Err(format!("missing or repeated value for {arg}"));
                }
            } else if arg.starts_with('-') {
                return Err(format!("unknown option: {arg}"));
            } else {
                result.positional.push(arg);
            }
        }
        Ok(result)
    }

    fn required(&self, key: &str) -> Result<&str, String> {
        self.fields
            .get(key)
            .map(String::as_str)
            .ok_or_else(|| format!("missing {key}"))
    }

    fn path(&self, key: &str, default: &str) -> PathBuf {
        PathBuf::from(self.fields.get(key).map(String::as_str).unwrap_or(default))
    }

    fn positional_file(&self) -> Result<&str, String> {
        if self.positional.len() != 1 {
            return Err("expected exactly one PDF file argument".into());
        }
        Ok(&self.positional[0])
    }

    fn no_positional(&self) -> Result<(), String> {
        if !self.positional.is_empty() {
            return Err("unexpected positional argument".into());
        }
        Ok(())
    }

    fn pages(&self) -> Result<(u32, u32), String> {
        let pages = self.required("--pages")?;
        let (first, last) = pages.split_once(':').ok_or("expected FIRST:LAST")?;
        Ok((
            first.parse::<u32>().map_err(|_| "invalid first page")?,
            last.parse::<u32>().map_err(|_| "invalid last page")?,
        ))
    }
}

fn show(value: &impl serde::Serialize) -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string_pretty(value).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn run_command(args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut args = args;
    let command = args.next().ok_or(USAGE)?;
    match command.as_str() {
        "inspect" => {
            let opts = Options::parse(args, &["--pages", "--url"], &["--join-soft-hyphens"])?;
            let filename = opts.positional_file()?;
            let (first, last) = opts.pages()?;
            let data =
                std::fs::read(Path::new(filename)).map_err(|e| format!("{filename}: {e}"))?;
            let mut result = inspect(
                &data,
                first,
                last,
                opts.switches.contains("--join-soft-hyphens"),
            )?;
            if let Some(url) = opts.fields.get("--url") {
                result["source_url"] = url.clone().into();
            }
            show(&result)
        }
        "add" => {
            let opts = Options::parse(
                args,
                &[
                    "--work-id",
                    "--title",
                    "--edition-id",
                    "--edition-label",
                    "--language",
                    "--license",
                    "--url",
                    "--catalog",
                    "--store",
                ],
                &[],
            )?;
            let filename = opts.positional_file()?;
            let data = std::fs::read(filename).map_err(|e| format!("{filename}: {e}"))?;
            let added = registry::add(
                &opts.path("--catalog", "registry/catalog.json"),
                &opts.path("--store", "local"),
                &data,
                registry::Add {
                    work_id: opts.required("--work-id")?,
                    title: opts.required("--title")?,
                    edition_id: opts.required("--edition-id")?,
                    label: opts.required("--edition-label")?,
                    language: opts.required("--language")?,
                    license: opts.required("--license")?,
                    source_url: opts.fields.get("--url").map(String::as_str),
                },
            )?;
            show(&added)
        }
        "list" => {
            let opts = Options::parse(args, &["--catalog"], &[])?;
            opts.no_positional()?;
            show(&registry::read_catalog(
                &opts.path("--catalog", "registry/catalog.json"),
            )?)
        }
        "check" => {
            let opts = Options::parse(args, &["--catalog", "--research"], &[])?;
            opts.no_positional()?;
            let path = opts.path("--catalog", "registry/catalog.json");
            if !path.is_file() {
                return Err(format!("catalogue does not exist: {}", path.display()));
            }
            let catalog = registry::read_catalog(&path)?;
            let mut result = serde_json::json!({
                "schema_version": catalog.schema_version,
                "authors": catalog.authors.len(),
                "works": catalog.works.len(),
                "editions": catalog.editions.len(),
                "research_citations": catalog.research_citations.len(),
                "local_pdf_editions": catalog.editions.iter().filter(|e|e.formal_logos.is_none()).count(),
                "referenced_editions": catalog.editions.iter().filter(|e|e.formal_logos.is_some()).count(),
                "formal_logos": catalog.formal_logos,
                "validation": "offline catalogue structure and references; source bytes not checked"
            });
            if let Some(path) = opts.fields.get("--research") {
                let research = research::read_research(Path::new(path), &catalog)?;
                result["research"] = serde_json::json!({
                    "profiles": research.profiles.len(),
                    "validation": "offline author, inventory, retrieval and evidence references"
                });
            }
            show(&result)
        }
        "citations" => {
            let opts = Options::parse(args, &["--catalog", "--source"], &[])?;
            opts.no_positional()?;
            let catalog = registry::read_catalog(&opts.path("--catalog", "registry/catalog.json"))?;
            let matches =
                citations::lookup(&catalog, opts.fields.get("--source").map(String::as_str));
            show(
                &serde_json::json!({"formal_logos": catalog.formal_logos, "evidence": "citations in research documents", "citations": matches}),
            )
        }
        "research" => {
            let opts = Options::parse(
                args,
                &["--catalog", "--research", "--author", "--format"],
                &["--deceased"],
            )?;
            opts.no_positional()?;
            let path = opts.path("--catalog", "registry/catalog.json");
            if !path.is_file() {
                return Err(format!("catalogue does not exist: {}", path.display()));
            }
            let catalog = registry::read_catalog(&path)?;
            let research = research::read_research(
                &opts.path("--research", "registry/author-research.json"),
                &catalog,
            )?;
            let report = research::report(
                &research,
                &catalog,
                opts.fields.get("--author").map(String::as_str),
                opts.switches.contains("--deceased"),
            )?;
            match opts
                .fields
                .get("--format")
                .map(String::as_str)
                .unwrap_or("json")
            {
                "json" => show(&report),
                "markdown" => {
                    print!("{}", research::render_markdown(&report));
                    Ok(())
                }
                _ => Err("--format must be json or markdown".into()),
            }
        }
        "authors" => {
            let opts = Options::parse(args, &["--catalog", "--name"], &[])?;
            opts.no_positional()?;
            let catalog = registry::read_catalog(&opts.path("--catalog", "registry/catalog.json"))?;
            let filter = opts.fields.get("--name").map(|s| s.to_lowercase());
            let mut authors: Vec<_> = catalog
                .authors
                .iter()
                .filter(|a| {
                    filter
                        .as_ref()
                        .is_none_or(|q| a.name.to_lowercase() == *q || a.id == *q)
                })
                .collect();
            authors.sort_by(|a, b| a.name.cmp(&b.name));
            if filter.is_some() && authors.is_empty() {
                return Err("unknown author attribution".into());
            }
            let rows: Vec<_> = authors.into_iter().map(|author| {
                let works: Vec<_> = catalog.works.iter().filter(|w|w.author_ids.contains(&author.id)).collect();
                let editions: Vec<_> = catalog.editions.iter().filter(|e|works.iter().any(|w|w.id==e.work_id)).collect();
                serde_json::json!({
                    "id": author.id, "name": author.name,
                    "works": works,
                    "witness_editions": editions.len(),
                    "registered_passages": editions.iter().filter_map(|e|e.formal_logos.as_ref()).map(|r|r.passages.len()).sum::<usize>()
                })
            }).collect();
            show(
                &serde_json::json!({"formal_logos":catalog.formal_logos,"author_attributions":rows}),
            )
        }
        "import" => {
            let opts = Options::parse(
                args,
                &["--formal-logos", "--revision", "--work-groups", "--catalog"],
                &[],
            )?;
            opts.no_positional()?;
            let repository = formal_logos::Repository::open(
                Path::new(opts.required("--formal-logos")?),
                opts.fields
                    .get("--revision")
                    .map(String::as_str)
                    .unwrap_or("HEAD"),
            )?;
            let groups = import::read_groups(Some(
                &opts.path("--work-groups", "registry/work-groups.json"),
            ))?;
            show(&import::import(
                &repository,
                &groups,
                &opts.path("--catalog", "registry/catalog.json"),
            )?)
        }
        "connections" => {
            let opts = Options::parse(
                args,
                &[
                    "--formal-logos",
                    "--source",
                    "--locator",
                    "--format",
                    "--catalog",
                ],
                &[],
            )?;
            opts.no_positional()?;
            let catalog = registry::read_catalog(&opts.path("--catalog", "registry/catalog.json"))?;
            let binding = catalog
                .formal_logos
                .as_ref()
                .ok_or("catalogue has no imported Formal Logos revision")?;
            if binding.repository != "moritzbrantner/formal-logos" {
                return Err("unsupported Formal Logos repository binding".into());
            }
            let repository = formal_logos::Repository::open(
                Path::new(opts.required("--formal-logos")?),
                &binding.revision,
            )?;
            let report = formal_logos::connections(
                &repository,
                opts.required("--source")?,
                opts.fields.get("--locator").map(String::as_str),
            )?;
            if !catalog.works.iter().any(|w| {
                w.formal_logos_sources
                    .iter()
                    .any(|s| s.id == report.source.id)
            }) {
                return Err("source is absent from the imported catalogue".into());
            }
            match opts
                .fields
                .get("--format")
                .map(String::as_str)
                .unwrap_or("json")
            {
                "json" => show(&report),
                "markdown" => {
                    print!("{}", formal_logos::render_markdown(&report));
                    Ok(())
                }
                _ => Err("--format must be json or markdown".into()),
            }
        }
        "verify" => {
            let opts = Options::parse(args, &["--catalog", "--store"], &[])?;
            opts.no_positional()?;
            let count = registry::verify(
                &opts.path("--catalog", "registry/catalog.json"),
                &opts.path("--store", "local"),
            )?;
            show(&serde_json::json!({"verified_unique_objects": count}))
        }
        "prepare" => {
            let opts = Options::parse(
                args,
                &["--edition-id", "--pages", "--catalog", "--store"],
                &["--join-soft-hyphens"],
            )?;
            opts.no_positional()?;
            let (first, last) = opts.pages()?;
            let (edition, bytes) = registry::load_edition(
                &opts.path("--catalog", "registry/catalog.json"),
                &opts.path("--store", "local"),
                opts.required("--edition-id")?,
            )?;
            let mut result = inspect(
                &bytes,
                first,
                last,
                opts.switches.contains("--join-soft-hyphens"),
            )?;
            result["edition_id"] = edition.id.into();
            result["work_id"] = edition.work_id.into();
            result["language"] = edition.language.into();
            result["license"] = edition.license.into();
            result["local_object_verified"] = true.into();
            result["remote_witness_verified"] = false.into();
            if let Some(url) = edition.source_url {
                result["source_url"] = url.into();
            }
            show(&result)
        }
        _ => Err(USAGE.into()),
    }
}

fn main() {
    if let Err(error) = run_command(std::env::args().skip(1)) {
        eprintln!("error: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_preserves_sha256_lowercase_hex_vectors() {
        assert_eq!(
            hash(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hash(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    fn fixture_pdf() -> Vec<u8> {
        let mut objects = vec![
            "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
            "<< /Type /Pages /Kids [3 0 R 4 0 R] /Count 2 >>".to_string(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 5 0 R >> >> /Contents 6 0 R >>".to_string(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 5 0 R >> >> /Contents 7 0 R >>".to_string(),
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
        ];
        for value in ["First synthetic page.", "Second synthetic page."] {
            let stream = format!("BT /F1 12 Tf 72 720 Td ({value}) Tj ET");
            objects.push(format!(
                "<< /Length {} >>\nstream\n{stream}\nendstream",
                stream.len()
            ));
        }
        let mut data = b"%PDF-1.4\n".to_vec();
        let mut offsets = Vec::new();
        for (index, object) in objects.iter().enumerate() {
            offsets.push(data.len());
            data.extend_from_slice(format!("{} 0 obj\n{object}\nendobj\n", index + 1).as_bytes());
        }
        let start = data.len();
        data.extend_from_slice(b"xref\n0 8\n0000000000 65535 f \n");
        for offset in offsets {
            data.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        data.extend_from_slice(
            format!("trailer\n<< /Size 8 /Root 1 0 R >>\nstartxref\n{start}\n%%EOF\n").as_bytes(),
        );
        data
    }

    #[test]
    fn normalization() {
        assert_eq!(join_soft_hyphens("Al\u{ad} pha"), "Alpha");
        assert_eq!(join_soft_hyphens("Al- pha"), "Al- pha");
    }

    #[test]
    fn invalid_pdf_and_range() {
        assert!(inspect(b"not PDF", 1, 1, false).is_err());
        assert!(inspect(b"%PDF-", 0, 1, false).is_err());
        assert!(inspect(b"%PDF-", 2, 1, false).is_err());
    }

    #[test]
    fn extracts_real_pdf_page_with_truth_core_hash_semantics() {
        let data = fixture_pdf();
        let result = inspect(&data, 2, 2, false).unwrap();
        assert_eq!(result["sha256"], hash(&data));
        assert_eq!(result["text_sha256"], hash(b"Second synthetic page."));
        assert_eq!(
            result["pdf_pages"],
            serde_json::json!({"first": 2, "last": 2})
        );
        assert_eq!(result["incipit"], "Second synthetic");
        assert!(inspect(&data, 1, 3, false).is_err());
    }

    #[test]
    fn registered_pdf_can_be_prepared_from_verified_local_object() {
        let dir = tempfile::tempdir().unwrap();
        let catalog = dir.path().join("registry/catalog.json");
        let store = dir.path().join("local");
        let pdf = fixture_pdf();
        let fields = registry::Add {
            work_id: "fixture",
            title: "Synthetic Work",
            edition_id: "fixture-first",
            label: "First synthetic edition",
            language: "eng",
            license: "Synthetic fixture",
            source_url: None,
        };
        registry::add(&catalog, &store, &pdf, fields).unwrap();
        let (edition, loaded_pdf) =
            registry::load_edition(&catalog, &store, "fixture-first").unwrap();
        let witness = inspect(&loaded_pdf, 1, 1, false).unwrap();
        assert_eq!(witness["sha256"].as_str(), edition.sha256.as_deref());
        assert_eq!(witness["text_sha256"], hash(b"First synthetic page."));
    }

    #[test]
    fn cli_rejects_missing_and_unknown_arguments() {
        assert!(run_command(["add".to_owned()].into_iter()).is_err());
        assert!(
            run_command(["verify".to_owned(), "--unrecognized".to_owned()].into_iter()).is_err()
        );
        assert!(run_command(["prepare".to_owned(), "extra".to_owned()].into_iter()).is_err());
    }
}
