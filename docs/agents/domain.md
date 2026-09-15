# Domain Docs

Engineering skills should use this repository's domain documentation when exploring or proposing changes.

## Before exploring

Read the root `CONTEXT.md`, if present, and any relevant ADRs in `docs/adr/`. If these files do not exist yet, proceed without flagging their absence. The domain-modeling workflow creates them when terms or decisions are actually resolved.

## Layout

This is a single-context repository:

```text
/
├── CONTEXT.md
├── docs/
│   └── adr/
└── src/
```

## Vocabulary and decisions

Use terms defined in `CONTEXT.md` consistently in issues, proposals, tests, and documentation. If a needed concept is absent, treat it as either a vocabulary gap for domain modeling or a sign that a defined term should be used instead.

When a proposed change conflicts with an ADR, state the conflict explicitly and explain why reopening that decision may be warranted rather than silently overriding it.
