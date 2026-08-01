---
id: issues-94
title: "Post-v1 status worktree orphans map"
description: "Wayfinder map: odm status (and project info) list orphan worktree slot dirs; docs + core-desk dogfood."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 status worktree orphans map

## Destination

After status packs map [[issues-89-post-v1-status-packs-map]] closed, pull the next AFK-ready observation slice from `worktrees.md` **Deferred**:

1. **Status orphan listing** — `odm status` reports orphan slot dirs per Project (same orphan definition as doctor/prune), separate from registered `worktree_slots`.
2. **Project info parity** — `odm project info` includes the same orphan list shape.
3. **Docs + dogfood** — reference honesty; core-desk exercises status orphans + prune clears them.

## Notes

- **Authority:** `docs/reference/worktrees.md` Deferred (`status` listing of orphan dirs), `cli.md`, landed `orphan_slot_names` / doctor `worktree_orphan:*` / `worktree_prune`.
- **Prereqs:** maps 77 + 89 children closed (registered slots + dirty + packs observation).
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Do not implement graph, env, generate remote/templating, pack marketplace/manifest, config-declared packs/slots, `agent start`, `init --interactive`.
  - Do not implement pin↔slot, auto-prune on `doctor --fix`, branch naming templates, global `--wt` depth.
  - Orphans remain **observation only** on status/info — cleanup stays explicit `worktree prune` / `prune --all`. Doctor orphan **warn** stays.
  - Do not mix orphans into `worktree list` registered slots (list stays registered-only).
  - No new crate; stay in `odm-core` + thin CLI/formatters.
  - Prefer promoting a shared public orphan-list helper from existing private `orphan_slot_names` so doctor/status/prune stay DRY (optional small refactor inside code tickets).

## Decisions so far

- Child tickets: [[issues-95-status-worktree-orphans]], [[issues-96-project-info-worktree-orphans]], [[issues-97-status-orphans-docs-honesty]], [[issues-98-core-desk-status-orphans-dogfood]].
- Prefer order: 95 unblocked first; 96 blocked by 95 (reuse shape); 97 blocked by 95+96; 98 blocked by 95+96.
- **95 closed:** `EntityStatus.worktree_orphans` on projects (`{name,path}`, sorted); soft-fail `[]`; human `orphans: …`; public `worktree_orphans` / `worktree_orphan_infos`; doctor DRY unchanged ids.
- **96 closed:** `ProjectInfoDto.worktree_orphans` always-present `Vec` (`{name,path}`); fill via list + `worktree_orphan_infos`; human `orphans: …` (unindented info style); registered slots excluded.
- **97 closed:** docs honesty — worktrees/cli/phased-delivery/CHANGELOG/README record landed status+info `worktree_orphans`; deferred no longer pure-TODO for orphan listing; doctor warn + prune remain cleanup.

## Not yet specified

- Exact human wording for orphan lines (implementer picks minimal: e.g. `orphans: a, b` under project).

## Out of scope

- `odm agent start`, `init --interactive`
- Graph, env, generate remote/templating, pack marketplace/manifest/config declarations
- Config-declared slots, pin↔slot, branch templates, auto-prune on doctor `--fix`
- Global `--wt` depth beyond existing path binding
- Changing `worktree list` to include orphans
- Release version bump / GitHub release

## Blocked by

None
