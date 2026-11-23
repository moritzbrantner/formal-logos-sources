# Aquinas: first source-preparation pass

Observed 2026-11-19. **Historical research checkpoint:** the edition and passage counts below describe the explicitly linked Formal Logos revision, not the subsequently refreshed catalogue; current totals are in the [catalogue freshness audit](../docs/catalogue-freshness.md). The seven Aquinas works already registered in Sources have been matched to the selected work inventory, and all seven now have a scoped retrieval route. The curated profile has **11 retrieval candidates**, up from four; five works previously had none. This includes the actual SCTA edition used to materialize and read **the complete ST I q.19 a.1 article** and map its eight argumentative sections to existing Formal Logos readings.

This completes the seven-work inventory subset and the selected article preparation. Whole-author bibliography reconciliation remains in progress. The source packet preserves the current philosophical assessment and adds no canonical claim, proof or premise judgment.

The implementation baseline is Sources [fdcb55a](https://github.com/moritzbrantner/formal-logos-sources/tree/fdcb55a475e5b0014855abaf75d6d700ba206c11). Catalogue counts remain bound to Formal Logos [597e4b2](https://github.com/moritzbrantner/formal-logos/tree/597e4b2eb02e092f1769a2e987437bc78e14893a); the article's current research links and extraction use [5cbba270](https://github.com/moritzbrantner/formal-logos/tree/5cbba270d1d99e3e852cff632cf42725cb90cb04). The imported catalogue was not refreshed or repinned. The [curated sidecar](../registry/author-research.json) owns retrieval metadata and the [generated author report](author-research.md) presents its counts.

## Seven-work match and precise scope

The catalogue at its pinned Formal Logos revision contains 50 referenced Aquinas witness editions and 92 passage references. These counts describe manifests and selections, not 50 complete printed editions or 92 independently verified claims.

| Registered work and exact work ID | Inventory match and useful route | What the route actually covers |
| --- | --- | --- |
| **De ente et essentia** — `fl-work-004fd39d-a45d-5624-b927-053cb0fd0fc9` | Philosophical opusculum; existing candidate: [oee.html](https://www.corpusthomisticum.org/oee.html) | Latin treatise in Baur 1933/Koch numbering. The two registered selections cover chapters 1–4, not a demonstrated complete reading. Existing retrieval remains unchanged. |
| **Summa contra gentiles** — `fl-work-0292b788-8058-5069-afde-4a19b82b5604` | Major work; new candidate: [scg1014.html](https://www.corpusthomisticum.org/scg1014.html) | Book I chapters 14–28, including registered I.18 and I.22; Leonine text corrected from Turin 1961 proofs. Nine registered manifests contain eleven passage references across books I–II. This one candidate covers only part of them. |
| **In octo libros Physicorum Aristotelis expositio** — `fl-work-235966cc-024d-5630-9426-50912ce6af4f` | Index title *In libros Physicorum*; new candidate: [cpy07.html](https://www.corpusthomisticum.org/cpy07.html) | Latin books VII–VIII, Leonine text as published at Turin in 1954. The two existing English manifests select VII lectures 1–2 and VIII lectures 7, 9 and 22. Latin-to-English alignment is still work to do. |
| **Quaestiones disputatae de potentia** — `fl-work-256fbf26-eb53-5a27-807e-20756ca6f661` | Disputed question collection; new candidates: [qdp3.html](https://www.corpusthomisticum.org/qdp3.html) and [qdp5.html](https://www.corpusthomisticum.org/qdp5.html) | Question 3 and questions 5–6 respectively, Turin 1953. The two existing selections are q.3 a.5 and q.5 a.1. Neither page represents the entire collection. |
| **Sententia libri Metaphysicae** — `fl-work-3b5c2cfb-01dd-5ea7-a5e1-4f221e1378af` | Aristotelian commentary; new candidate: [cmp09.html](https://www.corpusthomisticum.org/cmp09.html) | Book IX, Turin 1950, including registered lectures 7–9. The five existing manifests contain six selections in books II, V, IX and XII. Those other books remain separate retrieval resources. |
| **Compendium theologiae** — `fl-work-c642b3c0-41ef-5798-8834-d3b03018f5ab` | Theological opusculum; new candidate: [ott101.html](https://www.corpusthomisticum.org/ott101.html) | Book I chapters 1–184, Turin 1954, including the registered chapters 12–15. This page excludes I.185–246 and book II. |
| **Summa theologiae** — `fl-work-c77377cf-501b-56c6-be40-b518e9aad101` | Major work; existing candidates: [sth1003.html](https://www.corpusthomisticum.org/sth1003.html), [sth1015.html](https://www.corpusthomisticum.org/sth1015.html), [Gutenberg 17611](https://www.gutenberg.org/ebooks/17611); new [SCTA q.19 witness](#selected-primary-witness) | Latin Prima pars q.3–14 and q.15–27, English Prima pars, and the selected Latin q.19 TEI. These resources do not cover the whole Summa. The imported catalogue has 29 manifests and 65 passage references for this work. Gutenberg was not reverified in this inventory pass. |

## Concrete retrieval findings

The imported Physics catalogue URL `https://www.corpusthomisticum.org/cpy.html` returned **HTTP 404** on a direct request. The index instead points to `cpy011.html`, `cpy012.html`, `cpy03.html`, `cpy05.html` and `cpy07.html`; the first and last rendered in this pass. Preserve the canonical imported URL until Formal Logos itself adopts any correction. The new candidate supplies the useful VII–VIII route without rewriting upstream records.

The Metaphysics catalogue URL [cmp00.html](https://www.corpusthomisticum.org/cmp00.html) renders the **prooemium only**. Existing book-II, V and XII witnesses already name `cmp02.html`, `cmp05.html` and `cmp12.html`; the new IX candidate names `cmp09.html`. The provider’s inventory supplies separate resources for all twelve books.

For Compendium, the remaining index routes are [ott1185.html](https://www.corpusthomisticum.org/ott1185.html) for I.185–246 and [ott2.html](https://www.corpusthomisticum.org/ott2.html) for book II. The latter rendered in this pass; the former was identified from the inventory only.

For SCG, registered Latin selections outside the I.14–28 candidate already cite `scg1001.html` (I.2–3), `scg2006.html` (II.15) and `scg2056.html` (II.80). These are follow-on candidate gaps even after every registered work has at least one candidate. The index also distinguishes earlier redactions from its main four-book text.

## Access, rights and edition distinctions

The six new Corpus Thomisticum HTML candidates rendered on the web and returned HTTP 200 by direct GET on 2026-11-19. Each of those pages’ own footers claims ©2019 Fundación Tomás de Aquino for that edition and reserves rights. Those candidates use `open_online` access and `not_established` rights; they do not infer reuse permission from free reading access or Aquinas’s death. This improves on relying only on a generic site notice. The existing De ente page instead carries a 2011 edition notice. The separate SCTA candidate records its explicit provider licence, as described below.

The [provider’s recommended-editions bibliography](https://www.corpusthomisticum.org/reoptedi.html) distinguishes several recommended print editions from the served text pages:

| Work | Served digital textual basis | Recommended edition recorded by provider |
| --- | --- | --- |
| De ente | Baur 1933, Koch emendations | Leonine 43 (1976), pp.315–381 |
| Compendium | Turin 1954 | Leonine 42 (1979), pp.5–205 |
| De potentia | Turin 1953 | P. M. Pession, tenth edition, 1965, pp.1–276 |
| Metaphysics | Turin 1950 | Cathala/Spiazzi, second edition, 1971 |
| Physics | Leonine text, Turin 1954 | Leonine 2, Rome 1884 |

These are distinct edition routes, not grounds for silently changing existing witness identities. The [Leonine volume list](https://www.corpusthomisticum.org/repedleo.html) still labels its planned De potentia volume 21 and Metaphysics volume 46 as in preparation; this is a report of that provider’s listing, not an independently established current publication census.

## A concrete future OCR input

The Leonine list links the Physics volume 2 scan at [Internet Archive / University of Toronto](https://archive.org/details/operaomniaiussui02thom). Its landing page offers PDF, existing OCR text/HOCR and JP2 page images, and identifies volume 2 and the Robarts contributor. It records 520 scanned pages and Tesseract OCR. This is a useful candidate for an OCR comparison against a bounded Latin lecture.

There is a bibliographic detail to resolve before pinning: Archive metadata says publication date 1882, while the scholarly volume list dates volume 2 to 1884. Confirm the scan’s title page rather than treating the collection’s metadata year as the volume year. No PDF bytes, page images or OCR accuracy were checked, and no reuse licence was established in this pass. This optional scan is not included in the new retrieval candidates.

The same Leonine list links volume 42 at `https://gallica.bnf.fr/ark:/12148/bpt6k9494h.r` and volume 43 at `https://gallica.bnf.fr/ark:/12148/bpt6k9495t.r`. Both linked routes returned 403 through the web reader. They remain bibliography leads; this pass does not claim usable downloads or text access.

## What remains outside this completed subset pass

The [Opera omnia index](https://www.corpusthomisticum.org/iopera.html) also lists Scriptum super Sententiis, De veritate, De malo, other philosophical opuscula, biblical commentaries, sermons and further classes not covered by the seven imported work records. Each needs an explicit inclusion/exclusion and retrieval disposition before making a whole-author claim.

Attribution boundaries matter even within matched works: [the De potentia editorial note](https://www.corpusthomisticum.org/qdp4n.html) assigns the supplied q.4 article replies to Vincentius de Castronovo, and the Opera index attributes the Summa supplement separately to Rainaldus. Do not merge such continuations into Aquinas-authored text.

The [selected authenticity bibliography](https://www.corpusthomisticum.org/reopauth.html) cites Emery’s work catalogue, Leonine introductions, disputed works and unpublished fragments. Checking that page does not equal reconciling those underlying references. Its `not_started` status is intentionally preserved; the main Opera omnia inventory moves only to `in_progress`.

## Selected primary witness

Reuse [Summa theologiae](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/sources/aquinas-summa-theologiae.json) — source `c77377cf-501b-56c6-be40-b518e9aad101` — and its [q.19 passage manifest](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/passages/aquinas-summa-theologiae-q19.json). Article passage `2dc7a89a-5d81-5bf7-97b3-a429018d7e4a` has conventional locator `I q.19 a.1`, locator prefix `I q.19 a.1 ` and container selector `TAca84-d1e144-Dd1e182`. Sources edition ID is `fl-edition-9f4a7306acc7e926cea766c74713de83ab59b8162f044b56abc17104249c52d9`.

The [actual pinned TEI](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml) is original-language Latin (`xml:lang="la"`; catalogue `lat`), with no translation basis. Its header names Thomas Aquinas as author, Jeffrey C. Witt as transcription editor and TEI encoder, SCTA as authority, Rome 1888 volume 4 as print witness, edition `0.0.0-dev` dated 2023-07-31, and draft revision status. Those digital preparation roles are not coauthorship of the historical argument. The file contains the question’s prologue and all twelve article containers; only article 1 was read and role-mapped completely in this pass.

The header explicitly applies **CC BY-NC-SA 4.0** to this digital witness. Record that provider licence for this file; do not infer rights from Aquinas’s death or substitute the different Corpus Thomisticum site terms. [Header evidence](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L2-L45).

Pinned edition URI: `http://scta.info/resource/TAca84-d1e144/cod-cnLxE4`. Repository revision: `f9065dc8ce7bc790b8c972e49ee19d661f028be6`. Path: `TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml`. [Immutable raw URL](https://raw.githubusercontent.com/scta-texts/TAca84/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml). Actual bytes: 93,454; verified Git blob `94bc9e4ecf18163bcb1ba1af99ff610515ab8e49`; SHA-256 `5b80fb7dc4348c206b9ffb05bc54a5e0dadcbbcb97dd3295877111ab269cea1e`.

## Complete section and attribution map

Each XML ID below is prefixed `TAca84-d1e144-`. They are subsection selectors inside the existing canonical article passage, not new canonical passage IDs. The final column gives corresponding numbered entries on the already registered [Corpus Thomisticum q.15–27 reading aid](https://www.corpusthomisticum.org/sth1015.html); its live HTML is not a replacement for the retained TEI.

| Conventional locator | Attribution role | Pinned XML subsection | Role-sensitive reading | CT entry |
| --- | --- | --- | --- | --- |
| `I q.19 a.1 arg. 1` | objection | [`d1e199`](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L105) | An end is the object of will; no end can be assigned to God; hence no divine will. This is the challenged objection, answered in ad 1. | 29180 |
| `I q.19 a.1 arg. 2` | objection | [`d1e215`](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L113) | Appetite is taken to imply a missing good and imperfection; hence no divine will. This is the challenged objection, answered in ad 2. | 29181 |
| `I q.19 a.1 arg. 3` | objection | [`d1e227`](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L119) | The objection cites Aristotle on will as a moved mover and God as unmoved. Its negative conclusion is answered in ad 3; the embedded authorities are reported here by Aquinas. | 29182 |
| `I q.19 a.1 s. c.` | sed contra / cited authority | [`d1e238`](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L125) | Aquinas cites the Apostle, Romans 12:2, as authority for divine will. This scriptural citation is distinct from his response’s philosophical reasoning. | 29183 |
| `I q.19 a.1 co.` | author response | [`d1e245`](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L129) | Aquinas argues from intellect to will through the analogy of form-relative inclination, distinguishing seeking absent good and resting in possessed good; concludes divine will and identifies divine willing with divine being. | 29184 |
| `I q.19 a.1 ad 1` | author reply | [`d1e295`](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L153) | Answers arg. 1: no other thing is God’s end; God himself is the end of what he makes, through his essential goodness. | 29185 |
| `I q.19 a.1 ad 2` | author reply | [`d1e311`](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L161) | Answers arg. 2: willing includes love and delight in possessed good. The divine application is to the good always possessed. | 29186 |
| `I q.19 a.1 ad 3` | author reply | [`d1e333`](https://github.com/scta-texts/TAca84/blob/f9065dc8ce7bc790b8c972e49ee19d661f028be6/TAca84-d1e144/cod-cnLxE4_TAca84-d1e144.xml#L172) | Answers arg. 3: divine will’s principal object is God’s own goodness/essence; the description of understanding and willing as motion is qualified. Plato is an embedded reported authority. | 29187 |

The article also contains two TEI headings and a printed-title paragraph (`d1e190`). These are structural text, not a ninth argumentative role. The sed contra’s nested quotation is `TAca84-d1e144-Qd1e244`, explicitly linked to `http://scta.info/resource/rom12_2@12-17`.

## Existing canonical readings to preserve

All three existing claims remain `formalized` in the source-reading records. This records their current corpus status, not new verification of philosophical truth.

| Section | Claim ID and record | Statement ID and record |
| --- | --- | --- |
| `I q.19 a.1 co.: in quolibet habente intellectum, est voluntas` | [a5dccbe4-d4a1-53c8-a63f-cfb585784033](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/claims/aquinas-st-i-q19-a1-will-follows-intellect.json) | [090be689-b77f-51a1-9d5d-caf339169e4b](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/statements/will-follows-intellect.json) |
| `I q.19 a.1 co.; ad 3` | [c051288f-18fe-5239-99c7-14f8e789dbfb](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/claims/q19-divine-will-act.json) | [8a24e93a-016e-54ef-a34c-95fff4f077a9](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/statements/q19-divine-will-act.json) |
| `I q.19 a.1 ad 2` | [01005f7f-b540-5f1b-96af-05efa6650f83](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/claims/q19-possessed-good.json) | [90ce7aae-3721-54b4-9210-f84fe27297a6](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/statements/q19-possessed-good.json) |

These preserve, respectively: will follows intellect; divine willing is divine being and is not moved by another; and will includes love/delight in possessed good. No existing direct claim in this article binds an objection, sed contra or ad 1. Their absence as separately registered claims is not missing article coverage and does not call for manufacturing new claims.

### Definitions and sense boundaries

- HAS_WILL_1: [7c64b499-6dba-5e03-bdf3-5acea95d8836](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/definitions/will.json).
- HAS_INTELLECT_1: [88bc0655-9dd0-5879-ba1b-3fca9f86c1f4](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/definitions/intellect.json).
- Divine willing as divine being: [801225d9-ddd5-540f-96db-22778e09cd6e](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/definitions/divine-will-act.json).
- Exercise of will / possessed-good love: [9f448669-6387-5244-a2ec-f9f90618c037](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/definitions/act-of-will.json).
- Capacity versus exercise: [941efd70-d56f-53f9-b5f9-8acd36711a54](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/disambiguations/appetite-power-exercise.json).
- Passive soul appetite versus divine willing: [1d765cae-eca9-558f-9542-8291df359058](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/disambiguations/creature-passive-divine-will.json).
- Cognition versus intellect: [c0fda0ee-3508-5101-94ba-043e3eb28b0e](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/disambiguations/intellect-and-sense-cognition.json).
- Cognitive reception and knowing: [e557eab8-2eed-5b24-ade9-a23b5b26f9b3](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/disambiguations/q14-reception-senses.json).
- Love senses; possessed-good love is LOVE_3: [28c9980e-0cf7-4dcc-939a-e5c842ca4f34](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/disambiguations/28c9980e-0cf7-4dcc-939a-e5c842ca4f34.json).

### Argument connections and retained open inputs

At the pinned Formal Logos revision, statement `090be689-b77f-51a1-9d5d-caf339169e4b` has **54 direct premise consumers** and one producing argument. The two other article statements have no direct premise consumers. Counts describe graph usage, not warrant.

- Existing producer [22a7366c-57ed-5f39-8421-89e6664fa4e0](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/arguments/will-from-intellectual-inclination.json) and result [9b390598-4e9d-5183-a012-ee8fd1176f50](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/results/WILL-FROM-INTELLECTUAL-INCLINATION.json) bind `TruthCore.Results.Causality.IntellectualInclination.willFromIntellectualInclination`. It retains three OPEN inputs: intellect has intelligible form (`763e8f08-55ca-5e87-99dd-7423d6534073`); every form has inclination (`d4b35781-27f4-5c21-b03e-df696df7e19f`); and recognition of intelligible inclination as will (`b7b10ad2-fafe-552e-8bf4-205d49f2f070`). The first two are explicitly analyst formulations with attribution deferred. Reading the source again does not promote them to new Aquinas claims.
- Modular consumer [571e36f0-c486-474e-9807-226b4bd4ce7f](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/arguments/571e36f0-c486-474e-9807-226b4bd4ce7f.json) and result [dd55f57a-b256-4e73-b57e-6e2cca1c06f7](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/results/dd55f57a-b256-4e73-b57e-6e2cca1c06f7.json) consume GodHasIntellect (`29818edd-9f5a-5414-baa2-9a7d0c8abf18`) plus the will-follows principle to conclude GodHasWill (`66aecdd7-ea4e-55c6-9377-10db08893854`). Its intended will is capacity-level intellectual appetite; it supplies no God-existence witness, actual exercise, creation choice or recipient-directed love.
- Preserve the existing [intellect–will research note](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/docs/intellectual-appetite-source.md) and [2026-11-18 appetite application finding](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/corpus/attempts/e5a36b24-e0f4-4d9c-8cd4-cef767078a7e.json): recognition by unchanged HasWill and a nonpassive first-source application remain OPEN. The [current immaterial-intellect assessment](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/docs/immaterial-intellect-reception.md) keeps its substantive judgments and recommendation to move to q.11. This source packet does not restart that investigation.

## Practical retrieval and parsing findings

The browser could not render the raw GitHub XML; GitHub’s file API returned the exact pinned content, whose independently recomputed Git blob matched. Do not describe the browser error as source unavailability.

Use the existing `source_evidence.py` extractor at the pinned Formal Logos revision. It resolves the article’s unique XML ID and joins words divided by `<lb break="no"/>`. Ordinary `itertext()` plus whitespace joining would split words. The retained transcription also contains visible artifacts (irregular capitalization, `PnaETEREA`, `Ab`/`Ap` for reply headings, malformed Roman numerals and stray apparatus marks). Do not silently repair or rehash the witness. Structural XML IDs, article order, conventional locators and the independently consulted Latin reading aid support the role map; heading keyword matching alone is fragile.

The full selected article was extracted and read, including all eight argumentative sections. The canonical materialization receipt is 2,719 bytes including its trailing newline, SHA-256 `cac2c42290da19da8b5c084d586c40d7edd6d72658839b8b5caf992d0e63eb96`. Full source text remains local. OCR is not required for this machine-readable witness; the observed transcription defects make it a useful role/parsing baseline for later OCR comparisons without changing present source authority.

Exact role offsets in that unchanged materialization, using zero-based UTF-8 byte ranges with the end excluded: arg. 1 `[81,260)`, arg. 2 `[261,423)`, arg. 3 `[424,597)`, s. c. `[598,679)`, co. `[680,1686)`, ad 1 `[1687,1929)`, ad 2 `[1930,2307)`, ad 3 `[2308,2718)`. Every range was checked against the same canonical extractor applied to its section XML ID. The JSON preparation receipt also records each section hash; these ranges require that exact materialization hash.

## Existing tools: selected-source run

The complete Latin article passed the existing Document Search → Philosophy Extractor → Belief Lab reference path on 2026-11-19. No producer source files or dependency declarations changed. No model was invoked or downloaded.

### Result

- Document Search ingested one original text file, 2,719 bytes including its final newline. Its extracted representation is the complete 2,718-byte article, exactly equal to the canonical materialization except for that final newline.
- Repeat ingestion reported `unchanged: 1`, with no additions or updates. The corpus-core export was byte-identical after repeat ingestion and after closing/reopening the filesystem corpus.
- Philosophy Extractor's existing exporter produced **one `metadata` reference and one `source_text` reference**, linked by parentage.
- Belief Lab's actual native `explain` command admitted and stored **2 records in that command's temporary in-memory evidence store**, with **0 inferences**. This is not a saved personal investigation or persistent attribution ledger.
- All **8 section ranges and SHA-256 hashes** from the article packet matched the exported whole-article span: three objections, cited authority, author response, and three replies.
- The three pinned producer/consumer checkouts remained clean. The filesystem corpus occupied 84,639 bytes after closing; it contains one 2,719-byte payload and its SQLite metadata, with no search index built.

The local run artifacts under ignored `local/aquinas-first-pass/` retain command arguments and exits (`handoff-commands.json`), content/role checks (`handoff-verification.json`), ingest/restart results (`document-search-run.json`), and the producer exports. First ingestion was observed at `2026-11-19T12:29:57.033Z`. The following identities and recipe record the result without committing full source text or local databases.

### Revisions

| Owner | Commit |
| --- | --- |
| Formal Logos materializer | [5cbba270](https://github.com/moritzbrantner/formal-logos/blob/5cbba270d1d99e3e852cff632cf42725cb90cb04/scripts/source_evidence.py) |
| Document Search | [527f30f2](https://github.com/moritzbrantner/document-search/blob/527f30f27dffe536775e1705a1210bfe044ad0c6/src/filesystem/corpusStore.ts) |
| Philosophy Extractor | [18d38cd7](https://github.com/moritzbrantner/philosophy-extractor/blob/18d38cd7fc199569cdfc45de3653a2005d29d6cf/packages/exporters/examples/export_belief_sources.rs) |
| Belief Lab | [44fa0891](https://github.com/moritzbrantner/belief-lab/blob/44fa089183b8586f5953725497388a12bac921cf/apps/belief-cli/src/explain.rs) |

The original TEI is retained separately. Document Search's input asset is the canonically materialized `.txt` file; its native processor name remains **`document-search/html-extraction`**, because the existing text import normalizes plain text through that extraction function. The exporter did not invent a new processor name or claim to have extracted the TEI itself.

### Native identities

| Field | Value |
| --- | --- |
| Input file SHA-256 | `cac2c42290da19da8b5c084d586c40d7edd6d72658839b8b5caf992d0e63eb96` |
| Extracted article SHA-256 | `068d42d36c220b90407b4d9808c8cbe6011cbd540f0a7a7901c435a112c3c206` |
| Document Search source scope | `source:938283bfe994e556bf04cb5fc4d50e1f2df963dcd98e2f0288172a1c9021a30a` |
| Representation | `representation:57c2c7c77f48cb81707e898a96c0bee3506c846766b01284202d2d0701b288c0` |
| Whole-article segment | `segment:16c94674d30345734a35e02086fed6c7a81cacf784b44dd25da980c334a21422` |
| Segment record revision | `sha256:adbf01d15fcd2ca7f980c775f0a4e1b984d4802a8da477c294597e4368f77363` |
| Source-text evidence | `evidence:philosophy-source:sha256:d90386131361e2dc2531c102788bafeac8b2a029b630ddc7de9c15fcad1ca758` |
| Parent metadata evidence | `evidence:philosophy-source:sha256:032cdfb914dd29719944bdc1f5e4e90fc34c84089fe6a58bef76cd26fbe9331c` |
| Evidence batch revision | `sha256:d790d0480adf227fd8269968d4fe0269c5a99fe61088fb9c2217206c2929c5cd` |

These are the producer's retained local-corpus identities. The source URI is the actual imported text file. Recreating a corpus under a different source path can create different source IDs; retain the exported references and their source artifact.


### Reproduce the canonical article

Use a separate Formal Logos checkout at the pinned revision and its existing selected-passage command:

```sh
python3 "$FORMAL_LOGOS_DIR/scripts/source_evidence.py" materialize \
  2dc7a89a-5d81-5bf7-97b3-a429018d7e4a \
  --output "$FORMAL_LOGOS_SOURCES_DIR/local/aquinas-first-pass/q19-a1.txt"
```

`FORMAL_LOGOS_DIR` and `FORMAL_LOGOS_SOURCES_DIR` name absolute checkout paths. Create the local output directory first. The canonical materializer verifies and reuses its retained raw-byte cache; a fresh checkout must acquire the pinned witness. This run supplied the GitHub API bytes to that cache only after the Git blob matched the existing manifest, then ran the unchanged materializer. Put the resulting text file alone in the Document Search input directory for the recipe below.

### Small reusable API recipe

Document Search has no filesystem `export` CLI command at this revision. Its supported filesystem API supplies the missing composition step; no new exporter is needed:

```ts
import { writeFileSync } from "node:fs";
import { openFilesystemCorpus } from "./src/filesystem/corpusStore.ts";

const corpus = await openFilesystemCorpus("/absolute/path/to/corpus");
try {
  await corpus.ingestFolder("/absolute/path/to/input", {
    importedAt: new Date().toISOString(),
  });
  writeFileSync(
    "/absolute/path/to/corpus-core.jsonl",
    await corpus.exportCorpusCore(
      "git:527f30f27dffe536775e1705a1210bfe044ad0c6",
    ),
  );
} finally {
  corpus.close();
}
```

Save that TypeScript snippet in the pinned Document Search checkout root so the relative import resolves, then run it with Bun after the normal dependency bootstrap. The local acceptance driver `document-handoff.ts` additionally checks the known article hash, full-text preservation, repeated export, and reopening.

From the pinned Philosophy Extractor checkout:

```sh
cargo run --offline --locked -q -p philosophy-extractor-exporters \
  --example export_belief_sources -- /absolute/path/to/corpus-core.jsonl \
  git:18d38cd7fc199569cdfc45de3653a2005d29d6cf > belief-evidence.json
```

Run the exporter a second time with `--source-spans` and a distinct output file to retain its existing `source_span_interchange@1` text/locator artifact:

```sh
cargo run --offline --locked -q -p philosophy-extractor-exporters \
  --example export_belief_sources -- /absolute/path/to/corpus-core.jsonl \
  git:18d38cd7fc199569cdfc45de3653a2005d29d6cf --source-spans > source-spans.json
```

From the pinned Belief Lab checkout:

```sh
cargo run --offline --locked -q -p belief-cli -- \
  explain /absolute/path/to/belief-evidence.json \
  --profile semantic_research --json > belief-import-report.json
```

The `--offline` runs above succeeded after ordinary dependencies had been acquired. No judgments file is supplied.

### Remaining person-attribution boundary

The existing Philosophy Extractor exporter emits **no `subject`**. Belief Lab's person-attribution validator requires evidence with an exact matching `(source.repository, source.scopeId, subject)` for the selected source-local person. Therefore this run imported source evidence; it did **not** create Aquinas observations, person-confidence assessments, or a reviewed identity link to the Sources author profile.

The structural role map stays in this article packet, anchored to the exported source-span text through the reviewed section ranges. The native source-span export contains the whole article, without role fields. `belief_evidence_interchange@1` is strict and reference-only: text, section roles, and philosophical interpretations must not be inserted as extra fields. A future producer-owned, provenance-bearing author binding must precede person observations; supplying an author label must never automatically turn objections or quotations into endorsements. Belief Lab's current public attribution comparison is its Rust/WASM API and workbench, not a native `compare` CLI command.

The current Philosophy Extractor reference adapter accepts `text` and `web_page` sources. Document Search's PDF assets use `document`, so direct PDF-to-belief export remains a separate compatibility gap. This run legitimately used the materialized text asset and retained its upstream TEI provenance separately.

### Runtime note

Normal `bun install --frozen-lockfile` first failed during wasm-pack's download of Binaryen 117. Its supported PATH lookup resolved the environment issue: the exact official Binaryen archive was acquired and verified against its published SHA-256 `3dc677006555b355ea2da5e82602065a161d5e83eaefd3f759afa00b96e83212`, then the unchanged install succeeded. Runtime: Bun 1.4.4, Rust 1.100.0, wasm-pack 0.14.0, wasm-opt 117. The original failure and successful bootstrap have separate retained logs. No optimization setting or source dependency was modified.

## Scope closed and next work

This source-preparation pass is closed for the seven registered work matches, the complete q.19 a.1 role map, and the observed text-reference handoff at the revisions above. The full Aquinas bibliography remains in progress; the imported counts and canonical judgments retain their stated revisions. The run exercises text ingestion and reference export, with no search index or PDF-page acceptance claim.

The next Aquinas reading packet is **essence, dependence and uniqueness**: selected De ente material, SCG I.22 and ST I q.3/q.11. Reuse the current q.14/q.19 assessments while following that positive research direction. The next author remains Aristotle. An author-specific Belief Lab pilot needs the narrow producer-side subject binding and reviewed observations described above; a source author label alone is insufficient. The selected Physics scan supplies a bounded OCR comparison when a scan is needed. The existing Russell–Copleston and YouTube discovery packets retain their place in the [author program](../docs/specs/author-research-program.md).

## Validation

The repository's normal local gate passed: `cargo fmt --check`, all **32 tests**, `cargo clippy --locked --all-targets -- -D warnings`, and `cargo run --locked -- check --research registry/author-research.json`. The author report was regenerated from the curated sidecar. Independent review checked work ownership, scoped completion, provider notices, all eight article sections, canonical links and graph counts. Routine CI continues to use offline fixtures; the historical-source acquisition and cross-project run above are selected research acceptance, not new CI requirements.

Shared conventions were resolved from `moritzbrantner/coding-agent-conventions` at `833179f39f2f3848341f93e7a9acb4950b6803f0`. No source/edition identity, imported catalogue entry, owner implementation or canonical philosophical record changed.
