---
id: issues-74
title: "core-desk dogfood prune + doctor dirty/orphan"
description: "Extend core-desk README and integration gate for orphan warn, dirty-slot warn, and worktree prune."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# core-desk dogfood prune + doctor dirty/orphan

## Description

`examples/core-desk` and `core_desk_worktree_status_find_gate` dogfood worktree add/list, status slots, and find `--limit`, but not **doctor orphan/dirty** or **`project worktree prune`**. Those surfaces landed in map 66 without an offline copy-paste path or harness lock.

## Affected

- `examples/core-desk/README.md`
- `crates/odm/tests/core_desk.rs`

## Impact

Dogfood drift; regressions in prune / dirty-orphan doctor may only be caught by unit/CLI tests, not the primary desk gate.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** test + docs  
**Summary:** Extend core-desk dogfood docs and integration harness. No new product features.

**Bindings:**

- Parent map: [[issues-72-post-v1-honesty-dogfood-map]]
- Existing: `core_desk_worktree_status_find_gate`, `cli_worktree.rs` prune tests, doctor dirty/orphan unit tests in `doctor_worktree.rs`
- Orphan = valid slot-name dir under `worktrees/<project>/` **not** registered via git worktree
- Dirty = registered slot with uncommitted changes (`git` dirty)

**Behavior lock:**

1. **README** (`examples/core-desk/README.md`): extend Worktree section (keep existing add/list/status/find examples) with a short path such as:
   - Create an empty orphan dir under `worktrees/alpha/<name>/` (document mkdir) → `odm doctor` shows `worktree_orphan:alpha:<name>`
   - Optional: dirty a registered slot (e.g. touch a file in `worktrees/alpha/dogfood`) → doctor shows `worktree_dirty:alpha:dogfood`
   - `odm project worktree prune alpha` removes empty orphan; `--force` for non-empty (mention exit `3` when non-empty remain without force)
   - Note doctor `--fix` does **not** delete orphans or clean dirty slots
2. **Integration** in `core_desk.rs` — extend existing worktree gate **or** add sibling `core_desk_prune_dirty_doctor_gate` reusing `setup_temp_core_desk`:
   - After sync + `worktree add alpha dogfood --branch …` (or reuse pattern from existing gate):
     - mkdir empty orphan `worktrees/alpha/stale-orphan` (valid slot name)
     - `doctor --json` (or human) → includes check id `worktree_orphan:alpha:stale-orphan` (Warn, fixable false)
     - Write an untracked/modified file in the registered dogfood slot → doctor includes `worktree_dirty:alpha:dogfood`
     - `project worktree prune alpha --json` → pruned contains stale-orphan; path `worktrees/alpha/stale-orphan`; registered dogfood still present
     - Optional cleanup: rm worktree dogfood
   - Skip without git (same as existing).
3. Do not commit real `worktrees/` under examples/core-desk.
4. `cargo test` green.

**Acceptance criteria:**

- [x] core-desk README documents orphan doctor warn + prune (and dirty warn or points at it)
- [x] Integration asserts doctor orphan id after creating orphan on core-desk alpha
- [x] Integration asserts doctor dirty id after dirtying a registered slot
- [x] Integration asserts prune removes the empty orphan and leaves registered slot
- [x] `cargo test` green

**Out of scope:**

- New CLI flags or doctor auto-fix for orphans/dirty
- Graph / agent start / generate remote
- Changing prune/doctor semantics

## Acceptance

Mirror Agent Brief checklist.

## Answer

Extended core-desk dogfood for orphan/dirty doctor and prune:

- **README** (`examples/core-desk/README.md`): Worktree section documents mkdir orphan → `worktree_orphan:alpha:<name>`, optional dirty → `worktree_dirty:alpha:dogfood`, `project worktree prune` / `--force` / exit 3, and that `doctor --fix` does not delete orphans or clean dirty slots.
- **Integration** (`core_desk_prune_dirty_doctor_gate`): after sync + worktree add, creates empty `stale-orphan`, asserts doctor warn ids for orphan and dirty, prune JSON removes orphan and leaves registered `dogfood`.

No product code changes. `cargo test` green.
