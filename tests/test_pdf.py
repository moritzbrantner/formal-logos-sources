"""Synthetic PDF parity and failure checks; no external acquisition."""

import hashlib
import subprocess
import unittest
from unittest import mock

from formal_logos_sources.pdf import PdfError, inspect


def fixture() -> bytes:
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R 4 0 R] /Count 2 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 5 0 R >> >> /Contents 6 0 R >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 5 0 R >> >> /Contents 7 0 R >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
    ]
    for value in (b"First synthetic page.", b"Second synthetic page."):
        stream = b"BT /F1 12 Tf 72 720 Td (" + value + b") Tj ET"
        objects.append(b"<< /Length " + str(len(stream)).encode() + b" >>\nstream\n" + stream + b"\nendstream")
    data = b"%PDF-1.4\n"
    offsets = [0]
    for index, obj in enumerate(objects, 1):
        offsets.append(len(data))
        data += str(index).encode() + b" 0 obj\n" + obj + b"\nendobj\n"
    start = len(data)
    data += b"xref\n0 8\n0000000000 65535 f \n"
    data += b"".join(f"{offset:010} 00000 n \n".encode() for offset in offsets[1:])
    return data + f"trailer\n<< /Size 8 /Root 1 0 R >>\nstartxref\n{start}\n%%EOF\n".encode()


class PdfTest(unittest.TestCase):
    def test_real_poppler_selected_pages(self):
        data = fixture()
        result = inspect(data, 2, 2)
        self.assertEqual(result["sha256"], hashlib.sha256(data).hexdigest())
        self.assertEqual(result["text_sha256"], hashlib.sha256(b"Second synthetic page.").hexdigest())
        self.assertEqual(result["pdf_pages"], {"first": 2, "last": 2})

    def test_range_and_input_rejected(self):
        for first, last in ((0, 1), (2, 1), (1, 3)):
            with self.subTest(first=first, last=last), self.assertRaises(PdfError):
                inspect(fixture(), first, last)
        with self.assertRaisesRegex(PdfError, "not a PDF"):
            inspect(b"fake", 1, 1)

    def test_extraction_failures(self):
        with mock.patch("formal_logos_sources.pdf.subprocess.run", side_effect=subprocess.TimeoutExpired("pdftotext", 30)):
            with self.assertRaises(PdfError):
                inspect(fixture(), 1, 1)
        with mock.patch("formal_logos_sources.pdf.subprocess.run", return_value=subprocess.CompletedProcess([], 0, b"\f", b"")):
            with self.assertRaisesRegex(PdfError, "no extractable text"):
                inspect(fixture(), 1, 1)

    def test_soft_hyphen_policy(self):
        raw = b"Alpha\xc2\xad beta\f"
        with mock.patch("formal_logos_sources.pdf.subprocess.run", return_value=subprocess.CompletedProcess([], 0, raw, b"")):
            result = inspect(fixture(), 1, 1, soft_hyphens=True)
        self.assertEqual(result["text_sha256"], hashlib.sha256(b"Alphabeta").hexdigest())
        self.assertEqual(result["pdf_pages"]["text_normalization"], "join_soft_hyphens")


if __name__ == "__main__":
    unittest.main()
