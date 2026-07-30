# Triage Labels

Triage roles are **tags** on issue frontmatter under `docs/planning/issues/`.
There is no GitHub label store.

| Role in skills | Tag on the issue | Meaning |
| -------------- | ---------------- | ------- |
| `needs-triage` | `needs-triage` | Maintainer needs to evaluate |
| `needs-info` | `needs-info` | Waiting on reporter for more info |
| `ready-for-agent` | `ready-for-agent` | Fully specified; AFK agent may take it (`status: open` + `## Agent Brief`) |
| `ready-for-human` | `ready-for-human` | Needs a human |
| `wontfix` | *(use `status: wontfix`)* | Will not be actioned; move to `closed/` |

Also useful (not role labels):

- `issue-type`: `bug` | `feature-request` | `observation`
- `severity`: `critical` | `high` | `medium` | `low`

When a skill says "apply the AFK-ready triage label", add tag `ready-for-agent`,
keep `status: open`, and ensure an `## Agent Brief` is present.
