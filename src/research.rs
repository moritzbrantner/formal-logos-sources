//! Curated people, bibliography inventories and retrieval plans, separate from imported witnesses.
use crate::registry::{Catalog, CorpusBinding};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Research {
    pub schema_version: u32,
    pub profiles: Vec<Profile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub attributions: Vec<Attribution>,
    pub life: Life,
    pub scope_note: String,
    pub inventories: Vec<Inventory>,
    pub retrievals: Vec<Retrieval>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attribution {
    pub attribution_id: String,
    pub relation: AttributionRelation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionRelation {
    SoleAuthor,
    Coauthor,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Life {
    pub status: LifeStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub death_year: Option<i32>,
    pub evidence: Vec<Evidence>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifeStatus {
    Deceased,
    Living,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub url: String,
    pub title: String,
    pub checked_on: String,
    pub note: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Inventory {
    pub id: String,
    pub title: String,
    pub url: String,
    pub scope: String,
    pub publication_status: PublicationStatus,
    pub reconciliation_status: ReconciliationStatus,
    pub evidence: Vec<Evidence>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationStatus {
    Ongoing,
    Published,
    Selected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationStatus {
    NotStarted,
    InProgress,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Retrieval {
    pub id: String,
    pub work_id: String,
    pub title: String,
    pub language: String,
    pub format: RetrievalFormat,
    pub url: String,
    pub coverage: String,
    pub access: Access,
    pub rights: Rights,
    pub evidence: Vec<Evidence>,
    pub next_step: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalFormat {
    Html,
    Tei,
    Pdf,
    Epub,
    Text,
    Bibliographic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    OpenOnline,
    CatalogueOnly,
    RequestRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rights {
    pub status: RightsStatus,
    pub scope: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    pub evidence: Vec<Evidence>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RightsStatus {
    ProviderLicense,
    ProviderPublicDomain,
    PermissionRequired,
    NotEstablished,
}

impl Research {
    fn validate(&self, catalog: &Catalog) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported author research schema: {}",
                self.schema_version
            ));
        }
        let authors: BTreeSet<_> = catalog.authors.iter().map(|a| a.id.as_str()).collect();
        let works: BTreeMap<_, _> = catalog.works.iter().map(|w| (w.id.as_str(), w)).collect();
        let mut profile_ids = HashSet::new();
        let mut inventory_ids = HashSet::new();
        let mut retrieval_ids = HashSet::new();
        let mut attribution_owners: BTreeMap<&str, Vec<AttributionRelation>> = BTreeMap::new();
        for profile in &self.profiles {
            unique_id("profile", &profile.id, &mut profile_ids)?;
            nonempty("profile name", &profile.name)?;
            nonempty("profile scope note", &profile.scope_note)?;
            let mut aliases = HashSet::new();
            for alias in &profile.aliases {
                nonempty("profile alias", alias)?;
                if !aliases.insert(alias.to_lowercase()) {
                    return Err(format!("profile {} repeats alias {alias}", profile.id));
                }
            }
            if profile.attributions.is_empty() {
                return Err(format!("profile {} has no attribution mapping", profile.id));
            }
            let mut mapped = BTreeSet::new();
            for attribution in &profile.attributions {
                if !authors.contains(attribution.attribution_id.as_str()) {
                    return Err(format!(
                        "profile {} references unknown attribution {}",
                        profile.id, attribution.attribution_id
                    ));
                }
                if !mapped.insert(attribution.attribution_id.as_str()) {
                    return Err(format!(
                        "profile {} repeats attribution {}",
                        profile.id, attribution.attribution_id
                    ));
                }
                attribution_owners
                    .entry(&attribution.attribution_id)
                    .or_default()
                    .push(attribution.relation);
            }
            match (profile.life.status, profile.life.death_year) {
                (LifeStatus::Deceased, Some(year)) if year != 0 => {}
                (LifeStatus::Deceased, _) => {
                    return Err(format!(
                        "deceased profile {} requires a nonzero death year (negative for BCE)",
                        profile.id
                    ));
                }
                (_, Some(_)) => {
                    return Err(format!(
                        "profile {} has a death year without deceased status",
                        profile.id
                    ));
                }
                (_, None) => {}
            }
            evidence(
                "life status",
                &profile.life.evidence,
                profile.life.status != LifeStatus::Unknown,
            )?;
            for inventory in &profile.inventories {
                unique_id("inventory", &inventory.id, &mut inventory_ids)?;
                nonempty("inventory title", &inventory.title)?;
                nonempty("inventory scope", &inventory.scope)?;
                url(&inventory.url)?;
                evidence("inventory", &inventory.evidence, true)?;
            }
            for retrieval in &profile.retrievals {
                unique_id("retrieval", &retrieval.id, &mut retrieval_ids)?;
                let work = works.get(retrieval.work_id.as_str()).ok_or_else(|| {
                    format!(
                        "retrieval {} references unknown work {}",
                        retrieval.id, retrieval.work_id
                    )
                })?;
                if !work.author_ids.iter().any(|a| mapped.contains(a.as_str())) {
                    return Err(format!(
                        "retrieval {} references work {} outside profile {} attributions",
                        retrieval.id, retrieval.work_id, profile.id
                    ));
                }
                for (label, value) in [
                    ("retrieval title", &retrieval.title),
                    ("retrieval language", &retrieval.language),
                    ("retrieval coverage", &retrieval.coverage),
                    ("retrieval next step", &retrieval.next_step),
                    ("rights scope", &retrieval.rights.scope),
                ] {
                    nonempty(label, value)?;
                }
                url(&retrieval.url)?;
                evidence("retrieval", &retrieval.evidence, true)?;
                if let Some(jurisdiction) = &retrieval.rights.jurisdiction {
                    nonempty("rights jurisdiction", jurisdiction)?;
                }
                if retrieval.rights.status == RightsStatus::ProviderPublicDomain
                    && retrieval.rights.jurisdiction.is_none()
                {
                    return Err(format!(
                        "retrieval {} provider public-domain claim requires a jurisdiction",
                        retrieval.id
                    ));
                }
                evidence(
                    "rights",
                    &retrieval.rights.evidence,
                    retrieval.rights.status != RightsStatus::NotEstablished,
                )?;
            }
        }
        for (attribution, owners) in attribution_owners {
            if owners.len() > 1 && owners.contains(&AttributionRelation::SoleAuthor) {
                return Err(format!(
                    "attribution {attribution} maps to multiple profiles but includes a sole-author relation"
                ));
            }
        }
        Ok(())
    }
}

fn nonempty(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        return Err(format!(
            "{label} must be nonempty, trimmed text without control characters"
        ));
    }
    Ok(())
}

fn unique_id<'a>(kind: &str, id: &'a str, seen: &mut HashSet<&'a str>) -> Result<(), String> {
    if id.is_empty()
        || id.len() > 128
        || !id
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        || !id.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-')
        })
    {
        return Err(format!("invalid {kind} id: {id}"));
    }
    if !seen.insert(id) {
        return Err(format!("duplicate {kind} id: {id}"));
    }
    Ok(())
}

fn url(value: &str) -> Result<(), String> {
    let remainder = value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"));
    if remainder.is_none_or(|s| s.split(['/', '?', '#']).next().is_none_or(str::is_empty))
        || value.chars().any(|c| c.is_whitespace() || c.is_control())
    {
        return Err(format!("invalid HTTP(S) link: {value}"));
    }
    Ok(())
}

fn valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 10
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || !bytes
            .iter()
            .enumerate()
            .all(|(i, b)| matches!(i, 4 | 7) || b.is_ascii_digit())
    {
        return false;
    }
    let Ok(year) = value[..4].parse::<u32>() else {
        return false;
    };
    let Ok(month) = value[5..7].parse::<u32>() else {
        return false;
    };
    let Ok(day) = value[8..].parse::<u32>() else {
        return false;
    };
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            29
        }
        2 => 28,
        _ => return false,
    };
    year != 0 && day != 0 && day <= days
}

fn evidence(label: &str, items: &[Evidence], required: bool) -> Result<(), String> {
    if required && items.is_empty() {
        return Err(format!("{label} requires evidence"));
    }
    for item in items {
        url(&item.url)?;
        nonempty("evidence title", &item.title)?;
        nonempty("evidence note", &item.note)?;
        if !valid_date(&item.checked_on) {
            return Err(format!(
                "evidence checked_on must be a valid YYYY-MM-DD date: {}",
                item.checked_on
            ));
        }
    }
    Ok(())
}

/// Read and validate the curated sidecar once, against an already validated catalogue.
pub fn read_research(path: &Path, catalog: &Catalog) -> Result<Research, String> {
    let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let research: Research =
        serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    research.validate(catalog)?;
    Ok(research)
}

#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub formal_logos: Option<CorpusBinding>,
    pub total_attribution_strings: usize,
    pub mapped_attribution_strings: usize,
    pub unprofiled_attribution_strings: usize,
    pub total_profiles: usize,
    pub deceased_profiles: usize,
    pub selected_profiles: usize,
    pub profiles: Vec<ProfileReport>,
    pub scope: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProfileReport {
    pub profile: Profile,
    pub attributions: Vec<AttributionReport>,
    pub works: Vec<WorkReport>,
    pub catalogued_works: usize,
    pub witness_editions: usize,
    pub registered_passages: usize,
    pub works_without_retrieval_candidates: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AttributionReport {
    pub id: String,
    pub name: String,
    pub relation: AttributionRelation,
}

#[derive(Clone, Debug, Serialize)]
pub struct WorkReport {
    pub id: String,
    pub title: String,
    pub sources: Vec<SourceLink>,
    pub witness_editions: usize,
    pub registered_passages: usize,
    pub retrieval_candidate_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SourceLink {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub citation_system: String,
}

/// Derive navigation and coverage counts; this performs no retrieval or reading assessment.
pub fn report(
    research: &Research,
    catalog: &Catalog,
    author: Option<&str>,
    deceased_only: bool,
) -> Result<Report, String> {
    let matched = if let Some(query) = author {
        let query = query.to_lowercase();
        let matches: Vec<_> = research
            .profiles
            .iter()
            .filter(|p| {
                p.id.to_lowercase() == query
                    || p.name.to_lowercase() == query
                    || p.aliases.iter().any(|a| a.to_lowercase() == query)
            })
            .collect();
        match matches.as_slice() {
            [] => return Err("unknown author profile".into()),
            [profile] => Some(profile.id.as_str()),
            _ => return Err("ambiguous author profile; select an exact unique profile id".into()),
        }
    } else {
        None
    };
    let mapped: BTreeSet<_> = research
        .profiles
        .iter()
        .flat_map(|p| p.attributions.iter().map(|a| a.attribution_id.as_str()))
        .collect();
    let mut profiles = Vec::new();
    for profile in &research.profiles {
        if matched.is_some_and(|id| id != profile.id)
            || (deceased_only && profile.life.status != LifeStatus::Deceased)
        {
            continue;
        }
        let attribution_ids: BTreeSet<_> = profile
            .attributions
            .iter()
            .map(|a| a.attribution_id.as_str())
            .collect();
        let mut attributions: Vec<_> = profile
            .attributions
            .iter()
            .filter_map(|a| {
                catalog
                    .authors
                    .iter()
                    .find(|c| c.id == a.attribution_id)
                    .map(|c| AttributionReport {
                        id: c.id.clone(),
                        name: c.name.clone(),
                        relation: a.relation,
                    })
            })
            .collect();
        attributions.sort_by(|a, b| a.id.cmp(&b.id));
        let mut works = Vec::new();
        for work in catalog.works.iter().filter(|w| {
            w.author_ids
                .iter()
                .any(|a| attribution_ids.contains(a.as_str()))
        }) {
            let editions: Vec<_> = catalog
                .editions
                .iter()
                .filter(|e| e.work_id == work.id)
                .collect();
            let mut retrieval_candidate_ids: Vec<_> = profile
                .retrievals
                .iter()
                .filter(|r| r.work_id == work.id)
                .map(|r| r.id.clone())
                .collect();
            retrieval_candidate_ids.sort();
            let mut sources: Vec<_> = work
                .formal_logos_sources
                .iter()
                .map(|s| SourceLink {
                    id: s.id.clone(),
                    title: s.title.clone(),
                    url: catalog.formal_logos.as_ref().map(|binding| {
                        format!(
                            "https://github.com/{}/blob/{}/{}",
                            binding.repository, binding.revision, s.path
                        )
                    }),
                    citation_system: s.citation_system.clone(),
                })
                .collect();
            sources.sort_by(|a, b| a.id.cmp(&b.id));
            works.push(WorkReport {
                id: work.id.clone(),
                title: work.title.clone(),
                sources,
                witness_editions: editions.len(),
                registered_passages: editions
                    .iter()
                    .filter_map(|e| e.formal_logos.as_ref())
                    .map(|e| e.passages.len())
                    .sum(),
                retrieval_candidate_ids,
            });
        }
        works.sort_by(|a, b| a.title.cmp(&b.title).then_with(|| a.id.cmp(&b.id)));
        let mut profile = profile.clone();
        profile
            .attributions
            .sort_by(|a, b| a.attribution_id.cmp(&b.attribution_id));
        profile.inventories.sort_by(|a, b| a.id.cmp(&b.id));
        profile.retrievals.sort_by(|a, b| a.id.cmp(&b.id));
        profiles.push(ProfileReport {
            profile,
            attributions,
            catalogued_works: works.len(),
            witness_editions: works.iter().map(|w| w.witness_editions).sum(),
            registered_passages: works.iter().map(|w| w.registered_passages).sum(),
            works_without_retrieval_candidates: works
                .iter()
                .filter(|w| w.retrieval_candidate_ids.is_empty())
                .map(|w| w.id.clone())
                .collect(),
            works,
        });
    }
    profiles.sort_by(|a, b| {
        b.registered_passages
            .cmp(&a.registered_passages)
            .then_with(|| a.profile.id.cmp(&b.profile.id))
    });
    Ok(Report {
        formal_logos: catalog.formal_logos.clone(),
        total_attribution_strings: catalog.authors.len(),
        mapped_attribution_strings: mapped.len(),
        unprofiled_attribution_strings: catalog
            .authors
            .iter()
            .filter(|a| !mapped.contains(a.id.as_str()))
            .count(),
        total_profiles: research.profiles.len(),
        deceased_profiles: research
            .profiles
            .iter()
            .filter(|p| p.life.status == LifeStatus::Deceased)
            .count(),
        selected_profiles: profiles.len(),
        profiles,
        scope: "Counts describe registered catalogue references and curated plans, not complete bibliographies, reading coverage, acquired bytes or permission to redistribute. Jointly attributed works can occur under more than one person; profile totals are not additive.",
    })
}

fn plain(value: &str) -> String {
    let mut escaped = String::new();
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '\n' | '\r' => escaped.push(' '),
            '\\' | '[' | ']' | '*' | '_' | '`' | '|' => {
                escaped.push('\\');
                escaped.push(character);
            }
            _ => escaped.push(character),
        }
    }
    escaped
}

fn link(title: &str, target: &str) -> String {
    let target = target
        .replace(' ', "%20")
        .replace('<', "%3C")
        .replace('>', "%3E")
        .replace('(', "%28")
        .replace(')', "%29")
        .replace('\\', "%5C");
    format!("[{}]({target})", plain(title))
}

fn evidence_markdown(out: &mut String, label: &str, items: &[Evidence]) {
    if items.is_empty() {
        return;
    }
    out.push_str(&format!("\n{label}:\n\n"));
    for item in items {
        out.push_str(&format!(
            "- {} (checked {}): {}\n",
            link(&item.title, &item.url),
            item.checked_on,
            plain(&item.note)
        ));
    }
}

fn life_label(life: &Life) -> String {
    match (life.status, life.death_year) {
        (LifeStatus::Deceased, Some(year)) if year < 0 => {
            format!("Deceased, {} BCE", year.unsigned_abs())
        }
        (LifeStatus::Deceased, Some(year)) => format!("Deceased, {year}"),
        (LifeStatus::Deceased, None) => "Deceased".into(),
        (LifeStatus::Living, _) => "Living (as recorded by the cited evidence)".into(),
        (LifeStatus::Unknown, _) => "Life status not established".into(),
    }
}

/// Render the same curated facts and derived counts as the JSON report.
pub fn render_markdown(report: &Report) -> String {
    let mut out = format!(
        "# Author research and retrieval plan\n\n{}\n\nAttribution strings: **{} total**, **{} mapped to profiles**, **{} without a profile**. There are **{} curated profiles**, of which **{} are recorded deceased**; this view selects **{}**. Unprofiled attributions have no life-status classification here.\n",
        report.scope,
        report.total_attribution_strings,
        report.mapped_attribution_strings,
        report.unprofiled_attribution_strings,
        report.total_profiles,
        report.deceased_profiles,
        report.selected_profiles
    );
    if let Some(binding) = &report.formal_logos {
        out.push_str(&format!(
            "\nCatalogue source: {}.\n",
            link(
                &format!("{} at {}", binding.repository, binding.revision),
                &format!(
                    "https://github.com/{}/tree/{}",
                    binding.repository, binding.revision
                )
            )
        ));
    }
    out.push_str("\nProfiles are ordered by registered passage count, then profile ID. Counts indicate existing corpus use, not philosophical importance or research completion.\n\n| Profile | Works | Witnesses | Passages | Retrieval candidates | Works without candidates |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for row in &report.profiles {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            plain(&row.profile.name),
            row.catalogued_works,
            row.witness_editions,
            row.registered_passages,
            row.profile.retrievals.len(),
            row.works_without_retrieval_candidates.len()
        ));
    }
    for row in &report.profiles {
        let profile = &row.profile;
        out.push_str(&format!(
            "\n## {}\n\n**{}**. {}\n\nMapped source attributions:\n\n",
            plain(&profile.name),
            life_label(&profile.life),
            plain(&profile.scope_note)
        ));
        for attribution in &row.attributions {
            let relation = match attribution.relation {
                AttributionRelation::SoleAuthor => "sole author",
                AttributionRelation::Coauthor => {
                    "coauthor; the original joint attribution is retained"
                }
            };
            out.push_str(&format!("- {} — {relation}.\n", plain(&attribution.name)));
        }
        evidence_markdown(&mut out, "Life-status evidence", &profile.life.evidence);
        out.push_str("\n### Catalogued works\n\n| Work and canonical source | Witnesses | Passages | Retrieval candidates |\n| --- | ---: | ---: | ---: |\n");
        for work in &row.works {
            let sources = if work.sources.is_empty() {
                plain(&work.title)
            } else {
                work.sources
                    .iter()
                    .map(|s| {
                        s.url
                            .as_ref()
                            .map_or_else(|| plain(&s.title), |url| link(&s.title, url))
                    })
                    .collect::<Vec<_>>()
                    .join("; ")
            };
            out.push_str(&format!(
                "| {sources} | {} | {} | {} |\n",
                work.witness_editions,
                work.registered_passages,
                work.retrieval_candidate_ids.len()
            ));
        }
        out.push_str("\n### Bibliography inventories\n");
        if profile.inventories.is_empty() {
            out.push_str("\nNo bibliography inventory is recorded yet.\n");
        }
        for inventory in &profile.inventories {
            let publication = match inventory.publication_status {
                PublicationStatus::Ongoing => "ongoing",
                PublicationStatus::Published => "published",
                PublicationStatus::Selected => "selected material",
            };
            let reconciliation = match inventory.reconciliation_status {
                ReconciliationStatus::NotStarted => "not started",
                ReconciliationStatus::InProgress => "in progress",
            };
            out.push_str(&format!(
                "\n#### {}\n\n{}\n\nPublication: {publication}. Catalogue reconciliation: {reconciliation}.\n",
                link(&inventory.title, &inventory.url),
                plain(&inventory.scope)
            ));
            evidence_markdown(&mut out, "Inventory evidence", &inventory.evidence);
        }
        out.push_str("\n### Retrieval candidates\n\nThese entries identify possible editions and access routes. They do not register acquired files or authorize redistribution.\n");
        if profile.retrievals.is_empty() {
            out.push_str("\nNo retrieval candidate is recorded yet.\n");
        }
        for retrieval in &profile.retrievals {
            let work = row.works.iter().find(|w| w.id == retrieval.work_id);
            let work_title = work.map_or(retrieval.work_id.as_str(), |w| w.title.as_str());
            let format = match retrieval.format {
                RetrievalFormat::Html => "HTML",
                RetrievalFormat::Tei => "TEI",
                RetrievalFormat::Pdf => "PDF",
                RetrievalFormat::Epub => "EPUB",
                RetrievalFormat::Text => "text",
                RetrievalFormat::Bibliographic => "bibliographic record",
            };
            let access = match retrieval.access {
                Access::OpenOnline => "open online",
                Access::CatalogueOnly => "catalogue only",
                Access::RequestRequired => "request required",
            };
            let rights = match retrieval.rights.status {
                RightsStatus::ProviderLicense => "provider licence",
                RightsStatus::ProviderPublicDomain => "provider public-domain claim",
                RightsStatus::PermissionRequired => "permission required",
                RightsStatus::NotEstablished => "not established",
            };
            out.push_str(&format!(
                "\n#### {}\n\nWork: {}. Language: {}. Format: {format}. Access: {access}.\n\nCoverage: {}\n\nRights — {rights}: {}\n",
                link(&retrieval.title, &retrieval.url),
                plain(work_title),
                plain(&retrieval.language),
                plain(&retrieval.coverage),
                plain(&retrieval.rights.scope)
            ));
            if let Some(jurisdiction) = &retrieval.rights.jurisdiction {
                out.push_str(&format!(
                    "\nRights jurisdiction: {}.\n",
                    plain(jurisdiction)
                ));
            }
            evidence_markdown(&mut out, "Edition/access evidence", &retrieval.evidence);
            evidence_markdown(&mut out, "Rights evidence", &retrieval.rights.evidence);
            out.push_str(&format!("\nNext step: {}\n", plain(&retrieval.next_step)));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{
        Author, Edition, EditionReference, PassageReference, SourceReference, Work,
    };
    use serde_json::json;

    fn fixture_catalog() -> Catalog {
        let mut catalog = Catalog {
            authors: vec![
                Author {
                    id: "joint".into(),
                    name: "First and Second".into(),
                },
                Author {
                    id: "other".into(),
                    name: "Other".into(),
                },
                Author {
                    id: "unprofiled".into(),
                    name: "Unprofiled institution".into(),
                },
            ],
            formal_logos: Some(CorpusBinding {
                repository: "moritzbrantner/formal-logos".into(),
                revision: "a".repeat(40),
            }),
            ..Catalog::default()
        };
        for (id, attribution, count) in [("joint-work", "joint", 2), ("other-work", "other", 1)] {
            catalog.works.push(Work {
                id: id.into(),
                title: id.into(),
                author_ids: vec![attribution.into()],
                formal_logos_sources: vec![SourceReference {
                    id: format!("source-{id}"),
                    path: format!("corpus/sources/{id}.json"),
                    title: id.into(),
                    kind: "primary_text".into(),
                    work_urn: format!("urn:fixture:{id}"),
                    catalog_url: "https://example.org/catalogue".into(),
                    citation_system: "section".into(),
                }],
            });
            catalog.editions.push(Edition {
                id: format!("edition-{id}"), work_id: id.into(), label: "Synthetic witness".into(),
                language: "eng".into(), license: "Synthetic fixture".into(), source_url: None,
                format: "html".into(), sha256: None,
                formal_logos: Some(EditionReference {
                    manifest_path: format!("corpus/passages/{id}.json"), source_id: format!("source-{id}"),
                    snapshot: json!({"kind":"web_snapshot","source_id":format!("source-{id}"),"source_url":"https://example.org/text"}),
                    passages: (0..count).map(|n| PassageReference {
                        id: format!("passage-{id}-{n}"), label: format!("Section {n}"), locator: n.to_string(),
                        claim_locator_prefixes: vec![], selectors: BTreeMap::new(),
                    }).collect(),
                }),
            });
        }
        catalog.validate().unwrap();
        catalog
    }

    fn fixture_research() -> Research {
        serde_json::from_value(json!({
            "schema_version": 1,
            "profiles": [{
                "id":"first", "name":"First Person", "aliases":["First"],
                "attributions":[{"attribution_id":"joint","relation":"coauthor"}],
                "life":{"status":"deceased","death_year":-322,"evidence":[proof()]},
                "scope_note":"Only registered works are represented; bibliography reconciliation has not started.",
                "inventories":[{"id":"first-inventory","title":"Collected writings","url":"https://example.org/inventory",
                    "scope":"An ongoing edition, not a closed bibliography.","publication_status":"ongoing",
                    "reconciliation_status":"not_started","evidence":[proof()]}],
                "retrievals":[{"id":"first-edition","work_id":"joint-work","title":"A particular edition","language":"eng","format":"html",
                    "url":"https://example.org/text","coverage":"The first two sections.","access":"open_online",
                    "rights":{"status":"not_established","scope":"No redistribution permission has been established.","evidence":[]},
                    "evidence":[proof()],"next_step":"Inspect the edition's rights notice before acquiring bytes."}]
            }]
        })).unwrap()
    }

    fn proof() -> serde_json::Value {
        json!({"url":"https://example.org/evidence","title":"Institutional record","checked_on":"2026-11-19","note":"Synthetic evidence for the recorded field."})
    }

    #[test]
    fn joint_attributions_are_explicit_and_do_not_double_count_global_coverage() {
        let catalog = fixture_catalog();
        let mut research = fixture_research();
        let mut second = research.profiles[0].clone();
        second.id = "second".into();
        second.name = "Second Person".into();
        second.aliases = vec!["Second".into()];
        second.inventories.clear();
        second.retrievals.clear();
        research.profiles.push(second);
        research.validate(&catalog).unwrap();
        let result = report(&research, &catalog, None, true).unwrap();
        assert_eq!(result.total_attribution_strings, 3);
        assert_eq!(result.mapped_attribution_strings, 1);
        assert_eq!(result.unprofiled_attribution_strings, 2);
        assert_eq!(result.profiles[0].registered_passages, 2);
        assert_eq!(result.profiles[1].registered_passages, 2);
        assert_eq!(
            result.profiles[0].works_without_retrieval_candidates.len(),
            0
        );
        assert_eq!(
            result.profiles[1].works_without_retrieval_candidates,
            ["joint-work"]
        );
        research.profiles[1].attributions[0].relation = AttributionRelation::SoleAuthor;
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("sole-author")
        );
    }

    #[test]
    fn unknown_and_foreign_references_fail_without_name_inference() {
        let catalog = fixture_catalog();
        let mut research = fixture_research();
        research.profiles[0].attributions[0].attribution_id = "First and Second".into();
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("unknown attribution")
        );
        research.profiles[0].attributions[0].attribution_id = "joint".into();
        research.profiles[0].retrievals[0].work_id = "absent-work".into();
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("unknown work")
        );
        research.profiles[0].retrievals[0].work_id = "other-work".into();
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("outside profile")
        );
    }

    #[test]
    fn death_and_rights_require_their_own_evidence() {
        let catalog = fixture_catalog();
        let mut research = fixture_research();
        research.profiles[0].life.evidence.clear();
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("life status requires evidence")
        );
        research.profiles[0].life = fixture_research().profiles.remove(0).life;
        research.profiles[0].life.death_year = Some(0);
        assert!(research.validate(&catalog).is_err());
        research.profiles[0].life.death_year = Some(-322);
        research.profiles[0].retrievals[0].rights.status = RightsStatus::ProviderPublicDomain;
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("jurisdiction")
        );
        research.profiles[0].retrievals[0].rights.jurisdiction = Some("United States".into());
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("rights requires evidence")
        );
        research.profiles[0].retrievals[0].rights.evidence =
            serde_json::from_value(json!([proof()])).unwrap();
        research.validate(&catalog).unwrap();
        research.profiles[0].life.status = LifeStatus::Living;
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("death year without deceased")
        );
        research.profiles[0].life.death_year = None;
        research.validate(&catalog).unwrap();
        assert!(
            report(&research, &catalog, None, true)
                .unwrap()
                .profiles
                .is_empty()
        );
    }

    #[test]
    fn exact_filters_reject_ambiguous_aliases_and_order_is_stable() {
        let catalog = fixture_catalog();
        let mut research = fixture_research();
        let mut other = research.profiles[0].clone();
        other.id = "a-other".into();
        other.name = "Other Person".into();
        other.attributions = vec![Attribution {
            attribution_id: "other".into(),
            relation: AttributionRelation::SoleAuthor,
        }];
        other.inventories.clear();
        other.retrievals.clear();
        research.profiles.insert(0, other);
        research.validate(&catalog).unwrap();
        assert!(
            report(&research, &catalog, Some("FIRST"), false)
                .unwrap_err()
                .contains("ambiguous")
        );
        assert!(
            report(&research, &catalog, Some("First Pe"), false)
                .unwrap_err()
                .contains("unknown")
        );
        let filtered = report(&research, &catalog, Some("FIRST PERSON"), false).unwrap();
        assert_eq!(filtered.selected_profiles, 1);
        assert_eq!(filtered.mapped_attribution_strings, 2);
        let all = report(&research, &catalog, None, false).unwrap();
        assert_eq!(all.profiles[0].profile.id, "first");
        assert_eq!(all.profiles[1].profile.id, "a-other");
        research.profiles.reverse();
        assert_eq!(
            serde_json::to_value(all).unwrap(),
            serde_json::to_value(report(&research, &catalog, None, false).unwrap()).unwrap()
        );
    }

    #[test]
    fn evidence_dates_links_and_duplicate_ids_are_checked() {
        let catalog = fixture_catalog();
        let mut research = fixture_research();
        research.profiles[0].life.evidence[0].checked_on = "2026-02-29".into();
        assert!(research.validate(&catalog).is_err());
        research.profiles[0].life.evidence[0].checked_on = "2024-02-29".into();
        research.validate(&catalog).unwrap();
        research.profiles[0].retrievals[0].url = "https://example.org/has whitespace".into();
        assert!(research.validate(&catalog).is_err());
        research.profiles[0].retrievals[0].url = "file:///tmp/source".into();
        assert!(research.validate(&catalog).is_err());
        research.profiles[0].retrievals[0].url = "https://".into();
        assert!(research.validate(&catalog).is_err());
        research.profiles[0].retrievals[0].url = "https://example.org".into();
        let duplicate = research.profiles[0].retrievals[0].clone();
        research.profiles[0].retrievals.push(duplicate);
        assert!(
            research
                .validate(&catalog)
                .unwrap_err()
                .contains("duplicate retrieval")
        );
    }

    #[test]
    fn missing_files_and_unknown_json_fields_are_errors() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("research.json");
        let catalog = fixture_catalog();
        assert!(read_research(&path, &catalog).is_err());
        let research = fixture_research();
        let mut value = serde_json::to_value(research).unwrap();
        value["profiles"][0]["silently_inferred_dead"] = true.into();
        fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(
            read_research(&path, &catalog)
                .unwrap_err()
                .contains("unknown field")
        );
        value["profiles"][0]
            .as_object_mut()
            .unwrap()
            .remove("silently_inferred_dead");
        fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        read_research(&path, &catalog).unwrap();
    }

    #[test]
    fn markdown_exposes_evidence_rights_gaps_and_pinned_canonical_links() {
        let catalog = fixture_catalog();
        let research = fixture_research();
        let result = report(&research, &catalog, None, false).unwrap();
        let markdown = render_markdown(&result);
        assert!(markdown.contains("https://example.org/inventory"));
        assert!(markdown.contains("https://example.org/evidence"));
        assert!(markdown.contains("Rights — not established"));
        assert!(markdown.contains("Catalogue reconciliation: not started"));
        assert!(markdown.contains("322 BCE"));
        assert!(markdown.contains("Next step: Inspect the edition"));
        assert!(markdown.contains(&format!(
            "/blob/{}/corpus/sources/joint-work.json",
            "a".repeat(40)
        )));
        assert!(markdown.contains("**2 without a profile**"));
    }
}
