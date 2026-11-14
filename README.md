# Formal Logos Sources

Source acquisition and preparation for [Formal Logos (currently `truth-core`)](https://github.com/moritzbrantner/truth-core).

This repository prepares **source artifacts**, not philosophical propositions or proofs. Formal Logos remains authoritative for its source/passages/claims JSON, interpretation, Lean statements, and verification. There is no build-time or CI dependency from Formal Logos to this repository.

## First supported workflow: local PDFs

Requires Rust and Poppler `pdftotext` (install `poppler-utils`).

```sh
cargo run -- inspect /path/to/legally-obtained.pdf --pages 2:4 --url https://publisher.example/paper.pdf
cargo test
```

`inspect` emits JSON containing `sha256` (original PDF), `text_sha256` (selected normalized page text), physical page range, a short incipit, and character count. These match the `web_snapshot` PDF hashing/extraction convention in `truth-core` as of 2026-11-14. It **does not create a verified Truth Core witness**: the `--url` is descriptive and not fetched, and its remote bytes may differ from the inspected local file. The researcher must verify the actual remote source twice, its license, edition and exact locator, then create/validate the source and passage records using Truth Core's existing tooling. Do not copy the output directly into `corpus/passages/` without this review.

PDFs and extracted full text should remain local; `.gitignore` excludes local working directories. Do not commit copyrighted source bytes or full extracted copyrighted text. No OCR is attempted; image-only or textless selected pages fail explicitly.

## Boundary

- Sources: acquisition/preparation, reproducibility metadata, later OCR and indexing.
- Formal Logos: source identities, authoritative passage manifests, precise attribution, source fidelity, arguments, proof verification and Explorer.
- `youtube-corpus`: video collection/transcription; use timestamped references rather than duplicating its database.

Future integrations should export immutable, content-addressed artifacts and leave Formal Logos independently verifiable and usable offline.
