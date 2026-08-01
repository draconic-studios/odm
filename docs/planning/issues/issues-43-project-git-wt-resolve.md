---
id: issues-43
title: "project git --wt resolve"
description: "Honor --wt on odm project git using worktree slot path; pin maintain stays primary-only."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ready-for-agent
---

# project git --wt resolve

## Description

`odm project git <name> --wt <slot> -- …` and global `--wt` currently return `not_implemented("project git --wt")`. Actions already resolve `--wt` via `worktree_slot_path`. Git passthrough should match.

## Affected

- `crates/odm-core` `project_git` (or sibling)
- `crates/odm/src/main.rs` project git branch
- Integration/unit tests

## Impact

Agents cannot run git inside a slot through ODM; only Primary works.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-42-worktree-slot-lifecycle]] — slot paths and validation should exist; add may be used in tests

## Agent Brief

**Category:** feature  
**Summary:** Resolve `project git` working tree to a Worktree slot when `--wt` is set (command or global flag); keep pin auto-maintain Primary-only.

**Bindings:**
- `docs/reference/cli.md` (`project git` + `--wt`)
- `docs/reference/worktrees.md` (no auto-create; requires Project context)
- Actions cwd behavior already: missing slot path → error
- Map [[issues-40-worktree-slots-map]]

**Current behavior:**
- `project_git(git, ws, name, git_args)` always uses Primary `abs_checkout`
- main.rs: if effective_wt.is_some() → `not_implemented`

**Desired behavior:**
- Extend API e.g. `project_git(..., wt: Option<&str>)` or resolve path before `git.run`
- **effective wt** = command `--wt` or global `--wt` (command wins if both — match existing `wt.or(global_wt)` pattern)
- When wt set:
  1. Project must exist
  2. Validate slot name (same rules as lifecycle)
  3. `path = worktree_slot_path(root, project, slot)`
  4. Path must **exist** and be a git worktree/repo (`is_repo`) — else clear error (not found or operation); **do not** create
  5. `git.run(&path, git_args)` inherit stdio; exit code = git’s
- When wt set: **skip pin auto-maintain** even if HEAD changes in the slot (pin is Primary-oriented per map)
- When wt absent: keep today’s Primary + pin auto-maintain on HEAD change
- Global `--wt` without a project context on other commands: unchanged (still only meaningful where Project is known)

**Tests:**
- Unit with fake runner: argv `-C` target is slot path when wt set; pin maintain not called for wt (if maintain is observable)
- Or integration: add slot, `project git --wt slot -- rev-parse --show-toplevel` equals slot path

**Acceptance criteria:**
- [ ] `odm project git <proj> --wt <slot> -- status` runs in slot tree when slot exists
- [ ] Missing slot path → non-zero, no not-implemented message
- [ ] Invalid slot name → usage error
- [ ] Without `--wt`, Primary + pin behavior unchanged
- [ ] With `--wt`, pin file not updated due to slot HEAD movement
- [ ] `cargo test` green

**Out of scope:**
- `agent start --wt`
- Auto-create slot
- status/doctor slot columns

## Acceptance

- [ ] Agent Brief acceptance criteria all met

## Comments

Parent map: [[issues-40-worktree-slots-map]]
