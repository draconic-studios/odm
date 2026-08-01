---
id: issues-14
title: "Implement core map"
description: "Wayfinder map: working Rust ODM core (phase 2) plus examples/core-desk dogfood Workspace."
status: open
tags:
  - planning
  - issue
  - wayfinder-map
---

# Implement core map

## Destination

Working **Implement core** in this repo per `docs/reference/phased-delivery.md` phase 2: Cargo workspace with real `odm` bin + `odm-core` + `odm-git`, Workspace discovery / `.odm/` / config / pin basics, multi-git lifecycle (plain clones; shell-out `git`), CLI spine (`init`, `sync` / `pin` / `status` / `doctor`, `project` lifecycle, globals + exit codes), proven by unit tests and an integration harness against dogfood Workspace **`examples/core-desk`** (local bare fixtures). Not progen façade, actions, or Ship.

## Notes

- **Domain:** ODM = Orchestrated Development Management. Language: root `CONTEXT.md`. Design package is closed — [[issues-1-odm-design-docs-map]]; do **not** reopen fundamentals without an explicit decision change.
- **Authority docs:** `phased-delivery.md` (phase 2), `architecture.md`, `config.md`, `cli.md`, `multi-git.md`.
- **Skills every session:** `/grilling`, `/domain-modeling`, `/tdd` when landing code; `/prototype` for example layout; `/research` only when cited.
- **Execution override:** Resolving a ticket means the decision is recorded **and** the relevant code, tests, and/or `examples/core-desk` artifacts land (vertical slice as scoped by the ticket) — not abstract answers only.
- **Standing prefs from charting:**
  - **Git:** shell out to `git` on PATH via `odm-git` (no libgit2 day one).
  - **Config/pin I/O:** `serde` + YAML (`serde_yaml` / current maintained crate); **deny unknown fields**.
  - **Crates day one:** `crates/odm` (bin), `crates/odm-core`, `crates/odm-git` only — no empty progen/actions crates.
  - **CLI:** `clap` derive in the bin; handlers call core/git.
  - **Core command cut:** `init`, `sync`, `pin`, `status`, `doctor`, `project` list/add/rm/info/git. **No** `progen` store façade, `find`/`context`, `run`, sketch verbs.
  - **Config load:** full v1 Workspace schema (including `progens` / groups / actions / generators maps) validated on load; non-core commands absent.
  - **Example:** `examples/core-desk` — minimal consumer Workspace; managed checkouts from **local bare fixtures** (offline-safe).
  - **Proof:** integration harness drives the `odm` binary against a temp copy of core-desk + fixtures; plus unit tests for pure logic.
  - **Output:** human text stdout / diags stderr; `--json` where `cli.md` requires; no `tracing` required day one.
  - **Toolchain:** stable + edition 2021; modest MSRV only if pinned later — not a design reopen.
- **Refer by ticket name** (wikilink), never bare ids alone.
- **Prior map:** design package closed; open questions that remain design-fog stay on that map’s Not yet specified unless this map pulls them in.

## Decisions so far

- [[issues-15-vertical-slice-order-and-core-acceptance]] — Vertical slices 1–8 (skeleton → init → git/add → pin → status/gitignore → doctor → project rest → core-desk+harness); stubs exit 1; map-close checklist in ticket + `phased-delivery.md` phase 2.
- [[issues-16-odm-git-shell-contract]] — `odm-git`: `Git` + runner; absolute paths + `-C`; ops clone/fetch/init/head/clean/origin/detach/run; typed errors; capture stderr on fail; policy in core.

## Not yet specified

- Exact clap module layout inside the bin crate
- Atomic write strategy for config/pin (temp + rename vs in-place)
- Windows / `file://` fixture path quirks for bare repos
- Exact MSRV number (if any) and `rust-toolchain.toml` channel pin policy
- Config/pin file locking under concurrent CLI invocations
- Whether `doctor --fix` ever touches pin contents vs gitignore/config only
- CI: repo policy is no GitHub Actions — harness is local/`cargo test` only unless policy changes

## Out of scope

- Progen façade / federation / `find` / `context` (later Progen integration map)
- Actions / `odm run` / generators depth
- Public ODM v1 GitHub Release (Ship phase); dogfood builds OK
- Sketch productization: worktrees, agent packs, graph, env
- `serve` / MCP
- Git submodules; libgit2 as primary backend
- Reopening design fundamentals without explicit decision change
- Treating the ODM product repository as a consumer Workspace

## Comments

Charted from wayfinder session after design package close. Destination = phase-2 working core + `examples/core-desk`.
