---
id: issues-1
title: "ODM design docs map"
description: "Wayfinder map: coherent reviewable design package under docs/ for the Rust ODM redesign (no implementation)."
status: closed
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
    phased-delivery.md   # greenfield + phase spine; no migration.md
  docs/adr/              # sparingly
  ```
- **Specify fully:** workspace + config + multi-git (no submodules) + progen multi-store + CLI surface + doc conventions.
- **Sketch only:** worktrees/agent slots, code/doc graph + tags, env, generators, agent packs.
- **Standing prefs from charting:**
  - This repo becomes the Rust monorepo home later; Go `src/` is disposable inspiration.
  - `.odm/` = ODM state + config only; does **not** own managed checkouts/progens; core parts are **tracked**; ephemeral index/worktrees/cache gitignored.
  - Progens live at any path (`docs/…`, `documentation/…`, `notes/…`); config is source of truth for layout.
  - Query scope: default all progens; `--progen` one or many; named combinations only in ODM config; **no cross-store links inside a progen store**.
  - Workspace git optional (`odm init` git-inits by default); pin file (`.odm/odm.lock.yaml`) auto when Workspace is git; config under `.odm/`.
  - Distribute static binary; `odm init` bootstraps a consumer workspace (ODM product repo ≠ user workspace).
  - Integrate progen as crates (one binary); ODM owns UX; progen owns store/index/context internals.
- **Source draft:** user-supplied ODM Design (session); baseline refs: progenitor, life-engine docs, this repo's Go legacy.
- **Refer by ticket name** (wikilink), never bare ids alone.

## Decisions so far

- [[issues-3-research-progenitor-surface]] — progen stack is single-root MD+frontmatter+SQLite FTS; no multi-root; ODM must orchestrate federation. Notes: `docs/reference/research/progenitor-surface.md` (branch `research/progenitor-surface`).
- [[issues-4-research-legacy-go-odm]] — Go ODM is submodule+actions+partial plugins; replace config/VCS lifecycle; drop go-plugin path. Notes: `docs/reference/research/legacy-go-odm.md` (branch `research/legacy-go-odm`).
- [[issues-2-domain-glossary]] — ubiquitous language in root `CONTEXT.md`: Workspace, Project, Progen, Progen group, ODM state directory, Primary checkout, Worktree slot, Agent pack, Workspace config, Pin file, Action; never “brain”.
- [[issues-5-config-schema-spine]] — v1 Workspace config in `docs/reference/config.md`: maps by name; `progen_groups`; `actions`/`generators` as bundle file pointers; no layout templates; pin file by basename. *(path amended by Multi-git: config under `.odm/`)*
- [[issues-6-progen-scope-and-federation]] — federation in `docs/reference/progen.md`: default all Progens; `--progen` / `--progen-group` union; single-Progen writes; no cross-store wikilinks; external MD links OK; ODM orchestrates single-root ops.
- [[issues-7-multi-git-and-pins]] — plain clones in `docs/reference/multi-git.md`; config/pin under `.odm/`; sync=fetch-only; opt-in auto pin; no submodules.
- [[issues-8-odm-dot-directory-contract]] — `.odm/` layout + discovery in `docs/reference/architecture.md`; worktrees outside `.odm/`; config/pin tracked; caches ignored.
- [[issues-9-cli-surface-v1]] — command tree + globals in `docs/reference/cli.md`: `init`, `sync`/`pin`/`status`/`doctor`, `project` (+ git passthrough; worktree sketch), `progen` lifecycle+façade, top-level `find`/`context`, `run`, sketch `generate`/`agent`; exit 0–4; no serve/MCP.
- [[issues-10-vision-and-architecture-narrative]] — product one-liner, jobs, non-goals, ownership, system narrative, crate intent in `docs/reference/vision.md` + `docs/reference/architecture.md`.
- [[issues-11-migration-and-repo-home]] — no `migration.md`; greenfield Rust-first in this repo; phase spine + ship intent in `docs/reference/phased-delivery.md`.
- [[issues-12-sketch-sections-depth]] — sketch bar + `worktrees.md` / `graph.md` / `env-gen-packs.md`; serve/MCP and sketch absences explicit; not Ship gates.
- [[issues-13-design-package-acceptance]] — acceptance checklist (files, depth, no conflicts, open-question register, ready = later Implement core map only); green run closes this map. Recorded on the issue + `phased-delivery.md` Design package Done-means.

## Not yet specified

- Exact generator `template.toml` / Nx shell integration details
- tree-sitter vs graphify as code ingest
- Agent-pack link default on Windows (symlink vs copy)
- Whether multi-root federation is upstreamed into progen crates later
- Concrete frontmatter/kind parity with life-engine vs progen today
- Packaging channels beyond GitHub Releases (brew, etc.)
- Worktree branch naming templates and full `odm agent start` flow (beyond sketch)
- Full flag tables for every re-exported `odm progen` store verb (implement against progenitor + scope rules)

## Out of scope

- Implementing the Rust workspace / shipping binaries (later implement map)
- Deep Serve/MCP design (`odm serve`) — one-line non-goal in architecture
- Replacing pnpm/Nx inside consumer monorepos
- Git submodules as a supported model
- Calling the memory store a "brain" in product language

## Comments

Charted from wayfinder session: destination = docs package; scope tiers and standing prefs captured above.

Go `src/` + build `scripts/` removed outside the map (user request); recovery via tag `legacy-go-archive`. Research note still authoritative for migration docs.
