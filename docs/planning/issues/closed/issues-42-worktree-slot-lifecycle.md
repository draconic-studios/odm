---
id: issues-42
title: "Worktree slot lifecycle core + CLI"
description: "Implement odm project worktree list/add/rm with validation and JSON."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ready-for-agent
---

# Worktree slot lifecycle core + CLI

## Description

CLI stubs `odm project worktree …` exit not-implemented. Need full list/add/rm for slots at `worktrees/<project>/<slot>/` using git worktree ops and core path policy.

## Affected

- `odm-core` (lifecycle helpers)
- `odm` CLI (`ProjectCmd::Worktree`, main dispatch, optional DTOs)
- Tests under core and/or `crates/odm/tests`

## Impact

Users/agents cannot create parallel checkouts through ODM; only ad-hoc dirs work for actions `--wt`.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-41-odm-git-worktree-ops]]

## Agent Brief

**Category:** feature  
**Summary:** Implement Project worktree slot list/add/rm in core and wire `odm project worktree` CLI per map standing prefs.

**Bindings:**
- `docs/reference/worktrees.md`, `cli.md` (project worktree sketch)
- `odm-core::paths::worktree_slot_path`
- Parent map [[issues-40-worktree-slots-map]]
- Depends on [[issues-41-odm-git-worktree-ops]]

**CLI (lock):**
```text
odm project worktree list <project>
odm project worktree add <project> <slot> [--branch <b>]
odm project worktree rm <project> <slot> [--force]
```
- Replace trailing `rest: Vec<String>` stub with real subcommands (clap).
- Globals: `--root`, `--json` apply; `--project` is **not** required (project is positional).
- Unknown project name → usage exit `1`.
- Primary path missing or not a git repo → operation/usage error (exit `3` or `1` consistent with `project git`); never create a slot on non-git Project.

**Slot name validation:**
- Non-empty after trim
- Reject if contains `/`, `\`, NUL, or is `.` / `..`
- Reject if looks absolute (e.g. starts with `/` or Windows drive) — belt and suspenders even if separators already banned
- Invalid → usage error

**add:**
1. Resolve primary via config + `abs_checkout`
2. Ensure primary is git repo
3. `slot_path = worktree_slot_path(root, project, slot)`
4. If `slot_path` exists → error (do not clobber)
5. `create_dir_all` on `worktrees/<project>/` (parent only)
6. Call `git.worktree_add(primary, slot_path, branch.as_deref())`
7. Success human: e.g. `added worktree slot <slot> -> <rel-or-abs path>`; JSON: `{ "project", "slot", "path" }` (path relative to workspace root preferred, POSIX-ish string)

**list:**
1. Primary must be git repo
2. `git.worktree_list(primary)` filtered to paths under `worktree_slot_path` parent `worktrees/<project>/` **or** directory entries under that folder that appear in git’s list — slots are only those git reports inside the project’s worktrees prefix
3. Slot **name** = final path component
4. Human: one slot name per line (stable sort); JSON: `{ "project", "slots": [ { "name", "path" } ] }` sorted by name
5. Empty list is success exit `0`

**rm:**
1. Resolve slot_path; must exist and be in `git worktree list` for that primary (or exist as registered worktree) — if path missing → not found / usage
2. `git.worktree_remove(primary, slot_path, force)`
3. Do not delete unrelated files; git removes the worktree
4. Best-effort: ignore failure to remove empty `worktrees/<project>/` directory
5. Human success one-liner; JSON `{ "project", "slot", "path" }`

**Rules from sketch:**
- Primary checkout is never a slot
- No auto-create on other commands (this ticket only list/add/rm)
- `project rm` behavior unchanged (does not delete worktrees tree)

**Code placement:**
- Prefer `odm-core` module e.g. `worktree.rs` or functions beside membership — keep file sizes ≤1000 LOC
- CLI DTOs in existing commands pattern (`crates/odm/src/commands/`)

**Tests:**
- Unit: name validation; path shape; add refuses existing path; list filter (fake git runner)
- At least one integration-style test may wait for [[issues-44-worktree-integration-and-docs]] but core unit tests required here
- Update stub tests in `progen_vault.rs` / wherever `generate_and_agent_stubs` expects exit 1 for `project worktree` — **list/add with missing args** may become clap usage; bare `project worktree` should show help or usage, not necessarily “not implemented”

**Acceptance criteria:**
- [ ] `odm project worktree add|list|rm` implemented (no `not_implemented("project worktree")`)
- [ ] Slot name validation rejects path separators and `..`
- [ ] add creates a real git worktree under `worktrees/<project>/<slot>/` when run against a temp git primary (unit with fake runner **and/or** integration in this ticket if easy)
- [ ] list JSON shape stable as specified
- [ ] rm removes the worktree; `--force` passed through
- [ ] Non-git project hard-errors on add/list/rm
- [ ] `cargo test` green

**Out of scope:**
- `project git --wt` (next ticket)
- status/doctor orphan reporting
- Config-declared slots
- Branch naming templates
- Changing pin logic

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Landed worktree slot lifecycle in core + CLI:

- **Core** (`odm-core::worktree`): `validate_slot_name`, `worktree_list` / `worktree_add` / `worktree_rm` — primary must be git; slots only under `worktrees/<project>/`; fake-runner unit tests for validation, argv, list filter/sort, refuse existing path, force rm
- **CLI**: `odm project worktree list|add|rm` with clap subcommands; JSON DTOs `{ project, slots:[{name,path}] }` and `{ project, slot, path }`; human one-liners / one name per line
- Stub tests no longer expect not-implemented for bare `project worktree`
- Unblocks [[issues-43-project-git-wt-resolve]]

## Comments

Parent map: [[issues-40-worktree-slots-map]]
