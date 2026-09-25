# BuildTrace

BuildTrace is a Linux-first Cargo build capability recorder and policy-diff tool. It helps Rust teams review what dependency build scripts and procedural macros do during a build: files they access, programs they start, and network connections they make.

## Status

Pre-alpha. The repository currently contains the initial Rust CLI scaffold and the product plan; the commands below describe the intended interface and are not implemented yet.

## Intended workflow

```bash
# Inventory build scripts and procedural macros without executing them.
cargo buildtrace inventory

# Record build-time behavior while Cargo runs as a child process.
cargo buildtrace record -- cargo check --workspace

# Propose a reviewable, source-pinned policy.
cargo buildtrace propose > buildtrace.toml

# Fail CI when a dependency expands approved capabilities.
cargo buildtrace diff --policy buildtrace.toml
```

BuildTrace records evidence; it does not grant permissions, sandbox a build, or claim that a clean report proves a dependency is safe.

## Scope

The first release targets Linux and observes selected file, process, and network operations through `ptrace`. Reports are designed to be deterministic and reviewable in CI. Proc-macro attribution and trace completeness are reported with explicit confidence and limitation information.

Version 1 does not enforce permissions, trace application runtime behavior, support every operating system, send telemetry, or upload traces.

## Development

Requires a current Rust toolchain. From the repository root:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo run
```

Repository conventions for people and agents are in [AGENTS.md](AGENTS.md).

## Security and privacy

Raw traces may contain sensitive local paths and command data. BuildTrace is designed to keep traces local, avoid recording environment-variable values by default, redact credentials from arguments and URLs, and label uncertain or incomplete attribution rather than overstate it.

## Contributing

The project is not ready for external contributions yet. Please open an issue before proposing a substantial change, so its scope can be agreed against the product plan. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
