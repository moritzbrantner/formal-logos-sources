//! Read-only views of the committed Formal Logos corpus.
//!
//! A source reading and its research connections remain authoritative in Formal
//! Logos. These projections neither materialize witnesses nor compute proof status.

use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const REPOSITORY_URL: &str = "https://github.com/moritzbrantner/formal-logos";

#[derive(Clone, Debug)]
pub struct Record {
    pub path: String,
    pub value: Value,
}

#[derive(Debug)]
pub struct Repository {
    root: PathBuf,
    pub revision: String,
}

fn git(root: &Path) -> Command {
    let mut command = Command::new("git");
    command.arg("-C").arg(root);
    // Reading a partial clone must not start a fetch or acquire write locks.
    command.env("GIT_NO_LAZY_FETCH", "1");
    command.env("GIT_OPTIONAL_LOCKS", "0");
    command.env("GIT_NO_REPLACE_OBJECTS", "1");
    command.env("GIT_TERMINAL_PROMPT", "0");
    command
}

fn git_output(mut command: Command, operation: &str) -> Result<Vec<u8>, String> {
    let output = command
        .output()
        .map_err(|error| format!("{operation}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{operation}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(output.stdout)
}

impl Repository {
    pub fn open(path: &Path, revision: &str) -> Result<Self, String> {
        if revision.is_empty() {
            return Err("Formal Logos revision must not be empty".into());
        }
        let mut command = git(path);
        command.args([
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{revision}^{{commit}}"),
        ]);
        let bytes = git_output(command, "resolve Formal Logos commit")?;
        let revision = std::str::from_utf8(&bytes)
            .map_err(|_| "Git returned a non-UTF-8 commit identity")?
            .trim()
            .to_owned();
        if !matches!(revision.len(), 40 | 64)
            || !revision.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("Git returned an invalid commit identity".into());
        }
        Ok(Self {
            root: path.to_owned(),
            revision,
        })
    }

    /// Read JSON records from Git objects at the resolved commit. Working-tree
    /// edits, untracked files and replacement objects do not enter the projection.
    pub fn records(&self, kind: &str) -> Result<Vec<Record>, String> {
        if kind.is_empty()
            || !kind
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            return Err(format!("invalid corpus record kind: {kind:?}"));
        }
        let prefix = format!("corpus/{kind}/");
        let mut command = git(&self.root);
        command.args([
            "ls-tree",
            "--full-tree",
            "-r",
            "-z",
            &self.revision,
            "--",
            &prefix,
        ]);
        let tree = git_output(command, "list committed corpus records")?;
        let mut entries = Vec::new();
        for entry in tree
            .split(|byte| *byte == 0)
            .filter(|item| !item.is_empty())
        {
            let entry =
                std::str::from_utf8(entry).map_err(|_| "corpus Git path is not valid UTF-8")?;
            let (metadata, path) = entry.split_once('\t').ok_or("invalid Git tree entry")?;
            if !path.starts_with(&prefix) || !path.ends_with(".json") {
                continue;
            }
            let fields: Vec<_> = metadata.split_whitespace().collect();
            if fields.len() != 3 || fields[1] != "blob" || !matches!(fields[0], "100644" | "100755")
            {
                return Err(format!("{path}: corpus records must be regular Git blobs"));
            }
            entries.push((path.to_owned(), fields[2].to_owned()));
        }
        entries.sort();
        if entries.is_empty() {
            return Ok(Vec::new());
        }

        let input: Vec<u8> = entries
            .iter()
            .flat_map(|(_, object)| object.bytes().chain(std::iter::once(b'\n')))
            .collect();
        let mut child = git(&self.root)
            .args(["cat-file", "--batch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("read committed corpus objects: {error}"))?;
        let mut stdin = child.stdin.take().ok_or("cannot pipe Git object queries")?;
        // Feed input while wait_with_output drains both output pipes. A large
        // corpus can otherwise fill the input and output pipes simultaneously.
        let writer = std::thread::spawn(move || stdin.write_all(&input));
        let output = child.wait_with_output();
        let written = writer
            .join()
            .map_err(|_| "Git object query writer stopped unexpectedly")?;
        let output = output.map_err(|error| format!("read Git objects: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "read Git objects: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        written.map_err(|error| format!("send Git object queries: {error}"))?;

        let mut remaining = output.stdout.as_slice();
        let mut records = Vec::with_capacity(entries.len());
        for (path, object) in entries {
            let newline = remaining
                .iter()
                .position(|byte| *byte == b'\n')
                .ok_or_else(|| format!("{path}: incomplete Git object response"))?;
            let header = std::str::from_utf8(&remaining[..newline])
                .map_err(|_| format!("{path}: invalid Git object header"))?;
            let fields: Vec<_> = header.split_whitespace().collect();
            if fields.len() != 3 || fields[0] != object || fields[1] != "blob" {
                return Err(format!("{path}: unavailable committed Git blob: {header}"));
            }
            let size: usize = fields[2]
                .parse()
                .map_err(|_| format!("{path}: invalid Git blob length"))?;
            remaining = &remaining[newline + 1..];
            if size >= remaining.len() || remaining[size] != b'\n' {
                return Err(format!("{path}: truncated Git blob"));
            }
            let value = serde_json::from_slice(&remaining[..size])
                .map_err(|error| format!("{path}: invalid committed JSON: {error}"))?;
            remaining = &remaining[size + 1..];
            records.push(Record { path, value });
        }
        if !remaining.is_empty() {
            return Err("unexpected trailing Git object response".into());
        }
        Ok(records)
    }
}

fn string<'a>(record: &'a Record, key: &str) -> Result<&'a str, String> {
    record
        .value
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{}: {key} must be a non-empty string", record.path))
}

fn strings<'a>(record: &'a Record, key: &str) -> Result<Vec<&'a str>, String> {
    match record.value.get(key) {
        None => Ok(Vec::new()),
        Some(Value::Array(values)) => values
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| {
                        format!("{}: {key} contains a non-string or empty link", record.path)
                    })
            })
            .collect(),
        _ => Err(format!("{}: {key} must be a string array", record.path)),
    }
}

struct Index {
    records: Vec<Record>,
    aliases: BTreeMap<String, BTreeSet<usize>>,
    kind: String,
}

impl Index {
    fn new(records: Vec<Record>, kind: &str) -> Result<Self, String> {
        let mut aliases: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
        let mut ids = BTreeSet::new();
        for (index, record) in records.iter().enumerate() {
            let id = string(record, "id")?;
            if !ids.insert(id) {
                return Err(format!("duplicate {kind} ID: {id}"));
            }
            for key in ["id", "legacy_id", "label"] {
                if let Some(alias) = record.value.get(key) {
                    let alias = alias
                        .as_str()
                        .filter(|value| !value.is_empty())
                        .ok_or_else(|| format!("{}: invalid {key}", record.path))?;
                    aliases.entry(alias.to_owned()).or_default().insert(index);
                }
            }
        }
        Ok(Self {
            records,
            aliases,
            kind: kind.to_owned(),
        })
    }

    fn resolve(&self, query: &str) -> Result<&Record, String> {
        let matches = self
            .aliases
            .get(query)
            .ok_or_else(|| format!("unknown {} reference: {query}", self.kind))?;
        if matches.len() != 1 {
            return Err(format!("ambiguous {} reference: {query}", self.kind));
        }
        let index = matches
            .iter()
            .next()
            .ok_or_else(|| format!("unknown {} reference: {query}", self.kind))?;
        Ok(&self.records[*index])
    }

    fn linked(&self, owners: &[&Record], key: &str) -> Result<Vec<&Record>, String> {
        let mut resolved = BTreeMap::new();
        for owner in owners {
            for link in strings(owner, key)? {
                let record = self.resolve(link)?;
                resolved.insert(string(record, "id")?.to_owned(), record);
            }
        }
        Ok(resolved.into_values().collect())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct EntityRef {
    pub id: String,
    pub label: String,
    pub path: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AttemptConnection {
    pub attempt: EntityRef,
    pub recorded_outcome: String,
    pub summary: String,
}

#[derive(Debug, Serialize)]
pub struct ClaimConnection {
    pub claim: EntityRef,
    pub locator: String,
    pub interpretation: String,
    pub recorded_source_reading_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_basis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_witness_status: Option<String>,
    pub statement: EntityRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lean_declaration: Option<String>,
    pub definitions: Vec<EntityRef>,
    pub disambiguations: Vec<EntityRef>,
    pub direct_argument_consumers: Vec<EntityRef>,
    pub attempts: Vec<AttemptConnection>,
}

#[derive(Debug, Serialize)]
pub struct PassageConnection {
    pub passage: EntityRef,
    pub locator: String,
    pub witness: Value,
    pub selection: Value,
    pub claims: Vec<ClaimConnection>,
}

#[derive(Debug, Serialize)]
pub struct ConnectionReport {
    pub revision: String,
    pub source: EntityRef,
    pub author: String,
    pub title: String,
    pub citation_system: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator_filter: Option<String>,
    pub passages: Vec<PassageConnection>,
}

fn url_path(path: &str) -> String {
    let mut result = String::new();
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.' | b'~') {
            result.push(char::from(byte));
        } else {
            result.push_str(&format!("%{byte:02X}"));
        }
    }
    result
}

fn reference(record: &Record, revision: &str) -> Result<EntityRef, String> {
    Ok(EntityRef {
        id: string(record, "id")?.to_owned(),
        label: string(record, "label")?.to_owned(),
        path: record.path.clone(),
        url: format!(
            "{REPOSITORY_URL}/blob/{revision}/{}",
            url_path(&record.path)
        ),
        legacy_id: record
            .value
            .get("legacy_id")
            .and_then(Value::as_str)
            .map(str::to_owned),
    })
}

fn references(records: Vec<&Record>, revision: &str) -> Result<Vec<EntityRef>, String> {
    records
        .into_iter()
        .map(|record| reference(record, revision))
        .collect()
}

fn reverse_links<'a>(
    records: &'a [Record],
    key: &str,
    targets: &Index,
) -> Result<BTreeMap<String, Vec<&'a Record>>, String> {
    let mut reverse: BTreeMap<String, Vec<&Record>> = BTreeMap::new();
    for record in records {
        let mut ids = BTreeSet::new();
        for target in strings(record, key)? {
            ids.insert(string(targets.resolve(target)?, "id")?.to_owned());
        }
        for id in ids {
            reverse.entry(id).or_default().push(record);
        }
    }
    Ok(reverse)
}

fn locator_matches(locator: &str, filter: &str) -> bool {
    locator.strip_prefix(filter).is_some_and(|remainder| {
        remainder.is_empty()
            || remainder.chars().next().is_some_and(|next| {
                next.is_whitespace()
                    || matches!(
                        next,
                        '.' | ':' | ';' | ',' | '/' | '(' | '[' | '-' | '–' | '—'
                    )
            })
    })
}

/// Project existing source readings and their direct research connections.
pub fn connections(
    repository: &Repository,
    source_query: &str,
    locator: Option<&str>,
) -> Result<ConnectionReport, String> {
    if locator.is_some_and(str::is_empty) {
        return Err("locator filter must not be empty".into());
    }
    let sources = Index::new(repository.records("sources")?, "source")?;
    let source = sources.resolve(source_query)?;
    let source_id = string(source, "id")?;
    let mut passages = Vec::new();
    let mut snapshots = BTreeMap::new();
    for manifest in repository.records("passages")? {
        let snapshot = manifest
            .value
            .get("snapshot")
            .filter(|value| value.is_object())
            .ok_or_else(|| format!("{}: missing snapshot object", manifest.path))?;
        let snapshot_source = snapshot
            .get("source_id")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{}: missing snapshot source_id", manifest.path))?;
        if string(sources.resolve(snapshot_source)?, "id")? != source_id {
            continue;
        }
        let values = manifest
            .value
            .get("passages")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("{}: missing passages array", manifest.path))?;
        for value in values {
            let record = Record {
                path: manifest.path.clone(),
                value: value.clone(),
            };
            let id = string(&record, "id")?.to_owned();
            if snapshots.insert(id.clone(), snapshot.clone()).is_some() {
                return Err(format!("duplicate passage ID: {id}"));
            }
            passages.push(record);
        }
    }
    let passages = Index::new(passages, "passage")?;
    let statements = Index::new(repository.records("statements")?, "statement")?;
    let definitions = Index::new(repository.records("definitions")?, "definition")?;
    let disambiguations = Index::new(repository.records("disambiguations")?, "disambiguation")?;
    let arguments = Index::new(repository.records("arguments")?, "argument")?;
    let attempts = Index::new(repository.records("attempts")?, "attempt")?;
    let consumers_by_statement =
        reverse_links(&arguments.records, "premise_statement_ids", &statements)?;
    let attempts_by_statement = reverse_links(&attempts.records, "statement_ids", &statements)?;
    let senses_by_definition = reverse_links(
        &disambiguations.records,
        "sense_definition_ids",
        &definitions,
    )?;
    let mut claims_by_passage: BTreeMap<String, Vec<Record>> = BTreeMap::new();
    for claim in Index::new(repository.records("claims")?, "claim")?.records {
        if string(sources.resolve(string(&claim, "source_id")?)?, "id")? != source_id {
            continue;
        }
        let claim_locator = string(&claim, "locator")?;
        let mut matched = Vec::new();
        for passage in &passages.records {
            let prefixes = strings(passage, "claim_locator_prefixes")?;
            if prefixes.is_empty() {
                return Err(format!(
                    "{}: passage has no claim locator prefixes",
                    passage.path
                ));
            }
            if prefixes
                .iter()
                .any(|prefix| claim_locator.starts_with(prefix))
            {
                matched.push(string(passage, "id")?);
            }
        }
        if matched.len() != 1 {
            return Err(format!(
                "{}: claim must resolve to exactly one source passage; matched {}",
                claim.path,
                matched.len()
            ));
        }
        if let Some(explicit) = claim.value.get("passage_id") {
            let explicit = explicit
                .as_str()
                .ok_or_else(|| format!("{}: passage_id must be a string", claim.path))?;
            if string(passages.resolve(explicit)?, "id")? != matched[0] {
                return Err(format!(
                    "{}: passage_id disagrees with locator binding",
                    claim.path
                ));
            }
        }
        claims_by_passage
            .entry(matched[0].to_owned())
            .or_default()
            .push(claim);
    }

    let mut report = ConnectionReport {
        revision: repository.revision.clone(),
        source: reference(source, &repository.revision)?,
        author: string(source, "author")?.to_owned(),
        title: string(source, "work")?.to_owned(),
        citation_system: string(source, "citation_system")?.to_owned(),
        locator_filter: locator.map(str::to_owned),
        passages: Vec::new(),
    };
    for passage in &passages.records {
        let passage_locator = string(passage, "locator")?;
        if locator.is_some_and(|filter| !locator_matches(passage_locator, filter)) {
            continue;
        }
        let passage_id = string(passage, "id")?;
        let witness = snapshots
            .get(passage_id)
            .ok_or_else(|| format!("missing witness for passage {passage_id}"))?
            .clone();
        let mut selection = serde_json::Map::new();
        for key in [
            "cts_path",
            "tei_xml_id",
            "html_selector",
            "pdf_pages",
            "raw_text",
            "text_sha256",
            "start_seconds",
            "end_seconds",
            "speaker",
            "speaker_attribution",
            "span_sha256",
            "pages",
            "section",
        ] {
            if let Some(value) = passage.value.get(key) {
                selection.insert(key.to_owned(), value.clone());
            }
        }
        let mut connected = PassageConnection {
            passage: reference(passage, &repository.revision)?,
            locator: passage_locator.to_owned(),
            witness,
            selection: Value::Object(selection),
            claims: Vec::new(),
        };
        for claim in claims_by_passage.get(passage_id).into_iter().flatten() {
            let statement = statements.resolve(string(claim, "statement_id")?)?;
            let statement_id = string(statement, "id")?;
            let owners = [claim, statement];
            let linked_definitions = definitions.linked(&owners, "definition_ids")?;
            let mut linked_disambiguations: BTreeMap<_, _> = disambiguations
                .linked(&owners, "disambiguation_ids")?
                .into_iter()
                .map(|record| Ok((string(record, "id")?.to_owned(), record)))
                .collect::<Result<_, String>>()?;
            for definition in &linked_definitions {
                for disambiguation in senses_by_definition
                    .get(string(definition, "id")?)
                    .into_iter()
                    .flatten()
                {
                    linked_disambiguations
                        .insert(string(disambiguation, "id")?.to_owned(), *disambiguation);
                }
            }
            let consumers = references(
                consumers_by_statement
                    .get(statement_id)
                    .cloned()
                    .unwrap_or_default(),
                &repository.revision,
            )?;
            let mut linked_attempts = Vec::new();
            for attempt in attempts_by_statement
                .get(statement_id)
                .into_iter()
                .flatten()
            {
                linked_attempts.push(AttemptConnection {
                    attempt: reference(attempt, &repository.revision)?,
                    recorded_outcome: string(attempt, "outcome")?.to_owned(),
                    summary: string(attempt, "summary")?.to_owned(),
                });
            }
            connected.claims.push(ClaimConnection {
                claim: reference(claim, &repository.revision)?,
                locator: string(claim, "locator")?.to_owned(),
                interpretation: string(claim, "interpretation")?.to_owned(),
                recorded_source_reading_status: string(claim, "status")?.to_owned(),
                translation_basis: claim
                    .value
                    .get("basis")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                recorded_witness_status: claim
                    .value
                    .get("witness")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                statement: reference(statement, &repository.revision)?,
                lean_declaration: statement
                    .value
                    .get("lean_declaration")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                definitions: references(linked_definitions, &repository.revision)?,
                disambiguations: references(
                    linked_disambiguations.into_values().collect(),
                    &repository.revision,
                )?,
                direct_argument_consumers: consumers,
                attempts: linked_attempts,
            });
        }
        report.passages.push(connected);
    }
    if locator.is_some() && report.passages.is_empty() {
        return Err(format!(
            "no registered passage matches locator {locator:?} for {source_query}"
        ));
    }
    Ok(report)
}

fn markdown_text(text: &str) -> String {
    let mut escaped = String::new();
    for character in text.chars() {
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

fn link(reference: &EntityRef) -> String {
    format!("[{}]({})", markdown_text(&reference.label), reference.url)
}

pub fn render_markdown(report: &ConnectionReport) -> String {
    let mut output = format!(
        "# {} — research connections\n\nAuthor attribution: {}  \nFormal Logos revision: [{}]({REPOSITORY_URL}/tree/{})\n\nSource: {}\n\nCitation system: {}\n\nClaim statuses and attempt outcomes reproduce the recorded source readings and research work. Proof validity and premise warrant remain with Formal Logos. Registered passages do not measure reading or whole-work coverage.\n",
        markdown_text(&report.title),
        markdown_text(&report.author),
        report.revision,
        report.revision,
        link(&report.source),
        markdown_text(&report.citation_system),
    );
    if let Some(locator) = &report.locator_filter {
        output.push_str(&format!(
            "\nLocator selection: {}\n",
            markdown_text(locator)
        ));
    }
    output.push_str(&format!(
        "\nRegistered passages in this selection: {}\n",
        report.passages.len()
    ));
    for passage in &report.passages {
        output.push_str(&format!(
            "\n## {}\n\nPassage manifest: {}\n",
            markdown_text(&passage.locator),
            link(&passage.passage)
        ));
        for key in [
            "language",
            "edition_urn",
            "repository",
            "revision",
            "path",
            "license",
            "source_url",
        ] {
            if let Some(value) = passage.witness.get(key).and_then(Value::as_str) {
                output.push_str(&format!(
                    "\n- {}: {}",
                    markdown_text(key),
                    markdown_text(value)
                ));
            }
        }
        output.push('\n');
        if passage.claims.is_empty() {
            output.push_str(
                "\nNo source reading is linked to this registered passage in this revision.\n",
            );
        }
        for claim in &passage.claims {
            output.push_str(&format!("\n### {}\n\nSource reading: {}  \nRecorded source-reading status: {}  \nLocator: {}\n\n{}\n\nStatement: {}\n", markdown_text(&claim.statement.label), link(&claim.claim), markdown_text(&claim.recorded_source_reading_status), markdown_text(&claim.locator), markdown_text(&claim.interpretation), link(&claim.statement)));
            if let Some(basis) = &claim.translation_basis {
                output.push_str(&format!(
                    "\nRecorded translation basis: {}\n",
                    markdown_text(basis)
                ));
            }
            if let Some(status) = &claim.recorded_witness_status {
                output.push_str(&format!(
                    "\nRecorded witness status: {}\n",
                    markdown_text(status)
                ));
            }
            for (heading, records) in [
                ("Definitions", &claim.definitions),
                ("Disambiguations", &claim.disambiguations),
            ] {
                output.push_str(&format!("\n**{heading}**\n\n"));
                if records.is_empty() {
                    output.push_str("No links recorded.\n");
                }
                for record in records {
                    output.push_str(&format!("- {}\n", link(record)));
                }
            }
            output.push_str(&format!(
                "\n<details>\n<summary>Direct argument consumers: {}</summary>\n\n",
                claim.direct_argument_consumers.len()
            ));
            for argument in &claim.direct_argument_consumers {
                output.push_str(&format!("- {}\n", link(argument)));
            }
            output.push_str("\n</details>\n");
            output.push_str(&format!(
                "\n<details>\n<summary>Recorded research attempts: {}</summary>\n\nOpen an attempt for its findings, rejected approaches and remaining questions.\n\n",
                claim.attempts.len()
            ));
            for attempt in &claim.attempts {
                output.push_str(&format!(
                    "- {} — recorded outcome: {}.\n",
                    link(&attempt.attempt),
                    markdown_text(&attempt.recorded_outcome)
                ));
            }
            output.push_str("\n</details>\n");
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    fn run(root: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn put(root: &Path, path: &str, value: Value) {
        let path = root.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    }

    fn commit(root: &Path) -> String {
        run(root, &["add", "."]);
        run(
            root,
            &[
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "-qm",
                "fixture",
            ],
        );
        run(root, &["rev-parse", "HEAD"])
    }

    fn fixture() -> (TempDir, String) {
        let dir = TempDir::new().unwrap();
        let root = dir.path();
        run(root, &["init", "-q"]);
        put(
            root,
            "corpus/sources/summa.json",
            json!({"id":"source", "legacy_id":"AQUINAS-ST", "label":"Aquinas, Summa", "author":"Thomas Aquinas", "work":"Summa", "citation_system":"question and article"}),
        );
        put(
            root,
            "corpus/passages/q19.json",
            json!({"snapshot":{"source_id":"AQUINAS-ST", "language":"lat", "repository":"example/text", "revision":"witness-commit", "path":"q19.xml", "git_blob_sha":"witness-blob", "license":"recorded terms"}, "passages":[{"id":"passage-1", "label":"Article one", "locator":"I q.19 a.1", "claim_locator_prefixes":["I q.19 a.1 "], "tei_xml_id":"article-1"}, {"id":"passage-10", "label":"Article ten", "locator":"I q.19 a.10", "claim_locator_prefixes":["I q.19 a.10 "], "tei_xml_id":"article-10"}]}),
        );
        put(
            root,
            "corpus/claims/will.json",
            json!({"id":"claim", "label":"Will follows intellect reading", "source_id":"source", "statement_id":"STMT-WILL", "locator":"I q.19 a.1 co.", "interpretation":"A recorded interpretation.", "status":"formalized", "definition_ids":["DEF-WILL"], "disambiguation_ids":["sense-explicit"]}),
        );
        put(
            root,
            "corpus/statements/will.json",
            json!({"id":"statement", "legacy_id":"STMT-WILL", "label":"Will follows intellect", "definition_ids":["def-intellect"], "lean_declaration":"Example.Will"}),
        );
        put(
            root,
            "corpus/definitions/will.json",
            json!({"id":"def-will", "legacy_id":"DEF-WILL", "label":"Will"}),
        );
        put(
            root,
            "corpus/definitions/intellect.json",
            json!({"id":"def-intellect", "label":"Intellect"}),
        );
        put(
            root,
            "corpus/disambiguations/explicit.json",
            json!({"id":"sense-explicit", "label":"Explicit sense", "sense_definition_ids":["def-intellect"]}),
        );
        put(
            root,
            "corpus/disambiguations/implicit.json",
            json!({"id":"sense-inferred", "label":"Linked definition senses", "sense_definition_ids":["DEF-WILL"]}),
        );
        put(
            root,
            "corpus/arguments/consumer.json",
            json!({"id":"argument", "label":"Divine will argument", "premise_statement_ids":["STMT-WILL"]}),
        );
        put(
            root,
            "corpus/attempts/will.json",
            json!({"id":"attempt", "label":"Prior investigation", "statement_ids":["statement"], "outcome":"blocked", "summary":"An additional premise remains open."}),
        );
        let revision = commit(root);
        (dir, revision)
    }

    #[test]
    fn reads_only_the_resolved_commit_not_dirty_or_untracked_files() {
        let (dir, revision) = fixture();
        let repository = Repository::open(dir.path(), &revision).unwrap();
        put(
            dir.path(),
            "corpus/sources/summa.json",
            json!({"id":"changed"}),
        );
        put(
            dir.path(),
            "corpus/sources/untracked.json",
            json!({"id":"untracked"}),
        );
        let records = repository.records("sources").unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].value["id"], "source");
        commit(dir.path());
        assert_eq!(
            repository.records("sources").unwrap()[0].value["id"],
            "source"
        );
        assert_eq!(repository.revision, revision);
        assert!(repository.records("../sources").is_err());
        assert!(Repository::open(dir.path(), "--help").is_err());
    }

    #[test]
    fn connects_aliases_glossary_consumers_and_attempts_without_promotion() {
        let (dir, revision) = fixture();
        let repository = Repository::open(dir.path(), &revision).unwrap();
        let report = connections(&repository, "AQUINAS-ST", Some("I q.19 a.1")).unwrap();
        assert_eq!(report.passages.len(), 1);
        assert_eq!(report.passages[0].passage.id, "passage-1");
        let claim = &report.passages[0].claims[0];
        assert_eq!(claim.statement.id, "statement");
        assert_eq!(claim.recorded_source_reading_status, "formalized");
        assert_eq!(claim.definitions.len(), 2);
        assert_eq!(claim.disambiguations.len(), 2);
        assert_eq!(claim.direct_argument_consumers[0].id, "argument");
        assert_eq!(claim.attempts[0].recorded_outcome, "blocked");
        assert!(claim.claim.url.contains(&format!("/blob/{revision}/")));
        let serialized = serde_json::to_value(&report).unwrap();
        assert!(serialized.get("proved").is_none());
        let markdown = render_markdown(&report);
        assert!(markdown.contains("Recorded source-reading status: formalized"));
        assert!(markdown.contains("registered") || markdown.contains("Registered"));
        assert!(markdown.contains("Direct argument consumers: 1"));
        assert_eq!(
            connections(&repository, "Aquinas, Summa", Some("I q.19"))
                .unwrap()
                .passages
                .len(),
            2
        );
        assert!(connections(&repository, "source", Some("I q.19 a.2")).is_err());
    }

    #[test]
    fn rejects_missing_and_ambiguous_passage_bindings() {
        let (dir, _) = fixture();
        let path = dir.path().join("corpus/passages/q19.json");
        let original: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let mut ambiguous = original.clone();
        ambiguous["passages"][1]["claim_locator_prefixes"] = json!(["I q.19 a.1 "]);
        put(dir.path(), "corpus/passages/q19.json", ambiguous);
        let repository = Repository::open(dir.path(), &commit(dir.path())).unwrap();
        assert!(
            connections(&repository, "source", None)
                .unwrap_err()
                .contains("matched 2")
        );
        let mut missing = original;
        missing["passages"][0]["claim_locator_prefixes"] = json!(["unused "]);
        put(dir.path(), "corpus/passages/q19.json", missing);
        let repository = Repository::open(dir.path(), &commit(dir.path())).unwrap();
        assert!(
            connections(&repository, "source", None)
                .unwrap_err()
                .contains("matched 0")
        );
    }

    #[test]
    fn rejects_unknown_semantic_links_and_ambiguous_source_labels() {
        let (dir, _) = fixture();
        put(
            dir.path(),
            "corpus/definitions/will.json",
            json!({"id":"other-definition", "label":"Will"}),
        );
        let repository = Repository::open(dir.path(), &commit(dir.path())).unwrap();
        assert!(
            connections(&repository, "source", None)
                .unwrap_err()
                .contains("unknown definition reference")
        );
        put(
            dir.path(),
            "corpus/sources/other.json",
            json!({"id":"source-2", "label":"Aquinas, Summa", "author":"Another attribution", "work":"Other work", "citation_system":"page"}),
        );
        let repository = Repository::open(dir.path(), &commit(dir.path())).unwrap();
        assert!(
            connections(&repository, "Aquinas, Summa", None)
                .unwrap_err()
                .contains("ambiguous source reference")
        );
    }

    #[test]
    fn metadata_only_sources_have_no_invented_reading_status() {
        let (dir, _) = fixture();
        put(
            dir.path(),
            "corpus/sources/metadata.json",
            json!({"id":"metadata", "label":"A joint attribution", "author":"One and Another", "work":"Unacquired work", "citation_system":"page"}),
        );
        let repository = Repository::open(dir.path(), &commit(dir.path())).unwrap();
        let report = connections(&repository, "metadata", None).unwrap();
        assert_eq!(report.author, "One and Another");
        assert!(report.passages.is_empty());
        assert_eq!(serde_json::to_value(report).unwrap().get("read"), None);
    }
}
