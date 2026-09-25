## Code Style Rules

These are enforced by `rustfmt` and Clippy — do not fight them:

- **Formatting**: `cargo fmt` defaults (4 spaces, 100-column soft width).
  Run `cargo fmt --all --check` before committing.
- **Lints**: `cargo clippy --workspace --all-targets -- -D warnings` must be clean. Fix
  lints — do not suppress them without a strong, documented reason.
- **File length**: keep source files cohesive and modular (target `<= 500
  LOC` per file); extract helpers or modules when approaching the limit.
- **Crate boundaries**: keep them aligned with `plan.md` — CLI output in
  `buildtrace-cli`, report model in `buildtrace-core`, manifest reading in
  `buildtrace-cargo`. Code that will move together should already live together.
- **No dead code**: remove it or gate it behind the ticket that needs it.
  No commented-out code blocks and no leftover debug output.

Forbidden patterns:

- No redundant comments that restate the code
- No `todo!`/`unimplemented!` in merged code unless the tracking ticket is
  named next to it
- No `panic!`/`unwrap!`/`expect!` on production paths that handle external
  input — return a diagnostic and exit non-zero instead (`expect` is fine
  in tests and for truly invariant serialization)
