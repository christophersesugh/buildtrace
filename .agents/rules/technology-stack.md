## Technology Stack

| Layer | Technology |
|---|---|
| Language | Rust, edition 2024 |
| Layout | Cargo workspace (`crates/`): `buildtrace-cli` (commands, output, exit codes), `buildtrace-core` (report model, rendering), `buildtrace-cargo` (manifest reading, attribution); `buildtrace-linux` + `buildtrace-policy` arrive with `record`/`propose` |
| CLI | `clap` (derive) — binary name `cargo-buildtrace` so `cargo buildtrace` dispatch works |
| Serialization | `serde` + `serde_json` |
| Cargo metadata | `cargo_metadata` (metadata only — never execute dependency code) |
| Formatter | `rustfmt` via `cargo fmt` |
| Linter | Clippy via `cargo clippy -- -D warnings` |
| Testing | `cargo test` (in-module units, `tests/` integration, snapshots) |
| CI | GitHub Actions (`validate.yml`: fmt, clippy, build, test) |
| Platform | Linux-first; `inventory` itself stays portable (no `ptrace` in v0.0.1) |
