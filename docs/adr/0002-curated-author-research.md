# ADR 0002: Curated people and retrieval plans alongside the imported catalogue

Status: accepted for this implementation.

## Context

The corpus import preserves Formal Logos author attribution strings and rebuilds its
catalogue projection at a pinned revision. An attribution can name several people or an
institution. Curated life dates, bibliography inventories and possible retrieval editions
have a different authoring lifecycle from that imported metadata.

The existing edition contract represents an acquired local PDF or an existing canonical
Formal Logos witness. A possible future retrieval is neither of those states.

## Decision

Store curated person profiles and their inventories and retrieval candidates in
`registry/author-research.json`. Explicit attribution IDs connect each person to the
catalogue; explicit work IDs connect retrieval candidates. Preserve joint attributions.
Do not infer identity or authorship from name splitting or title similarity.

Validate the sidecar against the selected catalogue in `check --research FILE`. The
`research` command derives counts, source links and planning gaps from those same inputs.
It performs no acquisition. Provider access and rights statements remain separate,
scoped, dated evidence; deceased status supplies no edition-rights conclusion.

The corpus importer continues to own only its catalogue projection. It cannot erase
curated profiles. If an upstream refresh changes a referenced attribution or work,
the combined offline check identifies the reference that needs reconciliation.

## Consequences

The imported catalogue schema and immutable edition identities stay intact. CI validates
the two metadata files together using local data. Missing, ambiguous or unsupported
references are errors; bibliography reconciliation and reading completion are not inferred
from inventory links or passage counts.

Formal Logos retains canonical source, passage, claim, interpretation and proof authority.
No new dependency, network requirement or source-evidence handoff format is introduced.
