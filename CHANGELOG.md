# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`odm generate`** — v1 local template materialize: list Generators from bundles; `generate <name> --dest <path> [--force]` copies a local template tree under the Workspace (remote/url-only run deferred with a clear error).
- **`odm project worktree`** — Worktree slot add/list/rm and `--wt` path binding (no longer a not-implemented stub).
- **`odm agent pack`** — v1 local install/link/list/rm into an agent home (`--home`); Workspace registry `.odm/agent-packs.json`; `rm` drops registry entry and best-effort deletes dest (missing dest still OK; unknown name → exit `4`); `agent start` remains a not-implemented stub.
- **`odm agent prompt`** — v1 thin context work-package: packages one note’s Progen neighborhood to stdout (same path/JSON as `odm context`); `agent start` still stubbed.
- **`odm doctor` worktree orphan warn** — Warn checks `worktree_orphan:<project>:<slot>` for configured-project dirs under `worktrees/` that are not registered git worktrees (`fixable: false`; `--fix` does not delete).
- **`odm doctor` worktree dirty-slot warn** — Warn checks `worktree_dirty:<project>:<slot>` for registered worktree slots with a dirty working tree (`fixable: false`; `--fix` does not clean or stash).
- **`odm doctor` pack missing-path warn** — Warn checks `pack_missing:<name>` when a registered agent pack path has no path/symlink on disk (`fixable: false`; `--fix` does not edit registry or delete pack paths).
- **`odm project worktree prune`** — remove orphan slot dirs under `worktrees/<project>/` (same orphan definition as doctor). Default deletes empty orphans only (exit `3` if any non-empty orphan remains after partial cleanup); `--force` recursive-deletes orphans. Never deletes registered worktrees. Doctor `--fix` still does not delete orphans.
- **`odm project worktree prune --all`** — prune orphans across every configured Project (same empty/`--force` rules); skips missing primary / non-git projects; exit `3` on any skipped non-empty without `--force`.
- **`odm find --limit`** — max hits per Progen store (default **200**); `0` is rejected with usage exit `1`.
- **`odm status` worktree slots** — each project includes registered `worktree_slots` (`name` + `path` + `dirty`); human output lists slot names when non-empty (dirty slots get a ` dirty` suffix).
- **`odm status` / `project info` worktree orphans** — each project includes `worktree_orphans` (`name` + `path`; same orphan definition as doctor/prune); human `orphans: …` when non-empty; empty array when none / soft-fail. Observation only — doctor warn + `worktree prune` remain cleanup.
- **`odm status` agent packs** — top-level `agent_packs` (`name` + `source` + `path` + `mode` + `missing`) from the Workspace registry; human **Agent packs:** section when non-empty (` missing` suffix when path absent). Empty array when none / missing registry / soft-fail. Doctor still owns `pack_missing` warn.
- **`odm project info` worktree slots** — registered `worktree_slots` (`name` + `path` + `dirty`), same shape as status; empty array when none / non-git / soft-fail; human `worktrees: …` when non-empty.
- **Worktree slot dirty observation** — `worktree list`, `status`, and `project info` probe registered slot cleanliness via `git status` (`dirty`: `true` / `false` / `null` on probe error). Human list marks dirty slots with a ` dirty` suffix.

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
