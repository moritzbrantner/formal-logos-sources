//! Bibliographic links in the research documents of one committed corpus revision.
use crate::registry::Catalog;
use linkify::{LinkFinder, LinkKind};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Citation {
    pub id: String,
    pub cited_urls: Vec<CitedUrl>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CitedUrl {
    pub url: String,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub work_ids: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub edition_ids: BTreeSet<String>,
    pub cited_at: BTreeSet<Mention>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mention {
    pub path: String,
    pub line: usize,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
}

fn doi(raw: &str) -> Option<String> {
    let (registrant, suffix) = raw.strip_prefix("10.")?.split_once('/')?;
    if !(4..=9).contains(&registrant.len())
        || !registrant.bytes().all(|c| c.is_ascii_digit())
        || suffix.is_empty()
        || suffix.chars().any(char::is_whitespace)
    {
        return None;
    }
    Some(format!("doi:{}", raw.to_ascii_lowercase()))
}

fn digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit())
}

fn arxiv(raw: &str) -> Option<String> {
    let raw = raw.strip_suffix(".pdf").unwrap_or(raw);
    let id = raw
        .rsplit_once('v')
        .filter(|(_, version)| digits(version))
        .map_or(raw, |(id, _)| id);
    let valid = if let Some((archive, number)) = id.split_once('/') {
        !archive.is_empty()
            && archive
                .bytes()
                .all(|c| c.is_ascii_alphabetic() || matches!(c, b'.' | b'-'))
            && number.len() == 7
            && digits(number)
    } else if let Some((month, number)) = id.split_once('.') {
        month.len() == 4 && digits(month) && matches!(number.len(), 4 | 5) && digits(number)
    } else {
        false
    };
    valid.then(|| format!("arxiv:{id}"))
}

fn identifier(url: &Url) -> Option<String> {
    let host = url.host_str()?.trim_start_matches("www.");
    let path = url.path();
    match host {
        "doi.org" | "dx.doi.org" => doi(path.strip_prefix('/')?),
        "arxiv.org" | "export.arxiv.org" => arxiv(
            path.strip_prefix("/abs/")
                .or_else(|| path.strip_prefix("/pdf/"))
                .or_else(|| path.strip_prefix("/html/"))?,
        ),
        "link.springer.com" => doi(path.strip_prefix("/article/")?),
        "onlinelibrary.wiley.com" | "journals.uchicago.edu" | "link.aps.org" => doi(path
            .strip_prefix("/doi/pdf/")
            .or_else(|| path.strip_prefix("/doi/"))
            .or_else(|| path.strip_prefix("/accepted/"))?),
        "journals.plos.org" if path.ends_with("/article") => url
            .query_pairs()
            .find_map(|(k, v)| (k == "id").then(|| doi(&v)).flatten()),
        _ => None,
    }
}

fn web_url(raw: &str) -> Option<Url> {
    let url = Url::parse(raw).ok()?;
    (matches!(url.scheme(), "http" | "https")
        && url.has_host()
        && url.username().is_empty()
        && url.password().is_none())
    .then_some(url)
}

fn query_id(value: &str) -> Option<String> {
    if let Some(raw) = value
        .strip_prefix("arxiv:")
        .or_else(|| value.strip_prefix("arXiv:"))
    {
        return arxiv(raw);
    }
    if let Some(id) = doi(value
        .strip_prefix("doi:")
        .or_else(|| value.strip_prefix("DOI:"))
        .unwrap_or(value))
    {
        return Some(id);
    }
    let mut url = web_url(value)?;
    url.set_fragment(None);
    Some(identifier(&url).unwrap_or_else(|| url.into()))
}

#[derive(Default)]
struct Matches {
    works: BTreeSet<String>,
    editions: BTreeSet<String>,
}

#[derive(Default)]
struct Authority {
    urls: BTreeMap<String, Matches>,
    identifiers: BTreeMap<String, BTreeSet<String>>,
}

impl Authority {
    fn register(&mut self, reference: &str, work: &str, edition: Option<&str>) {
        if let Some(key) =
            query_id(reference).filter(|id| id.starts_with("doi:") || id.starts_with("arxiv:"))
        {
            self.identifiers
                .entry(key)
                .or_default()
                .insert(work.to_owned());
        }
        if let Some(url) = web_url(reference) {
            let at_url = self.urls.entry(url.to_string()).or_default();
            at_url.works.insert(work.to_owned());
            if let Some(id) = edition {
                at_url.editions.insert(id.to_owned());
            }
        }
    }

    fn from_catalog(catalog: &Catalog) -> Self {
        let mut authority = Self::default();
        for work in &catalog.works {
            for source in &work.formal_logos_sources {
                authority.register(&source.catalog_url, &work.id, None);
                authority.register(&source.work_urn, &work.id, None);
            }
        }
        for edition in &catalog.editions {
            if let Some(url) = &edition.source_url {
                authority.register(url, &edition.work_id, Some(&edition.id));
            }
            if let Some(reference) = &edition.formal_logos {
                let snapshot = &reference.snapshot;
                if let Some(url) = snapshot["wayback_url"].as_str() {
                    authority.register(url, &edition.work_id, Some(&edition.id));
                }
                if let Some(id) = snapshot["doi"].as_str() {
                    authority.register(id, &edition.work_id, None);
                }
            }
        }
        authority
    }

    fn resolve(&self, raw: &str) -> Option<(String, Matches)> {
        let mut url = web_url(raw)?;
        let exact = self.urls.get(url.as_str());
        url.set_fragment(None);
        let unanchored = self.urls.get(url.as_str());
        let id = identifier(&url);
        let mut matches = Matches::default();
        if let Some(works) = id.as_ref().and_then(|key| self.identifiers.get(key)) {
            matches.works.extend(works.iter().cloned());
        }
        for target in [exact, unanchored].into_iter().flatten() {
            matches.works.extend(target.works.iter().cloned());
            matches.editions.extend(target.editions.iter().cloned());
        }
        if id.is_none() && matches.works.is_empty() && !publication_url(&url) {
            return None;
        }
        Some((id.unwrap_or_else(|| url.into()), matches))
    }
}

// Unregistered documents must have a bibliographic identifier or a recognized publication route.
// This table contains host/path prefixes, with host names matched as whole names.
fn publication_url(url: &Url) -> bool {
    let host = url.host_str().unwrap_or("").trim_start_matches("www.");
    include_str!("../registry/research-publication-routes.txt")
        .lines()
        .any(|line| {
            let Some((provider, path)) = line.split_once('/') else {
                return false;
            };
            let resource = url.path().trim_start_matches('/');
            host == provider
                && (resource == path
                    || ((path.ends_with('/') || path.is_empty()) && resource.starts_with(path)))
        })
}

fn mentions(text: &str, mut visit: impl FnMut(&str, usize, &str)) {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let lines: Vec<_> = std::iter::once(0)
        .chain(text.match_indices('\n').map(|(i, _)| i + 1))
        .collect();
    let line = |offset| lines.partition_point(|i| *i <= offset);
    let mut in_code = false;
    let mut in_image = false;
    let mut link = None;
    let mut label = String::new();
    let mut prose: Vec<std::ops::Range<usize>> = Vec::new();
    for (event, span) in Parser::new(text).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => in_code = true,
            Event::End(TagEnd::CodeBlock) => in_code = false,
            Event::Start(Tag::Image { .. }) => in_image = true,
            Event::End(TagEnd::Image) => in_image = false,
            _ if in_code || in_image => {}
            Event::Start(Tag::Link { dest_url, .. }) => {
                link = Some((dest_url, line(span.start)));
                label.clear();
            }
            Event::End(TagEnd::Link) => {
                if let Some((url, number)) = link.take() {
                    visit(&url, number, if label == *url { "" } else { label.trim() });
                }
            }
            Event::SoftBreak | Event::HardBreak if link.is_some() => label.push(' '),
            Event::Text(value) | Event::Code(value) if link.is_some() => label.push_str(&value),
            Event::Text(_) | Event::Code(_) => {
                // The parser can split a bare URL at an underscore or another Markdown delimiter.
                if let Some(previous) = prose.last_mut()
                    && previous.end <= span.start
                    && text[previous.end..span.start]
                        .bytes()
                        .all(|c| matches!(c, b'*' | b'_' | b'~'))
                {
                    previous.end = span.end;
                } else {
                    prose.push(span);
                }
            }
            _ => {}
        }
    }
    for span in prose {
        for found in finder.links(&text[span.clone()]) {
            visit(found.as_str(), line(span.start + found.start()), "");
        }
    }
}

pub fn collect(catalog: &Catalog, documents: &[(String, String)]) -> Vec<Citation> {
    let authority = Authority::from_catalog(catalog);
    let mut cited: BTreeMap<String, BTreeMap<String, CitedUrl>> = BTreeMap::new();
    for (path, text) in documents {
        mentions(text, |url, line, label| {
            let Some((id, matched)) = authority.resolve(url) else {
                return;
            };
            cited
                .entry(id)
                .or_default()
                .entry(url.to_owned())
                .or_insert_with(|| CitedUrl {
                    url: url.to_owned(),
                    work_ids: matched.works,
                    edition_ids: matched.editions,
                    cited_at: BTreeSet::new(),
                })
                .cited_at
                .insert(Mention {
                    path: path.clone(),
                    line,
                    label: label.to_owned(),
                });
        });
    }
    cited
        .into_iter()
        .map(|(id, urls)| Citation {
            id,
            cited_urls: urls.into_values().collect(),
        })
        .collect()
}

pub fn validate(catalog: &Catalog) -> Result<(), String> {
    if catalog.research_citations.is_empty() {
        return Ok(());
    }
    if catalog.formal_logos.is_none() {
        return Err("research citations require a corpus revision".into());
    }
    let works: BTreeSet<_> = catalog.works.iter().map(|w| &w.id).collect();
    let editions: BTreeMap<_, _> = catalog
        .editions
        .iter()
        .map(|e| (&e.id, &e.work_id))
        .collect();
    let mut ids = BTreeSet::new();
    for citation in &catalog.research_citations {
        let mut urls = BTreeSet::new();
        if !ids.insert(&citation.id) || citation.cited_urls.is_empty() {
            return Err(format!("invalid or repeated citation {}", citation.id));
        }
        for target in &citation.cited_urls {
            if query_id(&target.url).as_ref() != Some(&citation.id)
                || !urls.insert(&target.url)
                || target.cited_at.is_empty()
                || target.cited_at.iter().any(|m| {
                    m.line == 0
                        || !m.path.starts_with("docs/")
                        || !m.path.ends_with(".md")
                        || m.path.split('/').any(|p| matches!(p, "" | "." | ".."))
                })
                || target.work_ids.iter().any(|w| !works.contains(w))
                || target.edition_ids.iter().any(|e| {
                    editions
                        .get(e)
                        .is_none_or(|work| !target.work_ids.contains(*work))
                })
            {
                return Err(format!(
                    "invalid citation target {} in {}",
                    target.url, citation.id
                ));
            }
        }
    }
    Ok(())
}

pub fn lookup<'a>(catalog: &'a Catalog, query: Option<&str>) -> Vec<&'a Citation> {
    let key = query.and_then(query_id);
    let source_work = query.and_then(|query| {
        catalog
            .works
            .iter()
            .find(|w| w.formal_logos_sources.iter().any(|s| s.id == query))
    });
    catalog
        .research_citations
        .iter()
        .filter(|citation| {
            query.is_none_or(|q| {
                key.as_ref() == Some(&citation.id)
                    || citation.id == q
                    || citation.cited_urls.iter().any(|target| {
                        target.work_ids.contains(q)
                            || target.edition_ids.contains(q)
                            || source_work.is_some_and(|w| target.work_ids.contains(&w.id))
                    })
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publication_identifiers_keep_different_works_and_locations_distinct() {
        assert_eq!(
            query_id("https://arxiv.org/pdf/astro-ph/9712344v3.pdf#page=2"),
            Some("arxiv:astro-ph/9712344".into())
        );
        assert_eq!(
            query_id("https://doi.org/10.1016/0370-2693(82)90866-8"),
            Some("doi:10.1016/0370-2693(82)90866-8".into())
        );
        assert_ne!(
            query_id("https://en.wikisource.org/w/index.php?oldid=17#A"),
            query_id("https://en.wikisource.org/w/index.php?oldid=18#A")
        );
        assert!(
            Authority::default()
                .resolve("https://arxiv.org/help")
                .is_none()
        );
        assert!(
            Authority::default()
                .resolve("https://www.wehd.com/373/911.html-example")
                .is_none()
        );
    }

    #[test]
    fn bare_publication_url_survives_inline_markdown_token_splitting() {
        let doc = "The source is https://www.fhs.unizg.hr/_download/repository/Curd%2C_A_Presocratics_Reader.pdf.\n";
        let citations = collect(
            &Catalog::default(),
            &[("docs/notes/example.md".into(), doc.into())],
        );
        assert_eq!(citations.len(), 1);
        assert!(
            citations[0].cited_urls[0]
                .url
                .ends_with("_Presocratics_Reader.pdf")
        );
    }

    #[test]
    fn shared_registered_url_preserves_every_candidate_and_identifier_only_matches_works() {
        let mut authority = Authority::default();
        authority.register("https://example.org/shared", "first", Some("edition1"));
        authority.register("https://example.org/shared", "second", Some("edition2"));
        let (_, found) = authority
            .resolve("https://example.org/shared#page=4")
            .unwrap();
        assert_eq!(found.works.len(), 2);
        assert_eq!(found.editions.len(), 2);
        authority.register(
            "https://arxiv.org/pdf/1710.05834v1",
            "first",
            Some("edition3"),
        );
        let (_, version2) = authority
            .resolve("https://arxiv.org/pdf/1710.05834v2")
            .unwrap();
        assert!(version2.works.contains("first"));
        assert!(version2.editions.is_empty());
    }
}
