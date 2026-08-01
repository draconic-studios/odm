---
id: issues-39
title: "Action run result and stdio modes"
description: "Structured Action RunResult with capture vs inherit so run --json is honest; cwd already from core paths."
status: closed
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - architecture
  - deepen
---

# Action run result and stdio modes

## Description

Actions is mostly a shell-out pass-through; the only real depth is cwd priority. Inherit-only stdio collides with `odm run --json`, forcing tests to scrape the last `{` from mixed stdout. Deepen a small run **interface** with explicit stdio mode and structured result — after path policy owns worktree/primary paths.

Domain: Action, Workspace, Project, Worktree slot.  
Architecture: thin **adapter** with one honest **interface**; don't gold-plate an Action runtime.

## Affected

- `odm-actions` run/list
- CLI `odm run` / `odm run --list`
- actions integration tests (`json_from_stdout_mixed`)

## Impact

Machine-readable `run --json` is unreliable when tasks print; tests encode the scrape hack.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-35-workspace-path-policy]] — cwd must use core Worktree slot / Primary checkout helpers before further actions API work

## Agent Brief

**Category:** enhancement  
**Summary:** Give Action execution a structured result and explicit stdio mode so the CLI can emit clean JSON without scraping task output; keep shell-out semantics and cwd priority.

**Current behavior:**
- `list_actions` is a shallow pass-through over workspace action map.
- `run_action` resolves cwd (task dir > worktree slot > project primary > root), runs each task via `sh -c`, stops on non-zero, returns only exit code.
- Extra args attach only to the last task (keep this rule).
- Stdio is always inherited; CLI JSON for run shares the same stdout as task output.
- Integration tests parse trailing JSON objects out of mixed stdout.

**Desired behavior:**
- Run API accepts a stdio mode: **Inherit** (default interactive/human) and **Capture** (or equivalent) for machine-readable CLI.
- Structured `RunResult`: overall code, per-task code, and when capturing, stdout/stderr bytes or strings per task (or aggregated — pick one and document).
- `odm run --json` uses capture (or separates CLI JSON from task streams) so the JSON document is well-formed and complete on stdout without requiring consumers to scrape.
- Human `odm run` without `--json` keeps inherit behavior (tasks still stream to the terminal).
- Cwd resolution uses Workspace path policy from core (no local `worktrees/` string assembly).
- Cwd target should not overload one parameter as “project name iff --wt else rel path”; prefer an explicit target enum or distinct fields so callers cannot confuse name vs path.
- `list_actions` may stay thin or return a small DTO for run list JSON — either is fine if CLI list JSON stays stable.
- Still no env injection framework, nx bridge, or dry-run unless already present.

**Key interfaces:**
- Action bundle task shape already validated in core (command, optional dir)
- Cwd priority order unchanged
- Exit code: action non-zero propagates; missing action / config errors keep existing CLI exit spine
- Run list JSON shape currently emitted by CLI — keep fields stable if present

**Acceptance criteria:**
- [ ] `odm run --json <action>` emits a single well-formed JSON object on stdout without interleaving raw task stdout into that object
- [ ] `odm run <action>` without `--json` still streams task stdio to the terminal
- [ ] Stop-on-first-failure and last-task-extra-args semantics preserved
- [ ] Cwd with `--project` / `--wt` matches prior path priority and uses core path helpers
- [ ] Integration tests no longer need trailing-`{` scrape helpers for run JSON
- [ ] Unit tests cover cwd target clarity and capture vs inherit result shape
- [ ] `cargo test` and `cargo clippy -- -D warnings` clean for touched crates

**Out of scope:**
- Parallel task execution
- Built-in env var injection / secrets
- Replacing shell-out with an in-process runner
- Expanding action bundle schema beyond what's needed for stdio/result
- Generator runtime

## Answer

Shipped structured Action run API:

- **`StdioMode`**: `Inherit` (human) vs `Capture` (machine)
- **`CwdTarget`**: `Root` | `Project { name }` | `Worktree { project, slot }` via `from_flags` — no overloaded name-vs-path param
- **`RunResult` / `TaskResult`**: overall + per-task exit codes; stdout/stderr only under Capture (UTF-8 lossy)
- **CLI**: `odm run --json` uses Capture and prints only `{ action, exitCode }`; human run inherits terminal stdio
- **Cwd**: still task dir > worktree slot > project primary > root via core `abs_checkout` / `worktree_slot_path`
- Integration scrape helper removed; unit tests cover capture/inherit shape and cwd target clarity

## Comments

From architecture review 2026-08-01 (candidate #6, Speculative).
