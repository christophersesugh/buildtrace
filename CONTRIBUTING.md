# Contributing to BuildTrace

BuildTrace is pre-alpha and not ready for external contributions yet. Please
open an issue before proposing a substantial change, so its scope can be
agreed against the product plan.

## Ground rules

- All contributions are under the repository license (MIT OR Apache-2.0).
- Keep raw traces and other sensitive local data out of issues and pull requests.
- One focused change per pull request, with tests.

## Development

Requires a current Rust toolchain. From the repository root:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo run
```

Repository conventions for people and agents are in `AGENTS.md`.
