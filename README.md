# Formal Logos Sources

Source catalogue and preparation for [Formal Logos](https://github.com/moritzbrantner/formal-logos).

This repository prepares **source artifacts**, not philosophical propositions or proofs. Formal Logos remains authoritative for its source/passages/claims JSON, interpretation, Lean statements, and verification. There is no build-time or CI dependency from Formal Logos to this repository.

## Catalogue and research connections

The committed catalogue covers Formal Logos revision
[`9eef0b04b958c1c5a618b8307ef43343495591bf`](https://github.com/moritzbrantner/formal-logos/tree/9eef0b04b958c1c5a618b8307ef43343495591bf):

| Recorded metadata | Count |
| --- | ---: |
| Author attribution strings | 131 |
| Bibliographic works after three reviewed groupings | 164 |
| Original Formal Logos source records | 167 |
| Referenced witness editions | 301 |
| Registered passages | 531 |
| Sources without registered passages | 19 |

Author entries preserve the original attribution, including joint authors and institutions;
they do not infer person identity, life dates, or deceased status. Curated person profiles
are recorded separately below. A witness edition is the
precise upstream manifestation or selected document described by one passage manifest. It
may cover only a chapter or extract, rather than an entire printed edition. These counts
describe the registered inventory, not complete bibliographies, reading coverage, or proof.

The two initial generated views expose existing Aquinas research:

- [Knowledge and immateriality, ST I q.14 a.1](reports/aquinas-knowledge.md): includes the existing reading with 0 direct argument consumers.
- [Will and intellect, ST I q.19 a.1](reports/aquinas-will.md): includes the existing reading with 57 direct argument consumers.

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
  --revision 9eef0b04b958c1c5a618b8307ef43343495591bf

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

The Formal Logos materializer can share the ignored `local/formal-logos-cache/` directory here for downloaded witnesses and extracted passages. Its `TRUTH_SOURCE_CACHE_DIR` setting takes that directory as an absolute path.

Full bibliography expansion, automatic source downloads, reading-coverage views and the
OCR/NLP integrations remain subsequent work. Existing citations and rights notices are
preserved as recorded; this import does not make a new rights determination.

## Author profiles and retrieval planning

[`registry/author-research.json`](registry/author-research.json) adds curated profiles for
**Aristotle, Thomas Aquinas, Leibniz, Kant, and Bertrand Russell**. The
[generated author report](reports/author-research.md) joins these profiles to **16 works,
65 referenced witnesses, and 196 passage references** in the original author study. It
includes **6 bibliography inventories** and **13 retrieval candidates for 8 works**;
the remaining **8 works** are visible as gaps in retrieval planning. These are counts of
registered material, not complete bibliographies or reading coverage.

```sh
# List the curated deceased people, ranked by existing passage count.
cargo run --locked -- research --deceased

# Inspect one person's inventory, work links, editions and next retrieval actions.
cargo run --locked -- research --author leibniz --format markdown

# Validate both the imported catalogue and the curated records, without network access.
cargo run --locked -- check --research registry/author-research.json
```

`research` emits JSON by default. `--author` matches an exact profile ID, name or declared
alias, case-insensitively; unknown or ambiguous matches fail. `--deceased` filters only
the curated profiles with supported deceased status. The report states how many source
attributions remain unprofiled: currently **125 of 131**. It does not classify those
attributions as living or deceased. `--research FILE` and `--catalog PATH` select other
inputs; missing files fail. Ordinary `check` still validates only the catalogue unless
`--research` is supplied; CI supplies it explicitly.

### Curated identities and bibliography scope

Each profile explicitly maps to the existing attribution IDs. Leibniz maps both to his
individual attribution and to the separate Leibniz–Clarke attribution as a coauthor.
Several people may share a joint attribution; a sole-author attribution cannot be assigned
to several profiles. A jointly attributed work can appear under more than one person, so
profile totals are not generally additive. Names are never split or merged automatically.

Life status records its evidence and observation date. Historical death years use negative
integers for BCE (`-322` means 322 BCE); there is no year zero. Unknown life status remains
explicit. Bibliography inventories record the provider's scope and publication state,
separately from our reconciliation progress. The current six inventories are located but
not yet reconciled into complete work lists. The first Aquinas pass has matched the seven
registered works to its selected inventory and added retrieval routes for all seven; see
the [Aquinas source-preparation report](reports/aquinas-first-pass.md) for the exact scope.
This schema has no completed-reconciliation state: recording an index link cannot mark
a bibliography finished.

### Edition access and rights

Every retrieval candidate names an existing work, a specific edition or document, its
language and format, its actual coverage, an access route, evidence, and a next action.
An entry may cover a single paper or a section of a work. Candidates do not create local
PDF editions or canonical Formal Logos witnesses.

Access and rights are separate fields. Rights states record a provider licence, a
provider public-domain claim with its jurisdiction, an explicit permission requirement,
or rights not yet established. The scope and cited notice carry the conditions: the
`provider_license` state alone does not mean unrestricted reuse. Death year is never used
to derive edition rights. These commands perform no downloads and make no new permission
decision.

Curated records are edited in the sidecar; the corpus importer does not rewrite them.
After a catalogue refresh, run the combined check to identify references needing
reconciliation. Unknown attribution/work IDs, a candidate assigned to another person's
work, missing required evidence, and unsupported fields fail validation. All evidence
links and observations remain metadata; CI does not contact their hosts.

Regenerate the committed report with:

```sh
cargo run --locked -- research --deceased --format markdown > reports/author-research.md
```

The Markdown report is an intentionally committed distribution view for GitHub readers;
the JSON sidecar and imported catalogue remain its inputs. See
[ADR 0002](docs/adr/0002-curated-author-research.md) for the persistence boundary.

## Author research program

The [author-by-author plan](docs/specs/author-research-program.md) gives a proposed
51-person historical sequence, a 50-person contemporary sequence, and an exhaustive
roster of all 128 current attribution records, including joint authors, institutions
and unresolved providers. Each entry has an opening research packet and a pinned
canonical source link. The plan defines scoped author passes, connects them to existing
Formal Logos findings, and identifies concrete reuse of Document Search, OCR, Belief Lab
and YouTube Corpus. Its capability observations are dated 2026-11-19; it does not mark
the passes or Sources adapters complete.

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
research, follow Formal Logos's [source-evidence contract](https://github.com/moritzbrantner/formal-logos/blob/9eef0b04b958c1c5a618b8307ef43343495591bf/docs/source-evidence.md),
including its initial pinning procedure, edition/rights/locator checks and reading in
context. Create canonical passage and claim records there through its existing workflow.
Routine CI checks metadata offline; it does not repeatedly acquire historical witnesses.

PDFs and extracted full text remain local; `.gitignore` excludes `local/` and PDF files. Do not commit copyrighted source bytes or full extracted copyrighted text. Back up `local/` separately if keeping the bytes matters; a Git clone alone restores only metadata. No OCR is attempted; image-only or textless selected pages fail explicitly. Work/edition metadata can be versioned and shared independently of source bytes.

## Checks

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo run --locked -- check --research registry/author-research.json
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
