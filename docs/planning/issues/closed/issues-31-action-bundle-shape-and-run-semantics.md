---
id: issues-31
title: "Action bundle shape and run semantics"
description: "Decision: tasks array, shell model, cwd, extra args, JSON, exit codes."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Action bundle shape and run semantics

## Question

Exact Action YAML shape and runtime semantics for `odm run`?

## Answer

### Bundle file (per `config.md`)

```yaml
# actions/core.yaml
hello:
  tasks:
    - run: echo hello-desk
bootstrap:
  tasks:
    - run: echo step1
    - run: echo step2
      dir: projects/alpha
```

- Action → **required non-empty** `tasks` array.
- Task → required non-empty `run` (shell command string) + optional `dir` (path relative to Workspace root).
- Merge bundles by Action name; duplicates across bundles → config error (exit `2`).
- Declared bundle path missing → config error (exit `2`).
- Empty `actions:` map in Workspace config → no Actions (list empty).

### Shell-out

- Each task: `sh -c <run>` with `cwd` resolved below; stdout/stderr **inherit**.
- Extra args (`odm run name -- a b`): applied only to the **last** task as  
  `sh -c '<run> "$@"' _ a b` so the script may use `"$@"`.
- Tasks run **in order**; first non-zero exit **stops** the pipeline; process exit = that code.
- All tasks success → exit `0` (or last task’s code if non-zero — same).
- Pre-exec failures (unknown action, bad cwd, unknown project): exit `1` / `2` per `cli.md`, never pretend to be the action.

### Cwd resolution (per task)

1. If `--wt` set: require `--project`; cwd base = `<workspace>/worktrees/<project>/<slot>/` (must exist) — then if task `dir` set, still use task `dir` under workspace (task dir wins when present); if no task `dir`, use wt path.
2. Else if `--project` set and no task `dir`: cwd = Project primary absolute path.
3. Else if task `dir` set: cwd = Workspace root + task `dir` (must exist).
4. Else: Workspace root.

Clarified rule (implement):

- **Task `dir` always wins** when set (relative to Workspace root) — matches config.md “dir relative to workspace”.
- When task `dir` absent: `--wt` (needs `--project`) > `--project` > Workspace root.

### CLI / JSON

- `odm run` → list names (human one-per-line; JSON `{ "actions": [ { "name", "tasks": [ { "run", "dir" } ] } ] }` sorted by name).
- `odm run <name> …` → `{ "action": "<name>", "exitCode": N }` when `--json`; still inherit action stdio.
- Actions only via `run` — never top-level commands.

## Comments

Locks implement detail without reopening design package.
