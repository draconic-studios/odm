---
id: issues-95
title: "status reports orphan worktree slot dirs"
description: "odm status projects include worktree_orphans (name+path) for disk dirs not registered as git worktrees."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# status reports orphan worktree slot dirs

## Description

`worktrees.md` Deferred still lists **status listing of orphan dirs**. Doctor already warns `worktree_orphan:<project>:<slot>`; prune removes them. Operators/agents need orphans on the workspace snapshot next to registered `worktree_slots`, without running doctor.

## Affected

- `crates/odm-core/src/worktree.rs` — promote/reuse orphan discovery (`orphan_slot_names` today private)
- `crates/odm-core/src/status.rs` + `status_tests.rs` — `EntityStatus`, `build_status`, `format_status_human`
- Optional small DRY: `doctor_worktree.rs` call shared helper (only if low-risk; not required for AC)
- File size: ≤1000 target / ≤1250 hard

## Impact

Status shows registered slots only; orphan dirs are invisible until doctor/prune — agents miss cleanup signals on the primary snapshot.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Extend project rows on `StatusSnapshot` so `odm status` / `status --json` lists orphan worktree slot directories per Project (same definition as doctor/prune).

**Bindings:**

- Parent map: [[issues-94-post-v1-status-orphans-map]]
- Orphan definition: valid slot-name directory under `worktrees/<project>/` **not** in the registered set from `worktree_list` (same as doctor `worktree_orphan_checks` / `worktree_prune`)
- Registered slots stay on `worktree_slots` (`name` + `path` + `dirty`) unchanged
- Prior additive status pattern: `worktree_slots` ([[issues-65-status-worktree-slots]]), dirty ([[issues-79-worktree-slot-dirty-observation]])

**Desired behavior:**

1. **Public helper (preferred):** expose something like `worktree_orphans(git, ws, project) -> Result<Vec<WorktreeOrphanInfo>, OdmError>` (or fill orphans without git when list fails — see soft-fail below). Shape: `{ "name", "path" }` where `path` is `worktrees/<project>/<slot>` (same relative path style as slots). Sorted by `name` ascending. No `dirty` on orphans (not registered worktrees).
2. **JSON:** each project `EntityStatus` gains `worktree_orphans: Option<Vec<…>>` serialized like slots:
   - Projects: always present after `build_status` as an array (empty when none / soft-fail).
   - Progens: omit (`None` / skip_serializing) — same as `worktree_slots`.
3. **build_status:** for each project, after filling `worktree_slots`:
   - On successful `worktree_list`, compute orphans vs registered names.
   - Soft-fail: list/primary errors → `worktree_orphans: []` (and slots already `[]`); never fail whole status.
   - Missing `worktrees/<project>/` → `[]`.
   - Invalid names / files under worktrees prefix ignored (same as prune/doctor).
4. **Human (`format_status_human`):** when `worktree_orphans` non-empty, add a line under the project (after `worktrees:` if any), e.g. `    orphans: stale, other`. Empty → silent (no `orphans:` line).
5. **TDD:** unit tests — mkdir orphan under worktrees/project → appears in status JSON; registered slot is **not** in orphans; empty when no orphans; soft-fail path; human shows orphan names; progen has no field.
6. Prefer DRY: doctor orphan scan may call the shared helper; do not change doctor check ids/messages/fixable.
7. No docs/CHANGELOG (see [[issues-97-status-orphans-docs-honesty]]). No core-desk (see 98). No project info (see 96). No prune/list CLI changes.
8. `cargo test` green; `cargo clippy --workspace --all-targets -- -D warnings` clean.

**Acceptance criteria:**

- [x] `status --json` project rows include `worktree_orphans` array of `{name,path}`
- [x] Orphan dir (valid name, not registered) listed; registered slot not listed as orphan
- [x] Empty / missing worktrees dir / soft-fail → `[]`; status still succeeds
- [x] Progens omit `worktree_orphans`
- [x] Human shows orphan names when non-empty; silent when empty
- [x] Doctor orphan warn behavior unchanged (ids still `worktree_orphan:…`)
- [x] File sizes within limits; `cargo test` green; clippy `-D warnings` clean

**Out of scope:**

- `project info` orphans (ticket 96)
- Docs / core-desk
- Auto-prune, doctor `--fix` delete
- `worktree list` including orphans
- Pin↔slot / config slots

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Shipped `worktree_orphans` on project status rows.

- **Types/API:** `WorktreeOrphanInfo { name, path }`; public `worktree_orphans` + `worktree_orphan_infos` (sorted, no dirty).
- **build_status:** single `worktree_list` per project fills slots + orphans; soft-fail / missing dir → `[]`.
- **Human:** `orphans: a, b` after `worktrees:` when non-empty; silent when empty.
- **Doctor:** DRY via `worktree_orphan_infos`; ids `worktree_orphan:{project}:{slot}` unchanged.
- **Tests:** status unit coverage for list/sort/registered-not-orphan, missing dir, soft-fail, human, progen omit.
