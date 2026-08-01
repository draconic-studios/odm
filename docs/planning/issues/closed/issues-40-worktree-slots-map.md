---
id: issues-40
title: "Worktree slots map"
description: "Wayfinder map: implement Project worktree slot lifecycle and --wt resolution beyond path binding."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Worktree slots map

## Destination

Productize **Worktree slots** per `docs/reference/worktrees.md` and `cli.md`: named git worktrees at `worktrees/<project>/<slot>/` (not under `.odm/`); CLI `odm project worktree list|add|rm`; `--wt` resolves for `project git` (actions already bind cwd). Parallel human/agent trees without touching Primary checkout.

## Notes

- **Domain:** root `CONTEXT.md` (Worktree slot, Primary checkout, Project, Workspace).
- **Authority:** `worktrees.md` (sketch → implement), `cli.md` (reserved names), `architecture.md` (placement), `multi-git.md` (Primary stays pin-oriented).
- **Prereqs already landed:** `odm-core::paths::worktree_slot_path`; actions `--wt` cwd; gitignore `worktrees/`; CLI stubs exit 1 not-implemented.
- **Execution override:** ticket resolution = decision recorded **and** code/tests land. Prefer `/tdd`.
- **Standing prefs (seeded 2026-08-01, AFK best-default):**
  - **Git shell:** extend `odm-git` with worktree add/list/remove (still shell-out, injectable `CommandRunner`); no libgit2.
  - **Orchestration:** core owns slot name validation, path via `worktree_slot_path`, primary-must-be-git checks, list/add/rm APIs; CLI thin adapter + JSON.
  - **CLI:**
    - `odm project worktree list <project>`
    - `odm project worktree add <project> <slot> [--branch <b>]`
    - `odm project worktree rm <project> <slot> [--force]`
  - **add:** create git worktree at slot path from Project primary; optional `--branch` (see task tickets); fail if slot path exists or primary not a git repo; never touch Primary tree contents beyond `git worktree` metadata.
  - **rm:** `git worktree remove` on the slot; `--force` maps to git force; do **not** require deleting empty parent dirs beyond best-effort; `project rm` still does **not** delete `worktrees/<project>/`.
  - **list:** slots known under `worktrees/<project>/` that are linked worktrees (prefer `git worktree list` filtered to slot prefix, plus/or directory scan — pick one honest approach in implement ticket).
  - **`--wt`:** `project git` resolves cwd to slot path; must exist; no auto-create; unknown project / non-git / missing slot → usage or operation error per exit spine; pin auto-maintain stays **Primary-only** (slot HEAD changes do not rewrite pin).
  - **Slot name:** non-empty token; no `/`, `\`, `..`, or absolute form; align with “name not path”.
  - **JSON (minimal):** list `{ "project", "slots": [ { "name", "path" } ] }`; add/rm success human one-liner or `{ "project", "slot", "path" }` when `--json`.
  - **Proof:** unit tests (git runner fake + core) + integration tests driving real `git worktree` in temp repos.
- **Deferred (map out of scope):** config-declared slots, branch naming templates, GC/prune, pin↔slot binding, status/doctor orphan checks, multi-Project slots, generators/agent packs.

## Decisions so far

- Placement and ownership locked in design package (`worktrees.md`).
- Path helper already in core (`issues-35`).
- Slice tickets: [[issues-41-odm-git-worktree-ops]], [[issues-42-worktree-slot-lifecycle]], [[issues-43-project-git-wt-resolve]], [[issues-44-worktree-integration-and-docs]].
- **issues-41 closed:** `odm-git` exposes `worktree_add` / `worktree_list` / `worktree_remove` + `WorktreeEntry` (porcelain); fake-runner unit tests lock argv and parse.
- **issues-42 closed:** core `worktree_list`/`add`/`rm` + slot name validation; CLI `odm project worktree list|add|rm` with JSON DTOs; unit tests via fake git runner.
- **issues-43 closed:** `project_git` honors `--wt` (command or global) via slot path + validation; pin auto-maintain stays Primary-only; CLI no longer not-implemented.
- **issues-44 closed:** integration tests `crates/odm/tests/cli_worktree.rs` (add/list/git --wt/rm roundtrip, missing slot exit 4, non-git primary exit 3); `worktrees.md` retitled v1+deferred; `cli.md` marks `project worktree` full v1 and `--wt` path binding.
- **Map complete:** all child tickets closed; destination met for v1 (deferred items remain explicit in docs).

## Not yet specified

- Exact human list columns beyond name + path (v1: name only per line; path in JSON).
- Whether `list` without project name is ever supported (v1: **require** project name).

## Out of scope

- Agent pack install / `agent start`
- Generators
- Graph
- Worktree declarations in Workspace config
- Changing actions cwd priority (already correct)

## Blocked by

None — phases 1–5 closed; path policy and actions `--wt` landed.

## Answer

Worktree slots v1 delivered end-to-end:

1. `odm-git` worktree ops (41)
2. Core + CLI lifecycle list/add/rm (42)
3. `project git --wt` path binding (43)
4. Real-git integration tests + reference docs (44)

Deferred features stay documented in `worktrees.md`. Map closed with issues-44.

## Comments

Seeded by swarm 2026-08-01 after empty frontier post architecture-deepen.
