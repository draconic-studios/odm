---
id: issues-26
title: "Actions map"
description: "Wayfinder map: phase 4 — Action bundles from config and odm run dispatch."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Actions map

## Destination

Phase **4. Actions** per `docs/reference/phased-delivery.md`: Action bundles loadable from Workspace config pointers; `odm run` dispatches them; shell-out model for command bodies; enough support that the “one desk” story works without ad-hoc wrappers for common tasks.

## Notes

- **Domain:** root `CONTEXT.md`. Design package closed.
- **Authority:** `phased-delivery.md` (phase 4), `config.md` (actions/bundles), `cli.md` (`run`), `architecture.md`.
- **Execution override:** ticket resolution = decision recorded **and** code/tests/examples land.
- **Standing prefs (charted 2026-08-01, best-default autonomous):**
  - **Crate:** `crates/odm-actions` owns resolve cwd + shell-out dispatch; `odm-core` keeps bundle load into `Workspace.actions` (layout truth).
  - **Bundle shape (locked to config.md):** Action name → `{ tasks: [ { run, dir? } ] }`; multi-task sequential fail-fast; empty tasks → load error.
  - **Shell:** Unix `sh -c` with extra args as `"$@"` on the **last** task only (`sh -c 'run "$@"' _ extras…`).
  - **Cwd:** task `dir` (rel Workspace) if set; else Workspace root; `--project` overrides base to Project primary path; `--wt <slot>` → `worktrees/<project>/<slot>/` (requires `--project`; path must exist).
  - **CLI:** `odm run` list; `odm run <name> [--project] [--wt] [--json] [--] [extra…]`; never top-level action verbs; exit code = action’s when executed.
  - **JSON:** list `{ "actions": [ { "name", "tasks" } ] }`; run `{ "action", "exitCode" }` (action stdout/stderr inherit; not captured into JSON).
  - **Dogfood:** `examples/core-desk` gains `actions/core.yaml` + config pointer with offline shell actions (`hello`, `fail`, multi-task).
  - **Generators:** stay sketch — not pulled into this map.
  - **Proof:** unit tests in `odm-actions` + `odm-core` bundle shape + integration tests driving `odm run`.
- **Skills:** `/tdd` when landing code.

## Decisions so far

- [[issues-30-actions-slice-order-and-acceptance]] — slices 1–5 + phase gate checklist.
- [[issues-31-action-bundle-shape-and-run-semantics]] — tasks array, shell-out, cwd, exit codes.
- **2026-08-01 implement land:** `odm-actions` crate; `odm run` list/dispatch; core-desk actions; integration tests green. Phase gate checklist complete.

## Not yet specified

_(none for phase gate)_

## Out of scope

- HashiCorp go-plugin / npm plugin installers
- Generator / `template.toml` full depth
- Agent-pack and worktree productization beyond `--wt` path binding
- Output chaining / richer executors (copy/env/plugins)
- Top-level action verbs (legacy Go)

## Blocked by

- [[issues-25-progen-integration-map]] (closed)

## Comments

**2026-08-01 close:** Phase gate complete; map closed so Ship can proceed.
