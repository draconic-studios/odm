---
id: issues-70
title: "doctor warns on dirty registered worktree slots"
description: "odm doctor Warn when a registered worktree slot working tree is dirty."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# doctor warns on dirty registered worktree slots

## Description

`worktrees.md` Deferred lists **doctor dirty-slot checks**. Orphan dirs already warn; registered slots that are **dirty** (uncommitted changes) are invisible to doctor. Operators and agents need a non-fatal signal before prune/rm or handoff.

## Affected

- `crates/odm-core/src/doctor_worktree.rs` (prefer extend here; keep `doctor.rs` thin)
- Git dirty probe (reuse existing dirty observation helpers if any; else `git status --porcelain` via `odm_git`)
- Tests
- `docs/reference/worktrees.md`, `cli.md` doctor bullet; optional CHANGELOG Unreleased

## Impact

Dirty agent slots go unnoticed until `worktree rm` fails or humans inspect each tree.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Doctor **Warn** checks for dirty **registered** worktree slots; not fixable; orphans unchanged.

**Bindings:**

- Parent map: [[issues-66-post-v1-dogfood-slot-depth-map]]
- Registered slots = same set as `worktree_list` (git worktree list filtered to `worktrees/<project>/`)
- Orphan checks stay separate (`worktree_orphan:…`)
- Primary checkout dirty is **out of scope** here (status/entity observation already covers primary dirty hints)

**Behavior lock:**

1. For each configured Project with git primary, for each **registered** slot path:
   - If working tree is dirty → `DoctorCheck` with:
     - `id`: `worktree_dirty:<project>:<slot>`
     - status: **Warn**
     - `fixable`: **false**
     - message mentions dirty slot path (`worktrees/<project>/<slot>`)
2. Clean registered slots → no dirty check.
3. Missing/unreadable slot path or git errors → skip that slot (no Fail storm); do not fail whole doctor solely for probe errors.
4. Non-git primary → no dirty slot checks (same as orphan skip).
5. `doctor --fix` does **not** clean or stash slot trees.
6. `ok` remains true when only Warns (existing doctor semantics).
7. TDD with mocked git runner and/or temp repos: dirty slot warns; clean does not; orphan still orphan-only.
8. Docs: `worktrees.md` — move dirty-slot doctor check out of pure Deferred into implemented note (keep other deferred items); `cli.md` doctor bullet one phrase; CHANGELOG Unreleased optional bullet.
9. Keep files under LOC limits; extend `doctor_worktree.rs` rather than bloating `doctor.rs`.
10. `cargo test` green.

**Acceptance criteria:**

- [ ] Dirty registered slot → Warn `worktree_dirty:<project>:<slot>`, fixable false
- [ ] Clean registered slot → no dirty warn
- [ ] Orphan behavior unchanged
- [ ] `--fix` does not modify slot trees
- [ ] worktrees.md + cli.md updated
- [ ] `cargo test` green

**Out of scope:**

- Failing doctor `ok` on dirty slots
- Auto-stash / auto-commit
- Primary dirty checks redesign
- Prune command (separate ticket)

## Acceptance

Mirror Agent Brief checklist.
