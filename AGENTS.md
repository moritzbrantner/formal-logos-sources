# Repository instructions

Maintain the source preparation boundary described in `README.md`. `truth-core` owns its authoritative source and passage records and formal research semantics; do not introduce a dependency from it to this project. Mirror its PDF extraction semantics exactly when exporting hashes. Source bytes and full copyrighted extraction remain local and untracked. Test using synthetic local fixtures and never fetch historical witnesses in routine CI.

Use Rust for implementation. Reuse existing repositories only where their crates provide a task-relevant primitive; `rust-kernels` currently has search/collection algorithms but no PDF extraction or cryptographic hashing adapter. Avoid adding cross-repository dependencies solely for nominal reuse.

Before changing the handoff contract, inspect `truth-core/docs/source-evidence.md` and `truth-core/scripts/source_evidence.py` at a pinned revision. Prefer one narrow source adapter and focused regression tests over a new ingestion framework.
