---
id: issues-96
title: "project info reports orphan worktree slot dirs"
description: "odm project info includes worktree_orphans same shape as status project rows."
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

# project info reports orphan worktree slot dirs

## Description

After status gains `worktree_orphans`, `odm project info` should expose the same orphan inventory for parity with registered `worktree_slots` (already on info).

## Affected

- Project info core/DTO path (same module that fills `worktree_slots` on info — likely `membership` / project command formatters)
- Unit and/or CLI tests for `project info --json` and human
- File size limits

## Impact

Info is the single-entity snapshot; without orphans, agents must call status or doctor for cleanup signals on one project.

## Proposed Fix

See Agent Brief.

## Blocked by

None (95 closed)

## Agent Brief

**Category:** feat  
**Summary:** Add `worktree_orphans` to `odm project info` JSON and human output, reusing the helper/shape from [[issues-95-status-worktree-orphans]].

**Bindings:**

- Parent map: [[issues-94-post-v1-status-orphans-map]]
- Must land after 95 (shared `{name,path}` shape + orphan definition)
- Existing info `worktree_slots` pattern ([[issues-69-project-info-worktree-slots]] / dirty observation)

**Desired behavior:**

1. **JSON:** `project info --json` includes `worktree_orphans: [ { "name", "path" } ]` always (empty array when none / non-git / soft-fail). Same path strings as status.
2. **Human:** when non-empty, show orphans (e.g. `orphans: stale` or a short section consistent with existing `worktrees:` line style on info). Empty → no orphan line.
3. **Errors:** unknown project → existing usage/not-found; non-git primary → existing behavior for slots (empty orphans array, not a new hard error solely for orphans).
4. **TDD:** orphan dir appears on info JSON; registered-only → empty orphans; human coverage.
5. No docs/core-desk in this ticket.
6. `cargo test` green; clippy `-D warnings` clean; file sizes OK.

**Acceptance criteria:**

- [ ] `project info --json` has `worktree_orphans` array `{name,path}`
- [ ] Orphan present → listed; registered slot not in orphans
- [ ] Empty / soft-fail / non-git → `[]` (info still succeeds as today)
- [ ] Human shows orphans when non-empty
- [ ] Reuses 95 helper/shape (no divergent orphan definition)
- [ ] `cargo test` green; clippy clean; file sizes OK

**Out of scope:**

- Docs / core-desk
- status changes beyond what 95 already did
- prune / doctor changes

## Acceptance

- [ ] Agent Brief acceptance criteria all met
