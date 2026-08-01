---
id: issues-51
title: "Agent pack core"
description: "Core API: registry + install/link/list local agent packs into --home/<name>."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# Agent pack core

## Description

CLI agent pack commands are stubs. Need a pure core API: resolve local pack source, install (copy) or link (symlink) into `<home>/<name>/`, and list via a Workspace registry under `.odm/`.

## Affected

- `crates/odm-core` (new module, export from `lib.rs`)
- Downstream: [[issues-52-agent-pack-cli]]

## Impact

Without core, `odm agent pack` stays not-implemented.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feature  
**Summary:** Add `odm-core` agent-pack APIs used by the CLI. Filesystem only; no network; no CLI wiring in this ticket.

**Bindings:**

- Parent map [[issues-50-agent-packs-map]] standing prefs
- `docs/reference/env-gen-packs.md` Agent packs
- `CONTEXT.md` Agent pack
- Path helpers: reuse `paths` / workspace root resolution patterns from generate where applicable
- Registry path under ODM state dir (same layout conventions as other `.odm/` state)

**Current behavior:**

- No pack module; no registry file

**Desired behavior:**

1. **Public API** (names flexible if clear), roughly:
   - `pack_list(ws) -> Result<Vec<PackEntry>, OdmError>`
   - `pack_install(ws, source, home, force) -> Result<PackEntry, OdmError>`
   - `pack_link(ws, source, home, force) -> Result<PackEntry, OdmError>`
   - `PackEntry { name, source, path, mode }` with `mode: Install | Link`
2. **Source resolution:** if relative, under workspace root via existing no-escape helper; must exist and be a directory. Name = basename.
3. **Dest:** `home.join(name)` — create `home` parents as needed. `home` may be absolute outside workspace (do **not** force home under workspace root).
4. **install / link / force** semantics per map Notes.
5. **Registry:** read/write `.odm/agent-packs.json` (or equivalent under state dir). Upsert by name on install/link. List returns registry entries sorted by name.
6. **Unit tests** with tempdirs: install copy, link symlink (cfg unix), force replace, missing source, relative escape rejected for source, list empty/after install, registry survives reload.
7. Do **not** wire CLI.
8. Module target ≤1000 LOC; YAGNI — no manifest, no marketplace, no rm command required.

**Acceptance criteria:**

- [ ] Public pack list/install/link API callable without CLI
- [ ] install copies tree to `<home>/<name>/` and records registry
- [ ] link symlinks (unix) and records registry
- [ ] force replaces existing dest; without force fails if exists
- [ ] relative source cannot escape workspace root
- [ ] Unit tests cover above
- [ ] `cargo test` green
- [ ] No CLI changes in this ticket

**Out of scope:**

- CLI / clap / JSON DTOs for stdout
- `agent start` / `agent prompt`
- core-desk fixture (unless tiny unit fixture only)
- Docs promotion beyond what tests need

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Shipped `odm-core` agent pack module:

- **API:** `pack_list`, `pack_install`, `pack_link`, `PackEntry`, `PackMode::{Install,Link}`
- **Registry:** `.odm/agent-packs.json` (pretty JSON array, upsert by name, sorted list)
- **Paths:** `agent_packs_path`; relative source via `resolve_under_root`; absolute source + home outside workspace allowed
- **install:** recursive copy of source contents into `<home>/<name>/`; force replaces whole dest
- **link:** unix symlink to absolute source; non-unix clear operation error
- **Tests:** 12 unit tests (tempdirs, force, escape, reload, unix link)
- **Out of scope held:** no CLI wiring

## Comments

Seeded with map issues-50. Closed by swarm implement cycle.
