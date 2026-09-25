# BuildTrace

## Agent skills

### Issue tracker

Issues are tracked locally as Markdown files under `.scratch/`. See `docs/agents/issue-tracker.md`.

### Domain docs

This is a single-context repository using `CONTEXT.md` and `docs/adr/`. See `docs/agents/domain.md`.

## Guidelines

BuildTrace is a Linux-first Cargo build capability recorder and policy-diff tool.

- **Facts over assumptions**: inspect source code and verify contracts directly. Do not rely on unverified assumptions.
- **Layout**: single `buildtrace` crate now; split into `crates/` when `record` lands. Keep module boundaries tidy so the split stays mechanical.
- **Gates**: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` must pass before a task counts as done.
- **Local-only files**: `plan.md`, `CONTEXT.md`, `docs/adr/`, `docs/agents/`, and `.scratch/` stay untracked. Never commit them.
- **Clean git hygiene**: never commit secrets, API keys, `.env` files, or gitignored build output.

## Workspace Rules

- [Development commands](./.agents/rules/development-commands.md)
- [Git conventions](./.agents/rules/git-conventions.md)
- [Testing](./.agents/rules/testing.md)
- [Code style](./.agents/rules/code-style.md)
- [Technology stack](./.agents/rules/technology-stack.md)

