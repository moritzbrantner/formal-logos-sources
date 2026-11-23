# Formal Logos catalogue reconciliation — 2026-11-23

This is an offline, metadata-only audit of the committed Formal Logos inventory.
Formal Logos remains authoritative for source identities, evidence, claims,
attributions, interpretations and proofs. No original text was downloaded or
verified; missing cached bytes do **not** imply a source was never consulted.

## Revisions and coverage

- Previous catalogue pin: [`597e4b2eb02e092f1769a2e987437bc78e14893a`](https://github.com/moritzbrantner/formal-logos/tree/597e4b2eb02e092f1769a2e987437bc78e14893a).
- Audited current `main`: [`a2011fb21d648d62046e72a5817517f572c23c92`](https://github.com/moritzbrantner/formal-logos/tree/a2011fb21d648d62046e72a5817517f572c23c92).
- Committed catalogue now pins that audited `main` revision. Later upstream
  commits require a new explicit freshness check and import.

| Inventory | Previous | Audited / imported | Delta |
| --- | ---: | ---: | ---: |
| Source records | 164 | 164 | 0 |
| Author attribution strings | 128 | 128 | 0 |
| Explicitly grouped bibliographic works | 161 | 161 | 0 |
| Witness manifests / referenced editions | 294 | 296 | +2 |
| Registered passages | 516 | 518 | +2 |
| Sources without registered passages | 19 | 19 | 0 |
| Local acquired PDF editions in the committed catalogue | 0 | 0 | 0 |
| Curated author profiles (independent sidecar) | 5 | 5 | 0 |

The work count uses the three existing explicit groups in
`registry/work-groups.json`; no URL- or title-based merging occurred.

## Precise upstream delta

- **Source JSON:** three changed, 161 unchanged, none added or removed.
  `aquinas-summa-theologiae.json` added the admitted SCTA edition URN
  `http://scta.info/resource/TAca84-d1e278/cod-cnLxE4`.
  The two Tractatus source files (`wittgenstein-tractatus.json` and
  `wittgenstein-tractatus-ogden.json`) changed editorial notes to explain
  that the LaTeX German and PDF Ogden English witnesses share a bibliographic
  work but not edition bytes, language or claim provenance. Their separate
  manifests and edition IDs remain unchanged.
- **Passage manifests:** two added, 294 unchanged, none revised or removed.
  `corpus/passages/aquinas-summa-theologiae-q13-a7.json` adds
  `12d8ecc5-37c5-4309-870a-f9273a43669e` (`I q.13 a.7`) and new
  edition ID `fl-edition-6f494fcd96242d9cc352d47acb5839ac7813ec9eaed87f8ace327d63c256ade2`.
  `corpus/passages/aquinas-summa-theologiae-q45.json` adds
  `fb7bd7bb-c007-43fa-9147-a017d0ae1f01` (`I q.45 a.3`) and new
  edition ID `fl-edition-239dc27dd89def902f744ef573d02e8d0daf47dcc82c39a0cff2acf0730fa512`.
  Their immutable SCTA blob SHA, revision, source URL, locator and TEI XML
  selector metadata are retained verbatim from the upstream manifests.
  Aquinas's *Summa theologiae* therefore increases from 29 editions /
  65 passages to 31 editions / 67 passages.
- **Claims:** two changed, 2,056 unchanged, none added or removed.
  `aquinas-st-i-q11-a1-composite-two-components.json` gains an attribution
  explanation; `aquinas-st-i-q2-a3-grades-have-maximum.json` changes only
  its label. Neither change reassigns the canonical source ID, witness locator
  or claim passage binding. Claims and interpretations are **not** copied into
  the Sources catalogue.
- **Work/author provenance:** the upstream source author fields and mapped
  work identities are unchanged; no local authors, groupings, acquisitions or
  research sidecar entries were removed.

## Staleness versus broken references

At the previous pin, the two new witness manifests and updated source notes
made the catalogue **stale relative to current main**. They did not break
links into the previous, still-available commit. After the explicit import,
the catalogue records the audited revision, its two added edition identities
and both new passage references; no original edition identity is overwritten.

`check` validates internal catalogue references and optionally curated records.
`freshness` additionally compares the catalogue with the projection rebuilt
from its **own pinned Git objects**, then independently compares that pinned
projection with a requested upstream revision. A missing/revised source,
selector, locator, passage or edition **at the pinned revision** is reported in
`pinned_catalogue_mismatches` and sets `broken_references`. Upstream
additions/changes/removals instead appear in `upstream_changes` and
`stale`; a different revision without an inventory change is still shown as
revision drift. The command refuses unavailable Git objects rather than
guessing whether records existed or fetching them.

`freshness` also compares raw source records, so changes to canonical-only
fields such as `edition_urns` and `notes` remain visible without duplicating
Formal Logos's source schema in this catalogue. It compares claim provenance
references without importing claim interpretation or proof data.

Run against a **locally available** Formal Logos checkout:

```sh
cargo run --locked -- freshness --formal-logos ../formal-logos --revision main
cargo run --locked -- check --research registry/author-research.json
```

For this audited pin, the expected result is `stale: false`,
`broken_references: false`, 164 source records, 296 witness editions and
518 passages. Neither result certifies remote source availability, rights,
source fidelity, or reading coverage.

## Documentation and report counts

The README's **catalogue inventory** previously described 294 referenced
editions / 516 passages at the older revision. Its **author-retrieval subset**
described 16 works / 65 witnesses / 196 passage references; these denominators
are different because the latter covers only the five curated profiles
(Aristotle, Aquinas, Leibniz, Kant and Russell). After the two Aquinas additions
they are respectively **296 / 518** and **16 / 67 / 198**. Retrieval planning
remains 6 bibliography inventories, 13 candidates for 8 works, with 8 works
without candidates. None measures completed reading or complete bibliography.

`reports/author-research.md` has the updated catalogue revision and the
recomputed Aquinas (52 witnesses / 94 passages) and Summa (31 / 67) counts.
The existing `reports/aquinas-knowledge.md` and `reports/aquinas-will.md`
remain explicitly pinned to their original `597e4b2` research snapshot.
Their argument-consumer counts (52 and 54) must not be presented as current
`main` counts without regenerating the full semantic view. The older
author-research program document is likewise a historically pinned planning
packet, not a live catalogue index.

To refresh a connection view **without losing its revision binding**, run
`connections` against the catalogue's recorded revision. This reads the
canonical committed claim, statement, definition, disambiguation, argument
and attempt records; the report's links all use the same exact commit:

```sh
cargo run --locked -- connections --formal-logos ../formal-logos \
  --source AQUINAS-ST --locator 'I q.14 a.1' --format markdown \
  > reports/aquinas-knowledge.md
cargo run --locked -- connections --formal-logos ../formal-logos \
  --source AQUINAS-ST --locator 'I q.19 a.1' --format markdown \
  > reports/aquinas-will.md
```

No full-source acquisition, mandatory cross-repository CI job or alternate
philosophical datastore is part of this reconciliation.
