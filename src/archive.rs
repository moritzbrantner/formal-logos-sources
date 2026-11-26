//! Archive-on-demand copies of web pages cited by the research documents.
//!
//! Each cited URL is fetched at most once (live, or from the Wayback Machine when the live
//! page fails). The response bytes go to the ignored content-addressed store; the retrieval
//! metadata goes to a separate committed registry file. An archived copy is not a catalogue
//! edition: work identity, rights and passage fidelity still need human judgment.
use crate::citations::{CitedUrl, web_url};
use crate::hash;
use crate::registry::{Catalog, atomic_write, checked_object, object_path};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const USER_AGENT: &str = "formal-logos-sources-archiver/1";
pub const WAYBACK: &str = "https://web.archive.org";
const SCHEMA_VERSION: u32 = 1;
const MAX_BYTES: u64 = 256 * 1024 * 1024;
/// Live requests to one host are refused after this many 403 responses in one run.
const HOST_403_LIMIT: u32 = 2;
/// Hosts that cannot be archived as page bytes (video players).
const UNARCHIVABLE_HOSTS: &[&str] = &["youtube.com", "youtu.be", "m.youtube.com"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Via {
    Live,
    Wayback,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// The fetched URL: a cited URL without its fragment.
    pub url: String,
    /// UTC time of the attempt that produced this record.
    pub retrieved_at: String,
    /// Where the stored bytes came from; absent when nothing could be archived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<Via>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    /// Capture timestamp (YYYYMMDDhhmmss) of bytes served by the Wayback Machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wayback_timestamp: Option<String>,
    /// HTTP status of the failed live request, when the server answered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_status: Option<u16>,
    /// Transport error or reason the live request failed or was not made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_error: Option<String>,
    /// Why no bytes were archived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Set by a reviewer when the stored bytes are not the cited content (a bot-check page,
    /// a redirect stub): the reason. Such a copy does not count as a local copy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_content: Option<String>,
}

impl Entry {
    fn archived(&self) -> bool {
        self.sha256.is_some()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebArchive {
    pub schema_version: u32,
    pub user_agent: String,
    pub entries: Vec<Entry>,
}

impl Default for WebArchive {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            user_agent: USER_AGENT.into(),
            entries: Vec::new(),
        }
    }
}

/// The URL under which a cited URL is fetched and recorded: http(s), fragment removed.
pub fn archive_key(raw: &str) -> Option<String> {
    let mut url = web_url(raw)?;
    url.set_fragment(None);
    Some(url.into())
}

fn utc_date(days: i64) -> (i64, u32, u32) {
    // Proleptic Gregorian civil date from days since 1970-01-01 (H. Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(month <= 2), month, day)
}

pub fn utc_timestamp(time: SystemTime) -> String {
    let seconds = time
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64);
    let (year, month, day) = utc_date(seconds.div_euclid(86_400));
    let rest = seconds.rem_euclid(86_400);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3600,
        rest % 3600 / 60,
        rest % 60
    )
}

fn valid_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 20
        && bytes.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == b'-',
            10 => *c == b'T',
            13 | 16 => *c == b':',
            19 => *c == b'Z',
            _ => c.is_ascii_digit(),
        })
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

impl WebArchive {
    /// Offline structural validation; the stored bytes are not consulted.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(format!(
                "unsupported web archive schema {}",
                self.schema_version
            ));
        }
        if self.user_agent.trim().is_empty() {
            return Err("web archive has no user agent".into());
        }
        let mut previous: Option<&str> = None;
        for entry in &self.entries {
            let fail = |what: &str| Err(format!("web archive entry {}: {what}", entry.url));
            if archive_key(&entry.url).as_deref() != Some(entry.url.as_str()) {
                return fail("url must be an http(s) URL without a fragment");
            }
            if previous.is_some_and(|p| p >= entry.url.as_str()) {
                return fail("entries must be sorted by url and unique");
            }
            previous = Some(&entry.url);
            if !valid_timestamp(&entry.retrieved_at) {
                return fail("retrieved_at must be YYYY-MM-DDThh:mm:ssZ");
            }
            if entry
                .live_status
                .is_some_and(|s| !(100..600).contains(&s) || (200..300).contains(&s))
            {
                return fail("live_status must be a failing HTTP status");
            }
            if entry.live_error.as_deref().is_some_and(str::is_empty) {
                return fail("live_error must not be empty");
            }
            if let Some(reason) = &entry.not_content
                && (reason.trim().is_empty() || entry.via.is_none())
            {
                return fail("not_content needs a reason and an archived copy");
            }
            match entry.via {
                Some(via) => {
                    if !entry.sha256.as_deref().is_some_and(valid_sha256)
                        || entry.bytes.is_none_or(|b| b == 0)
                        || entry.status.is_none_or(|s| !(200..300).contains(&s))
                        || entry.final_url.as_deref().and_then(web_url).is_none()
                        || entry.content_type.as_deref().is_some_and(str::is_empty)
                        || entry.error.is_some()
                    {
                        return fail(
                            "an archived copy needs a 2xx status, final_url, sha256 and nonzero bytes",
                        );
                    }
                    let live_failed = entry.live_status.is_some() || entry.live_error.is_some();
                    match via {
                        Via::Live if live_failed || entry.wayback_timestamp.is_some() => {
                            return fail("a live copy has no live failure or Wayback timestamp");
                        }
                        Via::Wayback
                            if !live_failed
                                || !entry.wayback_timestamp.as_deref().is_some_and(|t| {
                                    t.len() == 14 && t.bytes().all(|c| c.is_ascii_digit())
                                }) =>
                        {
                            return fail(
                                "a Wayback copy needs the live failure and a 14-digit capture timestamp",
                            );
                        }
                        _ => {}
                    }
                }
                None => {
                    if entry.error.as_deref().is_none_or(str::is_empty)
                        || entry.sha256.is_some()
                        || entry.bytes.is_some()
                        || entry.final_url.is_some()
                        || entry.status.is_some()
                        || entry.content_type.is_some()
                        || entry.wayback_timestamp.is_some()
                    {
                        return fail("a failed attempt records only its live failure and error");
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get(&self, url: &str) -> Option<&Entry> {
        self.entries
            .binary_search_by(|e| e.url.as_str().cmp(url))
            .ok()
            .map(|i| &self.entries[i])
    }

    fn upsert(&mut self, entry: Entry) {
        match self
            .entries
            .binary_search_by(|e| e.url.as_str().cmp(&entry.url))
        {
            Ok(i) => self.entries[i] = entry,
            Err(i) => self.entries.insert(i, entry),
        }
    }
}

pub fn read_archive(path: &Path) -> Result<WebArchive, String> {
    let archive: WebArchive = match fs::read(path) {
        Ok(bytes) => {
            serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))?
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => WebArchive::default(),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    archive
        .validate()
        .map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(archive)
}

fn write_archive(path: &Path, archive: &WebArchive) -> Result<(), String> {
    archive.validate()?;
    let serialized = serde_json::to_vec_pretty(archive).map_err(|e| e.to_string())?;
    atomic_write(path, &[serialized.as_slice(), b"\n"].concat(), false)
}

/// Where a local copy of a cited URL exists, if anywhere. Objects are checked for presence,
/// not re-hashed; `verify` and the Formal Logos materializer check bytes.
pub fn local_copy(
    catalog: &Catalog,
    archive: &WebArchive,
    store: &Path,
    source_cache: Option<&Path>,
    target: &CitedUrl,
) -> Option<serde_json::Value> {
    let editions = target
        .edition_ids
        .iter()
        .filter_map(|id| catalog.editions.iter().find(|e| &e.id == id));
    for edition in editions.clone() {
        if let Some(sha) = &edition.sha256
            && object_path(store, sha).is_file()
        {
            return Some(
                serde_json::json!({"kind": "edition", "edition_id": edition.id, "sha256": sha}),
            );
        }
    }
    if let Some(cache) = source_cache {
        for edition in editions {
            let Some(reference) = &edition.formal_logos else {
                continue;
            };
            let snapshot = &reference.snapshot;
            let web = snapshot["sha256"]
                .as_str()
                .and_then(|s| s.strip_prefix("sha256:"))
                .filter(|s| valid_sha256(s))
                .map(|s| cache.join("web").join(format!("{s}.html")));
            let raw = snapshot["git_blob_sha"]
                .as_str()
                .filter(|s| s.len() == 40 && s.bytes().all(|c| c.is_ascii_hexdigit()))
                .map(|s| cache.join("raw").join(s));
            if [web, raw].into_iter().flatten().any(|path| path.is_file()) {
                return Some(
                    serde_json::json!({"kind": "pinned_witness_cache", "edition_id": edition.id}),
                );
            }
        }
    }
    let entry = archive_key(&target.url).and_then(|key| archive.get(&key).cloned())?;
    if entry.not_content.is_some() {
        return None;
    }
    let sha = entry.sha256.as_deref()?;
    object_path(store, sha).is_file().then(|| {
        serde_json::json!({"kind": "web_archive", "via": entry.via, "retrieved_at": entry.retrieved_at, "sha256": sha})
    })
}

fn host(url: &str) -> String {
    web_url(url)
        .and_then(|u| u.host_str().map(str::to_owned))
        .unwrap_or_default()
}

/// Cited URLs (without fragments) that have no local copy of any kind, and those excluded.
pub fn targets(
    catalog: &Catalog,
    archive: &WebArchive,
    store: &Path,
    source_cache: Option<&Path>,
) -> (Vec<String>, Vec<(String, &'static str)>) {
    let mut covered = std::collections::BTreeSet::new();
    let mut wanted = std::collections::BTreeSet::new();
    let mut excluded = BTreeMap::new();
    for target in catalog
        .research_citations
        .iter()
        .flat_map(|c| &c.cited_urls)
    {
        let Some(key) = archive_key(&target.url) else {
            continue;
        };
        let video = UNARCHIVABLE_HOSTS.contains(&host(&key).trim_start_matches("www."));
        let copy = local_copy(catalog, archive, store, source_cache, target);
        // A copy that exists only through the archive record still counts as a target,
        // so that `--refresh` and `--retry-failed` can select it.
        if copy.as_ref().is_some_and(|c| c["kind"] != "web_archive") {
            covered.insert(key);
        } else if video {
            excluded.insert(key, "video page: not archived as page bytes");
        } else {
            wanted.insert(key);
        }
    }
    (
        wanted.difference(&covered).cloned().collect(),
        excluded
            .into_iter()
            .filter(|(k, _)| !covered.contains(k))
            .collect(),
    )
}

pub struct Response {
    pub status: u16,
    pub final_url: String,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
    pub retry_after: Option<u64>,
}

pub trait Fetch {
    fn get(&mut self, url: &str) -> Result<Response, String>;
}

/// HTTP(S) GET through the `curl` executable, following redirects and decoding
/// content encodings. Nothing from the user's curl configuration is read.
pub struct Curl {
    pub scratch: PathBuf,
}

impl Fetch for Curl {
    fn get(&mut self, url: &str) -> Result<Response, String> {
        fs::create_dir_all(&self.scratch).map_err(|e| e.to_string())?;
        let body = tempfile::NamedTempFile::new_in(&self.scratch).map_err(|e| e.to_string())?;
        let output = Command::new("curl")
            .args(["-q", "--silent", "--show-error", "--location"])
            .args(["--max-redirs", "10", "--compressed"])
            .args(["--proto", "=http,https", "--proto-redir", "=http,https"])
            .args(["--connect-timeout", "30", "--max-time", "300"])
            .args(["--max-filesize", &MAX_BYTES.to_string()])
            .args(["--user-agent", USER_AGENT])
            .args([
                "--header",
                "Accept: text/html,application/xhtml+xml,application/pdf;q=0.9,*/*;q=0.8",
            ])
            .arg("--output")
            .arg(body.path())
            .args(["--write-out", "%header{retry-after}\n%{json}", "--url", url])
            .output()
            .map_err(|e| format!("curl unavailable: {e}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let (retry_after, json) = stdout.split_once('\n').unwrap_or(("", &stdout));
        let report: serde_json::Value = serde_json::from_str(json).map_err(|_| {
            format!(
                "curl failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )
        })?;
        if !output.status.success() {
            return Err(report["errormsg"]
                .as_str()
                .map_or_else(|| format!("curl exit {}", output.status), str::to_owned));
        }
        let status = report["http_code"]
            .as_u64()
            .and_then(|s| u16::try_from(s).ok())
            .filter(|s| (100..600).contains(s))
            .ok_or("no HTTP status")?;
        Ok(Response {
            status,
            final_url: report["url_effective"].as_str().unwrap_or(url).to_owned(),
            content_type: report["content_type"]
                .as_str()
                .filter(|s| !s.is_empty())
                .map(str::to_owned),
            body: fs::read(body.path()).map_err(|e| e.to_string())?,
            retry_after: retry_after.trim().parse().ok(),
        })
    }
}

#[derive(Default)]
struct LiveFailure {
    status: Option<u16>,
    error: Option<String>,
    /// Where a redirected request ended, e.g. the publisher page behind a DOI.
    final_url: Option<String>,
}

pub struct RunOptions {
    /// Gap between requests to one host; never below 1.5 s for a public host, and raised
    /// for arXiv and the Wayback Machine.
    pub interval: Duration,
    /// Wait before the single retry of a 429/503 response without a usable Retry-After.
    pub backoff: Duration,
    pub wayback: String,
    pub refresh: bool,
    pub retry_failed: bool,
    pub limit: Option<usize>,
}

struct Session<'a> {
    fetch: &'a mut dyn Fetch,
    options: &'a RunOptions,
    last: HashMap<String, Instant>,
    refused: HashMap<String, u32>,
}

impl Session<'_> {
    fn gap(&self, host: &str) -> Duration {
        let floor = match host {
            "web.archive.org" | "archive.org" => Duration::from_secs(5),
            "arxiv.org" | "export.arxiv.org" => Duration::from_secs(3),
            // Loopback and reserved `.test` hosts (RFC 2606) serve only local test fixtures.
            "127.0.0.1" | "localhost" | "[::1]" => Duration::ZERO,
            _ if host.ends_with(".test") => Duration::ZERO,
            _ => Duration::from_millis(1500),
        };
        self.options.interval.max(floor)
    }

    fn request(&mut self, url: &str) -> Result<Response, String> {
        let host = host(url);
        let mut retried = false;
        loop {
            if let Some(last) = self.last.get(&host) {
                let gap = self.gap(&host);
                if last.elapsed() < gap {
                    std::thread::sleep(gap - last.elapsed());
                }
            }
            let response = self.fetch.get(url);
            self.last.insert(host.clone(), Instant::now());
            match response {
                Ok(r) if matches!(r.status, 429 | 503) && !retried => {
                    retried = true;
                    let wait = r
                        .retry_after
                        .map_or(self.options.backoff, Duration::from_secs)
                        .min(Duration::from_secs(120));
                    std::thread::sleep(wait);
                }
                other => return other,
            }
        }
    }

    fn live(&mut self, url: &str) -> Result<Response, LiveFailure> {
        let host = host(url);
        if self
            .refused
            .get(&host)
            .is_some_and(|n| *n >= HOST_403_LIMIT)
        {
            return Err(LiveFailure {
                error: Some(format!(
                    "not requested: {host} refused {HOST_403_LIMIT} earlier requests with 403"
                )),
                ..LiveFailure::default()
            });
        }
        match self.request(url) {
            Ok(r) if (200..300).contains(&r.status) && !r.body.is_empty() => Ok(r),
            Ok(r) => {
                // Count refusals against the host that answered, not a resolver such as doi.org.
                if r.status == 403 {
                    *self.refused.entry(self::host(&r.final_url)).or_default() += 1;
                }
                // A redirect into the Wayback Machine itself is not another page to look up.
                let redirected = (r.final_url != url
                    && web_url(&r.final_url).is_some()
                    && self::host(&r.final_url) != self::host(&self.options.wayback))
                .then_some(r.final_url);
                Err(if (200..300).contains(&r.status) {
                    LiveFailure {
                        error: Some("empty response body".into()),
                        final_url: redirected,
                        ..LiveFailure::default()
                    }
                } else {
                    LiveFailure {
                        status: Some(r.status),
                        final_url: redirected,
                        ..LiveFailure::default()
                    }
                })
            }
            Err(e) => Err(LiveFailure {
                error: Some(e),
                ..LiveFailure::default()
            }),
        }
    }

    /// The latest Wayback capture with HTTP status 200, as (timestamp, original URL).
    fn wayback(&mut self, url: &str) -> Result<(String, Response), String> {
        let base = self.options.wayback.trim_end_matches('/');
        let query = url::Url::parse_with_params(
            &format!("{base}/cdx/search/cdx"),
            [
                ("url", url),
                ("filter", "statuscode:200"),
                ("limit", "-1"),
                ("output", "json"),
                ("fl", "timestamp,original"),
            ],
        )
        .map_err(|e| e.to_string())?;
        let index = self
            .request(query.as_str())
            .map_err(|e| format!("Wayback index: {e}"))?;
        match index.status {
            200 => {}
            403 => return Err("Wayback excludes this URL (403)".into()),
            status => return Err(format!("Wayback index returned HTTP {status}")),
        }
        let rows: Vec<Vec<String>> = if index.body.iter().all(u8::is_ascii_whitespace) {
            Vec::new()
        } else {
            serde_json::from_slice(&index.body)
                .map_err(|e| format!("Wayback index is not JSON: {e}"))?
        };
        let (timestamp, original) = match rows.last().map(Vec::as_slice) {
            Some([timestamp, original]) if rows.len() > 1 => (timestamp, original),
            _ => return Err("no Wayback capture with HTTP 200".into()),
        };
        if timestamp.len() != 14 || !timestamp.bytes().all(|c| c.is_ascii_digit()) {
            return Err("Wayback index returned an invalid timestamp".into());
        }
        let capture = self
            .request(&format!("{base}/web/{timestamp}id_/{original}"))
            .map_err(|e| format!("Wayback capture: {e}"))?;
        if !(200..300).contains(&capture.status) || capture.body.is_empty() {
            return Err(format!(
                "Wayback capture {timestamp} returned HTTP {}",
                capture.status
            ));
        }
        // A capture can redirect to a neighbouring timestamp; record the one served.
        let served = capture
            .final_url
            .split("/web/")
            .nth(1)
            .and_then(|rest| rest.split_once("id_/"))
            .map(|(t, _)| t)
            .filter(|t| t.len() == 14 && t.bytes().all(|c| c.is_ascii_digit()))
            .unwrap_or(timestamp)
            .to_owned();
        Ok((served, capture))
    }

    fn archive(&mut self, url: &str) -> (Entry, Option<Vec<u8>>) {
        let mut entry = Entry {
            url: url.to_owned(),
            retrieved_at: utc_timestamp(SystemTime::now()),
            via: None,
            final_url: None,
            status: None,
            content_type: None,
            sha256: None,
            bytes: None,
            wayback_timestamp: None,
            live_status: None,
            live_error: None,
            error: None,
            not_content: None,
        };
        let (via, response) = match self.live(url) {
            Ok(response) => (Via::Live, response),
            Err(failure) => {
                entry.live_status = failure.status;
                entry.live_error = failure.error;
                // Try the cited URL's captures, then those of the page it redirected to.
                let mut result = self.wayback(url);
                if let (Err(first), Some(redirected)) = (&result, &failure.final_url) {
                    result = self.wayback(redirected).map_err(|second| {
                        format!("{first}; redirect target {redirected}: {second}")
                    });
                }
                match result {
                    Ok((timestamp, response)) => {
                        entry.wayback_timestamp = Some(timestamp);
                        (Via::Wayback, response)
                    }
                    Err(error) => {
                        entry.error = Some(error);
                        return (entry, None);
                    }
                }
            }
        };
        entry.retrieved_at = utc_timestamp(SystemTime::now());
        entry.via = Some(via);
        entry.final_url = Some(response.final_url).filter(|u| web_url(u).is_some());
        if entry.final_url.is_none() {
            entry.final_url = Some(url.to_owned());
        }
        entry.status = Some(response.status);
        entry.content_type = response.content_type;
        entry.sha256 = Some(hash(&response.body));
        entry.bytes = Some(response.body.len() as u64);
        (entry, Some(response.body))
    }
}

fn store_object(store: &Path, sha: &str, bytes: &[u8]) -> Result<bool, String> {
    let object = object_path(store, sha);
    if object.exists() {
        checked_object(store, sha)?;
        return Ok(false);
    }
    if let Err(error) = atomic_write(&object, bytes, true) {
        if object.exists() {
            checked_object(store, sha)?;
            return Ok(false);
        }
        return Err(error);
    }
    Ok(true)
}

/// Visit URLs one host at a time in rotation, so polite per-host gaps rarely stall the run.
fn interleave(urls: Vec<String>) -> Vec<String> {
    let mut by_host: BTreeMap<String, VecDeque<String>> = BTreeMap::new();
    for url in urls {
        by_host.entry(host(&url)).or_default().push_back(url);
    }
    let mut ordered = Vec::new();
    while !by_host.is_empty() {
        by_host.retain(|_, queue| {
            if let Some(url) = queue.pop_front() {
                ordered.push(url);
            }
            !queue.is_empty()
        });
    }
    ordered
}

pub struct Selection {
    /// URLs to request, in request order.
    pub urls: Vec<String>,
    pub already_archived: usize,
    pub recorded_failures: usize,
}

/// Choose which URLs a run requests. An archived URL is skipped while its object is in the
/// local store (a fresh clone without the bytes fetches it again); a recorded failure is
/// skipped unless `retry_failed` or `refresh` is set.
pub fn select(
    archive: &WebArchive,
    store: &Path,
    urls: &[String],
    options: &RunOptions,
) -> Result<Selection, String> {
    let mut keys: Vec<String> = urls
        .iter()
        .map(|u| archive_key(u).ok_or_else(|| format!("not an http(s) URL: {u}")))
        .collect::<Result<_, _>>()?;
    keys.sort();
    keys.dedup();
    let mut selection = Selection {
        urls: Vec::new(),
        already_archived: 0,
        recorded_failures: 0,
    };
    let mut wanted = Vec::new();
    for key in keys {
        match archive.get(&key) {
            _ if options.refresh => wanted.push(key),
            Some(e) if e.not_content.is_some() && !options.retry_failed => {
                selection.recorded_failures += 1;
            }
            Some(e)
                if e.sha256
                    .as_deref()
                    .is_some_and(|sha| object_path(store, sha).is_file()) =>
            {
                selection.already_archived += 1;
            }
            Some(e) if !e.archived() && !options.retry_failed => selection.recorded_failures += 1,
            _ => wanted.push(key),
        }
    }
    selection.urls = interleave(wanted);
    if let Some(limit) = options.limit {
        selection.urls.truncate(limit);
    }
    Ok(selection)
}

/// Archive the given URLs, recording every attempt in `archive_path` as it completes.
pub fn run(
    archive_path: &Path,
    store: &Path,
    urls: Vec<String>,
    options: &RunOptions,
    fetch: &mut dyn Fetch,
    progress: &mut dyn FnMut(&Entry),
) -> Result<serde_json::Value, String> {
    let parent = archive_path.parent().ok_or("archive file has no parent")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let lock = OpenOptions::new()
        .create(true)
        .append(true)
        .open(parent.join(".web-archive.lock"))
        .map_err(|e| e.to_string())?;
    lock.lock().map_err(|e| e.to_string())?;
    let mut archive = read_archive(archive_path)?;
    let selection = select(&archive, store, &urls, options)?;
    let mut session = Session {
        fetch,
        options,
        last: HashMap::new(),
        refused: HashMap::new(),
    };
    let (mut live, mut wayback, mut new_bytes, mut archived_bytes) = (0, 0, 0u64, 0u64);
    let mut failures = Vec::new();
    for url in &selection.urls {
        let (mut entry, body) = session.archive(url);
        // The same bytes fetched again are still the stub a reviewer marked.
        if let Some(previous) = archive.get(url)
            && previous.sha256.is_some()
            && previous.sha256 == entry.sha256
        {
            entry.not_content.clone_from(&previous.not_content);
        }
        if let (Some(body), Some(sha)) = (&body, &entry.sha256) {
            if store_object(store, sha, body)? {
                new_bytes += body.len() as u64;
            }
            archived_bytes += body.len() as u64;
            match entry.via {
                Some(Via::Wayback) => wayback += 1,
                _ => live += 1,
            }
        } else {
            failures.push(serde_json::json!({
                "url": entry.url, "host": host(&entry.url), "live_status": entry.live_status,
                "live_error": entry.live_error, "error": entry.error
            }));
        }
        progress(&entry);
        archive.upsert(entry);
        write_archive(archive_path, &archive)?;
    }
    drop(lock);
    Ok(serde_json::json!({
        "user_agent": USER_AGENT,
        "already_archived": selection.already_archived,
        "recorded_failures_skipped": selection.recorded_failures,
        "attempted": selection.urls.len(),
        "archived_live": live,
        "archived_wayback": wayback,
        "failed": failures.len(),
        "archived_bytes": archived_bytes,
        "new_object_bytes": new_bytes,
        "failures": failures,
        "evidence": "response bytes as served at retrieval; no work or edition identity, rights, or fidelity is established"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A scripted status and body, or a transport error.
    type Scripted<'a> = Result<(u16, &'a str), &'a str>;

    struct Script {
        responses: HashMap<String, VecDeque<Result<Response, String>>>,
        requests: Vec<String>,
    }

    impl Script {
        fn new(entries: Vec<(&str, Scripted)>) -> Self {
            let mut responses: HashMap<String, VecDeque<_>> = HashMap::new();
            for (url, response) in entries {
                responses.entry(url.to_owned()).or_default().push_back(
                    response
                        .map(|(status, body)| Response {
                            status,
                            final_url: url.to_owned(),
                            content_type: Some("text/html".into()),
                            body: body.as_bytes().to_vec(),
                            retry_after: None,
                        })
                        .map_err(str::to_owned),
                );
            }
            Self {
                responses,
                requests: Vec::new(),
            }
        }

        /// Make every scripted response for `url` end at `target`, as after a redirect.
        fn redirect(mut self, url: &str, target: &str) -> Self {
            for response in self.responses.get_mut(url).into_iter().flatten().flatten() {
                response.final_url = target.to_owned();
            }
            self
        }
    }

    impl Fetch for Script {
        fn get(&mut self, url: &str) -> Result<Response, String> {
            self.requests.push(url.to_owned());
            self.responses
                .get_mut(url)
                .and_then(VecDeque::pop_front)
                .unwrap_or_else(|| Err(format!("unexpected request {url}")))
        }
    }

    fn options() -> RunOptions {
        RunOptions {
            interval: Duration::ZERO,
            backoff: Duration::ZERO,
            wayback: "http://wayback.test".into(),
            refresh: false,
            retry_failed: false,
            limit: None,
        }
    }

    fn cdx(url: &str) -> String {
        url::Url::parse_with_params(
            "http://wayback.test/cdx/search/cdx",
            [
                ("url", url),
                ("filter", "statuscode:200"),
                ("limit", "-1"),
                ("output", "json"),
                ("fl", "timestamp,original"),
            ],
        )
        .unwrap()
        .into()
    }

    #[test]
    fn utc_timestamps_use_the_proleptic_gregorian_calendar() {
        assert_eq!(utc_timestamp(UNIX_EPOCH), "1970-01-01T00:00:00Z");
        let leap = UNIX_EPOCH + Duration::from_secs(951_782_400 + 3661);
        assert_eq!(utc_timestamp(leap), "2000-02-29T01:01:01Z");
        let later = UNIX_EPOCH + Duration::from_secs(1_798_761_599);
        assert_eq!(utc_timestamp(later), "2026-12-31T23:59:59Z");
    }

    #[test]
    fn live_wayback_and_failed_attempts_are_recorded_and_not_repeated() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("registry/web-archive.json");
        let store = dir.path().join("local");
        let gone = "https://gone.test/page";
        let blocked = [
            "https://wall.test/a",
            "https://wall.test/b",
            "https://wall.test/c",
        ];
        let mut script = Script::new(vec![
            ("https://ok.test/page", Ok((503, ""))),
            ("https://ok.test/page", Ok((200, "<p>live</p>"))),
            (gone, Ok((404, "missing"))),
            (
                &cdx(gone),
                Ok((
                    200,
                    r#"[["timestamp","original"],["20200102030405","https://gone.test/page"]]"#,
                )),
            ),
            (
                "http://wayback.test/web/20200102030405id_/https://gone.test/page",
                Ok((200, "<p>old</p>")),
            ),
            (blocked[0], Ok((403, ""))),
            (&cdx(blocked[0]), Ok((200, ""))),
            (blocked[1], Ok((403, ""))),
            (&cdx(blocked[1]), Ok((403, ""))),
            (&cdx(blocked[2]), Err("connection refused")),
        ]);
        let urls = vec![
            "https://ok.test/page#section".to_owned(),
            gone.to_owned(),
            blocked[0].to_owned(),
            blocked[1].to_owned(),
            blocked[2].to_owned(),
        ];
        let summary = run(
            &path,
            &store,
            urls.clone(),
            &options(),
            &mut script,
            &mut |_| {},
        )
        .unwrap();
        assert_eq!(summary["archived_live"], 1);
        assert_eq!(summary["archived_wayback"], 1);
        assert_eq!(summary["failed"], 3);
        // The third wall.test URL was not requested live after two 403 responses.
        assert!(!script.requests.iter().any(|r| r == blocked[2]));
        let archive = read_archive(&path).unwrap();
        let live = archive.get("https://ok.test/page").unwrap();
        assert_eq!(live.via, Some(Via::Live));
        assert_eq!(live.sha256.as_deref(), Some(hash(b"<p>live</p>").as_str()));
        assert!(checked_object(&store, live.sha256.as_ref().unwrap()).is_ok());
        let old = archive.get(gone).unwrap();
        assert_eq!((old.via, old.live_status), (Some(Via::Wayback), Some(404)));
        assert_eq!(old.wayback_timestamp.as_deref(), Some("20200102030405"));
        assert_eq!(
            archive.get(blocked[0]).unwrap().error.as_deref(),
            Some("no Wayback capture with HTTP 200")
        );
        assert_eq!(
            archive.get(blocked[1]).unwrap().error.as_deref(),
            Some("Wayback excludes this URL (403)")
        );
        assert!(
            archive
                .get(blocked[2])
                .unwrap()
                .live_error
                .as_deref()
                .unwrap()
                .contains("refused 2")
        );

        let before = script.requests.len();
        let rerun = run(&path, &store, urls, &options(), &mut script, &mut |_| {}).unwrap();
        assert_eq!(script.requests.len(), before);
        assert_eq!(
            (
                rerun["already_archived"].as_u64(),
                rerun["recorded_failures_skipped"].as_u64()
            ),
            (Some(2), Some(3))
        );
    }

    #[test]
    fn redirect_targets_are_tried_in_wayback_and_charged_with_their_refusals() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("web-archive.json");
        let store = dir.path().join("local");
        let doi = [
            "https://doi.test/10.1/x",
            "https://doi.test/10.1/y",
            "https://doi.test/10.1/z",
        ];
        let landing = ["https://publisher.test/x", "https://publisher.test/y"];
        let rows = r#"[["timestamp","original"],["20210101000000","https://publisher.test/x"]]"#;
        let mut script = Script::new(vec![
            (doi[0], Ok((403, ""))),
            (&cdx(doi[0]), Ok((200, ""))),
            (&cdx(landing[0]), Ok((200, rows))),
            (
                "http://wayback.test/web/20210101000000id_/https://publisher.test/x",
                Ok((200, "<p>landing</p>")),
            ),
            (doi[1], Ok((403, ""))),
            (&cdx(doi[1]), Ok((200, ""))),
            (&cdx(landing[1]), Ok((200, ""))),
            (doi[2], Ok((200, "<p>open</p>"))),
        ])
        .redirect(doi[0], landing[0])
        .redirect(doi[1], landing[1]);
        let urls = doi.map(str::to_owned).to_vec();
        let summary = run(&path, &store, urls, &options(), &mut script, &mut |_| {}).unwrap();
        assert_eq!(
            (
                &summary["archived_wayback"],
                &summary["failed"],
                &summary["archived_live"]
            ),
            (
                &serde_json::json!(1),
                &serde_json::json!(1),
                &serde_json::json!(1)
            )
        );
        let archive = read_archive(&path).unwrap();
        assert_eq!(
            archive.get(doi[0]).unwrap().wayback_timestamp.as_deref(),
            Some("20210101000000")
        );
        assert!(
            archive
                .get(doi[1])
                .unwrap()
                .error
                .as_deref()
                .unwrap()
                .contains("redirect target https://publisher.test/y: no Wayback capture")
        );
    }

    #[test]
    fn structure_validation_rejects_inconsistent_records() {
        let entry = Entry {
            url: "https://example.org/a".into(),
            retrieved_at: "2026-11-26T00:00:00Z".into(),
            via: Some(Via::Live),
            final_url: Some("https://example.org/a".into()),
            status: Some(200),
            content_type: Some("text/html".into()),
            sha256: Some("a".repeat(64)),
            bytes: Some(3),
            wayback_timestamp: None,
            live_status: None,
            live_error: None,
            error: None,
            not_content: None,
        };
        let mut archive = WebArchive {
            entries: vec![entry.clone()],
            ..WebArchive::default()
        };
        assert!(archive.validate().is_ok());
        archive.entries = vec![Entry {
            not_content: Some("publisher bot-check page".into()),
            ..entry.clone()
        }];
        assert!(archive.validate().is_ok());
        for broken in [
            Entry {
                url: "https://example.org/a#x".into(),
                ..entry.clone()
            },
            Entry {
                retrieved_at: "2026-11-26".into(),
                ..entry.clone()
            },
            Entry {
                status: Some(404),
                ..entry.clone()
            },
            Entry {
                via: Some(Via::Wayback),
                ..entry.clone()
            },
            Entry {
                via: None,
                ..entry.clone()
            },
            Entry {
                live_status: Some(403),
                ..entry.clone()
            },
            Entry {
                not_content: Some(" ".into()),
                ..entry.clone()
            },
        ] {
            archive.entries = vec![broken];
            assert!(archive.validate().is_err());
        }
        archive.entries = vec![entry.clone(), entry];
        assert!(archive.validate().is_err());
    }

    #[test]
    fn interleaving_rotates_hosts() {
        let urls = ["https://a.test/1", "https://a.test/2", "https://b.test/1"]
            .map(str::to_owned)
            .to_vec();
        assert_eq!(
            interleave(urls),
            ["https://a.test/1", "https://b.test/1", "https://a.test/2"]
        );
    }
}
