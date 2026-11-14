# Formal Logos Sources

Source acquisition and preparation for [Formal Logos (currently `truth-core`)](https://github.com/moritzbrantner/truth-core).

This repository prepares **source artifacts**, not philosophical propositions or proofs. Formal Logos remains authoritative for its source/passages/claims JSON, interpretation, Lean statements, and verification. There is no build-time or CI dependency from Formal Logos to this repository.

## Rust CLI: catalog, local PDF storage, passage preparation

Requires Rust and Poppler `pdftotext` (install `poppler-utils`).

```sh
# Add a work and a particular edition. Metadata goes to registry/catalog.json;
# original bytes go to ignored local/objects/sha256/<prefix>/<hash>.
cargo run -- add /path/to/legally-obtained.pdf \
  --work-id example-work --title 'Example Work' \
  --edition-id example-work-ed1 --edition-label 'First edition (2020)' \
  --language eng --license 'Publisher permission required' \
  --url https://publisher.example/paper.pdf

cargo run -- list
cargo run -- verify
cargo run -- prepare --edition-id example-work-ed1 --pages 2:4

# Direct single-file fingerprinting is also supported without registration.
cargo run -- inspect /path/to/legally-obtained.pdf --pages 2:4
```

`registry/catalog.json` is Git-trackable metadata (`schema_version: 1`) with stable work IDs and immutable edition IDs, separate from Formal Logos's canonical source/claim IDs. An edition records its title's work, label, language, license note, optional descriptive URL and source byte SHA-256. Adding the same edition again is idempotent; changed editions require new IDs. Distinct editions with identical bytes share one content-addressed local blob. `verify` checks **all referenced local blobs**, reporting missing/corrupted files (for example on a fresh clone). No remote fetch is performed. `--catalog` and `--store` override the default paths for all applicable commands.

`inspect` and `prepare` emit `sha256` (original PDF), `text_sha256` (selected normalized page text), physical `pdf_pages`, short `incipit`, and character count. These match the `web_snapshot` PDF hashing/extraction convention in `truth-core` at commit `2524680bd2b5bcdbface48d60d14a79c3f41375d` (2026-11-14). `prepare` first re-verifies the local blob's hash and includes `local_object_verified: true` and `remote_witness_verified: false`. The `--url`/`source_url` is descriptive and **never fetched or verified**.

**This does not create a verified Truth Core witness.** The researcher must verify the actual remote source twice, its license, edition, precise printed locator and source fidelity, then create/validate the canonical source and passage records using Truth Core's existing tooling. Do not copy the preparation output directly into `corpus/passages/` without this review.

PDFs and extracted full text remain local; `.gitignore` excludes `local/` and PDF files. Do not commit copyrighted source bytes or full extracted copyrighted text. Back up `local/` separately if keeping the bytes matters; a Git clone alone restores only metadata. No OCR is attempted; image-only or textless selected pages fail explicitly. Work/edition metadata can be versioned and shared independently of source bytes.

## Checks

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

## Boundary

- Sources: acquisition/preparation, reproducibility metadata, later OCR and indexing.
- Formal Logos: source identities, authoritative passage manifests, precise attribution, source fidelity, arguments, proof verification and Explorer.
- `youtube-corpus`: video collection/transcription; use timestamped references rather than duplicating its database.

Future integrations may add source discovery and indexing, while leaving Formal Logos independently verifiable and usable offline.
