---
id: issues-1
title: "ODM design docs map"
description: "Wayfinder map: coherent reviewable design package under docs/ for the Rust ODM redesign (no implementation)."
status: open
tags:
  - planning
  - issue
  - wayfinder-map
---

# ODM design docs map

## Destination

A coherent, reviewable design package under `docs/` (plus root `CONTEXT.md`) that defines the Rust ODM redesign — vision, concepts, architecture, config, CLI, multi-progen model, multi-git, migration — agreed enough that implementation can start without reopening fundamentals. **No production Rust rewrite and no Go deletion in this map** — documentation only.

## Notes

- **Domain:** ODM = Orchestrated Development Management; poly-repo workspace OS for humans + AI agents.
- **Language:** Use **progen** (noun) for a docs/memory store — never "brain". CLI façade is `odm progen …`.
- **Skills every session:** `/grilling`, `/domain-modeling`, `/research` (when cited), `/obsidian-vault` for docs placement. Prefer `/grill-with-docs` patterns: lock terms into `CONTEXT.md`, hard trade-offs into `docs/adr/`.
- **Execution override:** This map's destination *is* the docs package. Resolving a ticket means the decision is recorded **and** the relevant `docs/reference/*` (or ADR/CONTEXT) is written or updated — not abstract answers only.
- **Doc tree (agreed):**
  ```text
  CONTEXT.md
  docs/reference/
    vision.md
    architecture.md
    concepts.md          # may fold into CONTEXT.md if redundant
    config.md
    cli.md
    progen.md
    multi-git.md
    worktrees.md         # sketch
    graph.md             # sketch
    env-gen-packs.md     # sketch
    migration.md
    phased-delivery.md
  docs/adr/              # sparingly
  ```
- **Specify fully:** workspace + config + multi-git (no submodules) + progen multi-store + CLI surface + doc conventions.
- **Sketch only:** worktrees/agent slots, code/doc graph + tags, env, generators, agent packs.
- **Standing prefs from charting:**
  - This repo becomes the Rust monorepo home later; Go `src/` is disposable inspiration.
  - `.odm/` = ODM state + config only; does **not** own managed checkouts/progens; core parts are **tracked**; ephemeral index/worktrees/cache gitignored.
  - Progens live at any path (`docs/…`, `documentation/…`, `notes/…`); config is source of truth for layout.
  - Query scope: default all progens; `--progen` one or many; named combinations only in ODM config; **no cross-store links inside a progen store**.
  - Workspace git optional; pin file (`odm.lock.yaml`) opt-in.
  - Distribute static binary; `odm init` bootstraps a consumer workspace (ODM product repo ≠ user workspace).
  - Integrate progen as crates (one binary); ODM owns UX; progen owns store/index/context internals.
- **Source draft:** user-supplied ODM Design (session); baseline refs: progenitor, life-engine docs, this repo's Go legacy.
- **Refer by ticket name** (wikilink), never bare ids alone.

## Decisions so far

- [[issues-3-research-progenitor-surface]] — progen stack is single-root MD+frontmatter+SQLite FTS; no multi-root; ODM must orchestrate federation. Notes: `docs/reference/research/progenitor-surface.md` (branch `research/progenitor-surface`).
- [[issues-4-research-legacy-go-odm]] — Go ODM is submodule+actions+partial plugins; replace config/VCS lifecycle; drop go-plugin path. Notes: `docs/reference/research/legacy-go-odm.md` (branch `research/legacy-go-odm`).


## Not yet specified

- Exact generator `template.toml` / Nx shell integration details
- tree-sitter vs graphify as code ingest
- Agent-pack link default on Windows (symlink vs copy)
- Whether multi-root federation is upstreamed into progen crates later
- Concrete frontmatter/kind parity with life-engine vs progen today
- Packaging channels beyond GitHub Releases (brew, etc.)
- How much of standalone `progen` CLI UX is re-exported vs fully redesigned under `odm progen`
- Worktree branch naming templates and `odm agent start` flow (sketch depth TBD)
- Optional reader for legacy Go `odm.config.yaml` actions shape

## Out of scope

- Implementing the Rust workspace / shipping binaries (later implement map)
- Deep Serve/MCP design (`odm serve`) — one-line non-goal in architecture
- Replacing pnpm/Nx inside consumer monorepos
- Git submodules as a supported model
- Calling the memory store a "brain" in product language

## Comments

Charted from wayfinder session: destination = docs package; scope tiers and standing prefs captured above.

Go `src/` + build `scripts/` removed outside the map (user request); recovery via tag `legacy-go-archive`. Research note still authoritative for migration docs.
