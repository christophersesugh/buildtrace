# Issue tracker: Local Markdown

Issues and specs for this repository live as Markdown files in `.scratch/`.

## Conventions

- One feature per directory: `.scratch/<feature-slug>/`.
- The feature spec is `.scratch/<feature-slug>/spec.md`.
- Implementation issues are individual files at `.scratch/<feature-slug>/issues/<NN>-<slug>.md`, numbered from `01`; do not combine tickets into one file.
- Record state near the top of an issue as `Status: <value>`.
- Append comments and conversation under a `## Comments` heading.

## Publishing and reading issues

When a skill says to publish to the issue tracker, create the appropriate file under `.scratch/<feature-slug>/`. When it says to fetch a ticket, read the referenced file.

## Wayfinding operations

- The map is `.scratch/<effort>/map.md`, containing its notes, decisions-so-far, and fog of war.
- A child ticket is `.scratch/<effort>/issues/NN-<slug>.md`. Record its `Type:` as `research`, `prototype`, `grilling`, or `task`, and its `Status:` as `claimed` or `resolved`.
- Express dependencies as `Blocked by: NN, NN`. A ticket is unblocked only when every listed ticket is resolved.
- The frontier is the first numbered ticket that is open, unblocked, and unclaimed.
- Claim a ticket by setting `Status: claimed` before working on it.
- Resolve it by appending an `## Answer` section, setting `Status: resolved`, and adding a concise link and gist to the map's Decisions so far section.
