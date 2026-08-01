---
id: issues-72
title: "Post-v1 honesty + dogfood after slot depth map"
description: "Wayfinder map: phased-delivery/CHANGELOG/README honesty after prune+dirty+info slots; core-desk dogfood; clippy on doctor_worktree."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 honesty + dogfood after slot depth map

## Destination

After dogfood/slot-depth map [[issues-66-post-v1-dogfood-slot-depth-map]] closed (find limit, status slots, project info slots, doctor dirty, orphan prune), close the next AFK-ready slice:

1. **Reference + CHANGELOG honesty** — `phased-delivery.md` no longer lists orphan **GC** as deferred; Phase spine records prune, dirty-slot warn, find `--limit`, status/info `worktree_slots`. CHANGELOG records **project info** registered slots.
2. **core-desk dogfood** — README + integration gate exercise orphan doctor warn, dirty-slot warn, and `project worktree prune`.
3. **Clippy clean** — `doctor_worktree.rs` clippy warnings fixed (`for_kv_map`, `single_match`).
4. **Root README honesty** — consumer-facing status/quickstart mention prune / find `--limit` / info slots without claiming deferred product.

## Notes

- **Authority:** `worktrees.md`, `cli.md`, `CHANGELOG.md`, `phased-delivery.md`, `README.md`, `examples/core-desk/README.md`, landed APIs in `odm-core` / CLI tests.
- **Prereqs:** map 66 children closed (67–71).
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Docs tickets are docs-only unless the ticket says otherwise.
  - Do not implement graph, env, generate remote, pack marketplace, `agent start`, init interactive, config-declared slots, pin↔slot, branch templates, auto-prune on `doctor --fix`.
  - Prune remains explicit; doctor dirty/orphan stay Warn + non-fixable.

## Decisions so far

- Child tickets: [[issues-73-phased-delivery-changelog-slot-depth-honesty]], [[issues-74-core-desk-prune-dirty-dogfood]], [[issues-75-clippy-doctor-worktree]], [[issues-76-readme-slot-depth-honesty]].
- Prefer order: 73 (docs) unblocked; 76 (docs) unblocked; 75 (clippy) unblocked; 74 (dogfood) unblocked — all unblocked at seed.
- **issues-73 closed** — CHANGELOG Unreleased records `project info` `worktree_slots`; phased-delivery Phase spine includes prune, dirty-slot doctor, find `--limit`, status+info slots; deferred no longer lists bare GC (aligns with `worktrees.md`).

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
