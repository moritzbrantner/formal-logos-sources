# Author research program

Status: proposed working sequence, 2026-11-19. This is a research plan, not a record of completed author passes or an implementation of the processing adapters.

## Goal and starting point

Build a reusable account of each author's works, where a usable edition can be obtained, what we have actually examined, and how the findings connect to Formal Logos. Start with historical authors whose writing corpus can be bounded, while using contemporary papers, lectures and debates whenever they address the active question.

At this plan's 2026-11-19 snapshot, the catalogue contained **128 attribution records, 161 grouped works, 164 canonical source records, 294 referenced witnesses and 516 passage references**. These are historical planning denominators, not the latest inventory; see the [catalogue freshness audit](../catalogue-freshness.md) for the reconciled current counts. These counts come from Formal Logos revision [597e4b2](https://github.com/moritzbrantner/formal-logos/tree/597e4b2eb02e092f1769a2e987437bc78e14893a). A comparison with [2f783ab](https://github.com/moritzbrantner/formal-logos/tree/2f783abda4f10be0727c913242ff36b36f80f96f) found the same 164 source IDs and 128 attribution strings; passage counts in this plan remain those of the imported revision. An attribution may be a person, a joint byline, an institution or an unresolved provider.

Five curated profiles already exist: Aristotle, Aquinas, Leibniz, Kant and Russell. At the plan's starting point, their six bibliography inventories were located but unreconciled, and retrieval plans covered eight of the sixteen works linked to those profiles. The [first Aquinas source-preparation pass](../../reports/aquinas-first-pass.md) now matches all seven registered Aquinas works to its selected inventory and raises retrieval coverage to thirteen of those sixteen works. Whole-author inventory reconciliation remains open. See the [author report](../../reports/author-research.md), [catalogue](../../registry/catalog.json) and [curated profiles](../../registry/author-research.json).

The plan has three parts:

1. A **51-person historical sequence**, including three people presently represented only by joint records.
2. A **50-person contemporary sequence**, with additional joint-author packets and document collections explicitly accounted for.
3. A repeatable author pass that uses the existing document, OCR, video and belief-attribution projects.

The [complete roster](#complete-author-and-attribution-roster) accounts for all 128 exact attribution records. Its counts describe registered material, not importance or reading completion. The queue numbers below are planning labels only; they do not replace catalogue IDs.

## How we work through the queue

Keep **one active author pass**. Prepare bibliography and access routes for the next two authors in parallel, so a missing edition does not stop progress. Pull a later author forward when a specific current finding makes that comparison useful; record the reason in the active research note. The sequence is a default working order, not a dependency that forces every earlier author's entire corpus to be read first.

For each historical author, distinguish two scopes:

- **Bibliography scope:** account for the authored corpus covered by a named catalogue or critical edition, including letters, unpublished material, fragments, disputed works and lost works where that inventory lists them. Record what the inventory excludes and its publication state. A publisher's volume list or one online collection may be only a partial denominator.
- **Reading scope:** select the works and passages that bear on Formal Logos's current questions. Give every other inventoried item an explicit disposition, such as later topic, outside this pass, unavailable, disputed attribution or no surviving text. A disposition is not a claim that the item has been read.

For contemporary authors, use a dated publication snapshot and the registered works first. Do not make a completed career bibliography a prerequisite. Reopen a pass for a relevant new publication or a changed research question. Historical passes can also reopen for newly identified material, a better edition or a changed interpretation.

### Historical sequence: A01–A51

The opening order concentrates on the necessary-foundation and divine-attribute questions already developed in Formal Logos. Each pass preserves useful prior findings and follows a specific source question; it does not restart settled work merely because an author is being catalogued.

The current [Formal Logos research direction](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/AGENTS.md) prioritizes God-existence and attribute work. Historical Christian/Catholic identification, biblical and institutional packets are retained for later activation when that research phase is selected. Plutarch's identity question can be activated independently of that historical-theology phase.

| Pass | Author | First reading packet | Main connection to Formal Logos |
| --- | --- | --- | --- |
| A01 | Thomas Aquinas | Inventory the seven registered works; selected De ente et essentia and SCG I.22 material; ST I q.3/q.11, with q.14/q.19 as existing baselines | Essence/existence, unity, concurrent dependence, intellect and will; reuse the existing knowledge and will readings. |
| A02 | Aristotle | Physics, Metaphysics and Posterior Analytics, then the registered Categories and Ethics material | Act/potency, kinds of priority, causal explanation and first principles; align senses reused in the Aquinas pass. |
| A03 | Gottfried Wilhelm Leibniz | Monadology and Leibniz's papers in the Clarke correspondence | Sufficient reason, necessity, contingency and the relation between explanation and choice. |
| A04 | Samuel Clarke | Clarke's replies in the same correspondence | Identify the exact disagreements with Leibniz; process the shared work once and maintain separate attributed positions. |
| A05 | Benedict de Spinoza | The registered Ethics material and its immediate argument dependencies | Compare necessity, essence, explanation and unity with the distinctions already used in Formal Logos. |
| A06 | David Hume | The registered Treatise and Enquiry selections | Causal necessity, conceivability, induction and the scope of explanatory claims. |
| A07 | Immanuel Kant | The registered Critique of Pure Reason selections, retaining A/B locators | Separate the cosmological inference challenge, limits of cognition and existence/predication issues. |
| A08 | Bertrand Russell | Logical Atomism lectures V–VI; the existing Russell–Copleston recording as a separate media packet | Existence and quantification; reported, hypothetical and endorsed positions in debate. |
| A09 | Anselm of Canterbury | Monologion | Unity, perfection and the supreme good; use existing uniqueness work to choose the next passage. |
| A10 | Plotinus | Registered Enneads passages with surrounding arguments | Unity, composition, the One and Intellect. |
| A11 | Proclus | Elements of Theology | Unity, participation and dependence; compare the precise links with Plotinus. |
| A12 | Plato | Parmenides, Republic, Phaedo, Timaeus and Theaetetus, scoped to the registered questions | Forms, participation, intelligibility and soul; preserve dialogue-speaker roles. |
| A13 | Augustine of Hippo | City of God; expand the bibliography before selecting further works | Connect the registered historical/theological claim and identify relevant truth or intellect passages. |
| A14 | John Duns Scotus | The registered Metaphysics questions | Establish the first passage packet; investigate causal order, first principles and infinity where the text addresses them. |
| A15 | Réginald Garrigou-Lagrange | Dieu, son existence et sa nature | Distinguish inherited Thomist premises from added arguments and replies. |
| A16 | Abu Hamid al-Ghazali | The Incoherence of the Philosophers | Causal necessity and creation; locate an appropriate edition and the first selected passage. |
| A17 | Nicolas Malebranche | The Search After Truth | Created causal powers and occasionalism; establish a source-grounded comparison. |
| A18 | J. M. E. McTaggart | The Unreality of Time and The Nature of Existence II | Time, temporal ordering and the dependence claims actually used by current arguments. |
| A19 | Ludwig Wittgenstein | The four registered works, keeping periods separate | Meaning, criteria, ethics and certainty; distinguish text from later commentary. |
| A20 | Rudolf Carnap | The two registered essays | Framework-relative questions, semantics and the stated criticism of metaphysics. |
| A21 | A. J. Ayer | Language, Truth and Logic | The exact verification criterion and its consequences. |
| A22 | Moritz Schlick | Meaning and Verification | Compare his formulation with Ayer and Carnap without collapsing their positions. |
| A23 | Sextus Empiricus | Outlines of Pyrrhonism I | Suspension, regress and the commitments of the skeptical argument. |
| A24 | William of Ockham | Summa logicae I | Terms, universals and nominalist alternatives. |
| A25 | Alexius Meinong | On the Theory of Objects | Objects, existence and predication; connect to the Russell packet. |
| A26 | Saul A. Kripke | Identity and Necessity | Metaphysical necessity, identity and epistemic access. |
| A27 | David Lewis | Humean Supervenience Debugged | Laws and dependence as an explicit alternative account. |
| A28 | C. D. Broad | The Mind and Its Place in Nature | Mind, emergence and the scope of explanatory claims. |
| A29 | Michael Dummett | Elements of Intuitionism | Proof, meaning and the specific logical choices under comparison. |
| A30 | Wilfrid Sellars | Empiricism and the Philosophy of Mind | Conceptual content, epistemic authority and the given. |
| A31 | Alvin I. Goldman | What Is Justified Belief? | Reliabilist belief formation and the existing justification comparisons. |
| A32 | Charles Leslie Stevenson | The Emotive Meaning of Ethical Terms | Ethical language and disagreement. |
| A33 | Richard N. Boyd | How to Be a Moral Realist | A substantive naturalist account to compare with the moral-governance route. |
| A34 | William L. Rowe | The Problem of Evil and Some Varieties of Atheism | The evidential inference and existing replies. |
| A35 | D. Z. Phillips | Religion without Explanation; the shared Nielsen exchange | Religious language and explanation, preserving the two sides. |
| A36 | Peter Winch | Understanding a Primitive Society | Practices, rationality and conceptual frameworks. |
| A37 | Kai Nielsen | Wittgensteinian Fideism; the shared Phillips exchange | The actual criticism and response, with no duplicated processing of the joint work. |
| A38 | Pierre Duhem | The Aim and Structure of Physical Theory | Scientific explanation and underdetermination. |
| A39 | Isaac Newton | The registered Principia edition | Motion, laws and space; keep physical and metaphysical uses explicit. |
| A40 | Albert Einstein | On the Electrodynamics of Moving Bodies | Relativity, simultaneity and the temporal premises under discussion. |
| A41 | Max Born | The registered 1926 collision paper | The paper's probabilistic result and the causal inference being drawn from it. |
| A42 | James B. Hartle | Wave Function of the Universe, jointly with Hawking | Record the model's assumptions and the precise cosmological use in Formal Logos. |
| A43 | Stephen W. Hawking | The same joint paper; separate author bibliography | Reuse the shared packet and distinguish authorship from individual endorsement beyond the paper. |
| A44 | Charles Darwin | Origin of Species and the registered autobiography | Evolution and teleology; keep scientific results and personal-belief evidence distinct. |
| A45 | Justin Martyr | Second Apology | The registered apologetic claim and its context. |
| A46 | Clement of Rome | First Corinthians | The registered letter and historical inference; distinguish it from Paul's letter. |
| A47 | Josephus | Jewish Antiquities | The exact testimony, textual issue and historical claim being used. |
| A48 | Tacitus | Annals | The cited testimony and its inferential role. |
| A49 | Plutarch | Life of Theseus | Identity and persistence; bring this forward if that question becomes active. |
| A50 | Adolf von Harnack | History of Dogma I | Historical reconstruction and its relation to the existing doctrinal argument. |
| A51 | Ernest Renan | The Life of Jesus | The registered historical claims and the reasons offered for them. |

The 48 individual records in the historical roster plus Clarke, Hartle and Hawking produce these 51 person passes. The latter three remain represented by joint source records until explicit profile mappings are curated. Life-status evidence is completed as a profile is activated; the roster is not a blanket assertion that all identity records have already been verified.

### Start from the findings already present

The following are research destinations at the inspected Formal Logos revision, not fresh claims that an author endorses every formal condition in those notes. The pass should attach its reading to the existing distinctions and assessments.

| Existing research question | Author packet | Starting record and practical implication |
| --- | --- | --- |
| How are necessity, essence, adequate explanation and underivedness related? | Aquinas, Leibniz, Spinoza | [Five necessary-being senses](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/notes/necessary-being-five-senses.md): preserve the five meanings and investigate the identified SCG I.22 source connection. |
| What does sufficient reason cover, and how does it relate to choice? | Leibniz, Clarke, Hume | [PSR strength](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/notes/psr-strength-755.md) supplies formal distinctions; the [Leibniz–Clarke reading](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/notes/spatial-translation-and-reason.md) supplies the historical attribution. Do not turn the former's analyst rows into authored theses. |
| What is current dependence, and what kind of priority is involved? | Aristotle, Aquinas | [Temporal actualization](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/notes/temporal-actualization-fork.md) and [priority/independence](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/notes/priority-independence-logic-support-investigation.md) distinguish simultaneous instrumental dependence, existential priority and temporal order. |
| How is uniqueness connected to the source already identified? | Aquinas first | [Q11 uniqueness](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/q11-uniqueness-connection.md): focus on common nature and individuation. The existing [intellect investigation](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/immaterial-intellect-reception.md) recommends this reallocation after its inspected approach yielded no further discriminator. |
| What do intellect, will and an act of willing each add? | Aquinas; Leibniz/Clarke; Plotinus as a later comparison | [Intellectual appetite](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/intellectual-appetite-source.md): retain the existing capacity/exercise and natural/intellectual appetite distinctions. Reuse the q.14/q.19 readings as the integration baseline. |
| What do cosmological inference and existential language license? | Kant, Russell | The separate [Kant](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/kant-cosmological-inference.md) and [Russell](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/russell-existence.md) dossiers prevent cognitive-scope, predication and explanatory-adequacy questions from being collapsed. |
| How might moral order support intellect and will in the same source? | Aquinas, Boyd; contemporary companion packets | [Moral governance](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/notes/moral-governance-intellect-will.md) and [naturalistic moral realism](https://github.com/moritzbrantner/formal-logos/blob/2f783abda4f10be0727c913242ff36b36f80f96f/docs/naturalistic-moral-realism.md) already contain substantive comparison accounts. Extend a concrete remaining question rather than resetting those accounts to empty placeholders. |

Within A01, reconcile the existing records, then prioritize the essence/dependence and q.11 questions. Read further on q.14/q.19 when there is a concrete new source distinction or argument to investigate. This preserves both philosophical progress and the value of using their complete articles to test attribution tooling.

### Contemporary sequence and companion packets

The [contemporary roster](#contemporary-individuals) specifies B01–B50 in order, with a first research packet for each. Its main blocks are:

| Passes | Focus | Opening authors |
| --- | --- | --- |
| B01–B10 | Cosmological explanation, sustaining causation and sufficient reason | Koons, Pruss, Oppy, Feser, Schmid, Gorman, Rutten, Della Rocca, Dasgupta, Pearce |
| B11–B28 | Grounding, powers, composition, truth and argument discovery | Schaffer and Fine through McIntosh |
| B29–B37 | Moral language, realism, access and authority | van Roojen, Joyce, Street, Enoch, Railton, Gibbard, Darwall, Campbell, Sayre-McCord |
| B38–B50 | Warrant, hiddenness, physicalism, religious language and interpretive reference works | Plantinga and Rea through Fetzer |

Use these as companion readings when relevant, rather than making Aquinas wait for all 51 historical passes before reading Feser or Schmid. A commentator's entry gives us that commentator's account; primary-author attribution follows the cited original text. The historical and contemporary queues can therefore alternate while retaining one active author pass.

Named people appearing only in joint records also receive explicit attention:

| Person packet | Activate with | Scope of the first pass |
| --- | --- | --- |
| Christoph Benzmüller | Gödel/Scott formalization packets | Formalizer/coauthor contribution, exact theory and assumptions. |
| Dana Scott | The same packets | The attributed variant and commentary; establish life status from evidence rather than assuming it. |
| Bruno Woltzenlogel Paleo | Gödel's God in Isabelle/HOL | Coauthor contribution and formalization scope. |
| Cameron Bertuzzi | McIntosh's argument-survey video | Presenter/contributor roles and cited originators. |
| Peter D. Klein | Epistemological infinitism | His contribution and primary-source leads. |
| John Turri | The same joint reference entry | His contribution and primary-source leads. |
| Anat Biletzki | Wittgenstein commentary | Joint commentary, distinct from Wittgenstein's own assertions. |
| Anat Matar | The same commentary | Reviewed person mapping and contribution. |
| Hannes Leitgeb | Carnap commentary | Joint commentary and references to primary texts. |
| André Carus | The same commentary | Reviewed person mapping and contribution. |
| Stewart Candlish | Private-language commentary | The reconstruction and its textual basis. |
| George Wrisley | The same commentary | Reviewed person mapping and contribution. |

The five empirical research-team records are work-first packets: Li, Verma, Dye, Tompkins and Blount with their actual coauthors. Recover the complete byline from the paper when activated; a full career dossier for every coauthor is optional unless a research question calls for it. Institutions receive document/version passes. Biblical collections, Paul-attributed texts, interview participants and anonymous providers retain their distinct attribution tasks in the roster. Frank Turek and Steven Bonnell II are reviewed as quoted/interviewed speakers where applicable, not inferred coauthors.

### Next expansion beyond the current attribution roster

Add authors when an active question supplies a reason and a first source, rather than treating the existing catalogue as the limit of the project. Useful proposed additions are **Descartes and Berkeley** for mind/idealism; **Avicenna, Maimonides, Boethius, Bonaventure, Suárez and Christian Wolff** for the essence, necessity and intellect comparisons; **Bolzano and Frege** for logic and existence; **Gödel** for the underlying manuscript behind existing formalizations; and **Frederick Copleston** for the existing debate recording. These are proposed direct-author additions, not claims that their original works have already been catalogued. They can move ahead of lower-priority roster entries when their first packet addresses the current question.

## The repeatable author pass

### 1. Establish identity and the bibliography denominator

Start from the exact catalogue attribution. Link the individual profile explicitly, including joint authorship and aliases, without splitting names automatically. Record supported life status and source. Select a scholarly catalogue, critical edition inventory or institutional bibliography, with scope and observation date. Reconcile the existing works against it and identify omissions, alternate titles, translations, disputed items and portions of larger works.

Use the existing curated sidecar for supported profile/inventory/retrieval fields and the catalogue for work/edition metadata. A short author research note can record the reconciliation decisions and pass scope. This plan does not introduce another registry, status database or queue service.

### 2. Prepare the selected editions

Prefer a suitable existing machine-readable text, HTML or TEI source, or text-bearing PDF. For each candidate record the work, edition/editor/translator, language, actual coverage, provider, exact route, access observation and edition-specific rights evidence. A chapter or selected document must not be recorded as the complete book. Keep original bytes, acquisition receipt and hash in the existing local/external source store; keep shareable metadata in Git.

Use the existing local PDF preparation workflow for its current Poppler-based witness contract. The rights/access observation belongs to the edition and provider; the historical author queue supplies no automatic rights decision. If one edition is unavailable, record that blocker and pursue another suitable route or the next scoped packet.

### 3. Search, read and record attribution in context

Search the prepared corpus for the active question, inspect the selected passage and the context needed to understand it, and preserve the conventional locator. For a PDF also record its physical page; for OCR retain the page image/region and derived-text provenance; for video retain the exact timed span and speaker evidence.

Record the role of the text: author's assertion, objection, quotation, report, hypothetical, concession or reply. This is especially useful for Aquinas's articles, Plato's dialogues, correspondence and debates. Retain original-language terms and significant translation alternatives through the existing Formal Logos glossary/sense records. A lecture may identify a promising argument or primary source even before the primary passage is resolved.

### 4. Connect the result to the existing research

Follow the existing source → passage → claim → statement → argument/attempt connections before adding records. Classify what the reading contributes: confirms an existing reading, supplies another premise or route, narrows a term, states a substantive objection/reply, changes an assessment, or adds no distinct result. Reuse existing records where the meaning agrees; preserve a real difference when it does not.

Formal Logos owns the canonical passage, interpretation, argument and proof changes. Sources records the preparation and retrieval side. Belief Lab can record what a person is documented as endorsing; that attribution is separate from Formal Logos's assessment of the proposition. Neither a model confidence score nor an implication from something an author asserted establishes a further personal belief.

Create a focused research issue only when there is a concrete remaining question and a useful next action. Do not pre-create one issue per author or convert every hypothetical alternative into mandatory work. Use the project's existing bounded-research and provisional-assessment policies.

### 5. Close the scoped pass and name the next work

Each pass ends with a compact report containing:

- The bibliography denominator and what has been reconciled, including known gaps.
- The selected reading scope, editions and actual coverage.
- The passages/positions reviewed, with locators and explicit attribution roles.
- The existing Formal Logos records reused and the concrete additions or changed assessments.
- Unresolved access/text/interpretation questions, each with a specific next action or a reason to defer.
- The next author and the evidence or new question that would reopen this pass.

Use **“pass closed for this scope and revision”**, not “everything by this author is understood.” A pass may close with a provisional interpretation or a clearly recorded access gap. Claims about a complete bibliography require the declared inventory to be reconciled; reading completion remains limited to what was actually read. The current sidecar does not encode a completed reconciliation state, so this wording belongs in the research report until a concrete need justifies extending that schema.

## How the existing projects help

The source-processing ownership decision is already recorded in [ADR 0001](../adr/0001-reuse-source-processing-capabilities.md). The inspected upstream implementations provide useful components today. The missing work is the narrow Sources handoff and acceptance on the selected material.

| Existing owner | What can be reused | First useful application | Remaining acceptance or integration |
| --- | --- | --- | --- |
| **Document Search** | Filesystem corpus, content-addressed bytes, embedded PDF text, cached extraction, search, source-span/corpus-core exports | Find a passage in an actual selected edition and resolve it to the original page | Map physical page and printed locator explicitly; keep PDF.js search text distinct from Poppler witness text. Image-only PDFs currently yield no text. |
| **Visual Analysis / OCR** | Still-image OCR, structured blocks/lines/tokens, boxes, confidence and provenance; native TrOCR and the existing JSON-command backend | Recover one needed passage from a scanned page | Demonstrate useful recognition on that page/script. The inspected real model smoke test is a small English “HELLO” image, not historical-page acceptance. |
| **Media Intelligence** | Scene/frame composition, OCR producer integration, timed tracks and partial-failure handling | Text visible in an existing lecture or presentation | The real whole-frame TrOCR quality issue remains open; reuse the existing command seam when evaluating a suitable engine. Book pages use still-image OCR directly. |
| **YouTube Corpus** | Caption/ASR search, context, timed spans, diarization and deterministic evidence exports | Locate an existing debate/lecture span and follow its cited work | Review speaker identity and reported-author roles; upload date is not the expression date. Preserve producer-owned span hashes. |
| **Belief Lab** | Stance states, dated/conflicting observations, reviewed proposition relations, correction history and reference-only investigation projects | Maintain an author's documented-position ledger across texts and recordings | Map source-local people to curated author IDs; check BCE date handling; improve the existing attribution pilot before unattended batching. |
| **Formal Logos Sources** | Catalogue, curated inventories/retrieval candidates, PDF preparation, research-connection reports | Coordinate the author pass and produce traceable source candidates | Add a small adapter to existing exports and bounded excerpt resolution when the first pilot needs it. |
| **Formal Logos** | Canonical sources, passages, claims, terminology, arguments, attempts and proof checks | Integrate the finding into the existing philosophical research | Use its existing semantic workflow; no build or CI dependency on the processing tools. |

The reusable contracts include source_span_interchange@1, moenarch_corpus_core_batch@1 and belief_evidence_interchange@1; Belief Lab's v2 media evidence adds timing/scene fields. Sources should preserve those producer-owned structures and versions rather than create a competing interchange format. The current document-search implementation uses one-based physical PDF pages and zero-based corpus-core page_index, so conversion needs an explicit test. The current Belief Lab person key is source-local: (sourceRepository, scopeId, subjectId). Same-name strings are insufficient to merge people.

Belief Lab's portable investigation projects contain references and attribution history; full original-source bundles and automatic source recovery are not thereby provided. Its separate stance, attribution-confidence and conviction concepts are useful for the author ledger. Disjoint dated observations can show differing positions over time without silently declaring a change of mind; unknown or overlapping dates preserve conflicts for review.

### OCR route

Use existing text when it is adequate. When a selected edition requires OCR, render the needed page, call the existing still-image interface, preserve dimensions/region and compare a short selected passage with the image. Check reading order and meaning-changing errors, including negations, numerals, diacritics and ligatures where relevant. Keep raw OCR, a corrected transcript and the canonical witness representation distinct.

Accept clean Latin print, polytonic Greek, German/Fraktur or other scripts separately as the queue actually reaches those editions. A requested language option is metadata, not evidence of recognition quality. The existing JSON-command seam can host an established engine; the Media Intelligence issue already proposes RapidOCR first and Tesseract as an alternative, but neither is claimed accepted by this plan. A new OCR engine, whole-library OCR run or general ingestion service is unnecessary for the first author pass.

### Belief Lab and YouTube route

For primary recordings, retain the exact utterance, timing and reviewed speaker identity. For commentary about historical authors, maintain the relation **lecturer → reported position → cited original work**. The lecturer's words remain evidence of that lecture, even when they accurately explain Aquinas. This also lets a video supply a valuable discovery lead without manufacturing a primary quotation.

Belief Lab already has a real Russell–Copleston pilot with a 17-minute recording and an expected-attribution set. The documented Stage 4 result is **2 hits, 11 misses and 2 fabricated attributions against 13 expected entries**. One failure turns Russell's hypothetical “Then God exists” into an assertion. Reuse that known case to improve attribution roles instead of treating automatic extraction as author-pass completion. The pilot's golden speaker-label matching is evaluation machinery, not a general identity resolver.

Where the prepared environment retains the saved stages, its existing replay command can evaluate those artifacts without new inference:

~~~sh
python3 scripts/russell_copleston_stage4.py --replay-only
~~~

Check that the saved stage artifacts exist first; the repository alone does not supply every model and intermediate. New model execution is a separate implementation step when required.

## First implementation packets

These are successive, bounded deliverables. They are not calendar commitments or newly opened issues.

| Order | Deliverable | Acceptance |
| --- | --- | --- |
| 1 | **Aquinas inventory and source packet** | Reconcile the relevant bibliography scope, give the seven registered works a disposition, fill the next retrieval gaps, and select the first actual edition and complete article. |
| 2 | **One complete Aquinas article through the existing tools** | Include objections, the contrary position, response and replies. Produce reviewed attribution roles and links to existing claims/terms. Use q.19 a.1 or q.14 a.1 as an integration baseline; this need not reopen their existing philosophical assessment. |
| 3 | **One searchable-edition handoff** | A Document Search result resolves to the exact edition/page; the existing Sources workflow prepares the canonical representation independently. Repeat ingestion preserves identity and reuses existing extraction. |
| 4 | **One scanned-page handoff, when a selected edition requires it** | Preserve page/region and actual producer configuration; a selected passage is usable against the image. This task can proceed alongside later authors and does not block available machine-readable texts. |
| 5 | **Russell–Copleston attribution repair** | Reuse saved pilot stages where available; preserve timestamp/speaker identity; correct the known hypothetical/reporting mistakes and measure against the existing expected set. Repair belongs in the owning pipeline. |
| 6 | **One existing YouTube lecture as a discovery bridge** | Search the user's existing corpus for the active Aquinas/Aristotle/Leibniz question, export a timed span, separate lecturer from reported author and resolve one primary-work reference. |
| 7 | **Continue A02 and onward** | Apply the same pass report; prepare the next two bibliographies; bring contemporary companion packets forward for concrete questions. |

The first adapter work should carry existing source references, bounded text, representation provenance and reviewed author mappings. Acceptance must include the real selected source and the attribution role that motivated the adapter; synthetic offline regression tests can then cover identity, locator conversion, repeat import and misattributed quotations. Routine CI continues to avoid historical-source downloads and model inference.

## Where records belong and how progress is assessed

| Record | Owner |
| --- | --- |
| Works, editions, retrieval candidates, bibliography scope and person-profile mappings | Existing Sources catalogue and curated sidecar |
| Original bytes and acquired payloads | Existing ignored local store or the explicitly chosen external store |
| Search text, OCR, ASR and their extraction receipts | The producing project's existing artifacts/cache |
| Documented-person stance observations and review history | Belief Lab |
| Canonical passages, interpretation, definitions, claims, arguments and research assessments | Formal Logos |
| Author sequence and scoped pass decisions | This plan and small source-research notes; reference canonical records instead of copying them |

Assess progress by reconciled inventory coverage, useful editions resolved, selected passages reviewed, concrete research connections, changed findings and closed scoped passes. Report the denominator for any percentage. Keep retrieval coverage, reading coverage and philosophical progress separate. A large passage count or a successful extraction is not itself a new research result.

The initial [catalogue and connections #8](https://github.com/moritzbrantner/formal-logos-sources/pull/8), [profiles and retrieval #9](https://github.com/moritzbrantner/formal-logos-sources/pull/9), and [author plan #10](https://github.com/moritzbrantner/formal-logos-sources/pull/10) are merged. The [first Aquinas source-preparation report](../../reports/aquinas-first-pass.md) records the completed seven-work subset reconciliation and complete q.19 a.1 article reading, with observed tool-run results and remaining handoffs. It keeps full-bibliography completion, source preparation and downstream person attribution separate.

## Evidence used for this plan

The capability audit read repository code, documentation and test sources at the following revisions on 2026-11-19. It did not run upstream models or test suites. Open issues and PR status are observations at that date, not immutable capability guarantees.

| Repository | Inspected evidence |
| --- | --- |
| Formal Logos Sources, abf2ae0 | [README](https://github.com/moritzbrantner/formal-logos-sources/blob/abf2ae0cf0907438ebb1331fad22a80e45f1f263/README.md), catalogue and curated-profile implementation |
| Document Search, 527f30f | [README](https://github.com/moritzbrantner/document-search/blob/527f30f27dffe536775e1705a1210bfe044ad0c6/README.md), [PDF adapter](https://github.com/moritzbrantner/document-search/blob/527f30f27dffe536775e1705a1210bfe044ad0c6/src/filesystem/pdfAdapter.ts), [PDF tests](https://github.com/moritzbrantner/document-search/blob/527f30f27dffe536775e1705a1210bfe044ad0c6/src/test/filesystemPdf.test.ts) |
| Visual Analysis, f5a670e | [OCR README](https://github.com/moritzbrantner/visual-analysis/blob/f5a670e35b942be22dac8530e9d558b89eea65be/crates/image/image-analysis-ocr/README.md), [backend implementation](https://github.com/moritzbrantner/visual-analysis/blob/f5a670e35b942be22dac8530e9d558b89eea65be/crates/image/image-analysis-ocr/src/lib.rs), [external smoke test](https://github.com/moritzbrantner/visual-analysis/blob/f5a670e35b942be22dac8530e9d558b89eea65be/crates/image/image-analysis-ocr/tests/external_onnx_smoke.rs) |
| Media Intelligence, 4420e17 | [README](https://github.com/moritzbrantner/media-intelligence/blob/4420e175d440133e17fa815ccaaaec322fcbb0a4/README.md), [OCR adapter](https://github.com/moritzbrantner/media-intelligence/blob/4420e175d440133e17fa815ccaaaec322fcbb0a4/crates/media-intelligence-analysis/src/visible_text.rs), [adapter tests](https://github.com/moritzbrantner/media-intelligence/blob/4420e175d440133e17fa815ccaaaec322fcbb0a4/crates/media-intelligence-analysis/tests/ocr_adapter_execution.rs), [issue #34](https://github.com/moritzbrantner/media-intelligence/issues/34) and [draft PR #41](https://github.com/moritzbrantner/media-intelligence/pull/41) |
| Belief Lab, 44fa089 | [README and pilot results](https://github.com/moritzbrantner/belief-lab/blob/44fa089183b8586f5953725497388a12bac921cf/README.md), [evidence interchange](https://github.com/moritzbrantner/belief-lab/blob/44fa089183b8586f5953725497388a12bac921cf/docs/EVIDENCE_INTERCHANGE.md), [workbench/project format](https://github.com/moritzbrantner/belief-lab/blob/44fa089183b8586f5953725497388a12bac921cf/docs/WORKBENCH.md), [stance tests](https://github.com/moritzbrantner/belief-lab/blob/44fa089183b8586f5953725497388a12bac921cf/apps/belief-cli/tests/stance_states.rs), [project tests](https://github.com/moritzbrantner/belief-lab/blob/44fa089183b8586f5953725497388a12bac921cf/apps/belief-cli/tests/projects.rs) |
| YouTube Corpus, b5a64e3 | [README](https://github.com/moritzbrantner/youtube-corpus/blob/b5a64e3011e723bcfebed699bc50f43737b7bcc4/README.md), [export tests](https://github.com/moritzbrantner/youtube-corpus/blob/b5a64e3011e723bcfebed699bc50f43737b7bcc4/tests/corpus_exports.rs), [speaker-turn ADR](https://github.com/moritzbrantner/youtube-corpus/blob/b5a64e3011e723bcfebed699bc50f43737b7bcc4/docs/adr/0002-speaker-turn-corpus-core-compatibility.md) |

## Complete author and attribution roster

This appendix accounts for all **128 attribution records** in the current source roster. It is an exhaustive work queue, not a claim that the records identify 128 distinct people. The main plan supplies the working sequence; the tables below organize the available packets.

Work and passage counts come from the imported Formal Logos revision [`597e4b2eb02e092f1769a2e987437bc78e14893a`](https://github.com/moritzbrantner/formal-logos/tree/597e4b2eb02e092f1769a2e987437bc78e14893a): **161 works, 164 source records, 294 witnesses and 516 passage references**. The current source roster was also checked at `2f783abda4f10be0727c913242ff36b36f80f96f`; attribution strings and source IDs were unchanged.

“Historical” and “contemporary” are planning cohorts. **Contemporary does not mean known to be living.** Life-status verification remains a first-pass task where evidence is absent. Counts measure registered references, not reading completion, author importance or complete bibliographies. Each linked name opens its first canonical source record; the [catalogue](../../registry/catalog.json) retains every associated work and source ID.

### Historical individuals

These 48 individual records use the A-series order above. A04, A42 and A43 are the Clarke, Hartle and Hawking person passes attached to joint records below; they are not extra attribution rows.

| Queue | Attribution | Works | Passages | Initial question or packet |
| --- | --- | ---: | ---: | --- |
| A01 | [Thomas Aquinas](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/aquinas-compendium-theologiae.json) <!-- attribution:fl-author-c6276031dd1481f78e8c33e733b8b41f2df2aa2c7d139a9317931c417a88047b --> | 7 | 92 | Essence/existence, concurrent causation, intellect, will and goodness in the seven registered works. |
| A02 | [Aristotle](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/aristotle-categories.json) <!-- attribution:fl-author-a90dfe4fb4d5e4a3c287581bcdfabe9ec20e1339414df11ea62fbec0322be1a6 --> | 5 | 94 | Act/potency, causation, substance and demonstration; align terms reused by Aquinas. |
| A03 | [Gottfried Wilhelm Leibniz](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/leibniz-monadology.json) <!-- attribution:fl-author-ad43c70af99f8fee9da5cca0dfa197355080728a9a3e7fdc66cfcaa8931381a0 --> | 1 | 2 | Sufficient reason, possibility and necessary intellect; keep the Clarke exchange’s speakers distinct. |
| A05 | [Benedict de Spinoza](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/spinoza-ethics.json) <!-- attribution:fl-author-c74b410a2a69b93d586455470027bff89d955df3e6ce7f13c08bc91281c01826 --> | 1 | 1 | Necessity, dependence and contingency throughout the argument structure of the Ethics. |
| A06 | [David Hume](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/hume-treatise.json) <!-- attribution:fl-author-68fb360e628a5b6ed5b07d0abb30033db9a11a4609added604f7b083cf06ba3c --> | 2 | 5 | Causation, induction and explanation in the Treatise and Enquiry. |
| A07 | [Immanuel Kant](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/kant-critique-pure-reason.json) <!-- attribution:fl-author-7a3d6fdfa4ab91d8449eb92de2577bba981d381871e57f921bc492d0485d8339 --> | 1 | 4 | Limits of metaphysical inference; align the Critique’s A/B versions and translations. |
| A08 | [Bertrand Russell](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/russell-logical-atomism.json) <!-- attribution:fl-author-31685b2b98456454418c95924b1342194d5775f9823c7db7bd0bd844dcfa34cd --> | 1 | 2 | Negative/general facts and the registered logical-atomism lectures V–VI. |
| A09 | [Anselm of Canterbury](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/anselm-monologion.json) <!-- attribution:fl-author-43d715ab3a80904a1e90f656f63d7c302788a31b799f8ae1cbb238e683f94059 --> | 1 | 6 | Unity, perfection and the supreme good in the Monologion. |
| A10 | [Plotinus](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/plotinus-enneads.json) <!-- attribution:fl-author-e6da9218c2c029f171ec67dfb999d2489513396dda70eff2807d3336001f7cb3 --> | 1 | 7 | The One, Intellect and dependence in the registered Enneads passages. |
| A11 | [Proclus](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/proclus-elements-theology.json) <!-- attribution:fl-author-d93e14227ff8a958a86e6d335c63f3a946f17bd26a76ed77758da84b8ca7d326 --> | 1 | 1 | Unity, dependence and procession in Elements of Theology; compare Plotinus. |
| A12 | [Plato](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/plato-parmenides.json) <!-- attribution:fl-author-1554fc76c25040f83ac969a355aee85aed80f408f9469e255b829801213bf8d3 --> | 5 | 30 | Forms, intelligibility and soul across five dialogues; distinguish speakers from author endorsement. |
| A13 | [Augustine of Hippo](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/augustine-de-civitate-dei.json) <!-- attribution:fl-author-accc1434b6e774ac262831dae8c73cebd75155453176db30382634d82ce2cf01 --> | 1 | 1 | Start with City of God and its existing claim links; expand toward truth and divine knowledge. |
| A14 | [John Duns Scotus](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/duns-scotus-qq-metaph.json) <!-- attribution:fl-author-c568ddbc1da5cea7c489e87757c2478c742340857aecacc5d2a84e7753fe753f --> | 1 | 0 | Acquire the registered Metaphysics questions; investigate first principles, causal order and infinity. |
| A15 | [Réginald Garrigou-Lagrange, O.P.](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/garrigou-lagrange-dieu-1914.json) <!-- attribution:fl-author-9ddb6d6efc548070150f63ddf2af0e58a424e3a3b17f9e964e0d1a03c1c487e5 --> | 1 | 2 | Thomist arguments and agnostic alternatives; identify inherited and newly supplied premises. |
| A16 | [Abu Hamid al-Ghazali](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/ghazali-tahafut.json) <!-- attribution:fl-author-99215cb485b31f26456aa107032866b88a825754eeb0b28c993c0cf8bdeeb9f5 --> | 1 | 0 | Causal necessity and creation in the Incoherence; establish an edition and passages. |
| A17 | [Nicolas Malebranche](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/malebranche-recherche.json) <!-- attribution:fl-author-4bea8beb5652904fa0a83d520abf75da2aa12eaf2748b5d6b9e0e0742dab2ea5 --> | 1 | 0 | Occasionalism and created causal powers; retrieve the registered Recherche first. |
| A18 | [J. M. E. McTaggart](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/mctaggart-nature-of-existence-ii.json) <!-- attribution:fl-author-170ec9003650cd7642f592374f3259ed86b95a8262d8301bff5734dd1a1ce71e --> | 2 | 2 | A/B-series and existence claims in the two registered works on time. |
| A19 | [Ludwig Wittgenstein](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/wittgenstein-lecture-on-ethics.json) <!-- attribution:fl-author-8838946b8f6a1ddbaaa052ff891ddf2e99cb32c31ade5a078419ef8a031478d8 --> | 4 | 6 | Meaning, ethics and certainty; keep work periods and later interpretations separate. |
| A20 | [Rudolf Carnap](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/carnap-eso.json) <!-- attribution:fl-author-324b068656984728d2b0e78fa6b098aaf02c2c5c6029082d605217487c0218e2 --> | 2 | 4 | Framework choice, metaphysics and semantics in the two registered essays. |
| A21 | [A. J. Ayer](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/ayer-ltl-pelican-ch1.json) <!-- attribution:fl-author-1e58a92e43bddc5c724e6be5a5996568859d4eb7e20f96337daf53eaccb6fefb --> | 1 | 1 | Verification criteria and their consequences in Language, Truth and Logic. |
| A22 | [Moritz Schlick](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/schlick-meaning-and-verification.json) <!-- attribution:fl-author-8c1a37d799c560f3e9c2aa345bac8be67101e2ad9be9b76f7e4ed822cef7955f --> | 1 | 2 | Meaning and verification; compare the precise formulations used by Ayer and Carnap. |
| A23 | [Sextus Empiricus](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sextus-outlines-pyrrhonism.json) <!-- attribution:fl-author-678e46d4d4360228a1add363e4b74b32e97625d415df6e1548dbab1300e495e6 --> | 1 | 1 | Suspension, regress and epistemic commitments in the Outlines. |
| A24 | [William of Ockham](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/ockham-summa-logicae.json) <!-- attribution:fl-author-4d31f7f0de31f74a92edfce60f212dfc174c42bc681da52feddee1c1082e17b5 --> | 1 | 4 | Terms, universals and nominalism in Summa logicae I. |
| A25 | [Alexius Meinong](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/meinong-gegenstandstheorie.json) <!-- attribution:fl-author-c0760c1a02db1cfeabf54de59f65cb592d7b41d82ae13d7f1f429bd74da45090 --> | 1 | 0 | Objects, existence and predication; acquire the registered Gegenstandstheorie text. |
| A26 | [Saul A. Kripke](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/kripke-identity-necessity.json) <!-- attribution:fl-author-34093ef10138e481374e1d645abb37ff951819485860ce2f5a71a3837dc8f24a --> | 1 | 0 | Identity and necessity; distinguish metaphysical necessity from epistemic access. |
| A27 | [David Lewis](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/lewis-humean-supervenience.json) <!-- attribution:fl-author-1c1bed7fffc6294e11347dbe5705d4275faad6fd35a4b4dd87a4a34063987ac1 --> | 1 | 1 | Humean supervenience and laws as an explicit alternative account of dependence. |
| A28 | [C. D. Broad](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/broad-mind-and-its-place.json) <!-- attribution:fl-author-498ca1d88acddf415026aba10f24e9dbe7d8ca102a8474a4acf0e61fc97cc636 --> | 1 | 0 | Mind, emergence and explanatory limits; establish a witness for the registered book. |
| A29 | [Michael Dummett](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/dummett-elements-of-intuitionism.json) <!-- attribution:fl-author-d69d9a6d82af8788f6bfcdcd76321f63adf5b03748f43f972ef9efb729369343 --> | 1 | 0 | Intuitionistic proof and meaning; begin with Elements of Intuitionism. |
| A30 | [Wilfrid Sellars](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sellars-empiricism-mind.json) <!-- attribution:fl-author-d0ec62ff73f74bd2319b3b8b53403f452ac5d4ad819bf504d819b3d95f8a6f11 --> | 1 | 2 | Epistemic authority, conceptual content and the given. |
| A31 | [Alvin I. Goldman](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/goldman-justified-belief.json) <!-- attribution:fl-author-b046358286ad1b322fa6227c45c7e8e6828eecbd6b9ae338a66de59c86108994 --> | 1 | 1 | Reliabilist justification and belief formation in What Is Justified Belief? |
| A32 | [Charles Leslie Stevenson](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/stevenson-emotive-meaning-1937.json) <!-- attribution:fl-author-77682d6c94fbcecfd04f131237f7fe64e3fdd4446339efa69ec7e60de4a80816 --> | 1 | 1 | Emotivism and the function of ethical language. |
| A33 | [Richard N. Boyd](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/boyd-how-to-be-a-moral-realist.json) <!-- attribution:fl-author-fc6dd136ca5a7f99c0084a76047cd340564d7bdffdcb91fa090a8f2f220df45d --> | 1 | 1 | Naturalist moral realism as a specified comparison for the personal-source route. |
| A34 | [William L. Rowe](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/rowe-evil-atheism-1979.json) <!-- attribution:fl-author-b20ec450717a219b5abadb2646fe7970843e8c222efe8b0f888dbe89746ef379 --> | 1 | 1 | Evidential evil and its supporting inference; connect the registered argument to existing responses. |
| A35 | [D. Z. Phillips](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/phillips-religion-without-explanation.json) <!-- attribution:fl-author-142449bb7985435e28427f8d1f8d6e50e212aa05c395d1ad5d68135e4d8068d4 --> | 1 | 0 | Religious language and explanation; coordinate the joint exchange with Nielsen. |
| A36 | [Peter Winch](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/winch-understanding-a-primitive-society.json) <!-- attribution:fl-author-6fbfd77fe38c46e30c7d80f2ae6a3f4bad187bda39e8d596c4177d615d75b405 --> | 1 | 0 | Practices and conceptual frameworks in Understanding a Primitive Society. |
| A37 | [Kai Nielsen](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/nielsen-wittgensteinian-fideism-1967.json) <!-- attribution:fl-author-e24a78c791ebd86ef610ec2a505a9b5d2baaa93f616f096a2d0f350d95686e23 --> | 1 | 0 | Fideism and criticism of religious-language approaches; retain dialogue roles with Phillips. |
| A38 | [Pierre Duhem](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/duhem-theorie-physique.json) <!-- attribution:fl-author-8c2a4c3f03553c41b2320713282fdb45d3a8d83e5f39d601249e22de4c6b25c2 --> | 1 | 1 | Underdetermination and the structure of physical theory. |
| A39 | [Isaac Newton](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/newton-principia.json) <!-- attribution:fl-author-ff8e6f222fb14327ae8524e0c325d8907045f21e72b447d4ec1ae79cae30f3b3 --> | 1 | 3 | Space, laws and motion in the registered Principia edition. |
| A40 | [Albert Einstein](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/einstein-electrodynamics-1905.json) <!-- attribution:fl-author-930cf9e7c367bb802de67e2dd9d8a769402e7f79944ad755cdd1406be3e563e4 --> | 1 | 1 | Relativity and temporal premises; distinguish the paper from later extrapolation. |
| A41 | [Max Born](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/born-1926-stossvorgaenge.json) <!-- attribution:fl-author-9e5adf0065a1e506705970ea449f7a1ccb231e6064f927fc57d173a6c4bba25a --> | 1 | 0 | Quantum causality and indeterminism; recover the registered 1926 paper. |
| A44 | [Charles Darwin](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/darwin-origin.json) <!-- attribution:fl-author-046cd7ecbd90abb30aba5bddf113c3111c2ba6b34e25341ece31662f2c82d2d9 --> | 2 | 1 | Evolution and teleological inference; separate scientific arguments from autobiographical belief evidence. |
| A45 | [Justin Martyr](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/justin-apologia-secunda.json) <!-- attribution:fl-author-60c80e7f2b3c61b987ab6458bbf2370ab92b2b16640b3cf5ca800ff3bb9677a2 --> | 1 | 2 | Historical apologetic claims in the Second Apology and their context. |
| A46 | [Clement of Rome](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/clement-1cor.json) <!-- attribution:fl-author-37147bfd0ec7ee1bdc3f83fb7dd88fef9407fb04f7fe1c43c80323ef7e8679ff --> | 1 | 2 | The registered First Corinthians; preserve its distinction from Paul’s similarly titled letter. |
| A47 | [Josephus](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/josephus-ant.json) <!-- attribution:fl-author-06de42f44638c88cb8085e6274109308e3292aa69d289e582e255cd786de27dc --> | 1 | 1 | Antiquities testimony, variants and the precise historical inference being used. |
| A48 | [Tacitus](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/tacitus-ann.json) <!-- attribution:fl-author-05ddaee472a96947c16ac5dd27efd1025b4abae010b43a92f457c5f919d0126e --> | 1 | 1 | Annals testimony and its role in the existing historical argument. |
| A49 | [Plutarch](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/plutarch-theseus.json) <!-- attribution:fl-author-006873dfd89c62cf4bd361267fd0bf3f4f8d90ea689941df0059086f965ff3d5 --> | 1 | 1 | Identity and persistence uses of the Life of Theseus. |
| A50 | [Adolf von Harnack](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/harnack-history-of-dogma.json) <!-- attribution:fl-author-607262d074bdf37eb2905505e5e875c3af9a747abbfa986948b3a8d4a5c89471 --> | 1 | 2 | History of doctrine; distinguish historical reconstruction from theological acceptance. |
| A51 | [Ernest Renan](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/renan-life-of-jesus.json) <!-- attribution:fl-author-6e0f6dade839836b3caa31bba237171f006f90da0d3513e27e74b6b07bccdaca --> | 1 | 1 | Historical and religious claims in the registered Life of Jesus. |


### Contemporary individuals

These 50 records start with a dated pass through the registered works. Expand a bibliography when it adds material to an active question; do not wait for an author to stop publishing.

| Queue | Attribution | Works | Passages | Initial question or packet |
| --- | --- | ---: | ---: | --- |
| B01 | [Robert C. Koons](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/koons-fourth-way-reconstruction-2022.json) <!-- attribution:fl-author-a800f5a4b91ce3f31571f7ac0d995b923d2bd6394d5a055dfa1509acd75e0d5d --> | 4 | 7 | Cosmological arguments, defeasible inference and the Fourth Way across four registered works. |
| B02 | [Alexander R. Pruss](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/pruss-infinity-causation-paradox.json) <!-- attribution:fl-author-d497e58ef2c7be735efd669871dff6667185e26069ab93634290fb4c07809ed0 --> | 2 | 2 | Leibnizian explanation, infinity and causal paradoxes. |
| B03 | [Graham Oppy](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/oppy-koons-cosmological-argument.json) <!-- attribution:fl-author-5b0b379d452618925755af45cb97be09a2d635341a824f8f3dcc676d3ffea304 --> | 2 | 2 | Precisely stated objections to Koons and cosmic-cause arguments. |
| B04 | [Edward Feser](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/feser-inertia-reply-2021.json) <!-- attribution:fl-author-eb493e2ffc520159b91ba648c7d9cdde54518002e6f7dc64745d8c646440e227 --> | 1 | 1 | The registered existential-inertia reply; map its actual premise commitments. |
| B05 | [Joseph C. Schmid](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/schmid-inertia-reply-3.json) <!-- attribution:fl-author-9cff0c22ef42d3e43b59d7c958e67322d9e2b72329723e932557a81154b0bc10 --> | 1 | 1 | The registered existential-inertia analysis; pair each disputed premise with Feser’s response. |
| B06 | [Michael Gorman](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/youtube-unity-gorman.json) <!-- attribution:fl-author-e77a7d21c8a5ffaf534355f70f71b5b8966dca48428ea2733b08618eb660f360 --> | 1 | 2 | Divine unity and simplicity; retain exact media source spans. |
| B07 | [Gerardus Johannes Emanuel Rutten](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/rutten-renewed-theism-2012.json) <!-- attribution:fl-author-ec311da58de8160332b2b9dc2e24c43b3ffd301b62292ca1bbff17316b3ea4d8 --> | 1 | 2 | Contemporary cosmological routes and proposed repairs. |
| B08 | [Michael Della Rocca](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/della-rocca-psr.json) <!-- attribution:fl-author-028e11d18faebe4dd9e31d18ef05b673b61d12e57b8eee29589476af0797f84a --> | 1 | 2 | The scope and consequences of the principle of sufficient reason. |
| B09 | [Shamik Dasgupta](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/dasgupta-metaphysical-rationalism-2016.json) <!-- attribution:fl-author-554b059cf70419e84f032c13c2828af10256db97a4a944810a4a404714ba0706 --> | 1 | 2 | Metaphysical rationalism and its explanatory commitments. |
| B10 | [Kenneth L. Pearce](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/pearce-grounding-indeterminism-2025.json) <!-- attribution:fl-author-90096a10455ec70949218d3048008a056bbb2697863071c425c13ede66b09bb6 --> | 1 | 1 | Grounding indeterminism and its relation to metaphysical rationalism. |
| B11 | [Jonathan Schaffer](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/schaffer-monism.json) <!-- attribution:fl-author-028d9bc2e03e0291da593f034bb0c59e74d18c8ec24074fa96a2c6fac9a60d86 --> | 1 | 2 | Priority monism and dependence of parts on the whole. |
| B12 | [Kit Fine](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/fine-pure-logic-of-ground.json) <!-- attribution:fl-author-8a409d4658e57142c99d52e773c82f7b189d93c532dff2cbafbe91daa5a6c467 --> | 1 | 1 | The logic of ground; distinguish its formal rules from substantive grounding claims. |
| B13 | [Kelly Trogdon](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/trogdon-grounding-necessity.json) <!-- attribution:fl-author-2d59b7d251387a622ad43e9462b9c5840084fcf362e8e86096d9022cc0dcc20e --> | 1 | 1 | Necessary versus contingent grounding. |
| B14 | [Stephan Leuenberger](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/leuenberger-grounding-necessity.json) <!-- attribution:fl-author-384354efa04021dc0f0d168a15d3b479fb2614f287d9e1eeed5b3de8524f0af1 --> | 1 | 1 | Grounding and necessity; compare Trogdon and Richardson. |
| B15 | [Kevin Richardson](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/richardson-grounding-necessity.json) <!-- attribution:fl-author-032dcfec9cac4b3c26c9924f41dcb49463da544d2a6d3806de25ae8974db4b4f --> | 1 | 1 | How grounding can be necessary and contingent under different specifications. |
| B16 | [Naomi Thompson](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/thompson-metaphysical-interdependence.json) <!-- attribution:fl-author-4a175f9b394e28f1572f816583ca111606778d5bb34ec5d4ddcadb3df168dc95 --> | 1 | 0 | Metaphysical interdependence; acquire the registered chapter first. |
| B17 | [Einar Duenger Bohn](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/bohn-infinite-descent.json) <!-- attribution:fl-author-b59b2e5b6e6f731428d3dea121ce0f4e9f689e0b6cfb9783e60b54b1f295b56d --> | 1 | 0 | Indefinitely descending ground; recover the registered chapter and its assumptions. |
| B18 | [Evan Welchance](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/welchance-infinitism-theoretical-virtue.json) <!-- attribution:fl-author-4a28fefd4c3918ff057474d4c185eca7caf1b87a7bc2aa28e59b20148195dc62 --> | 1 | 1 | Metaphysical infinitism and the stated theoretical virtues. |
| B19 | [Shieva Kleinschmidt](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/kleinschmidt-reasoning-without-psr.json) <!-- attribution:fl-author-355978437589a1a09c15e31049c94a8dbc406d5c3352518d1b0b25d5ec92027e --> | 1 | 1 | Which reasoning survives without a general sufficient-reason principle? |
| B20 | [Alexander Bird](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/bird-antidotes.json) <!-- attribution:fl-author-797ce2064068b78aa42fad43cae6e251527ed209a98200f2b93041064dd67f53 --> | 1 | 1 | Causal powers and the antidotes challenge. |
| B21 | [James Woodward](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/woodward-human-face.json) <!-- attribution:fl-author-4cccbc9b92b535f29364a1f37b7f9ff59b765dc83f8698f77a1e4efee267747c --> | 1 | 1 | Causal explanation and intervention-based distinctions used in current claims. |
| B22 | [Peter van Inwagen](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/van-inwagen-essay-free-will.json) <!-- attribution:fl-author-fb66e0b7ead2c11ba0693c33f47d9f110afe28be8e96db13eb1df4c5a1ffe3ee --> | 2 | 1 | Mereology, personal existence and free will across the two registered works. |
| B23 | [Andrew Brenner](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/brenner-personal-ontology.json) <!-- attribution:fl-author-6a0292a54d335bb2d03381fe62a3d44f27f097716b23a97dda039434971b7cf7 --> | 1 | 1 | Mereological nihilism and personal ontology. |
| B24 | [Achille C. Varzi](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/varzi-mereology-sep.json) <!-- attribution:fl-author-e9a65935fc0925346670c6d4ff8df1b9accb107630e188b1ca1fd082b2e00fac --> | 1 | 1 | Mereological definitions and alternatives in the archived reference entry. |
| B25 | [Trenton Merricks](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/merricks-truth-ontology-precis.json) <!-- attribution:fl-author-292ff8aa2777e2f98fd8827aa4d52e3ca05d1769ffc37a70590b15d4c807dfc5 --> | 1 | 1 | Truth, ontology and the stated truthmaking commitments. |
| B26 | [Gideon Rosen](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-abstract-objects-2020.json) <!-- attribution:fl-author-f68dc5a15842eb628fff4059e9f94bf3cc60f69c572d6691ad84723ec44bf584 --> | 1 | 1 | Abstract-object alternatives and their exact commitments. |
| B27 | [Benjamin Edmund Murphy](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/murphy-2021-empty-world.json) <!-- attribution:fl-author-5b4f6fce25488d08cb51c0b8787528d7dd7da7e2492eb035e25027fb665fc45a --> | 1 | 1 | Truth when nothing exists; connect the registered argument to actualist alternatives. |
| B28 | [Chad McIntosh](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/capturing-christianity-100-slides.json) <!-- attribution:fl-author-2e5dd3929cda816f8764effe2794744523858237753b4607417788955798f7fc --> | 1 | 5 | Turn the registered argument survey into distinct, source-linked leads. |
| B29 | [Mark van Roojen](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-moral-cognitivism.json) <!-- attribution:fl-author-6fd231ee3a00408ad41e459caf61e85a14adbe0e64c586e88036a7cb9a44b33f --> | 1 | 14 | Cognitivism/non-cognitivism distinctions and the existing moral-language claims. |
| B30 | [Richard Joyce](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-moral-anti-realism.json) <!-- attribution:fl-author-de6631b5b83c11385e7804589798d80ff09b2359b89f68e5090d210cc7c479b4 --> | 1 | 2 | Moral anti-realism variants; preserve differences between error theory and other positions. |
| B31 | [Sharon Street](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/street-moral-access.json) <!-- attribution:fl-author-2bde69a6334c5a949b49bd0528a20f1932662ee5e7a9b4bc54e1181b7a5acf4b --> | 1 | 1 | The Darwinian challenge to realist value beliefs. |
| B32 | [David Enoch](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/enoch-moral-access.json) <!-- attribution:fl-author-e93280d59a237c2f0135e176a57ad4f73daec57fdf83502c04666bfdfd2b607c --> | 1 | 1 | The epistemological challenge to metanormative realism and the proposed response. |
| B33 | [Peter Railton](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/railton-moral-realism.json) <!-- attribution:fl-author-1f94cc6b5503536970a37806d3eab83815609e525429eddf05b1d3fd5f4d9e61 --> | 1 | 1 | The registered naturalist moral-realism account. |
| B34 | [Allan Gibbard](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/gibbard-reply-2022.json) <!-- attribution:fl-author-5c31e66803936b1efa40241bc8d9cd162b5b611e59a30cdb17b2892692293b7f --> | 1 | 1 | Normative language and commitments in the reply to commentators. |
| B35 | [Stephen Darwall](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/darwall-authority-second-personal.json) <!-- attribution:fl-author-b740b225d8d18e08b1f02c600c52e2fcccbf85256297a4655c5a8271ba15f264 --> | 1 | 1 | Authority and second-personal reasons. |
| B36 | [Richmond Campbell](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-moral-epistemology.json) <!-- attribution:fl-author-3dfeef4920665a7df69d6b9e629cff7f7824796965107cbab257e7b53971d0fc --> | 1 | 1 | Moral epistemology and the premise distinctions used in current arguments. |
| B37 | [Geoff Sayre-McCord](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-metaethics.json) <!-- attribution:fl-author-c419d9420d0b245485cd83b5baa6ce3da6a9dd291e4a988214a829caac757774 --> | 1 | 1 | Metaethical alternatives and their scope in the reference entry. |
| B38 | [Alvin Plantinga](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/plantinga-wcb-eaan-excerpt.json) <!-- attribution:fl-author-e283a528a7d6379d1d3615a9b82fa67df45c3d2e005270de83b02a273167b867 --> | 1 | 1 | Warrant and religious belief in the registered excerpt. |
| B39 | [Michael C. Rea](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/rea-divine-hiddenness-silence.json) <!-- attribution:fl-author-2eb237a3c38d142d717d9f937937fe328445a2fbcd899618e861ea7adb00bbba --> | 1 | 1 | Divine hiddenness and silence; retain the excerpt’s limited scope. |
| B40 | [J. L. Schellenberg](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/schellenberg-precis-2025.json) <!-- attribution:fl-author-c581be9cd8a661789cb07fb4a9f3a16e1765e93eda5bdbf433f5c4e2a4166568 --> | 1 | 1 | The current registered précis and its hiddenness-related claims. |
| B41 | [Alex O'Connor](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/oconnor-christianitys-biggest-problem.json) <!-- attribution:fl-author-65201e8c50aeed5b09821fac4939e86fc008df972eb931e9f79d7a4a826e3534 --> | 2 | 2 | The two registered video arguments; distinguish his claims from quoted speakers. |
| B42 | [David Papineau](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/papineau-rise-physicalism.json) <!-- attribution:fl-author-76af2da0592c42a2f5737bbd6901403e257bcc9102e687567a14f66393d3d6f1 --> | 1 | 2 | The rise of physicalism and its explanatory premises. |
| B43 | [Michael Scott](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-religious-language.json) <!-- attribution:fl-author-0705aa4a6851cad804f0cc1de8c1dbd3ae20aa3e58d07ed32283a1c6d52fc43c --> | 1 | 3 | Religious-language distinctions in the registered encyclopedia entry. |
| B44 | [Jennifer Hart Weed](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/iep-religious-language.json) <!-- attribution:fl-author-0cb8c965ffcfe4e28bb8ddb8e5d7a85cd96d61bfdeed5f0543830ec96b28ae02 --> | 1 | 1 | Religious-language distinctions; compare the independently authored reference entry. |
| B45 | [Richard Amesbury](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-fideism.json) <!-- attribution:fl-author-93de45b8d6b3e4bbc636bd745d023a833911f40a7fb2b53c35df5d5e41983141 --> | 1 | 3 | Fideism variants and their actual commitments. |
| B46 | [Christian Tapp](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/tapp-uniqueness-monologion-2014.json) <!-- attribution:fl-author-312b3034d1bbb9df5c85496dfcb8165595a0f863b385e7d2db3e2c832b0c585b --> | 1 | 1 | Divine uniqueness in Anselm’s Monologion; link commentary to the primary text. |
| B47 | [Robert Pasnau](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/pasnau-mind-human-nature.json) <!-- attribution:fl-author-33096cda67c841f65444f47985a9ee97d60c8c2e75f925bb9756209f26bdf5d9 --> | 1 | 1 | Aquinas on mind and human nature; preserve commentary versus primary-text attribution. |
| B48 | [Rosalie Iemhoff](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-intuitionism.json) <!-- attribution:fl-author-bb622946be88ef7d4cabc44fbe097d3ac78eb2e564f61a21e3ed3ed78d15d876 --> | 1 | 3 | Intuitionism and the relevant mathematical/logical commitments. |
| B49 | [Thomas Uebel](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-vienna-circle.json) <!-- attribution:fl-author-ee77de0da48cc2b49db3afaabe7dd8e3ee0e2b7ddc22b6f52f8e6e8a26d0946d --> | 1 | 1 | Vienna Circle distinctions; use the overview to locate primary sources. |
| B50 | [James H. Fetzer](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-hempel.json) <!-- attribution:fl-author-8ca58ac04fc0de0e9ffdbd84548428bccc07ea5a57c00de2a94d967919ff90c7 --> | 1 | 1 | The Hempel reference entry and its explicit scientific-explanation claims. |


### Joint authors and research teams

These 15 records remain joint. Process each work once, resolve its full byline or speaker roles, then link the resulting evidence to individual packets.

| Queue | Attribution | Works | Passages | Initial question or packet |
| --- | --- | ---: | ---: | --- |
| J01 | [Chad McIntosh; Cameron Bertuzzi](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/capturing-christianity-100.json) <!-- attribution:fl-author-21e0c7168ed125f5d49d4e8e84e74fcbf2bdc5694ac7ca900eb167e8bef54952 --> | 1 | 4 | Argument-survey presentation: separate presenter, contributor and cited-originator roles. |
| J02 | [Yunfeng Li et al.](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/4fb153f8-fccc-4b86-a83f-8e5e78602834.json) <!-- attribution:fl-author-6d82cd61011e3e7d9442f42c4a9df148f15246df05f5ff42e8e5a8283bf6c551 --> | 1 | 3 | Spore-germination study: recover the byline and map the precise empirical premise. |
| J03 | [Christoph Benzmüller and Dana Scott](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/afp-notes-goedel-ontological.json) <!-- attribution:fl-author-966ae68f648a62112d7ff56d61d9077a3be22da8cd90d824fa04ccfae63421fe --> | 1 | 2 | Gödel/Scott variants; keep manuscript attribution and formalization roles separate. |
| J04 | [Gottfried Wilhelm Leibniz and Samuel Clarke](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/leibniz-clarke-correspondence.json) <!-- attribution:fl-author-490ffd77c11b350e38b7ece932b23407cbcc7d08d71899c215550eb45590c88a --> | 1 | 2 | Read the correspondence once; assign each paper and reply to its speaker. |
| J05 | [Stewart Candlish and George Wrisley](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-private-language.json) <!-- attribution:fl-author-dcb12652758171a31290edf4d6632966d1636dfa5c97afd75ec25e7872fb37ef --> | 1 | 2 | Private-language arguments; distinguish commentators’ reconstructions from primary claims. |
| J06 | [Vandi Verma and colleagues](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/8a87e7f3-1a57-44a2-a2a5-8c784c55cdb0.json) <!-- attribution:fl-author-1d9fc2b9de78d36ef9549bc0808758780a9f99362478f5c6d4efb10e59b470de --> | 1 | 2 | Perseverance operations: recover the full byline and the exact empirical example used. |
| J07 | [Anat Biletzki and Anat Matar](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-wittgenstein.json) <!-- attribution:fl-author-886c724efde93cfe728bd1fb987624df9d3e345842f7afd5a20debe2f24ec7a6 --> | 1 | 1 | Wittgenstein reference entry; distinguish commentary from his own texts. |
| J08 | [Christoph Benzmüller and Bruno Woltzenlogel Paleo](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/afp-goedelgod.json) <!-- attribution:fl-author-7af9a59eba3ff855f8086fc9440bfe581ae9012e5df0b4c5955afedcdf948b3e --> | 1 | 1 | Scott’s ontological variant in Isabelle; preserve axioms, embedding and theorem scope. |
| J09 | [Hannes Leitgeb and André Carus](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/sep-carnap.json) <!-- attribution:fl-author-d1c2109a0499b9e0610e88b86abeb972e10ce9f01960d9680d0bb8a05a1c555c --> | 1 | 1 | Carnap reference entry; trace substantive interpretations to primary writings. |
| J10 | [J. B. Hartle and S. W. Hawking](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/hartle-hawking-wavefunction.json) <!-- attribution:fl-author-9556c5194e3e11e70a5c52c4ee4add3f2d802d08e5f376e94a46f9e9e5eab5fb --> | 1 | 1 | Wave function of the Universe; distinguish physical model assumptions from metaphysical conclusions. |
| J11 | [Natalie A. Dye and colleagues](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/dye-mechanical-feedback.json) <!-- attribution:fl-author-6e0cca0f53dd34fb4342a291696e6eef4aee1a7b288685b3847fdcff6e4b58c4 --> | 1 | 1 | Mechanosensitive pattern formation; preserve the paper’s actual experimental scope. |
| J12 | [Nathan Tompkins and colleagues](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/tompkins-chemical-morphogenesis.json) <!-- attribution:fl-author-377a6fa9e867af5e15f240ec4b0e982dd776a3dce05bf83e84e81e95ac8a751e --> | 1 | 1 | Chemical-cell morphogenesis: identify the empirical result and its limited philosophical use. |
| J13 | [Peter D. Klein and John Turri](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/klein-turri-infinitism.json) <!-- attribution:fl-author-5a9d8e51ef1d9ef924f514b764b4d5957f56bfd44b8ac632463b80f2b1e6717b --> | 1 | 1 | Epistemological infinitism; retain the joint reference entry and follow its primary leads. |
| J14 | [Zachary D. Blount and colleagues](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/blount-lenski-innovation.json) <!-- attribution:fl-author-d52eda6a9bfd8792f9419371fee50e824739024ca7f1a10f8cd35895a2874af6 --> | 1 | 1 | Citrate-environment evolution study; map only the empirical claim actually cited. |
| J15 | [Kai Nielsen and D. Z. Phillips](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/nielsen-phillips-wittgensteinian-fideism-2005.json) <!-- attribution:fl-author-adcdd7473a978e0466e075b49d5a79d8d0691949575bc16351db13c433bd6460 --> | 1 | 0 | Process the exchange once, then link opposing positions to separate person packets. |


### Institutions and councils

These eight records need document/version passes rather than individual biographies. Preserve promulgation, institutional authorship and revision information.

| Queue | Attribution | Works | Passages | Initial question or packet |
| --- | --- | ---: | ---: | --- |
| I01 | [First Vatican Council (1870)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/vatican-i-dei-filius.json) <!-- attribution:fl-author-efd1c8478572d15485d222e7c12d7888b51090af11e0ae668e63d2264e4f212f --> | 1 | 7 | Dei Filius: reason, faith and natural knowledge claims. |
| I02 | [Catholic Church (promulgated by John Paul II, 1992; Latin editio typica 1997)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/catechism-catholic-church-en.json) <!-- attribution:fl-author-9d0b290e556c978349498358e723df6f3e6ae7e4f260c8480b11632748878713 --> | 1 | 4 | Catechism teaching and edition/translation alignment. |
| I03 | [Second Vatican Council (1964)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/vatican-ii-lumen-gentium.json) <!-- attribution:fl-author-141f822b345a240e7d2540ad84e9e2b709ca391639ecb5a961806ba5cc177d6a --> | 1 | 3 | Lumen gentium: identify the exact institutional teaching and document section. |
| I04 | [Catholic Church (promulgated by John Paul II, 1983; Book VI revised 2021)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/codex-iuris-canonici-1983.json) <!-- attribution:fl-author-11c0a5a91e6ac13a2e80d53b8211b1fad85796f69bcc2ad8eb60aadc7546dd34 --> | 1 | 2 | Canon-law propositions; retain promulgation and Book VI revision boundaries. |
| I05 | [Congregation for the Doctrine of the Faith; John Paul II (Ad tuendam fidem)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/cdf-professio-fidei-1998.json) <!-- attribution:fl-author-387e81ac43498d02086c1bc8709c7fa655d496b6daccefd7f2c8a6ea943ba844 --> | 1 | 1 | Separate profession, promulgation and commentary documents within the official collection. |
| I06 | [Dicastery for Promoting Christian Unity and Commission on Faith and Order of the World Council of Churches](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/dpcu-wcc-wpcu-2025.json) <!-- attribution:fl-author-61c1bfae1e53a112dc6ef503d0953416e1cb906f06d8ec795a353e9c732ce7f2 --> | 1 | 1 | The 2025 prayer resources; preserve the joint institutional source and stated purpose. |
| I07 | [First Council of Constantinople (381)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/creed-constantinople-381.json) <!-- attribution:fl-author-fef5f3b7c00792ecf7f60a0993747c9e42e40edf0b5977fb991c84d1cb43b337 --> | 1 | 1 | Creed wording, version and the specific doctrinal proposition cited. |
| I08 | [Second Vatican Council (1965)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/vatican-ii-dei-verbum.json) <!-- attribution:fl-author-ed030f6eed01514e68d8ec67367d4d2d2a1fe896712093b099cdf2e3e6a76eee --> | 1 | 1 | Dei Verbum: revelation and interpretation claims in their document context. |


### Context-qualified names and text collections

These five records need attribution resolution. A collection, translation label, interview role or named opponent does not automatically define a new author.

| Queue | Attribution | Works | Passages | Initial question or packet |
| --- | --- | ---: | ---: | --- |
| C01 | [New Testament](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/nt-acts.json) <!-- attribution:fl-author-437120bba4e773ce50dcdd21e117ca0213831a83d033958aedfa5c8ba3b8ffc2 --> | 4 | 17 | Keep Acts, John, Mark and Matthew as separate text/attribution packets. |
| C02 | [Paul](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/nt-1cor.json) <!-- attribution:fl-author-818b5cc5f21d3e6e4e6071c06294528d44595022218446d8b79304d2b766327a --> | 3 | 6 | Process the three registered letters separately, preserving traditional attribution and edition evidence. |
| C03 | [Alex O'Connor (responding to Frank Turek)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/oconnor-turek-evil.json) <!-- attribution:fl-author-a9d0a9f13684dc43d61254ca5e5ece22cbc3754c94755215399902c457acb663 --> | 1 | 3 | Map presenter, quoted opponent and response spans; do not infer coauthorship. |
| C04 | [Paul (Nova Vulgata, editio typica altera 1986)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/nova-vulgata-romans.json) <!-- attribution:fl-author-75d3e2b90f35100213323ffb2478db489f9898840e4377918fae0ee29454ecd8 --> | 1 | 2 | Romans: distinguish traditional author attribution from the 1986 edition identifier. |
| C05 | [Alex O'Connor (interviewing Destiny, Steven Bonnell II)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/oconnor-destiny-ethics.json) <!-- attribution:fl-author-e72c499868f67c346abfb56645fed78fa2656f9450125e6b83fd4983045c1a8c --> | 1 | 1 | Map interviewer and interviewee statements with timestamps and explicit speaker evidence. |


### Anonymous or unresolved provider identities

These two records need identity/provenance work. Retain an unresolved identity explicitly when the evidence does not establish a person.

| Queue | Attribution | Works | Passages | Initial question or packet |
| --- | --- | ---: | ---: | --- |
| U01 | [Unattributed website author (identity not verified)](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/necessarybeing-survey.json) <!-- attribution:fl-author-33b9bcc737d03b80b7dccbbfd88317aef1fb5caf2c4cba578eecd68184263b5c --> | 1 | 67 | Resolve the Necessary Being survey’s identity/version; retain anonymity if unresolved. |
| U02 | [Carneades.org](https://github.com/moritzbrantner/formal-logos/blob/597e4b2eb02e092f1769a2e987437bc78e14893a/corpus/sources/carneades-free-will-2026.json) <!-- attribution:fl-author-69d1279591bb24725dca7f20e0cfb536bf74c8ffab828e936fd83d4cb5e7ea58 --> | 1 | 0 | Resolve the channel/provider and free-will survey provenance; do not equate it with ancient Carneades. |


### People appearing only through joint records

- **Samuel Clarke** gets his own person packet alongside Leibniz, while the correspondence remains one joint source record.
- **J. B. Hartle and S. W. Hawking** can each receive a historical-person packet. Their [UCSB](https://chancellor.ucsb.edu/memos/2023-06-21-sad-news-professor-emeritus-james-hartle) and [Cambridge](https://www.maths.cam.ac.uk/internal/faculty/stephen-hawking-1942-2018) institutional obituaries provide life-status evidence; their shared paper stays joint in this roster.
- Resolve and link **Christoph Benzmüller, Dana Scott, Bruno Woltzenlogel Paleo, Cameron Bertuzzi, Peter D. Klein, John Turri, Anat Biletzki, Anat Matar, Hannes Leitgeb, André Carus, Stewart Candlish and George Wrisley** through their actual contributions. Preserve the full bylines of the scientific teams as well. No life status is inferred from these joint strings.
- Nielsen/Phillips, McIntosh’s joint presentation and the context-qualified O’Connor records must link to existing person packets where identity is established, rather than generating duplicate people.

### Additional life-status observations for this plan

The repository already contains five curated deceased-author profiles. This planning pass additionally located evidence for [Anselm](https://www.anselm.edu/about/history-college/our-patron-saint), [Hume](https://humesociety.org/about/hume/), [Wittgenstein](https://archives.trin.cam.ac.uk/papers-of-ludwig-wittgenstein), [Carnap](https://www.phil.cmu.edu/projects/carnap/about.htm) and [Alvin I. Goldman](https://philosophy.rutgers.edu/news/news/department-news/1417-we-regret-to-announce-the-passing-of-emeritus-professor-alvin-goldman). These links are planning evidence; they do not modify the catalogue or curated person profiles. Add the relevant supported observation when activating each profile.
