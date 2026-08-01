---
id: issues-89
title: "Post-v1 status packs + pack observation dogfood map"
description: "Wayfinder map: odm status agent_packs inventory, doctor pack_missing CLI coverage, docs + core-desk dogfood."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 status packs + pack observation dogfood map

## Destination

After pack lifecycle map [[issues-82-post-v1-pack-lifecycle-hardening-map]] closed, pull the next AFK-ready observation slice from deferred lists:

1. **Status pack reports** — `env-gen-packs.md` / `cli.md` still list status pack reports as deferred; doctor already warns `pack_missing`. Mirror worktree_slots pattern: `odm status` includes registered agent packs (inventory + missing flag).
2. **Doctor pack_missing CLI coverage** — unit tests landed in 86; add bin-level integration so install→delete dest→doctor JSON shows the warn.
3. **Docs + dogfood** — reference honesty; core-desk exercises status packs and doctor pack_missing.

## Notes

- **Authority:** `docs/reference/env-gen-packs.md`, `cli.md`, `phased-delivery.md`; landed `pack_list` / `PackEntry`, `StatusSnapshot` + worktree_slots pattern, `doctor_pack::pack_missing_checks`.
- **Prereqs:** map 82 (pack rm + doctor pack_missing) closed.
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Do not implement graph, env, generate remote/templating, pack marketplace/manifest, config-declared packs, `agent start`, `init --interactive`.
  - Do not implement worktree deferred product (config slots, pin↔slot, auto-prune on doctor, branch templates, status orphan listing, global `--wt` depth).
  - Status packs are **registry inventory** (like registered worktree slots), not a second doctor. Include `missing: bool` per pack (path absent per same symlink_metadata rule as doctor). Doctor `pack_missing` warn stays.
  - No auto-rm on doctor `--fix`. No pack fields required on `project info`.
  - No new crate; stay in `odm-core` + thin CLI/formatters.

## Decisions so far

- Child tickets: [[issues-90-status-agent-packs]], [[issues-91-doctor-pack-missing-cli]], [[issues-92-status-packs-docs-honesty]], [[issues-93-core-desk-status-packs-dogfood]].
- Prefer order: 90 and 91 unblocked in parallel; 92 blocked by 90; 93 blocked by 90+91.
- **90 closed:** `StatusSnapshot.agent_packs` always present; each row name/source/path/mode/missing via `pack_list` + `symlink_metadata` (doctor-aligned); soft-fail `[]`; human `Agent packs:` section; packs-only not swallowed.
- **91 closed:** bin integration `doctor_pack_missing_after_deleted_dest` in `cli_agent_pack.rs` — install→no pack_missing→delete dest→`pack_missing:core-desk` warn fixable false; `--fix` leaves registry; pack rm clears.
- **92 closed:** docs honesty — cli/env-gen-packs/phased-delivery/CHANGELOG/README document status `agent_packs`; deferred lists no longer claim status pack reports unimplemented; marketplace/manifest/config/`agent start` still deferred.
- **93 closed:** core-desk README + `core_desk_status_packs_doctor_gate` — install→status missing false→delete dest→doctor pack_missing:demo + missing true→rm clears.

## Not yet specified

- _(map complete — all child tickets closed)_

## Out of scope

- `odm agent start`, `init --interactive`
- Graph, env, generate remote/templating, pack marketplace/manifest/config declarations
- Worktree deferred product
- Status orphan listing
- Auto-rm packs on doctor `--fix`
- Release version bump / GitHub release

## Blocked by

None
