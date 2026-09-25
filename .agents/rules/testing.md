## Testing

- Unit tests are **mandatory** for every feature. They live in-module under
  `#[cfg(test)]` and go through the highest seam possible (the inventory
  function over a manifest path, not internals).
- Integration tests live in `tests/` and drive the built binary against
  self-contained path-only fixture workspaces: one with a build script, one
  with a proc macro, one with neither.
- Snapshot tests pin the human table and JSON report output.
- Required coverage per feature: the no-execution guarantee (running
  inventory leaves no build artifacts from dependency code) and determinism
  (repeated runs are byte-identical).
- No real network endpoints in any test, ever.
- `cargo test` must pass with no special privileges before a task counts as
  done.
