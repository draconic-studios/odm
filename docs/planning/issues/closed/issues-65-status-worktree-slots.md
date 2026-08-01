---
id: issues-65
title: "status reports registered worktree slots per project"
description: "odm status includes registered worktree slots for each Project (not orphans)."
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

# status reports registered worktree slots per project

## Description

`odm status` snapshots Projects/Progens but not worktree slots. Operators must run `project worktree list` per project. Doctor already warns on **orphans**; status should list **registered** slots only (v1 list semantics).

## Affected

- `crates/odm-core/src/status.rs` / observation if needed
- `crates/odm-core/src/worktree.rs` (`worktree_list`)
- CLI human formatter for status (wherever status human text is built)
- Tests for status JSON/human
- `docs/reference/cli.md` status section; `worktrees.md` one sentence that status lists registered slots (orphans remain doctor-only)

## Impact

Agents lack a single workspace snapshot that includes parallel slot trees.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Extend status snapshot so each **Project** includes registered worktree slots; Progens unchanged; no orphan detection in status.

**Bindings:**

- `worktree_list` outcome: slot `name` + relative `path` (`worktrees/<project>/<slot>`)
- Existing `StatusSnapshot` / `EntityStatus` JSON (additive fields only)
- Parent map: [[issues-60-post-v1-polish-map]]
- Doctor orphans stay in doctor ([[issues-57-doctor-worktree-orphans]]); do not duplicate orphan warn in status

**Behavior lock:**

1. **JSON:** On each project entity in `status --json`, add:
   - `worktree_slots`: array of `{ "name": string, "path": string }`  
   - Sorted by `name` ascending.  
   - Empty array when none / primary not git / list fails softly → prefer empty array over failing entire status (if `worktree_list` errors, treat as empty slots for that project and do not fail status — match resilience of other optional git facts if already soft; if status already fails hard on git errors for that entity, stay consistent with existing entity sampling — prefer **not** failing whole status solely for worktree list errors: empty `worktree_slots`).
2. **Progens:** no `worktree_slots` field (or omit; do not add null noise on progens).
3. **Human:** after each project line (or in project block), if slots non-empty, show something like `worktrees: a, b` or one indented line per slot — keep readable; empty → no extra noise.
4. **Not included:** dirty flags, branch names, orphan dirs, pin interaction.
5. TDD: unit or integration — project with mocked/real registered slot appears in JSON; no slots → `[]`; progen rows lack the field or are unchanged shape aside from projects.
6. Docs: `cli.md` status bullet; `worktrees.md` note status lists registered slots only.
7. `cargo test` green.

**Acceptance criteria:**

- [x] `odm status --json` project entries include `worktree_slots` array with name/path
- [x] Registered slot from worktree list appears; orphan-only dirs do **not** appear (unless also registered — they are not)
- [x] Empty / non-git project → `worktree_slots: []` (or equivalent empty)
- [x] Human output mentions slots when present
- [x] cli.md + worktrees.md updated
- [x] `cargo test` green

**Out of scope:**

- Orphan detection in status
- Dirty slot checks
- Config-declared slots
- Changing doctor
- Pin↔slot

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

`build_status` soft-fills each Project’s `worktree_slots` via `worktree_list` (`WorktreeSlotInfo` name + relative path, already sorted). List errors → `[]` without failing status. Progens keep `worktree_slots: None` (omitted from JSON). Human adds `worktrees: a, b` only when non-empty. Docs: `cli.md` + `worktrees.md`. Tests cover registered slots, empty/non-git/list Err, progen omit, human noise rules.
