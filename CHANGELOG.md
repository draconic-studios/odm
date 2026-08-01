# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`odm generate`** — v1 local template materialize: list Generators from bundles; `generate <name> --dest <path> [--force]` copies a local template tree under the Workspace (remote/url-only run deferred with a clear error).
- **`odm project worktree`** — Worktree slot add/list/rm and `--wt` path binding (no longer a not-implemented stub).
- **`odm agent pack`** — v1 local install/link/list into an agent home (`--home`); Workspace registry `.odm/agent-packs.json`; `agent start` remains a not-implemented stub.
- **`odm agent prompt`** — v1 thin context work-package: packages one note’s Progen neighborhood to stdout (same path/JSON as `odm context`); `agent start` still stubbed.
- **`odm doctor` worktree orphan warn** — Warn checks `worktree_orphan:<project>:<slot>` for configured-project dirs under `worktrees/` that are not registered git worktrees (`fixable: false`; `--fix` does not delete).

### Changed

- `examples/core-desk` includes a sample Generator (`hello` → `templates/hello`) and a tiny demo agent pack (`agent-packs/demo`).
- Reference docs: `generate`, `agent pack`, and `agent prompt` documented as landed v1 (local / thin); remote/marketplace and `agent start` still deferred in `env-gen-packs.md`.

## [0.1.0] - 2026-08-01

First public spine release: core multi-git Workspace, Progen integration, and Actions behind one `odm` binary.

### Added

- **Core**
  - `odm init` — bootstrap a Workspace (`.odm/`, config, optional git)
  - `odm sync` / `odm pin` / `odm status` / `odm doctor` — multi-git lifecycle on plain clones (no submodules)
  - `odm project` lifecycle — add, list, rm, info, git
  - Workspace discovery, pin file, gitignore manage
- **Progen**
  - In-repo Obsidian-compatible vault engine under `odm progen …` (façade swap-ready for upstream crates later)
  - Lifecycle: add/list/rm/info; store: get/body/ls/tree/backlinks/reindex/doctor
  - Top-level `odm find` and `odm context` with multi-Progen scope (`--progen`, `--progen-group`)
- **Actions**
  - `odm run` — list and dispatch Action bundles (`tasks: [{ run, dir? }]`) from Workspace config
  - Shell-out model; cwd via task dir / `--project` / `--wt`; exit-code passthrough
- **CLI**
  - Globals: `--root`, `--json`, `--project`, `--wt`, `--progen`, `--progen-group`
  - Sketch stubs at release: `generate`, `agent`, `project worktree` (exit 1 not-implemented) — see [Unreleased] for generate + worktree landing after 0.1.0
- **Dogfood**
  - `examples/core-desk` — offline Workspace exercising core, path-only Progen, groups, and shell Actions
- **Ship**
  - `scripts/release-build.sh` — release tarball under `dist/odm-<version>-<target>.tar.gz`
  - Consumer install docs (README, `docs/reference/install.md`)
