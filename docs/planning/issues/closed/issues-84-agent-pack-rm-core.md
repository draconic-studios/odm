---
id: issues-84
title: "Core API: agent pack rm (uninstall)"
description: "Add odm-core pack_rm to drop registry entry and remove install/link destination."
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

# Core API: agent pack rm (uninstall)

## Description

Agent packs v1 has install/link/list only ([[issues-51-agent-pack-core]]). Operators and dogfood cannot cleanly remove a registered pack from `.odm/agent-packs.json` and `<home>/<name>`. Add pure core `pack_rm` (no CLI in this ticket).

## Affected

- `crates/odm-core/src/agent_pack.rs`
- `crates/odm-core/src/lib.rs` re-export
- Unit tests in `agent_pack` module

## Impact

Stale registry entries and leftover homes accumulate; force-reinstall is the only cleanup path today.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** `pack_rm(ws, name) -> Result<PackEntry, OdmError>` removes the named registry entry and best-effort deletes its destination path.

**Bindings:**

- Parent map: [[issues-82-post-v1-pack-lifecycle-hardening-map]]
- Existing: `pack_list` / `pack_install` / `pack_link`, `PackEntry`, `PackMode`, `.odm/agent-packs.json`, `remove_dest` helpers already used by force install/link
- Domain: `CONTEXT.md` Agent pack; `env-gen-packs.md` local v1

**Desired behavior:**

1. **API:** `pub fn pack_rm(ws: &Workspace, name: &str) -> Result<PackEntry, OdmError>`
2. **Lookup:** trim name; empty → usage exit `1`. Unknown name (not in registry) → `not_found` exit `4`.
3. **Return value:** the `PackEntry` as it was in the registry **before** mutation (name/source/path/mode).
4. **Filesystem:** if `entry.path` exists, remove it (file, symlink, or directory tree) using the same spirit as force-replace `remove_dest`. If path is already missing, **still succeed** after registry drop (stale-registry cleanup).
5. **Registry:** rewrite `.odm/agent-packs.json` without that name; preserve other entries; stable pretty JSON array shape unchanged.
6. **No** `--keep-files` flag in v1 (YAGNI).
7. **Unit tests** (tempdirs): rm after install; rm after link (unix); unknown name → not_found; missing dest still drops registry and returns entry; other packs preserved.
8. Do **not** wire CLI (that is [[issues-85-agent-pack-rm-cli]]).
9. File size: `agent_pack.rs` must stay ≤1250 (prefer ≤1000); split only if needed.

**Acceptance criteria:**

- [x] `pack_rm` public and re-exported from `odm_core`
- [x] Removes registry entry by name
- [x] Deletes existing dest (install dir or symlink)
- [x] Missing dest + present registry → success + registry cleaned
- [x] Unknown name → not_found
- [x] Unit tests cover above
- [x] `cargo test` green
- [x] No CLI changes in this ticket

**Out of scope:**

- CLI / clap / JSON DTOs
- Doctor pack checks ([[issues-86-doctor-pack-missing]])
- Marketplace, manifest, config-declared packs
- `agent start`

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Added `odm_core::pack_rm(ws, name) -> Result<PackEntry, OdmError>`:

- Empty/whitespace name → usage (exit 1); unknown name → not_found (exit 4)
- Returns pre-mutation `PackEntry`; drops registry row; best-effort `remove_dest` when path exists; missing dest still succeeds
- Re-exported from `lib.rs`; six unit tests (install/link/unknown/empty/stale dest/preserve other)
- No CLI (issues-85). `agent_pack.rs` ~781 LOC.
