//! HTML passage extraction matching Formal Logos `web_snapshot` passages.
//!
//! This mirrors `scripts/source_evidence.py` in truth-core at commit
//! `c7368b99086aff67fa4f59f2157260e10b87ac69`: its `HtmlTokens` subclass of Python
//! 3.14's `html.parser.HTMLParser` (`convert_charrefs=True`), `normalize_space`,
//! block-per-line rendering and the `element_id`, `heading` and `from`/`to`
//! selectors. The tokenizer reproduces the parser's tag, comment, declaration,
//! raw-text and character-reference rules, including its tolerant handling of
//! malformed markup, so the same page bytes and selector give the same
//! `text_sha256`. `html5_entities.tsv` is Python's `html.entities.html5` table.
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Start(String, Vec<(String, String)>),
    End(String),
    Text(String),
}

impl Token {
    /// Python's attribute dictionary keeps the last value of a repeated attribute.
    fn attr(&self, name: &str) -> Option<&str> {
        match self {
            Token::Start(_, attrs) => attrs
                .iter()
                .rev()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.as_str()),
            _ => None,
        }
    }
}

const BLOCK_TAGS: &[&str] = &[
    "address",
    "article",
    "aside",
    "blockquote",
    "br",
    "center",
    "dd",
    "div",
    "dl",
    "dt",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hr",
    "li",
    "main",
    "nav",
    "ol",
    "p",
    "pre",
    "section",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "tr",
    "ul",
];
const SKIPPED_TAGS: &[&str] = &["script", "style", "noscript", "template", "title"];
const VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];
const CDATA_CONTENT_ELEMENTS: &[&str] =
    &["script", "style", "xmp", "iframe", "noembed", "noframes"];
const RCDATA_CONTENT_ELEMENTS: &[&str] = &["textarea", "title"];
const P_CLOSERS: &[&str] = &[
    "address",
    "article",
    "aside",
    "blockquote",
    "details",
    "dialog",
    "div",
    "dl",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "hr",
    "main",
    "menu",
    "nav",
    "ol",
    "p",
    "pre",
    "search",
    "section",
    "table",
    "ul",
];

fn heading_level(tag: &str) -> Option<u8> {
    match tag {
        "h1" => Some(1),
        "h2" => Some(2),
        "h3" => Some(3),
        "h4" => Some(4),
        "h5" => Some(5),
        "h6" => Some(6),
        _ => None,
    }
}

/// Elements whose end tag may be omitted, with the start tags that imply their end.
fn implied_end_by(tag: &str) -> Option<&'static [&'static str]> {
    Some(match tag {
        "p" => P_CLOSERS,
        "li" => &["li"],
        "dt" | "dd" => &["dt", "dd"],
        "tr" => &["tr", "thead", "tbody", "tfoot"],
        "td" | "th" => &["td", "th", "tr", "thead", "tbody", "tfoot"],
        "thead" | "tbody" => &["tbody", "tfoot"],
        "option" => &["option", "optgroup"],
        "optgroup" => &["optgroup"],
        _ => return None,
    })
}

/// Python's `str.isspace()`, used by `str.split()` and `str.strip()`.
fn py_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n'
            | '\u{b}'
            | '\u{c}'
            | '\r'
            | '\u{1c}'..='\u{1f}'
            | ' '
            | '\u{85}'
            | '\u{a0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200a}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202f}'
            | '\u{205f}'
            | '\u{3000}'
    )
}

pub fn normalize_space(text: &str) -> String {
    text.split(py_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn py_strip(text: &str) -> &str {
    text.trim_matches(py_space)
}

// ---------------------------------------------------------------------------
// Character references (Python's `html.unescape`)
// ---------------------------------------------------------------------------

fn entities() -> &'static std::collections::HashMap<&'static str, String> {
    static TABLE: std::sync::OnceLock<std::collections::HashMap<&'static str, String>> =
        std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        include_str!("html5_entities.tsv")
            .lines()
            .map(|line| {
                let (name, points) = line.split_once('\t').expect("entity table line");
                let value = points
                    .split(' ')
                    .map(|p| {
                        char::from_u32(u32::from_str_radix(p, 16).expect("entity code point"))
                            .expect("entity scalar value")
                    })
                    .collect();
                (name, value)
            })
            .collect()
    })
}

fn invalid_charref(num: u32) -> Option<char> {
    Some(match num {
        0x00 => '\u{fffd}',
        0x0d => '\r',
        0x80 => '\u{20ac}',
        0x81 => '\u{81}',
        0x82 => '\u{201a}',
        0x83 => '\u{0192}',
        0x84 => '\u{201e}',
        0x85 => '\u{2026}',
        0x86 => '\u{2020}',
        0x87 => '\u{2021}',
        0x88 => '\u{02c6}',
        0x89 => '\u{2030}',
        0x8a => '\u{0160}',
        0x8b => '\u{2039}',
        0x8c => '\u{0152}',
        0x8d => '\u{8d}',
        0x8e => '\u{017d}',
        0x8f => '\u{8f}',
        0x90 => '\u{90}',
        0x91 => '\u{2018}',
        0x92 => '\u{2019}',
        0x93 => '\u{201c}',
        0x94 => '\u{201d}',
        0x95 => '\u{2022}',
        0x96 => '\u{2013}',
        0x97 => '\u{2014}',
        0x98 => '\u{02dc}',
        0x99 => '\u{2122}',
        0x9a => '\u{0161}',
        0x9b => '\u{203a}',
        0x9c => '\u{0153}',
        0x9d => '\u{9d}',
        0x9e => '\u{017e}',
        0x9f => '\u{0178}',
        _ => return None,
    })
}

fn invalid_codepoint(num: u32) -> bool {
    matches!(num, 0x1..=0x8 | 0xb | 0xe..=0x1f | 0x7f..=0x9f | 0xfdd0..=0xfdef)
        || (num & 0xfffe == 0xfffe && num <= 0x10ffff)
}

fn numeric_reference(digits: &str, radix: u32) -> String {
    // Python's int() is unbounded; anything above U+10FFFF has the same result.
    let num = digits
        .chars()
        .try_fold(0u32, |acc, c| {
            let digit = c.to_digit(radix)?;
            acc.checked_mul(radix)?.checked_add(digit)
        })
        .unwrap_or(u32::MAX);
    if let Some(c) = invalid_charref(num) {
        return c.to_string();
    }
    if (0xd800..=0xdfff).contains(&num) || num > 0x10ffff {
        return '\u{fffd}'.to_string();
    }
    if invalid_codepoint(num) {
        return String::new();
    }
    char::from_u32(num).map_or_else(String::new, |c| c.to_string())
}

/// Python's `html.unescape`: `&(#[0-9]+;?|#[xX][0-9a-fA-F]+;?|[^\t\n\f <&#;]{1,32};?)`.
pub fn unescape(text: &str) -> String {
    if !text.contains('&') {
        return text.to_owned();
    }
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '&' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        let start = i + 1;
        let decimal = chars.get(start) == Some(&'#')
            && chars.get(start + 1).is_some_and(char::is_ascii_digit);
        let hex = chars.get(start) == Some(&'#')
            && matches!(chars.get(start + 1), Some('x' | 'X'))
            && chars.get(start + 2).is_some_and(char::is_ascii_hexdigit);
        if decimal || hex {
            let first = if decimal { start + 1 } else { start + 2 };
            let mut end = first;
            while chars.get(end).is_some_and(|c| {
                if decimal {
                    c.is_ascii_digit()
                } else {
                    c.is_ascii_hexdigit()
                }
            }) {
                end += 1;
            }
            let digits: String = chars[first..end].iter().collect();
            if chars.get(end) == Some(&';') {
                end += 1;
            }
            out.push_str(&numeric_reference(&digits, if decimal { 10 } else { 16 }));
            i = end;
            continue;
        }
        let mut end = start;
        while end < chars.len()
            && end - start < 32
            && !matches!(
                chars[end],
                '\t' | '\n' | '\u{c}' | ' ' | '<' | '&' | '#' | ';'
            )
        {
            end += 1;
        }
        if end == start {
            out.push('&');
            i += 1;
            continue;
        }
        if chars.get(end) == Some(&';') {
            end += 1;
        }
        let name: String = chars[start..end].iter().collect();
        let table = entities();
        if let Some(value) = table.get(name.as_str()) {
            out.push_str(value);
        } else if let Some(x) = (2..name.chars().count()).rev().find(|&x| {
            table.contains_key(chars[start..start + x].iter().collect::<String>().as_str())
        }) {
            let prefix: String = chars[start..start + x].iter().collect();
            out.push_str(&table[prefix.as_str()]);
            out.extend(&chars[start + x..end]);
        } else {
            out.push('&');
            out.push_str(&name);
        }
        i = end;
    }
    out
}

/// `html.parser._unescape_attrvalue`: only exact named references not followed by `=`.
fn unescape_attr(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = String::with_capacity(value.len());
    let mut copied = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut end = start;
        let numeric = bytes.get(start) == Some(&b'#');
        if numeric {
            let hex = matches!(bytes.get(start + 1), Some(b'x' | b'X'))
                && bytes.get(start + 2).is_some_and(u8::is_ascii_hexdigit);
            if hex {
                end = start + 2;
                while bytes.get(end).is_some_and(u8::is_ascii_hexdigit) {
                    end += 1;
                }
            } else {
                end = start + 1;
                while bytes.get(end).is_some_and(u8::is_ascii_digit) {
                    end += 1;
                }
                if end == start + 1 {
                    end = start;
                }
            }
        } else if bytes.get(start).is_some_and(u8::is_ascii_alphabetic) {
            while bytes.get(end).is_some_and(u8::is_ascii_alphanumeric) {
                end += 1;
            }
        }
        if end == start {
            i += 1;
            continue;
        }
        if matches!(bytes.get(end), Some(b';' | b'=')) {
            end += 1;
        }
        let reference = &value[i..end];
        let replace =
            numeric || (!reference.ends_with('=') && entities().contains_key(&reference[1..]));
        out.push_str(&value[copied..i]);
        if replace {
            out.push_str(&unescape(reference));
        } else {
            out.push_str(reference);
        }
        copied = end;
        i = end;
    }
    out.push_str(&value[copied..]);
    out
}

// ---------------------------------------------------------------------------
// Tokenizer (Python 3.14 `html.parser.HTMLParser`, fed once and closed)
// ---------------------------------------------------------------------------

fn tag_space(b: u8) -> bool {
    matches!(b, b'\t' | b'\n' | b'\r' | b'\x0c' | b' ')
}

/// `[\t\n\r\f ]*=[\t\n\r\f ]*('[^']*'|"[^"]*"|(?!['"])[^>\t\n\r\f ]*)`, with the
/// regex engine's backtracking. Returns the end and the raw value.
fn attr_value(s: &[u8], p: usize) -> Option<(usize, usize, usize)> {
    let mut q = p;
    while q < s.len() && tag_space(s[q]) {
        q += 1;
    }
    if s.get(q) != Some(&b'=') {
        return None;
    }
    q += 1;
    let after_equals = q;
    while q < s.len() && tag_space(s[q]) {
        q += 1;
    }
    loop {
        match s.get(q) {
            Some(&quote @ (b'\'' | b'"')) => {
                if let Some(close) = s[q + 1..].iter().position(|&b| b == quote) {
                    let end = q + 1 + close + 1;
                    return Some((end, q, end));
                }
                // Backtrack the whitespace after '='; a bare value may be empty.
                if q == after_equals {
                    return None;
                }
                q -= 1;
            }
            _ => {
                let mut end = q;
                while end < s.len()
                    && !matches!(s[end], b'>' | b'\t' | b'\n' | b'\r' | b'\x0c' | b' ')
                {
                    end += 1;
                }
                return Some((end, q, end));
            }
        }
    }
}

fn attr_name_end(s: &[u8], p: usize) -> Option<usize> {
    let lookbehind = p > 0
        && matches!(
            s[p - 1],
            b'\'' | b'"' | b'\t' | b'\n' | b'\r' | b'\x0c' | b' ' | b'/'
        );
    if !lookbehind
        || p >= s.len()
        || matches!(s[p], b'\t' | b'\n' | b'\r' | b'\x0c' | b' ' | b'/' | b'>')
    {
        return None;
    }
    let mut q = p + 1;
    while q < s.len()
        && !matches!(
            s[q],
            b'\t' | b'\n' | b'\r' | b'\x0c' | b' ' | b'/' | b'=' | b'>'
        )
    {
        q += 1;
    }
    Some(q)
}

fn tag_name_end(s: &[u8], p: usize) -> usize {
    let mut q = p + 1;
    while q < s.len() && !matches!(s[q], b'\t' | b'\n' | b'\r' | b'\x0c' | b' ' | b'/' | b'>') {
        q += 1;
    }
    q
}

/// `(?:[\t\n\r\f ]|/(?!>))*`
fn tolerant_space(s: &[u8], mut q: usize) -> usize {
    while q < s.len() && (tag_space(s[q]) || (s[q] == b'/' && s.get(q + 1) != Some(&b'>'))) {
        q += 1;
    }
    q
}

/// `locatetagend` matched at `p` (the tag name); returns its end.
fn locate_tag_end(s: &[u8], p: usize) -> usize {
    let mut q = tag_name_end(s, p);
    let slash_space = |b: u8| tag_space(b) || b == b'/';
    while q < s.len() && slash_space(s[q]) {
        q += 1;
    }
    while let Some(name_end) = attr_name_end(s, q) {
        q = attr_value(s, name_end).map_or(name_end, |(end, _, _)| end);
        while q < s.len() && slash_space(s[q]) {
            q += 1;
        }
    }
    if s.get(q) == Some(&b'>') {
        q += 1;
    }
    q
}

struct Parser<'a> {
    raw: &'a str,
    tokens: Vec<Token>,
    skip_depth: usize,
    cdata: Option<(String, bool)>,
    title: Option<String>,
    in_title: bool,
}

impl Parser<'_> {
    fn data(&mut self, text: String) {
        if self.in_title
            && let Some(title) = &mut self.title
        {
            title.push_str(&text);
        }
        if self.skip_depth == 0 {
            self.tokens.push(Token::Text(text));
        }
    }

    fn start(&mut self, tag: String, attrs: Vec<(String, String)>) {
        if tag == "title" && self.title.is_none() {
            self.title = Some(String::new());
            self.in_title = true;
        }
        if SKIPPED_TAGS.contains(&tag.as_str()) {
            self.skip_depth += 1;
        } else if self.skip_depth == 0 {
            self.tokens.push(Token::Start(tag, attrs));
        }
    }

    fn end(&mut self, tag: String) {
        if tag == "title" {
            self.in_title = false;
        }
        if SKIPPED_TAGS.contains(&tag.as_str()) {
            self.skip_depth = self.skip_depth.saturating_sub(1);
        } else if self.skip_depth == 0 {
            self.tokens.push(Token::End(tag));
        }
    }

    fn find(&self, needle: &str, from: usize) -> Option<usize> {
        self.raw.get(from..)?.find(needle).map(|p| p + from)
    }

    fn parse_starttag(&mut self, i: usize) -> Option<usize> {
        let s = self.raw.as_bytes();
        let endpos = locate_tag_end(s, i + 1);
        if s[endpos - 1] != b'>' {
            return None;
        }
        let name_end = tag_name_end(s, i + 1);
        let tag = self.raw[i + 1..name_end].to_lowercase();
        let mut k = tolerant_space(s, name_end);
        let mut attrs = Vec::new();
        while k < endpos {
            let Some(attr_end) = attr_name_end(s, k) else {
                break;
            };
            let name = self.raw[k..attr_end].to_lowercase();
            let (end, value) = match attr_value(s, attr_end) {
                Some((end, from, to)) => {
                    let mut value = &self.raw[from..to];
                    if value.len() >= 2
                        && (value.starts_with('\'') && value.ends_with('\'')
                            || value.starts_with('"') && value.ends_with('"'))
                    {
                        value = &value[1..value.len() - 1];
                    }
                    (end, unescape_attr(value))
                }
                None => (attr_end, String::new()),
            };
            attrs.push((name, value));
            k = tolerant_space(s, end);
        }
        let end = py_strip(&self.raw[k.min(endpos)..endpos]);
        if end != ">" && end != "/>" {
            self.data(self.raw[i..endpos].to_owned());
            return Some(endpos);
        }
        if end.ends_with("/>") {
            self.start(tag.clone(), attrs);
            self.end(tag);
        } else {
            self.start(tag.clone(), attrs);
            if CDATA_CONTENT_ELEMENTS.contains(&tag.as_str()) || tag == "plaintext" {
                self.cdata = Some((tag, false));
            } else if RCDATA_CONTENT_ELEMENTS.contains(&tag.as_str()) {
                self.cdata = Some((tag, true));
            }
        }
        Some(endpos)
    }

    fn parse_endtag(&mut self, i: usize) -> Option<usize> {
        let s = self.raw.as_bytes();
        self.find(">", i + 2)?;
        if !s.get(i + 2).is_some_and(u8::is_ascii_alphabetic) {
            if s.get(i + 2) == Some(&b'>') {
                return Some(i + 3);
            }
            return self.find(">", i + 2).map(|p| p + 1);
        }
        let j = locate_tag_end(s, i + 2);
        if s[j - 1] != b'>' {
            return None;
        }
        let tag = self.raw[i + 2..tag_name_end(s, i + 2)].to_lowercase();
        self.end(tag);
        self.cdata = None;
        Some(j)
    }

    fn parse_comment(&self, i: usize) -> Option<usize> {
        let s = self.raw.as_bytes();
        let mut p = i + 4;
        while p < s.len() {
            if s[p..].starts_with(b"-->") {
                return Some(p + 3);
            }
            if s[p..].starts_with(b"--!>") {
                return Some(p + 4);
            }
            p += 1;
        }
        let rest = &s[i + 4..];
        if rest.starts_with(b"->") {
            Some(i + 6)
        } else if rest.starts_with(b">") {
            Some(i + 5)
        } else {
            None
        }
    }

    fn parse_html_declaration(&self, i: usize) -> Option<usize> {
        if self.raw[i..].starts_with("<!--") {
            return self.parse_comment(i);
        }
        if self.raw[i..].starts_with("<![CDATA[") {
            return self.find("]]>", i + 9).map(|j| j + 3);
        }
        if is_doctype(&self.raw[i..]) {
            return self.find(">", i + 9).map(|j| j + 1);
        }
        self.find(">", i + 2).map(|p| p + 1)
    }

    /// `</elem(?=[\t\n\r\f />])`, ASCII case-insensitive.
    fn cdata_end(&self, elem: &str, from: usize) -> Option<usize> {
        if elem == "plaintext" {
            return None;
        }
        let s = self.raw.as_bytes();
        let pattern = format!("</{elem}");
        let mut p = from;
        while let Some(found) = self.find("</", p) {
            let after = found + pattern.len();
            if s.len() >= after
                && s[found..after].eq_ignore_ascii_case(pattern.as_bytes())
                && s.get(after)
                    .is_some_and(|&b| tag_space(b) || matches!(b, b'/' | b'>'))
            {
                return Some(found);
            }
            p = found + 1;
        }
        None
    }

    /// `goahead(0)` followed by `close()`: equivalent to one pass with `end=True`.
    fn run(&mut self) {
        let raw = self.raw;
        let s = raw.as_bytes();
        let n = s.len();
        let mut i = 0;
        while i < n {
            let (j, escapable) = match self.cdata.clone() {
                None => (raw[i..].find('<').map_or(n, |p| p + i), true),
                Some((elem, escapable)) => match self.cdata_end(&elem, i) {
                    Some(j) => (j, escapable),
                    None => break,
                },
            };
            if i < j {
                let text = &raw[i..j];
                self.data(if escapable {
                    unescape(text)
                } else {
                    text.to_owned()
                });
            }
            i = j;
            if i == n {
                break;
            }
            let rest = &raw[i..];
            let parsed = if s.get(i + 1).is_some_and(u8::is_ascii_alphabetic) {
                self.parse_starttag(i)
            } else if rest.starts_with("</") {
                self.parse_endtag(i)
            } else if rest.starts_with("<!--") {
                self.parse_comment(i)
            } else if rest.starts_with("<?") {
                self.find(">", i + 2).map(|p| p + 1)
            } else if rest.starts_with("<!") {
                self.parse_html_declaration(i)
            } else {
                self.data("<".to_owned());
                Some(i + 1)
            };
            i = match parsed {
                Some(k) => k,
                None => {
                    // Unterminated construct at end of input.
                    if rest.starts_with("</") && i + 2 == n {
                        self.data("</".to_owned());
                    }
                    n
                }
            };
        }
        if i < n {
            let escapable = self.cdata.as_ref().is_none_or(|(_, escapable)| *escapable);
            let text = &raw[i..n];
            self.data(if escapable {
                unescape(text)
            } else {
                text.to_owned()
            });
        }
    }
}

fn is_doctype(rest: &str) -> bool {
    rest.chars().take(9).collect::<String>().to_lowercase() == "<!doctype"
}

pub struct Page {
    pub tokens: Vec<Token>,
    /// Text of the first `<title>` element, as the parser reports it.
    pub title: Option<String>,
}

pub fn parse(text: &str) -> Page {
    let mut parser = Parser {
        raw: text,
        tokens: Vec::new(),
        skip_depth: 0,
        cdata: None,
        title: None,
        in_title: false,
    };
    parser.run();
    Page {
        tokens: parser.tokens,
        title: parser.title,
    }
}

// ---------------------------------------------------------------------------
// Rendering and selection
// ---------------------------------------------------------------------------

/// Text lines between block boundaries, each with the index of its first text token.
fn html_lines(tokens: &[Token]) -> Vec<(usize, String)> {
    let mut lines = Vec::new();
    let mut pieces = String::new();
    let mut first: Option<usize> = None;
    let flush =
        |pieces: &mut String, first: &mut Option<usize>, lines: &mut Vec<(usize, String)>| {
            let line = normalize_space(pieces);
            if let (false, Some(index)) = (line.is_empty(), *first) {
                lines.push((index, line));
            }
            pieces.clear();
            *first = None;
        };
    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Text(text) => {
                if first.is_none() && !py_strip(text).is_empty() {
                    first = Some(index);
                }
                pieces.push_str(text);
            }
            Token::Start(tag, _) | Token::End(tag) => {
                if BLOCK_TAGS.contains(&tag.as_str()) {
                    flush(&mut pieces, &mut first, &mut lines);
                }
            }
        }
    }
    flush(&mut pieces, &mut first, &mut lines);
    lines
}

pub fn render_text(tokens: &[Token]) -> String {
    html_lines(tokens)
        .into_iter()
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn unique(matches: &[usize], what: &str) -> Result<usize, String> {
    match matches {
        [one] => Ok(*one),
        _ => Err(format!(
            "html_selector {what} matched {} positions",
            matches.len()
        )),
    }
}

/// Index just past the element that starts at `start`, including implied end tags.
fn element_end(tokens: &[Token], start: usize) -> usize {
    let Token::Start(tag, _) = &tokens[start] else {
        return start + 1;
    };
    if VOID_TAGS.contains(&tag.as_str()) {
        return start + 1;
    }
    let mut open: Vec<&str> = vec![tag];
    for (index, token) in tokens.iter().enumerate().skip(start + 1) {
        match token {
            Token::Start(name, _) => {
                while let Some(top) = open.last() {
                    if implied_end_by(top).is_some_and(|closers| closers.contains(&name.as_str())) {
                        open.pop();
                    } else {
                        break;
                    }
                }
                if open.is_empty() {
                    return index;
                }
                if !VOID_TAGS.contains(&name.as_str()) {
                    open.push(name);
                }
            }
            Token::End(name) if !VOID_TAGS.contains(&name.as_str()) => {
                if let Some(position) = open.iter().rposition(|open_tag| open_tag == name) {
                    open.truncate(position);
                    if open.is_empty() {
                        return index + 1;
                    }
                } else if implied_end_by(tag).is_some() && name != "p" {
                    return index;
                }
            }
            _ => {}
        }
    }
    tokens.len()
}

fn marker_position(tokens: &[Token], marker: &Marker) -> Result<usize, String> {
    match marker {
        Marker::Anchor(value) => {
            let matches: Vec<usize> = tokens
                .iter()
                .enumerate()
                .filter(|(_, token)| {
                    matches!(token, Token::Start(..))
                        && (token.attr("id") == Some(value) || token.attr("name") == Some(value))
                })
                .map(|(index, _)| index)
                .collect();
            unique(&matches, &format!("anchor {value:?}"))
        }
        Marker::Text(value) => {
            let wanted = normalize_space(value);
            let matches: Vec<usize> = html_lines(tokens)
                .into_iter()
                .filter(|(_, line)| *line == wanted)
                .map(|(index, _)| index)
                .collect();
            unique(&matches, &format!("text {value:?}"))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Marker {
    Anchor(String),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector {
    ElementId(String),
    Heading(String),
    Range { from: Marker, to: Option<Marker> },
}

impl Selector {
    /// Accepts exactly the shapes of truth-core's `validate_html_selector`.
    pub fn from_json(value: &Value) -> Result<Self, String> {
        let object = value
            .as_object()
            .filter(|o| !o.is_empty())
            .ok_or("html_selector must be an object")?;
        let nonempty = |v: &Value| -> Option<String> {
            v.as_str().filter(|s| !s.is_empty()).map(str::to_owned)
        };
        if object.len() == 1 {
            for (key, make) in [
                ("element_id", Selector::ElementId as fn(String) -> Selector),
                ("heading", Selector::Heading),
            ] {
                if let Some(v) = object.get(key) {
                    return nonempty(v)
                        .map(make)
                        .ok_or_else(|| "html_selector value must be a non-empty string".into());
                }
            }
        }
        if object.contains_key("from") && object.keys().all(|k| k == "from" || k == "to") {
            let marker = |side: &str| -> Result<Marker, String> {
                let error = || {
                    format!("html_selector {side} must be {{\"anchor\": ...}} or {{\"text\": ...}}")
                };
                let m = object[side]
                    .as_object()
                    .filter(|m| m.len() == 1)
                    .ok_or_else(error)?;
                let (kind, v) = m.iter().next().ok_or_else(error)?;
                let v = nonempty(v).ok_or_else(error)?;
                match kind.as_str() {
                    "anchor" => Ok(Marker::Anchor(v)),
                    "text" => Ok(Marker::Text(v)),
                    _ => Err(error()),
                }
            };
            let from = marker("from")?;
            let to = if object.contains_key("to") {
                Some(marker("to")?)
            } else {
                None
            };
            return Ok(Selector::Range { from, to });
        }
        Err("html_selector must be {element_id}, {heading}, or {from, to?}".into())
    }

    pub fn select<'t>(&self, tokens: &'t [Token]) -> Result<&'t [Token], String> {
        match self {
            Selector::ElementId(id) => {
                let matches: Vec<usize> = tokens
                    .iter()
                    .enumerate()
                    .filter(|(_, token)| {
                        matches!(token, Token::Start(..)) && token.attr("id") == Some(id)
                    })
                    .map(|(index, _)| index)
                    .collect();
                let start = unique(&matches, &format!("element_id {id:?}"))?;
                Ok(&tokens[start..element_end(tokens, start)])
            }
            Selector::Heading(heading) => {
                let wanted = normalize_space(heading);
                let matches: Vec<usize> = tokens
                    .iter()
                    .enumerate()
                    .filter(|(index, token)| match token {
                        Token::Start(tag, _) if heading_level(tag).is_some() => {
                            normalize_space(&render_text(
                                &tokens[*index..element_end(tokens, *index)],
                            )) == wanted
                        }
                        _ => false,
                    })
                    .map(|(index, _)| index)
                    .collect();
                let start = unique(&matches, &format!("heading {heading:?}"))?;
                let Token::Start(tag, _) = &tokens[start] else {
                    unreachable!("heading matches are start tags")
                };
                let level = heading_level(tag).unwrap_or(6);
                let end = tokens
                    .iter()
                    .enumerate()
                    .skip(start + 1)
                    .find(|(_, token)| match token {
                        Token::Start(name, _) => heading_level(name).is_some_and(|l| l <= level),
                        _ => false,
                    })
                    .map_or(tokens.len(), |(index, _)| index);
                Ok(&tokens[start..end])
            }
            Selector::Range { from, to } => {
                let start = marker_position(tokens, from)?;
                let end = match to {
                    Some(to) => {
                        let end = marker_position(tokens, to)?;
                        if end <= start {
                            return Err("html_selector 'to' precedes 'from'".into());
                        }
                        end
                    }
                    None => tokens.len(),
                };
                Ok(&tokens[start..end])
            }
        }
    }
}

/// Decode, select and render one passage, as truth-core's `extract_web_text`.
pub fn extract(bytes: &[u8], encoding: &str, selector: &Selector) -> Result<String, String> {
    let page = decode(bytes, encoding)?;
    let text = render_text(selector.select(&parse(&page).tokens)?);
    if text.is_empty() {
        return Err("extracted passage text is empty".into());
    }
    Ok(text)
}

/// Strict UTF-8 and ISO-8859-1 are supported; other page encodings fail instead of guessing.
/// ISO-8859-1 maps each byte to the code point of the same value, as Python's `latin-1`
/// codec does (not the WHATWG `windows-1252` reading), so it never fails. Corpus
/// Thomisticum pages pinned by truth-core use it.
pub fn decode(bytes: &[u8], encoding: &str) -> Result<String, String> {
    match encoding.to_ascii_lowercase().replace('_', "-").as_str() {
        "utf-8" | "utf8" => {
            String::from_utf8(bytes.to_vec()).map_err(|e| format!("page is not valid utf-8: {e}"))
        }
        "iso-8859-1" | "iso8859-1" | "latin-1" | "latin1" | "l1" => {
            Ok(bytes.iter().map(|&b| char::from(b)).collect())
        }
        _ => Err(format!("unsupported page encoding: {encoding}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn text(html: &str, selector: Value) -> Result<String, String> {
        extract(html.as_bytes(), "utf-8", &Selector::from_json(&selector)?)
    }

    #[test]
    fn python_whitespace_and_references() {
        assert_eq!(normalize_space(" a\u{a0}b\u{1c}c\n\t d "), "a b c d");
        assert_eq!(
            unescape("&amp; &copy 2009 &notit; &#169; &#x41;"),
            "& © 2009 ¬it; © A"
        );
        assert_eq!(
            unescape("&#128;&#0;&#xD800;&#1;&#99999999999;"),
            "€\u{fffd}\u{fffd}\u{fffd}"
        );
        assert_eq!(
            unescape("AT&T &#; &unknown; &&amp;"),
            "AT&T &#; &unknown; &&"
        );
        assert_eq!(unescape_attr("a&amp;b&ampx&amp=c&#38;"), "a&b&ampx&amp=c&");
    }

    #[test]
    fn tokens_follow_python_parser_rules() {
        let page = parse(
            "<!DOCTYPE html><html><head><title>A &amp; B</title><script>if (a<b) {}</script></head>\
             <BODY><P Class=x id='one'>Hi<br/>there<!-- note --></p><a href=foo/>x</a> 1 < 2\
             <iframe><b>raw</b></iframe><![CDATA[hidden]]><p",
        );
        assert_eq!(page.title.as_deref(), Some("A & B"));
        assert_eq!(
            page.tokens,
            vec![
                Token::Start("html".into(), vec![]),
                Token::Start("head".into(), vec![]),
                Token::End("head".into()),
                Token::Start("body".into(), vec![]),
                Token::Start(
                    "p".into(),
                    vec![("class".into(), "x".into()), ("id".into(), "one".into())]
                ),
                Token::Text("Hi".into()),
                Token::Start("br".into(), vec![]),
                Token::End("br".into()),
                Token::Text("there".into()),
                Token::End("p".into()),
                Token::Start("a".into(), vec![("href".into(), "foo/".into())]),
                Token::Text("x".into()),
                Token::End("a".into()),
                Token::Text(" 1 ".into()),
                Token::Text("<".into()),
                Token::Text(" 2".into()),
                Token::Start("iframe".into(), vec![]),
                Token::Text("<b>raw</b>".into()),
                Token::End("iframe".into()),
            ]
        );
    }

    #[test]
    fn heading_selector_runs_to_next_heading_of_same_or_higher_level() {
        let html = "<h1>Question</h1><h2 id=a1>Article 1. Whether <i>one</i>?</h2><p>Body one\
                    <h3>Sub</h3><p>Detail</p><h2>Article 2. Whether two?</h2><p>Body two</p>";
        assert_eq!(
            text(html, json!({"heading": "Article 1.  Whether one?"})).unwrap(),
            "Article 1. Whether one?\nBody one\nSub\nDetail"
        );
        assert_eq!(
            text(html, json!({"heading": "Article 2. Whether two?"})).unwrap(),
            "Article 2. Whether two?\nBody two"
        );
        assert!(
            text(html, json!({"heading": "Missing"}))
                .unwrap_err()
                .contains("matched 0")
        );
    }

    #[test]
    fn element_and_range_selectors_use_implied_end_tags() {
        let html = "<div><p id=first>One <b>bold</b><p>Two</div><ul><li id=item>A<li>B</ul>\
                    <a name=s1></a><p>Three</p><a id=s2></a><p>Four</p>";
        assert_eq!(
            text(html, json!({"element_id": "first"})).unwrap(),
            "One bold"
        );
        assert_eq!(text(html, json!({"element_id": "item"})).unwrap(), "A");
        assert_eq!(
            text(
                html,
                json!({"from": {"anchor": "s1"}, "to": {"anchor": "s2"}})
            )
            .unwrap(),
            "Three"
        );
        assert_eq!(
            text(
                html,
                json!({"from": {"text": "Two"}, "to": {"text": "Four"}})
            )
            .unwrap(),
            "Two\nA\nB\nThree"
        );
        assert!(
            text(
                html,
                json!({"from": {"anchor": "s2"}, "to": {"anchor": "s1"}})
            )
            .is_err()
        );
    }

    #[test]
    fn iso_8859_1_maps_bytes_to_code_points() {
        let selector = Selector::from_json(&json!({"element_id": "x"})).unwrap();
        // 0xE6 is æ; 0x96 is a C1 control in ISO-8859-1 (an en dash only in windows-1252).
        let page = b"<p id=x>pr\xe6dicta\x96 &ordf;</p>";
        for name in ["ISO-8859-1", "iso_8859_1", "latin-1", "Latin1"] {
            assert_eq!(
                extract(page, name, &selector).unwrap(),
                "pr\u{e6}dicta\u{96} \u{aa}"
            );
        }
        assert!(extract(b"<p id=x>pr\xe6</p>", "utf-8", &selector).is_err());
    }

    #[test]
    fn rejects_invalid_selectors_and_encodings() {
        for bad in [
            json!({}),
            json!({"heading": ""}),
            json!({"heading": "a", "element_id": "b"}),
            json!({"to": {"text": "x"}}),
            json!({"from": {"css": "x"}}),
        ] {
            assert!(Selector::from_json(&bad).is_err(), "{bad}");
        }
        let selector = Selector::from_json(&json!({"element_id": "x"})).unwrap();
        assert!(extract(b"<p id=x>a</p>", "windows-1252", &selector).is_err());
        assert!(extract(b"\xff", "utf-8", &selector).is_err());
        assert!(
            extract(b"<p id=x> </p>", "utf-8", &selector)
                .unwrap_err()
                .contains("empty")
        );
    }
}
