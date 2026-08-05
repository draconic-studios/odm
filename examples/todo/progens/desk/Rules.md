---
id: rules
title: Read-only rules
---

# Read-only rules

This Workspace clones real GitHub repos. **Do not** commit, push, reset, or rewrite history inside managed checkouts during dogfood.

Allowed:

- `odm sync` / `odm pin status` / `odm pin apply` (local detached HEAD only)
- `odm project git <name> -- status|log|rev-parse|branch` (read-only)
- worktree add/list/prune (local slots only; never push slot branches)
- progen reindex / find / context

Forbidden:

- `git commit` / `git push` / `git reset --hard` against remotes
- editing files inside `projects/*` or `progens/sheets` for “tests”
- force-push or remote branch create

Search token: TodoRulesToken
