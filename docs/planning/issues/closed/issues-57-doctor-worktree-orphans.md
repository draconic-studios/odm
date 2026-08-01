---
id: issues-57
title: "Doctor warn on worktree slot orphans"
description: "odm doctor warns when worktrees/<project>/<slot> dirs exist but are not registered git worktrees."
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

# Doctor warn on worktree slot orphans

## Description

`worktrees.md` deferred list includes doctor obligations for orphan slot dirs. After `project rm` or manual deletes, stray directories under `worktrees/<project>/` can remain. Doctor should **warn** (not fail the overall ok solely for orphans — use Warn status so `ok` stays true unless other Fails exist).

## Affected

- `crates/odm-core/src/doctor.rs` (and helpers; may use `odm_git` worktree list + `paths::worktree_slot_path`)
- Doctor unit tests in `doctor.rs` or adjacent
- Optional: one integration assertion if a doctor CLI test harness already exists; otherwise unit tests suffice
- Docs: `worktrees.md` Deferred bullet for doctor orphans — note **warn landed** or move to implemented rules; `cli.md` doctor section if it lists checks

## Impact

Operators cannot see stale slot dirs; agents leave disk clutter without signal.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Add doctor checks that warn on orphan worktree slot paths for configured Projects.

**Bindings:**

- `docs/reference/worktrees.md` (path layout, v1 list semantics)
- Existing doctor patterns: `DoctorCheck { id, status, message, fixable }`, `CheckStatus::Warn`
- Git: `Git::worktree_list` / primary checkout like `worktree_list` in `odm-core/src/worktree.rs`
- Parent map: [[issues-55-post-v1-hardening-map]]

**Behavior lock:**

1. For each **configured Project** whose primary is a git repo:
   - If `worktrees/<project>/` does not exist → no checks for that project (or a single pass is fine — no noise).
   - Read directory entries that are **directories** (skip `.` / `..`); treat each child name as a candidate slot name (skip names that fail slot-name validation silently or with no check).
   - Obtain registered git worktrees for the primary; filter those under the slot prefix (same idea as `worktree_list`).
   - **Orphan:** directory exists at `worktrees/<project>/<slot>/` but path is **not** a registered git worktree path → one `DoctorCheck` per orphan:
     - `id`: `worktree_orphan:<project>:<slot>`
     - `status`: `Warn`
     - `message`: clear, includes relative path `worktrees/<project>/<slot>`
     - `fixable`: `false` (do **not** delete on `doctor --fix`)
2. Project primary **not** git: skip orphan scan for that project (worktree commands already error; no new Fail required).
3. Unknown dirs under `worktrees/` that are **not** configured project names: **ignore** (v1 lock).
4. Registered worktrees with missing dirs are **out of scope** (git’s problem / list already handles).
5. TDD: unit tests with temp workspace + fake or real git runner pattern used elsewhere in doctor/worktree tests.
6. `cargo test` green; no drive-by refactors.

**Acceptance criteria:**

- [x] Orphan slot dir → doctor report includes `worktree_orphan:…` Warn, fixable false
- [x] Healthy registered slot (after real or mocked worktree list match) does not warn as orphan
- [x] `doctor --fix` does not delete orphan dirs
- [x] Missing `worktrees/` → no failure from this feature
- [x] Docs: `worktrees.md` no longer lists “doctor obligations for orphans” as fully deferred if warn is landed (one honest sentence)
- [x] `cargo test` green

**Out of scope:**

- GC/prune commands
- Config-declared slots
- Pin↔slot
- Multi-project workspace-level slots
- status command changes
- Auto-create slots

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Landed `worktree_orphan_checks` in `odm-core` doctor: for each configured Project with a git primary and existing `worktrees/<project>/`, disk slot dirs (valid names) not present in `worktree_list` emit `DoctorCheck` id `worktree_orphan:<project>:<slot>`, status **Warn**, `fixable: false`. Non-git primaries and unknown top-level `worktrees/` names are skipped. `doctor --fix` does not delete orphans. Docs updated in `worktrees.md`, `cli.md`, `phased-delivery.md`. Unit tests cover orphan, healthy slot, missing dir, non-git, unknown project, and fix-no-delete.
