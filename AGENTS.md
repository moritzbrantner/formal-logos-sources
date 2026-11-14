# Repository instructions

Maintain the source preparation boundary described in `README.md`. `truth-core` owns its authoritative source and passage records and formal research semantics; do not introduce a dependency from it to this project. Mirror its PDF extraction semantics exactly when exporting hashes. Source bytes and full copyrighted extraction remain local and untracked. Test using synthetic local fixtures and never fetch historical witnesses in routine CI.

Before changing the handoff contract, inspect `truth-core/docs/source-evidence.md` and `truth-core/scripts/source_evidence.py` at a pinned revision. Prefer one narrow source adapter and focused regression tests over a new ingestion framework.
