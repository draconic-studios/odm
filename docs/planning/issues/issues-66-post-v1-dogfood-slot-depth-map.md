---
id: issues-66
title: "Post-v1 dogfood + worktree slot depth map"
description: "Wayfinder map: CHANGELOG honesty after find/status slots, core-desk dogfood, project info slots, doctor dirty slots, orphan prune."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 dogfood + worktree slot depth map

## Destination

After polish map [[issues-60-post-v1-polish-map]] closed (`find --limit`, status registered slots), close the next AFK-ready slice:

1. **CHANGELOG honesty** — Unreleased records `find --limit` and status `worktree_slots`.
2. **core-desk dogfood** — README + integration gate exercise worktree add/list, status slots, and `find --limit`.
3. **`project info` slots** — single-project info reports registered worktree slots (same shape as status).
4. **Doctor dirty registered slots** — Warn when a registered slot working tree is dirty (not orphans).
5. **Worktree orphan prune** — explicit CLI to remove doctor-warned orphan slot dirs (manual GC; not auto on doctor --fix).

## Notes

- **Authority:** `worktrees.md` Deferred, `cli.md`, `CHANGELOG.md`, `examples/core-desk/README.md`, status/worktree APIs already landed.
- **Prereqs:** worktree v1, doctor orphan warn, status `worktree_slots`, `find --limit`.
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Docs tickets are docs-only unless the ticket says otherwise.
  - Status/info slots = **registered** git worktrees only; orphans stay doctor (+ prune).
  - Dirty-slot doctor checks are **Warn**, `fixable: false` (do not auto-clean dirty trees).
  - Prune only removes **orphan** dirs (not registered worktrees); never touches Primary.
  - Do not implement graph, env, generate remote, pack marketplace, `agent start`, init interactive, config-declared slots, pin↔slot, branch templates.

## Decisions so far

- Child tickets: [[issues-67-changelog-find-limit-status-slots]], [[issues-68-core-desk-worktree-dogfood]], [[issues-69-project-info-worktree-slots]], [[issues-70-doctor-dirty-worktree-slots]], [[issues-71-worktree-orphan-prune]].
- Prefer order: 67 (docs) unblocked; 68 independent; 69 after status shape known (already landed); 70 independent; 71 after orphan semantics (already landed) — all unblocked at seed.
- **67 closed:** CHANGELOG Unreleased records `find --limit` (default 200) and status registered `worktree_slots`; orphans remain doctor-only; no prune/dirty claims yet.

## Not yet specified

- _(none for this map — further deferred product work stays out of scope)_

## Out of scope

- `odm agent start`, `init --interactive`
- Graph, env, generate remote/templating, pack marketplace/manifest
- Config-declared slots, pin↔slot, branch naming templates
- Auto-delete orphans on `doctor --fix`
- Release version bump / GitHub release

## Blocked by

None
