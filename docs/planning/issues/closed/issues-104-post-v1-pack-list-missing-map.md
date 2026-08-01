---
id: issues-104
title: "Post-v1 pack list missing observation map"
description: "Wayfinder map: odm agent pack list includes missing (status/doctor parity); docs + core-desk dogfood."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Post-v1 pack list missing observation map

## Destination

After generate dry-run map [[issues-99-post-v1-generate-dry-run-map]] closed, pull the next AFK-ready observation slice left open by status packs [[issues-89-post-v1-status-packs-map]] (ticket 90 out-of-scope: pack list JSON shape):

1. **Pack list missing core/DTO** — `odm agent pack list --json` entries gain `missing: bool` (same path rule as status `agent_packs` / doctor `pack_missing`); install/link/rm single-entry JSON keep the same field set.
2. **CLI human + integration** — human list marks missing packs; bin tests cover present/absent.
3. **Docs honesty** — `cli.md` / env-gen-packs / CHANGELOG / README as needed.
4. **core-desk dogfood** — list missing after dest delete, then rm clears.

## Notes

- **Authority:** `docs/reference/cli.md` (agent pack list), `env-gen-packs.md`, landed `PackEntryDto` / `pack_list`, status `missing` via `symlink_metadata`.
- **Prereqs:** map 89 (status packs) closed; pack rm + doctor pack_missing landed.
- **Execution:** ticket close = decision + code/tests/docs as scoped. TDD for code tickets.
- **Standing prefs (AFK defaults):**
  - Do not implement graph, env, generate remote/templating/`template.toml`, pack marketplace/manifest, config-declared packs, `agent start`, `init --interactive`.
  - Do not implement worktree deferred product (config slots, pin↔slot, auto-prune on doctor, branch templates, global `--wt` depth).
  - `missing` rule identical to status/doctor: `path.symlink_metadata().is_err()` → missing; dangling symlink present → not missing.
  - List JSON items and install/link/rm single-entry JSON share the same fields (cli.md contract).
  - Human list stays one entry per line; append ` missing` suffix only when missing (present packs stay bare name for simple parsers).
  - No doctor changes; no status shape changes; no new crate.
  - File size ≤1000 target / ≤1250 hard.

## Decisions so far

- Child tickets: [[issues-105-pack-list-missing-dto]], [[issues-106-pack-list-missing-cli]], [[issues-107-pack-list-missing-docs-honesty]], [[issues-108-core-desk-pack-list-missing-dogfood]].
- Prefer order: 105 unblocked first; 106 blocked by 105; 107 blocked by 105+106; 108 blocked by 105+106.
- [[issues-105-pack-list-missing-dto]]: `PackEntryDto.missing` via `symlink_metadata().is_err()`; human list ` name missing` suffix; unit tests green.

## Not yet specified

_(none for this slice — AFK defaults lock missing behavior)_

## Out of scope

- `odm agent start`, `init --interactive`
- Graph, env, generate remote/templating, pack marketplace/manifest/config declarations
- Worktree deferred product
- Doctor / status shape changes (already have missing)
- Release version bump / GitHub release

## Blocked by

None
