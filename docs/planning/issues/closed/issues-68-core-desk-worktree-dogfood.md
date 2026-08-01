---
id: issues-68
title: "core-desk dogfood worktree + status slots + find --limit"
description: "Extend examples/core-desk README and core_desk integration gate for worktree, status slots, find --limit."
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

# core-desk dogfood worktree + status slots + find --limit

## Description

`examples/core-desk` and `crates/odm/tests/core_desk.rs` cover sync/pin/status/doctor and (via other tests) generate/pack/progen, but the **full gate** and README do not dogfood **worktree slots**, **status `worktree_slots`**, or **`find --limit`**. Post-0.1.0 surfaces lack a single offline path operators can copy.

## Affected

- `examples/core-desk/README.md`
- `crates/odm/tests/core_desk.rs` (`core_desk_full_gate` or a sibling test in the same file)
- Optionally README root quickstart only if it still omits worktree after core-desk update (prefer core-desk only)

## Impact

Dogfood drift; regressions in status slots / worktree / find limit may not be caught by the primary harness.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** test + docs  
**Summary:** Extend core-desk dogfood docs and integration harness. No product feature work beyond what already exists.

**Bindings:**

- Parent map: [[issues-66-post-v1-dogfood-slot-depth-map]]
- Existing: `cli_worktree.rs`, status unit tests, `find_limit_*` in `progen_vault.rs`, core-desk sync gate
- Worktree add needs a **git** primary after `odm sync` (alpha/beta fixtures)

**Behavior lock:**

1. **README** (`examples/core-desk/README.md`): after sync/status/doctor (or in a clear Worktree section), document:
   - `odm project worktree add alpha <slot> --branch <b>` (use a unique slot name e.g. `dogfood`)
   - `odm project worktree list alpha`
   - `odm status` / `odm status --json` showing slots
   - `odm find DeskUniqueToken --limit 5` (or similar) after reindex
   - Keep existing generate/pack/progen examples; do not delete them.
2. **Integration** in `core_desk.rs` (extend `core_desk_full_gate` **or** add `core_desk_worktree_status_find_gate` that reuses `setup_temp_core_desk`):
   - After successful `sync` (and pin apply optional):
     - `project worktree add alpha dogfood --branch odm-dogfood` (or equivalent) → success
     - `status --json` → the alpha project entry has `worktree_slots` array containing `{ "name": "dogfood", ... }` with path `worktrees/alpha/dogfood`
     - After `progen reindex` (if not already): `find DeskUniqueToken --limit 1 --json` → success, hits length ≤ 1, and at least one hit when token present
   - Cleanup optional: `worktree rm` so temp dir is tidy (nice-to-have).
3. Skip without git (same pattern as existing tests).
4. Do **not** require prune/dirty-doctor features.
5. `cargo test` green.

**Acceptance criteria:**

- [x] core-desk README documents worktree add/list + status slots + find --limit
- [x] Integration test asserts status JSON `worktree_slots` after add on core-desk alpha
- [x] Integration test exercises `find --limit` against core-desk progen token
- [x] `cargo test` green

**Out of scope:**

- New CLI flags
- Committing real `worktrees/` or `projects/` under examples/core-desk
- Doctor dirty / prune

## Acceptance

Mirror Agent Brief checklist.

## Answer

Extended core-desk dogfood for post-0.1.0 worktree/status/find surfaces:

- **README** (`examples/core-desk/README.md`): Worktree section documents `project worktree add alpha dogfood --branch odm-dogfood`, list, status human/JSON `worktree_slots`, optional rm; progen section adds `find DeskUniqueToken --limit 5`. Existing generate/pack/progen examples kept.
- **Integration** (`crates/odm/tests/core_desk.rs`): sibling `core_desk_worktree_status_find_gate` — sync → worktree add → assert alpha `worktree_slots` contains dogfood at `worktrees/alpha/dogfood` → reindex → `find DeskUniqueToken --limit 1 --json` (one hit, id `welcome`) → optional rm. Skips without git.
- No product code changes. `cargo test` green.
