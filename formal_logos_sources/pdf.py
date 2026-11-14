"""Offline PDF fingerprint preparation compatible with Truth Core PDF spans."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess


class PdfError(ValueError):
    """An input document or its selected text cannot be safely prepared."""


def inspect(data: bytes, first: int, last: int, *, soft_hyphens: bool = False) -> dict:
    if not isinstance(first, int) or not isinstance(last, int) or first < 1 or last < first:
        raise PdfError("pages must satisfy 1 <= first <= last")
    if not data.startswith(b"%PDF-"):
        raise PdfError("input is not a PDF")
    try:
        result = subprocess.run(
            ["pdftotext", "-f", str(first), "-l", str(last), "-enc", "UTF-8", "-", "-"],
            input=data, capture_output=True, timeout=30, check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as exc:
        raise PdfError(f"Poppler pdftotext unavailable or timed out: {exc}") from exc
    if result.returncode:
        raise PdfError(f"PDF extraction failed: {result.stderr.decode('utf-8', errors='replace').strip()}")
    try:
        decoded = result.stdout.decode("utf-8")
    except UnicodeDecodeError as exc:
        raise PdfError("PDF extractor returned invalid UTF-8") from exc
    if soft_hyphens:
        decoded = re.sub(r"\u00ad[ \t\r\n]*", "", decoded)
    if result.stdout.count(b"\f") != last - first + 1:
        raise PdfError("selected range exceeds available PDF pages")
    if any(not page.strip() for page in decoded.split("\f")[:-1]):
        raise PdfError("selected page has no extractable text; OCR is not supported")
    normalized = " ".join(decoded.split())
    if not normalized:
        raise PdfError("PDF extraction produced empty text")
    incipit = normalized[:80].rsplit(" ", 1)[0]
    pages = {"first": first, "last": last}
    if soft_hyphens:
        pages["text_normalization"] = "join_soft_hyphens"
    return {
        "sha256": hashlib.sha256(data).hexdigest(),
        "text_sha256": hashlib.sha256(normalized.encode("utf-8")).hexdigest(),
        "pdf_pages": pages,
        "incipit": incipit,
        "characters": len(normalized),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    command = sub.add_parser("inspect", help="fingerprint selected pages of a local PDF")
    command.add_argument("file", type=Path)
    command.add_argument("--pages", required=True, help="physical PDF pages FIRST:LAST (inclusive)")
    command.add_argument("--url", help="descriptive source URL; NOT fetched or verified")
    command.add_argument("--join-soft-hyphens", action="store_true")
    args = parser.parse_args()
    try:
        parts = args.pages.split(":")
        if len(parts) != 2:
            raise ValueError("expected FIRST:LAST")
        first, last = map(int, parts)
        output = inspect(args.file.read_bytes(), first, last, soft_hyphens=args.join_soft_hyphens)
        if args.url:
            output["source_url"] = args.url
        print(json.dumps(output, indent=2, ensure_ascii=False))
        return 0
    except (ValueError, OSError) as exc:
        parser.exit(2, f"error: {exc}\n")


if __name__ == "__main__":
    raise SystemExit(main())
