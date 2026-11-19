# Formal Logos Sources

Source catalogue and preparation for [Formal Logos](https://github.com/moritzbrantner/formal-logos).

This repository prepares **source artifacts**, not philosophical propositions or proofs. Formal Logos remains authoritative for its source/passages/claims JSON, interpretation, Lean statements, and verification. There is no build-time or CI dependency from Formal Logos to this repository.

## Catalogue and research connections

The committed catalogue starts from Formal Logos revision
[`597e4b2eb02e092f1769a2e987437bc78e14893a`](https://github.com/moritzbrantner/formal-logos/tree/597e4b2eb02e092f1769a2e987437bc78e14893a):

| Recorded metadata | Count |
| --- | ---: |
| Author attribution strings | 128 |
| Bibliographic works after three reviewed groupings | 161 |
| Original Formal Logos source records | 164 |
| Referenced witness editions | 294 |
| Registered passages | 516 |
| Sources without registered passages | 19 |

Author entries preserve the original attribution, including joint authors and institutions;
they do not infer person identity, life dates, or deceased status. A witness edition is the
precise upstream manifestation or selected document described by one passage manifest. It
may cover only a chapter or extract, rather than an entire printed edition. These counts
describe the registered inventory, not complete bibliographies, reading coverage, or proof.

The two initial generated views expose existing Aquinas research:

- [Knowledge and immateriality, ST I q.14 a.1](reports/aquinas-knowledge.md): includes the existing reading with 52 direct argument consumers.
- [Will and intellect, ST I q.19 a.1](reports/aquinas-will.md): includes the existing reading with 54 direct argument consumers.

Each view links the exact source passage, claims, statements, definitions,
disambiguations, consuming arguments and existing research attempts. Interpretation and
work-status text comes from Formal Logos. A recorded `formalized` claim is not a statement
that its premise is true or that its consuming arguments have been verified.

Requires Rust and Git; these commands read committed local Git objects and never fetch
historical sources:

```sh
cargo run --locked -- check
cargo run --locked -- authors --name 'Thomas Aquinas'

# Import an exact local Formal Logos commit. HEAD is also accepted and resolved once.
cargo run --locked -- import --formal-logos ../formal-logos \
  --revision 597e4b2eb02e092f1769a2e987437bc78e14893a

# Uses the revision recorded in the catalogue, even when the checkout has moved on.
cargo run --locked -- connections --formal-logos ../formal-logos \
  --source AQUINAS-ST --locator 'I q.19 a.1' --format markdown
```

`import` uses `registry/work-groups.json` by default (`--work-groups FILE` selects another
mapping). The three explicit groupings concern the *Tractatus*, *Philosophical
Investigations*, and Ayer's *Language, Truth and Logic*. Every source UUID and original
source title is retained. Shared URLs and similar names never merge works automatically.
The import is deterministic and repeatable; local works, independent author records and
acquired PDF editions are preserved. A refresh that would orphan a local edition refuses
without changing the catalogue. The recorded source commit must be available locally;
partial-clone missing objects fail instead of starting a network fetch.

`connections` accepts a canonical source UUID, legacy ID, or exact source label. Omitting
`--locator` selects all registered passages for that source. A locator selects its exact
range and subordinate locators (`a.1` does not select `a.10`). JSON is the default output;
`--format markdown` produces a shareable report. All repository links use the imported
commit. Passage bindings follow the existing source ID and locator-prefix contract.
The report is a derived view, not another philosophical database or research tracker.

### Metadata and acquisition are separate

`registry/catalog.json` uses schema version 2. Works can exist without editions or bytes.
Each work links its author attributions and original source records; each imported edition
retains the upstream snapshot, exact passage IDs, conventional locators and extraction
selectors. Remote Git/SHA-256 hashes remain **reference metadata** inside `formal_logos`;
an imported edition has no local `sha256`. Its immutable ID covers the reference metadata.
Changed reference metadata produces a new referenced edition on refresh.

`check` validates catalogue structure and internal references offline. `verify` checks
only acquired local PDFs. Importing a witness does not mark it acquired, read, freshly
verified, or philosophically established. Canonical evidence and source-kind rules remain
in Formal Logos; this implementation adds no evidence kind or dependency to that project.

Full bibliography expansion, automatic source downloads, reading-coverage views and the
OCR/NLP integrations remain subsequent work. Existing citations and rights notices are
preserved as recorded; this import does not make a new rights determination.

## Local PDF storage and passage preparation

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

Local PDF editions record the work, label, language, licence note, optional descriptive URL
and source byte SHA-256. Adding the same edition again is idempotent; changed editions
require new IDs. Distinct editions with identical bytes share one content-addressed local
blob. `verify` checks all acquired local blobs, reporting missing/corrupted files (for
example on a fresh clone); referenced upstream editions are not claimed verified. No
remote fetch is performed. `--catalog` and `--store` override the applicable default paths.
Existing schema-v1 local catalogues remain readable and become v2 on a successful mutation,
preserving their work/edition IDs and hashes.

`inspect` and `prepare` emit `sha256` (original PDF), `text_sha256` (selected normalized page text), physical `pdf_pages`, short `incipit`, and character count. These match the `web_snapshot` PDF hashing/extraction convention in `truth-core` at commit `2524680bd2b5bcdbface48d60d14a79c3f41375d` (2026-11-14). `prepare` first re-verifies the local blob's hash and includes `local_object_verified: true` and `remote_witness_verified: false`. The `--url`/`source_url` is descriptive and **never fetched or verified**.

**Preparation does not create an accepted Formal Logos witness.** For selected source
research, follow Formal Logos's [source-evidence contract](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/docs/source-evidence.md),
including its initial pinning procedure, edition/rights/locator checks and reading in
context. Create canonical passage and claim records there through its existing workflow.
Routine CI checks metadata offline; it does not repeatedly acquire historical witnesses.

PDFs and extracted full text remain local; `.gitignore` excludes `local/` and PDF files. Do not commit copyrighted source bytes or full extracted copyrighted text. Back up `local/` separately if keeping the bytes matters; a Git clone alone restores only metadata. No OCR is attempted; image-only or textless selected pages fail explicitly. Work/edition metadata can be versioned and shared independently of source bytes.

## Checks

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo run --locked -- check
```

## Boundary

- Sources: acquisition/preparation, reproducibility metadata, later OCR and indexing.
- Formal Logos: source identities, authoritative passage manifests, precise attribution, source fidelity, arguments, proof verification and Explorer.
- `youtube-corpus`: video collection/transcription; use timestamped references rather than duplicating its database.

Future integrations may add source discovery and indexing, while leaving Formal Logos independently verifiable and usable offline.

Regenerate the committed Aquinas examples against the catalogue's recorded revision:

```sh
cargo run --locked -- connections --formal-logos ../formal-logos \
  --source AQUINAS-ST --locator 'I q.14 a.1' --format markdown > reports/aquinas-knowledge.md
cargo run --locked -- connections --formal-logos ../formal-logos \
  --source AQUINAS-ST --locator 'I q.19 a.1' --format markdown > reports/aquinas-will.md
```

The reports need no external repository at read time. Regeneration uses an explicitly
supplied local checkout; ordinary CI needs neither that checkout nor a source host.

The multi-repository capability and trust boundaries are recorded in [ADR 0001](docs/adr/0001-reuse-source-processing-capabilities.md). OCR, NLP, philosophical reconstruction and belief attribution are **planned integrations**, not implemented CLI features yet.

## GitHub integration

The `validate` workflow uses the shared reusable workflow. `main` requires its Rust check and resolved PR conversations before merging. GitHub auto-merge is enabled for explicitly enrolled eligible PRs; it does not automatically enroll every PR. Routine Cargo dependency updates use the [shared Renovate policy](https://github.com/moritzbrantner/coding-agent-conventions/blob/main/default.json), with GitHub-managed auto-merge after required checks and explicit approval for major updates. Renovate must be installed/enabled for this repository for those dependency PRs to appear.
