---
id: issues-69
title: "project info reports registered worktree slots"
description: "odm project info includes registered worktree_slots like status."
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

# project info reports registered worktree slots

## Description

`odm status` now includes registered `worktree_slots` per project, but `odm project info <name>` does not. Agents inspecting one project must call `project worktree list` separately.

## Affected

- `crates/odm/src/commands/project.rs` (`ProjectInfoDto`, `project_info`, human formatter)
- Tests for project info JSON/human
- `docs/reference/cli.md` project info bullet

## Impact

Inconsistent snapshot surfaces; extra round-trips for agents.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Add registered worktree slots to `project info` JSON and human output; same semantics as status slots.

**Bindings:**

- Parent map: [[issues-66-post-v1-dogfood-slot-depth-map]]
- Reuse `worktree_list` / slot name+path shape from status (`WorktreeSlotInfo` or equivalent `{name, path}` with `path` = `worktrees/<project>/<slot>`)
- Status soft-fail policy: if list errors → empty array, do not fail entire info solely for worktree list errors
- Do **not** change `project list` unless trivial and needed for compile (YAGNI: info only)

**Behavior lock:**

1. **JSON:** `ProjectInfoDto` gains `worktree_slots: Vec<{name, path}>` (always present on info; empty when none / non-git / soft-fail). Sorted by name ascending.
2. **Human:** when non-empty, show slot names (e.g. `worktrees: a, b` or indented lines); empty → no extra noise.
3. **Not included:** dirty flags, branches, orphans.
4. TDD: unit and/or integration — registered slot appears; empty for non-git; soft-fail → `[]`.
5. Docs: `cli.md` under `project info` — mention registered slots field.
6. `cargo test` green.

**Acceptance criteria:**

- [x] `odm project info <p> --json` includes `worktree_slots` array with name/path
- [x] Registered slot appears; empty when none
- [x] Human mentions slots when present
- [x] cli.md updated
- [x] `cargo test` green

**Out of scope:**

- project list changes
- Orphan/dirty in info
- Config-declared slots

## Acceptance

Mirror Agent Brief checklist.

## Answer

`ProjectInfoDto` always includes `worktree_slots: Vec<WorktreeSlotInfo>` filled via soft-fail `worktree_list` (same as status). Human prints `worktrees: a, b` only when non-empty. Unit + `cli_worktree` integration + `cli.md` updated. `cargo test` green.
