---
id: issues-81
title: "core-desk dogfood prune --all and slot dirty status"
description: "Extend core-desk README + integration gate for prune --all and dirty slots on status."
status: closed
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# core-desk dogfood prune --all and slot dirty status

## Description

core-desk already dogfoods per-project prune and doctor dirty/orphan. After multi-project prune and slot dirty observation land, the example README and integration gate should exercise the new surfaces so regressions fail loudly.

## Affected

- `examples/core-desk/README.md`
- `crates/odm/tests/core_desk.rs` (or adjacent integration test)

## Impact

New CLI/JSON shapes lack an offline desk gate.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-78-worktree-prune-all]]
- [[issues-79-worktree-slot-dirty-observation]]

## Agent Brief

**Category:** test  
**Summary:** Dogfood `prune --all` and dirty slot observation on core-desk without expanding product scope.

**Bindings:**

- Parent map: [[issues-77-post-v1-worktree-observation-map]]
- Desk already has alpha/beta fixtures and worktree dogfood paths from maps 66/72.

**Behavior lock:**

1. **README (`examples/core-desk/README.md`):**
   - Document `odm project worktree prune --all` (and optional `--force`) as workspace-wide orphan GC.
   - Note that `odm status --json` / `project worktree list` include `dirty` on registered slots; optional dirty demo (write a file in a slot, show status/list, clean up).
   - Keep doctor orphan/dirty and per-project prune docs accurate (no contradictions).

2. **Integration gate:**
   - Extend existing core-desk worktree/prune test **or** add a focused test that:
     - After sync, creates an empty orphan under `worktrees/alpha/` (and ideally a second project orphan if beta is git after sync — if beta path is heavy, one project + `--all` still proving multi-project code path with only alpha orphans is OK).
     - Runs `odm project worktree prune --all` and asserts orphan removed / exit 0.
     - Adds a registered slot, makes it dirty, asserts `status --json` or `project worktree list --json` has `"dirty": true` for that slot (then cleanup rm --force).
   - Do not require network; use existing temp-copy harness patterns in `core_desk.rs`.

3. No production feature work beyond test/README. `cargo test` green.

**Acceptance criteria:**

- [x] core-desk README documents prune --all and dirty on status/list
- [x] Integration test covers prune --all success path
- [x] Integration test covers dirty true on list or status JSON for a registered slot
- [x] `cargo test` green

**Out of scope:**

- New product features
- Reference docs spine (issues-80)
- agent start / generate remote

## Acceptance

Mirror Agent Brief checklist.

## Answer

Dogfood gate landed without product changes.

- **README:** `examples/core-desk/README.md` documents `prune --all` / `--force` + JSON shape; status/list `dirty` field and optional dirty demo; doctor orphan/dirty + per-project prune wording kept accurate (no auto-prune).
- **Test:** `core_desk_prune_all_and_slot_dirty_gate` — empty alpha orphan → `prune --all` JSON (`all: true`, pruned entry, dir gone); registered dirty slot → status + list `"dirty": true`; `rm --force` cleanup.
- `cargo test` green.
