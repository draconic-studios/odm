---
id: issues-77
title: "Post-v1 worktree multi-prune + slot dirty observation map"
description: "Wayfinder map: workspace-wide worktree prune --all; dirty flag on registered slots in list/status/info; docs + dogfood."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 worktree multi-prune + slot dirty observation map

## Destination

After honesty/dogfood map [[issues-72-post-v1-honesty-dogfood-map]] closed, pull the next AFK-ready slice from `worktrees.md` **Deferred**:

1. **Multi-project prune** — `odm project worktree prune --all [--force]` walks every configured Project and applies the same orphan GC as per-project prune.
2. **Slot dirty observation** — registered worktree slots expose `dirty` on `worktree list`, `status`, and `project info` (doctor dirty-warn already landed; status/list still name+path only).
3. **Docs + dogfood** — `worktrees.md` / `cli.md` / `phased-delivery.md` / CHANGELOG / core-desk honest and exercised.

## Notes

- **Authority:** `docs/reference/worktrees.md` Deferred, `cli.md`, landed `worktree_prune` / doctor dirty checks, `WorktreeSlotInfo`.
- **Prereqs:** maps 66 + 72 children closed (per-project prune, doctor orphan/dirty, status/info slots).
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Do not implement graph, env, generate remote/templating, pack marketplace/manifest, `agent start`, `init --interactive`.
  - Do not implement config-declared slots, pin↔slot, branch naming templates, auto-prune on `doctor --fix`.
  - Per-project `prune <project>` remains; `--all` is additive.
  - Orphans stay doctor-warn + prune only — status still does **not** list orphan dirs.
  - Dirty on slots is observation only — no clean/stash/fix.

## Decisions so far

- Child tickets: [[issues-78-worktree-prune-all]], [[issues-79-worktree-slot-dirty-observation]], [[issues-80-worktree-observation-docs-honesty]], [[issues-81-core-desk-worktree-observation-dogfood]].
- Prefer order: 78 and 79 unblocked in parallel; 80 blocked by 78+79; 81 blocked by 78+79.
- **78 closed:** `odm project worktree prune --all [--force]` — multi-project orphan GC via `worktree_prune_all`; soft-skip non-git/missing; exit 3 on skipped nonempty; distinct `--all` JSON with `skipped_nonempty`.

## Not yet specified

- _(none for this map — further deferred product stays out of scope)_

## Out of scope

- `odm agent start`, `init --interactive`
- Graph, env, generate remote/templating, pack marketplace
- Config-declared slots, pin↔slot, branch templates
- Auto-delete orphans on `doctor --fix`
- Status listing of orphan dirs
- Release version bump / GitHub release

## Blocked by

None
