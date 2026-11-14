use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn hash(input: &[u8]) -> String {
    format!("{:x}", Sha256::digest(input))
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

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() != Some("inspect") {
        return Err("usage: formal-logos-sources inspect FILE --pages FIRST:LAST [--url URL] [--join-soft-hyphens]".into());
    }
    let filename = args.next().ok_or("missing PDF file")?;
    let mut pages = None;
    let mut url = None;
    let mut soft_hyphens = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--pages" => pages = Some(args.next().ok_or("missing page range")?),
            "--url" => url = Some(args.next().ok_or("missing URL")?),
            "--join-soft-hyphens" => soft_hyphens = true,
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }
    let pages = pages.ok_or("missing --pages FIRST:LAST")?;
    let (first, last) = pages.split_once(':').ok_or("expected FIRST:LAST")?;
    let first = first.parse::<u32>().map_err(|_| "invalid first page")?;
    let last = last.parse::<u32>().map_err(|_| "invalid last page")?;
    let data = std::fs::read(Path::new(&filename)).map_err(|e| format!("{filename}: {e}"))?;
    let mut result = inspect(&data, first, last, soft_hyphens)?;
    if let Some(url) = url {
        result["source_url"] = url.into();
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
