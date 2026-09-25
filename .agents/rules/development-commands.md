## Development Commands

Run all commands from the **repo root** unless otherwise specified.

```bash
# Check formatting (must pass)
cargo fmt --check

# Lint with warnings denied (must pass)
cargo clippy --all-targets -- -D warnings

# Run the full test suite (must pass)
cargo test

# Run a single test by name
cargo test <name>

# Build the binary
cargo build

# Run inventory against the current directory
cargo run -- inventory

# Run inventory against another manifest
cargo run -- inventory --manifest-path <path/to/Cargo.toml>
```

The installed binary is named `cargo-buildtrace`, so `cargo buildtrace
inventory` works once it is on `PATH`.
