## Development Commands

Run all commands from the **repo root** unless otherwise specified. The repo
is a Cargo workspace (`crates/`); `buildtrace-linux` and `buildtrace-policy`
arrive with `record`/`propose`.

```bash
# Check formatting (must pass)
cargo fmt --all --check

# Lint with warnings denied (must pass)
cargo clippy --workspace --all-targets -- -D warnings

# Run the full test suite (must pass)
cargo test --workspace

# Run a single test by name
cargo test --workspace <name>

# Build all crates
cargo build --workspace

# Run inventory against the current directory
cargo run -p buildtrace-cli -- inventory

# Run inventory against another manifest
cargo run -p buildtrace-cli -- inventory --manifest-path <path/to/Cargo.toml>
```

The installed binary is named `cargo-buildtrace`, so `cargo buildtrace
inventory` works once it is on `PATH`.
