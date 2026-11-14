# ADR 0001: Reuse the existing source-processing owners

Status: Accepted for this repository, 2026-11-14. Individual cross-repository adapters remain unimplemented until acceptance tests prove them.

## Context

Formal Logos needs reproducible philosophical source material, not a new OCR, NLP, search, or belief-attribution engine. The project's present Rust CLI stores metadata in `registry/catalog.json` and original local PDFs by SHA-256. Truth Core still owns canonical source/passage/claim records and the formal proof corpus. Several existing repositories already own capabilities the source pipeline needs. Reimplementing them here would introduce competing interpretations of the same evidence.

## Decision: one source, multiple derived evidence layers

`formal-logos-sources` owns a work/edition inventory, permitted acquisition policy, local materialization and the choice of producer revisions. It may orchestrate existing tools and export **candidates** plus their exact source references. No producer output silently becomes a Truth Core claim or a verified formal premise.

| Capability | Existing owner / usable surface | Boundary for Formal Logos Sources |
| --- | --- | --- |
| Document-folder ingestion and PDF text | [`document-search`](https://github.com/moritzbrantner/document-search), local `bun run corpus ...` and versioned `CorpusSnapshot`; PDF.js has page-local paragraphs and extractor receipts | Evaluate reuse of its filesystem corpus and extraction export before extending our current byte store or making another PDF parser. Its NFC/paragraph normalization is **not** Truth Core's Poppler witness text hash. |
| Image/PDF OCR | [`visual-analysis`](https://github.com/moritzbrantner/visual-analysis) OCR capability; [`media-intelligence`](https://github.com/moritzbrantner/media-intelligence) Rust analysis/composition facade | Feed explicitly requested scanned pages to an existing producer, retaining page/region, confidence and model/configuration provenance. The media-intelligence OCR downstream build contract exists; real model-backed OCR acceptance is not yet established. |
| Linguistic analysis | [`nlp-stack`](https://github.com/moritzbrantner/nlp-stack) Rust capabilities | Supply text and language/span provenance; keep extracted linguistic relations distinct from asserted claims. |
| Generic claim candidates | [`proposition-api`](https://github.com/moritzbrantner/proposition-api) `/extract` source-grounded drafts | Preserve exact quote offsets, nesting/reporting, speaker evidence, uncertainty and provider receipts. The model's candidate is neither endorsement nor truth. |
| Philosophical interpretation | [`philosophy-extractor`](https://github.com/moritzbrantner/philosophy-extractor) stages and JSON artifacts | Reuse for terminology, argument and formalization *candidates*. Its default deterministic fallbacks do not establish model-backed extraction quality or Lean verification. |
| Person-belief attribution | [`belief-lab`](https://github.com/moritzbrantner/belief-lab) `belief_evidence_interchange@1` and evidence/judgment pipeline | Optional downstream analysis for who expressed a belief; never treat it as an objective philosophical warrant score or infer personal assent from quoted text. |
| Speech and media source spans | [`youtube-corpus`](https://github.com/moritzbrantner/youtube-corpus), [`audio-analysis`](https://github.com/moritzbrantner/audio-analysis) and Media Intelligence | Reuse timestamped producer evidence and diarization uncertainty. Do not turn uncertain speaker labels into person identities. |
| Search and indexing | [`search-engine`](https://github.com/moritzbrantner/search-engine) Rust Tantivy index; `document-search` existing local search | Search projections and indexes are rebuildable, never an alternative source database. Avoid another homegrown Tantivy wrapper until a concrete missing adapter is demonstrated. |
| Canonical formal evidence | [`truth-core`](https://github.com/moritzbrantner/truth-core) `corpus/sources`, `corpus/passages`, `corpus/claims` and `source_evidence.py` | Truth Core alone accepts source fidelity, passage bindings, interpretation alternatives and Lean proof consequences. It must build and check offline without this repository. |

## Evidence states and transformation contract

1. **Acquired:** identity, legally usable access, edition, original bytes and SHA-256 are recorded. A URL in local metadata is *not* an independently verified remote witness.
2. **Extracted:** page/region or media-time selectors, text, extractor version/configuration and failures are recorded. OCR-only pages remain OCR-derived and may be imperfect; changed extraction is a new derived version.
3. **Analyzed:** NLP and candidate extraction may produce reported speech, propositions, relations and draft argument structures. Each candidate points to an exact source span and a producer receipt. Unknown, conflicting and tentative readings remain explicit.
4. **Reviewed:** a researcher checks the cited text, context and interpretation against the source. Only then can an accepted reading be represented in Truth Core's existing source and claim records, with the correct witness kind.
5. **Formalized:** Lean and the existing Truth Core gates verify advertised logical consequence, not the source's meaning or the premise's metaphysical truth.

Each transition has its own status; no success in one stage implies success in the next. Raw source bytes stay in ignored local storage, derived text and model artifacts stay local, and Git tracks only permissible metadata and short provenance/citation records. Neither OCR output nor a local PDF checksum alone is an admissible Truth Core `web_snapshot`: the authoritative remote witness rules and exact Poppler extraction contract still apply.

## First integration acceptance

Build one actual end-to-end, offline-replayable source run before introducing generic orchestration infrastructure:

1. Select a legitimately available real public-domain or openly licensed philosophical PDF with an identified immutable edition and known source URL. Record its original bytes, legal terms, and exact source identity. Never use a synthetic document as the acceptance source.
2. Exercise the existing document ingestion/export path and the current Formal Logos catalog against that identical PDF. Compare identity, physical page locators, extraction results and storage costs; reconcile duplicate blob stores before adding a second index or parser.
3. For an image-only or mixed page, exercise the existing Rust OCR producer and preserve an explicit *unprocessed/failed/empty/produced* status; check useful text against the source image before using it. Do not reclassify OCR text as a verified machine-readable source witness by default.
4. Pass one short, source-grounded passage through NLP, claim-candidate extraction and philosophical reconstruction. Preserve all offsets, nested attribution, provider revisions and alternatives. Run Belief Lab only when a real person's stance is part of the question.
5. Produce a proposal referencing an existing or new Truth Core source/passages record, but require Truth Core's own materialization, offline binding validation, semantic review and formalization gates before promotion. Confirm Truth Core CI has no dependency on this repository or on hosted model inference.
6. Test repeat ingestion (no duplicate work), a modified source (new hash and invalidation), an unavailable producer, a misattributed quotation, a corrupted blob, and restart/offline replay. Record exact producer revisions and measured time/disk use.

## Deferred / deliberately not decided

- Whether `document-search`'s filesystem corpus becomes the local blob/extraction backend or merely exchanges exported artifacts with this repository. The acceptance above decides from real evidence; do not migrate existing metadata or delete blobs prematurely.
- Whether OCR-backed evidence needs a new *permitted* Truth Core witness representation. That would require an explicit Truth Core owner decision, not a local workaround.
- Whether the orchestration needs background scheduling, queues, or an HTTP service. Start with bounded local Rust CLI composition, then justify further infrastructure with demonstrated workload requirements.

This repository does not replace the existing providers or create a new canonical philosophical database.
