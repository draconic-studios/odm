---
id: issues-50
title: "Agent packs map"
description: "Wayfinder map: implement odm agent pack list|install|link for local packs into an agent home."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Agent packs map

## Destination

Productize **Agent packs** (v1 local only) per `docs/reference/env-gen-packs.md`, `cli.md`, and `CONTEXT.md`: portable skill/prompt bundles installed or linked into an **agent-native home** (not under `.odm/`, not Project trees by default). CLI `odm agent pack list|install|link`. Leave `agent start` / `agent prompt` as stubs.

## Notes

- **Domain:** root `CONTEXT.md` (Agent pack).
- **Authority:** `env-gen-packs.md` (sketch intent), `cli.md` (reserved names), `architecture.md` (no pack payloads under `.odm/`; optional state/registry under ODM state OK).
- **Prereqs already landed:** CLI `Agent` / `AgentCmd::{Pack,Start,Prompt}` stubs → `not_implemented`; stub test exit 1; domain term in `CONTEXT.md`.
- **Execution override:** ticket resolution = decision recorded **and** code/tests land. Prefer TDD.
- **Standing prefs (seeded 2026-08-01, AFK best-default):**
  - **No new crate** for v1 — `odm-core` module (e.g. `agent_pack`) owns install/link/list + registry; CLI thin adapter + JSON.
  - **No Workspace config declarations** for packs (still deferred per `config.md`).
  - **CLI shape:**
    - `odm agent pack list`
    - `odm agent pack install <source> --home <path> [--force]`
    - `odm agent pack link <source> --home <path> [--force]`
  - **Source:** local filesystem path to a **directory**. Relative paths resolve under Workspace root (no escape). Absolute paths allowed. Pack **name** = final path component (directory basename). No remote/marketplace; no `pack.toml` required in v1.
  - **Home:** required `--home` — agent-native root (may be outside Workspace; expand only what std/clap give — no magic agent matrix). Pack materializes at `<home>/<name>/` (home itself is not replaced).
  - **install:** recursive copy of source directory contents into `<home>/<name>/`. If dest exists and is non-empty (or is a file/symlink) without `--force` → operation error exit `3`. With `--force`, replace dest (remove existing file/dir/symlink then copy).
  - **link:** create symlink `<home>/<name>` → absolute resolved source. Same exists/`--force` policy (replace). On platforms without symlink support, clear operation error (no silent copy fallback in v1).
  - **list:** read Workspace registry (below). Human: one name per line sorted (empty → `(no agent packs)`). JSON: `{ "packs": [ { "name", "source", "path", "mode" } ] }` where `mode` is `"install"` | `"link"`, `path` is resolved install/link path, `source` is recorded source string.
  - **Registry:** Workspace-local state file under ODM state directory, e.g. `.odm/agent-packs.json` (JSON array or object — implementer picks one stable shape). Updated on successful install/link. List is registry-backed (does not require scanning arbitrary homes). Stale entries (path missing) still list; optional `exists: bool` in JSON is nice-to-have not required.
  - **Unknown / bad source:** missing source dir → not_found or usage exit `1`/`4` per existing spine honesty; prefer `4` not_found for missing path, `1` for bad args.
  - **Workspace required:** pack commands need a discoverable Workspace (same as generate).
  - **Proof:** unit tests (tempdirs, force, symlink where unix); integration tests via real CLI; optional tiny pack fixture under `examples/core-desk` or test-only dirs.
- **Deferred (map out of scope):** `agent start`, `agent prompt`, pack manifest schema, marketplace, Windows junction policy beyond honest error, config-declared packs, status/doctor pack reports, env injection, graph.

## Decisions so far

- Slice tickets: [[issues-51-agent-pack-core]], [[issues-52-agent-pack-cli]], [[issues-53-agent-pack-integration-and-docs]].
- Parallel hardening (not on this map): [[issues-54-readme-post-010-docs-drift]].
- **Core landed (issues-51):** `odm-core` `agent_pack` module — `pack_list` / `pack_install` / `pack_link`, registry `.odm/agent-packs.json`, `agent_packs_path`.
- **CLI landed (issues-52):** `odm agent pack list|install|link` thin adapter; human + `--json`; install/link JSON is a single entry object (same fields as list items); `start`/`prompt` remain stubs.

## Not yet specified

- Whether list JSON includes `exists` (optional) — core registry omits it; CLI does not add it.
- Registry shape locked by core: pretty JSON array of `{ name, source, path, mode }` with `mode` lowercase `install`|`link`.

## Out of scope

- `odm agent start` / `odm agent prompt` productization
- Graph
- Env profiles
- Remote pack fetch
- Changing generate/worktree behavior

## Blocked by

None — generators map closed; agent CLI stubs exist.

## Comments

Seeded by swarm 2026-08-01 after empty frontier post generators + clippy hardening.
