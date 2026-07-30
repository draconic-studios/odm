

- Commits as work packages: `<type>(<scope>): <description>` — `feat` | `fix` | `test` | `refactor` | `chore`
- No `Co-Authored-By` lines
- TDD, DRY, YAGNI; prefer one-liner solutions when clear
- No CI/CD or GitHub Actions

## Agent skills



### Issue tracker

Issues live as markdown under `docs/planning/issues/`. See `docs/agents/issue-tracker.md`.

### Triage labels

Roles are frontmatter tags: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`; reject via `status: wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context layout (`CONTEXT.md` + `docs/adr/` at repo root). See `docs/agents/domain.md`.
