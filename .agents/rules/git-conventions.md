## Git Conventions

- Branch naming: `feat/<short-description>`, `fix/<short-description>`,
  `chore/<short-description>`. When working a tracker ticket, prefix the
  ticket number: `feat/06-inventory-cli-skeleton`.
- Work on short-lived branches, one ticket per branch. Merge to `main` when
  the gates pass. Tiny doc or chore commits may go direct to `main`.
- Local-only files (`plan.md`, `CONTEXT.md`, `docs/adr/`, `docs/agents/`,
  `.scratch/`) are gitignored — never force-add them.
- Never commit secrets, API keys, `.env` files, or `target/` output.

### Commit Messages

Commits use the Conventional Commits format unless the user asks otherwise:

```text
type: subject
type(scope): subject
```

Rules:

- Keep the commit type lowercase: `feat`, `fix`, `chore`, `refactor`,
  `docs`, `style`, `test`, `perf`, `ci`, `build`, `revert`
- Write a short subject after the colon, imperative mood, under 100 characters
- Use the body to explain what changed and why when helpful
- Reference the local tracker ticket in the body, e.g.
  `.scratch/v0-0-1-minimal-release/issues/06-inventory-cli-skeleton.md`

Type guide:

- `feat` - a new feature is introduced with the changes
- `fix` - a bug fix has occurred
- `chore` - changes that do not relate to a fix or feature and do not modify
  `src` or test files
- `refactor` - refactored code that neither fixes a bug nor adds a feature
- `docs` - updates to documentation such as the `README` or other markdown
  files
- `style` - changes that do not affect the meaning of the code, usually
  formatting-related
- `test` - including new tests or correcting previous tests
- `perf` - performance improvements
- `ci` - continuous integration related changes
- `build` - changes that affect the build system or external dependencies
- `revert` - reverts a previous commit
