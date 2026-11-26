//! Local HTML mirrors registered as content-addressed editions, and offline
//! comparison of their passages with imported Formal Logos `web_snapshot` pins.
use crate::hash;
use crate::html::{self, Selector};
use crate::registry::{self, Add, Catalog, LocalEdition};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// One page of New Advent's *Summa theologiae*, with its locator verified
/// against both the file number and the page title.
#[derive(Debug, PartialEq, Eq)]
pub struct SummaPage {
    pub number: u32,
    pub locator: String,
    pub question_title: String,
    pub online_edition_year: String,
}

const SUMMA_PARTS: &[(char, &str, &str)] = &[
    ('1', "Prima Pars", "I"),
    ('2', "Prima Secundae Partis", "I-II"),
    ('3', "Secunda Secundae Partis", "II-II"),
    ('4', "Tertia Pars", "III"),
    ('5', "Supplementum", "Suppl."),
    ('6', "Appendix I", "Suppl. App. I"),
    ('7', "Appendix II", "Suppl. App. II"),
];
const SUMMA_TITLE_PREFIX: &str = "SUMMA THEOLOGICA: ";
const SUMMA_EDITION_LINES: &[&str] = &[
    "Second and Revised Edition, 1920",
    "Literally translated by Fathers of the English Dominican Province",
];

/// `NNNN.htm` with a part digit and a three-digit question number; other files
/// (part and site indexes) are not question pages.
pub fn summa_page_number(file_name: &str) -> Option<u32> {
    let stem = file_name.strip_suffix(".htm")?;
    (stem.len() == 4 && stem.bytes().all(|b| b.is_ascii_digit()))
        .then(|| stem.parse().ok())
        .flatten()
}

/// Derive the locator from the file number and require the page title to agree.
pub fn summa_page(number: u32, bytes: &[u8]) -> Result<SummaPage, String> {
    let digits = format!("{number:04}");
    let part_digit = digits.chars().next().unwrap_or('0');
    let question: u32 = digits[1..].parse().map_err(|_| "invalid question number")?;
    let &(_, part_title, prefix) = SUMMA_PARTS
        .iter()
        .find(|(digit, _, _)| *digit == part_digit)
        .ok_or_else(|| format!("{number}: unknown Summa part digit {part_digit}"))?;
    if question == 0 {
        return Err(format!("{number}: question number 0"));
    }
    let page = html::parse(&html::decode(bytes, "utf-8")?);
    let title = html::normalize_space(page.title.as_deref().unwrap_or_default());
    let body = title
        .strip_prefix(SUMMA_TITLE_PREFIX)
        .ok_or_else(|| format!("{number}: page title is not a Summa page: {title:?}"))?;
    let (question_title, locator) = if part_digit == '7' {
        // Appendix II is a single unnumbered question.
        let name = body
            .strip_suffix(&format!(" ({part_title})"))
            .filter(|_| question == 1)
            .ok_or_else(|| format!("{number}: title {title:?} does not match {part_title}"))?;
        (name, prefix.to_owned())
    } else {
        let suffix = format!(" ({part_title}, Q. {question})");
        let name = body.strip_suffix(&suffix).ok_or_else(|| {
            format!("{number}: file number gives {part_title}, Q. {question}; title is {title:?}")
        })?;
        (name, format!("{prefix} q.{question}"))
    };
    if question_title.is_empty() {
        return Err(format!("{number}: page title names no question"));
    }
    let text = html::render_text(&page.tokens);
    for line in SUMMA_EDITION_LINES {
        if !text.lines().any(|l| l == *line) {
            return Err(format!("{number}: edition statement {line:?} is missing"));
        }
    }
    let years: Vec<&str> = text
        .lines()
        .filter_map(|l| {
            l.strip_prefix("Online Edition Copyright © ")?
                .strip_suffix(" by Kevin Knight")
        })
        .collect();
    let [year] = years.as_slice() else {
        return Err(format!(
            "{number}: expected one online edition copyright line, found {}",
            years.len()
        ));
    };
    if year.len() != 4 || !year.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("{number}: invalid online edition year {year:?}"));
    }
    Ok(SummaPage {
        number,
        locator,
        question_title: question_title.to_owned(),
        online_edition_year: (*year).to_owned(),
    })
}

#[derive(Debug, Serialize)]
pub struct Skipped {
    pub file: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct MirrorSummary {
    pub layout: &'static str,
    pub work_id: String,
    pub added: usize,
    pub unchanged: usize,
    pub editions_by_part: BTreeMap<String, usize>,
    /// Question-page file names whose content contradicts the derived locator.
    pub refused: Vec<Skipped>,
    /// Files that are not question pages (indexes, other material).
    pub not_question_pages: Vec<String>,
}

pub struct MirrorOptions<'a> {
    pub work_id: &'a str,
    pub language: &'a str,
    pub license: &'a str,
}

fn part_name(locator: &str) -> String {
    locator
        .rsplit_once(" q.")
        .map_or(locator, |(part, _)| part)
        .to_owned()
}

/// Register every verifiable New Advent Summa question page in `dir`.
/// Rerunning with the same files and options changes nothing.
pub fn add_newadvent_summa(
    dir: &Path,
    catalog_path: &Path,
    store: &Path,
    options: MirrorOptions<'_>,
) -> Result<MirrorSummary, String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .map(|entry| {
            let entry = entry.map_err(|e| e.to_string())?;
            entry
                .file_name()
                .into_string()
                .map_err(|_| "non-UTF-8 file name in mirror".to_owned())
        })
        .collect::<Result<_, _>>()?;
    names.sort();
    let mut pages = Vec::new();
    let mut refused = Vec::new();
    let mut not_question_pages = Vec::new();
    for name in names {
        let path = dir.join(&name);
        let Some(number) = summa_page_number(&name).filter(|_| path.is_file()) else {
            not_question_pages.push(name);
            continue;
        };
        let bytes = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        match summa_page(number, &bytes) {
            Ok(page) => pages.push((page, bytes)),
            Err(reason) => refused.push(Skipped { file: name, reason }),
        }
    }
    let editions: Vec<_> = pages
        .iter()
        .map(|(page, bytes)| {
            let id = format!(
                "newadvent-summa-{}-{:04}",
                page.online_edition_year, page.number
            );
            let label = format!(
                "New Advent online edition ({}) of the 1920 English Dominican translation, {}: {}",
                page.online_edition_year, page.locator, page.question_title
            );
            let url = format!("https://www.newadvent.org/summa/{:04}.htm", page.number);
            let fields = Add {
                work_id: options.work_id,
                title: "",
                edition_id: &id,
                label: &label,
                language: options.language,
                license: options.license,
                source_url: Some(&url),
            };
            registry::local_edition(&fields, "html", Some("utf-8"), Some(&page.locator), bytes)
        })
        .collect();
    let items = editions
        .into_iter()
        .zip(&pages)
        .map(|(edition, (_, bytes))| LocalEdition {
            edition,
            work_title: None,
            bytes,
        })
        .collect();
    let results = registry::add_local(catalog_path, store, items)?;
    let mut editions_by_part = BTreeMap::new();
    for (edition, _) in &results {
        *editions_by_part
            .entry(part_name(edition.locator.as_deref().unwrap_or_default()))
            .or_insert(0) += 1;
    }
    Ok(MirrorSummary {
        layout: "newadvent-summa",
        work_id: options.work_id.to_owned(),
        added: results.iter().filter(|(_, added)| *added).count(),
        unchanged: results.iter().filter(|(_, added)| !*added).count(),
        editions_by_part,
        refused,
        not_question_pages,
    })
}

/// Prepare one HTML passage. `text_sha256` is the bare hex SHA-256 of the
/// normalized text; Formal Logos records the same digest with a `sha256:` prefix.
pub fn prepare_html(
    bytes: &[u8],
    encoding: &str,
    selector_json: &serde_json::Value,
    include_text: bool,
) -> Result<serde_json::Value, String> {
    let selector = Selector::from_json(selector_json)?;
    let text = html::extract(bytes, encoding, &selector)?;
    let excerpt: String = text.chars().take(80).collect();
    let incipit = if text.chars().count() > 80 {
        excerpt
            .rsplit_once(' ')
            .map_or(excerpt.as_str(), |(head, _)| head)
            .to_owned()
    } else {
        excerpt
    };
    let mut result = serde_json::json!({
        "sha256": hash(bytes), "text_sha256": hash(text.as_bytes()),
        "html_selector": selector_json, "encoding": encoding, "incipit": incipit,
        "characters": text.chars().count(), "lines": text.lines().count()
    });
    if include_text {
        result["text"] = text.into();
    }
    Ok(result)
}

#[derive(Debug, Serialize)]
pub struct Comparison {
    pub passage_id: String,
    pub locator: String,
    pub manifest_path: String,
    pub source_url: String,
    pub local_edition_id: String,
    pub pinned_text_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_text_sha256: Option<String>,
    pub matches: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Compare every imported HTML `web_snapshot` passage with each acquired local
/// HTML edition of the same `source_url`. Offline: reads only the catalogue and
/// the verified local objects.
pub fn compare_witnesses(catalog: &Catalog, store: &Path) -> Result<Vec<Comparison>, String> {
    let mut local: BTreeMap<&str, Vec<&registry::Edition>> = BTreeMap::new();
    for edition in &catalog.editions {
        if let (Some(url), Some(_), "html") = (
            edition.source_url.as_deref(),
            edition.sha256.as_deref(),
            edition.format.as_str(),
        ) {
            local.entry(url).or_default().push(edition);
        }
    }
    let mut comparisons = Vec::new();
    for edition in &catalog.editions {
        let Some(reference) = &edition.formal_logos else {
            continue;
        };
        let snapshot = &reference.snapshot;
        if snapshot["kind"] != "web_snapshot" || edition.format != "html" {
            continue;
        }
        let Some(url) = snapshot["source_url"].as_str() else {
            continue;
        };
        for candidate in local.get(url).into_iter().flatten() {
            let sha = candidate.sha256.as_deref().unwrap_or_default();
            let bytes = registry::checked_object(store, sha)?;
            let encoding = candidate.encoding.as_deref().unwrap_or("utf-8");
            for passage in &reference.passages {
                let (Some(selector), Some(pinned)) = (
                    passage.selectors.get("html_selector"),
                    passage
                        .selectors
                        .get("text_sha256")
                        .and_then(|v| v.as_str()),
                ) else {
                    continue;
                };
                let local_hash = prepare_html(&bytes, encoding, selector, false)
                    .map(|r| format!("sha256:{}", r["text_sha256"].as_str().unwrap_or_default()));
                comparisons.push(Comparison {
                    passage_id: passage.id.clone(),
                    locator: passage.locator.clone(),
                    manifest_path: reference.manifest_path.clone(),
                    source_url: url.to_owned(),
                    local_edition_id: candidate.id.clone(),
                    pinned_text_sha256: pinned.to_owned(),
                    matches: local_hash.as_deref() == Ok(pinned),
                    local_text_sha256: local_hash.as_ref().ok().cloned(),
                    error: local_hash.err(),
                });
            }
        }
    }
    comparisons.sort_by(|a, b| {
        (&a.manifest_path, &a.locator, &a.local_edition_id).cmp(&(
            &b.manifest_path,
            &b.locator,
            &b.local_edition_id,
        ))
    });
    Ok(comparisons)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(title: &str, year: &str, body: &str) -> Vec<u8> {
        format!(
            "<html><head><title>{title}</title></head><body><h1>Question</h1>{body}\
             <div class=pub><p><em>The Summa</em><br>Second and Revised Edition, 1920<br>\
             Literally translated by Fathers of the English Dominican Province<br>\
             Online Edition Copyright &#169; {year} by Kevin Knight <br>Synthetic</p></div></body></html>"
        )
        .into_bytes()
    }

    #[test]
    fn locators_come_from_file_numbers_and_must_agree_with_titles() {
        let ok = summa_page(
            1019,
            &page(
                "SUMMA THEOLOGICA: The will of God (Prima Pars, Q. 19)",
                "2009",
                "",
            ),
        )
        .unwrap();
        assert_eq!(ok.locator, "I q.19");
        assert_eq!(ok.question_title, "The will of God");
        assert_eq!(ok.online_edition_year, "2009");
        let ok = summa_page(
            3001,
            &page(
                "SUMMA THEOLOGICA: Faith (Secunda Secundae Partis, Q. 1)",
                "2009",
                "",
            ),
        )
        .unwrap();
        assert_eq!(ok.locator, "II-II q.1");
        let ok = summa_page(
            7001,
            &page("SUMMA THEOLOGICA: Purgatory (Appendix II)", "2009", ""),
        )
        .unwrap();
        assert_eq!(ok.locator, "Suppl. App. II");
        // A copied page whose number disagrees with its title is refused, not renumbered.
        assert!(
            summa_page(
                3645,
                &page(
                    "SUMMA THEOLOGICA: Temperance (Secunda Secundae Partis, Q. 170)",
                    "2009",
                    ""
                )
            )
            .unwrap_err()
            .contains("Q. 645")
        );
        assert!(summa_page(1019, &page("SUMMA THEOLOGICA: Prima Pars", "2009", "")).is_err());
        assert!(
            summa_page(
                8001,
                &page("SUMMA THEOLOGICA: X (Prima Pars, Q. 1)", "2009", "")
            )
            .is_err()
        );
        assert!(
            summa_page(
                1000,
                &page("SUMMA THEOLOGICA: X (Prima Pars, Q. 0)", "2009", "")
            )
            .is_err()
        );
        let unsigned = b"<title>SUMMA THEOLOGICA: X (Prima Pars, Q. 1)</title><p>Text</p>";
        assert!(
            summa_page(1001, unsigned)
                .unwrap_err()
                .contains("edition statement")
        );
        assert_eq!(summa_page_number("1019.htm"), Some(1019));
        assert_eq!(summa_page_number("1.htm"), None);
        assert_eq!(summa_page_number("index.html"), None);
    }

    #[test]
    fn mirror_import_is_idempotent_and_compares_pinned_passages() {
        let dir = tempfile::tempdir().unwrap();
        let mirror = dir.path().join("summa");
        fs::create_dir_all(&mirror).unwrap();
        let body = "<h2 id=article1>Article 1. Whether one?</h2><p>First body.</p>\
                    <h2 id=article2>Article 2. Whether two?</h2><p>Second body.</p>";
        fs::write(
            mirror.join("1002.htm"),
            page(
                "SUMMA THEOLOGICA: Synthetic (Prima Pars, Q. 2)",
                "2009",
                body,
            ),
        )
        .unwrap();
        fs::write(
            mirror.join("2645.htm"),
            page(
                "SUMMA THEOLOGICA: Copy (Prima Secundae Partis, Q. 5)",
                "2009",
                body,
            ),
        )
        .unwrap();
        fs::write(mirror.join("index.html"), b"<title>Home</title>").unwrap();
        let catalog_path = dir.path().join("registry/catalog.json");
        let store = dir.path().join("local");
        registry::update_catalog(&catalog_path, |catalog| {
            catalog.works.push(registry::Work {
                id: "summa".into(),
                title: "Summa theologiae".into(),
                author_ids: vec![],
                formal_logos_sources: vec![],
            });
            Ok(())
        })
        .unwrap();
        let options = || MirrorOptions {
            work_id: "summa",
            language: "eng",
            license: "Synthetic fixture",
        };
        let first = add_newadvent_summa(&mirror, &catalog_path, &store, options()).unwrap();
        assert_eq!((first.added, first.unchanged), (1, 0));
        assert_eq!(first.refused.len(), 1);
        assert_eq!(first.not_question_pages, vec!["index.html"]);
        assert_eq!(first.editions_by_part["I"], 1);
        let bytes = fs::read(&catalog_path).unwrap();
        let again = add_newadvent_summa(&mirror, &catalog_path, &store, options()).unwrap();
        assert_eq!((again.added, again.unchanged), (0, 1));
        assert_eq!(fs::read(&catalog_path).unwrap(), bytes);
        assert!(
            add_newadvent_summa(
                &mirror,
                &catalog_path,
                &store,
                MirrorOptions {
                    work_id: "missing",
                    ..options()
                }
            )
            .unwrap_err()
            .contains("unknown work")
        );

        let catalog = registry::read_catalog(&catalog_path).unwrap();
        let edition = &catalog.editions[0];
        assert_eq!(edition.id, "newadvent-summa-2009-1002");
        assert_eq!(edition.locator.as_deref(), Some("I q.2"));
        assert_eq!(
            edition.source_url.as_deref(),
            Some("https://www.newadvent.org/summa/1002.htm")
        );
        let (_, local_bytes) = registry::load_edition(&catalog_path, &store, &edition.id).unwrap();
        let prepared = prepare_html(
            &local_bytes,
            "utf-8",
            &serde_json::json!({"heading": "Article 1. Whether one?"}),
            true,
        )
        .unwrap();
        assert_eq!(prepared["text"], "Article 1. Whether one?\nFirst body.");
        assert_eq!(
            prepared["text_sha256"],
            hash(b"Article 1. Whether one?\nFirst body.")
        );

        // Pin two passages upstream: one with the local text, one with other text.
        let pinned = |text: &[u8]| format!("sha256:{}", hash(text));
        let passage = |id: &str, heading: &str, sha: String| registry::PassageReference {
            id: id.into(),
            label: id.into(),
            locator: id.into(),
            claim_locator_prefixes: vec![],
            selectors: BTreeMap::from([
                (
                    "html_selector".into(),
                    serde_json::json!({ "heading": heading }),
                ),
                ("text_sha256".into(), sha.into()),
            ]),
        };
        registry::update_catalog(&catalog_path, |catalog| {
            catalog.formal_logos = Some(registry::CorpusBinding {
                repository: "example/formal-logos".into(),
                revision: "1".repeat(40),
            });
            catalog.works[0]
                .formal_logos_sources
                .push(registry::SourceReference {
                    id: "source-summa".into(),
                    path: "corpus/sources/summa.json".into(),
                    title: "Summa theologiae".into(),
                    kind: "primary_text".into(),
                    work_urn: "urn:example:summa".into(),
                    catalog_url: "https://example.org/summa".into(),
                    citation_system: "part, question, article".into(),
                });
            catalog.editions.push(registry::Edition {
                id: "reference".into(),
                work_id: "summa".into(),
                label: "Pinned live page".into(),
                locator: None,
                language: "en".into(),
                license: "Synthetic metadata".into(),
                source_url: None,
                format: "html".into(),
                encoding: None,
                sha256: None,
                formal_logos: Some(registry::EditionReference {
                    manifest_path: "corpus/passages/q2.json".into(),
                    source_id: "source-summa".into(),
                    snapshot: serde_json::json!({
                        "kind": "web_snapshot", "source_id": "source-summa",
                        "source_url": "https://www.newadvent.org/summa/1002.htm"
                    }),
                    passages: vec![
                        passage(
                            "a1",
                            "Article 1. Whether one?",
                            pinned(b"Article 1. Whether one?\nFirst body."),
                        ),
                        passage("a2", "Article 2. Whether two?", pinned(b"Revised")),
                        passage("a3", "Article 3. Missing?", pinned(b"Missing")),
                    ],
                }),
            });
            Ok(())
        })
        .unwrap();
        let catalog = registry::read_catalog(&catalog_path).unwrap();
        let results = compare_witnesses(&catalog, &store).unwrap();
        let outcome: Vec<_> = results
            .iter()
            .map(|c| (c.locator.as_str(), c.matches, c.error.is_some()))
            .collect();
        assert_eq!(
            outcome,
            [
                ("a1", true, false),
                ("a2", false, false),
                ("a3", false, true)
            ]
        );
        assert!(
            results
                .iter()
                .all(|c| c.local_edition_id == "newadvent-summa-2009-1002")
        );
    }
}
