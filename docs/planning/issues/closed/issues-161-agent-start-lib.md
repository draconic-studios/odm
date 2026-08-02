---
id: issues-161
title: "Agent start lib (cwd + exec)"
description: "Library API: resolve Project/wt cwd and one-shot exec argv for odm agent start; unit tests with true/false."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# Agent start lib (cwd + exec)

## Description

`odm agent start` is a not-implemented stub. Need a pure library API that resolves cwd (Project Primary or worktree slot) and one-shot execs a user-supplied program argv — no CLI wiring in this ticket.

## Affected

- `crates/odm-actions` (preferred: reuse `CwdTarget` / `resolve_cwd` / `StdioMode`) — or thin module if a cleaner seam appears; no new crate
- Unit tests with tempdirs + `true`/`false`/`echo`
- File size target ≤1000 / hard 1250

## Impact

Without lib, CLI cannot leave the stub honestly.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Add testable start exec API used by the CLI. No clap/main wiring.

**Bindings:**

- Parent map: [[issues-158-agent-start-map]] Decisions (v1 one-shot shell-out)
- `docs/reference/cli.md` / `env-gen-packs.md` start intent
- Reuse `CwdTarget`, `resolve_cwd`, `StdioMode` from `odm-actions` where possible
- Exit spine: unknown project → usage; missing project path / missing wt dir → not_found (4); spawn fail → operation (3)

**Map Decisions (v1 lock — do not reopen):**

- One-shot **direct exec** (`Command::new(program)` + args), not `sh -c`, not a session/daemon/MCP
- **`--project` required** at the API boundary (reject root-only cwd for start)
- Optional wt slot via existing worktree path rules (no auto-create)
- **Independent of packs and prompt** — do not read pack registry or call context/prompt
- No runtime detection matrix; no default agent binary; caller supplies program + args
- Inherit vs capture stdio modes (mirror `run` for JSON later)

**Desired behavior:**

1. **Public API** (names flexible if clear), roughly:
   - `start_agent(ws, opts) -> Result<StartResult, OdmError>`
   - `StartOptions { project, wt: Option, program, args, stdio }`
   - `StartResult { exit_code, cwd, stdout: Option, stderr: Option }`
2. Resolve cwd: Project Primary when `wt` is None; `worktrees/<project>/<slot>/` when set. Missing project name → usage. Missing on-disk primary/slot → not_found.
3. Exec `program` with `args` in that cwd. Empty program → usage.
4. **Inherit:** child stdio inherited; streams None. **Capture:** fill stdout/stderr strings.
5. Return child exit code (signal → treat as 1 like actions).
6. **Unit tests** (temp workspace with a project path dir; optional fake wt dir):
   - happy `true` → exit 0
   - `false` → non-zero passthrough
   - capture echoes stdout
   - unknown project / missing path / wt without project / empty program error paths
7. Do **not** wire CLI, docs, or dogfood.

**Acceptance criteria:**

- [x] Public start API callable without CLI
- [x] Cwd binds to project primary and wt slot correctly
- [x] Direct exec; exit code passthrough
- [x] Inherit + capture stdio modes
- [x] Unit tests cover happy + error paths
- [x] `cargo test` green
- [x] No CLI/docs/dogfood changes

**Out of scope:**

- CLI clap / JSON DTOs
- Pack auto-apply, prompt composition, env injection productization
- Runtime matrix / agent discovery
- serve/MCP/init

## Answer

Shipped `odm_actions::start_agent` + `StartOptions` / `StartResult` in `crates/odm-actions/src/start.rs`. Reuses `resolve_cwd` / `CwdTarget` / `StdioMode`; project required; direct `Command::new(program)` exec; inherit/capture; exit passthrough. 10 unit tests green. No CLI wiring.

## Comments

Minted from [[issues-158-agent-start-map]] 2026-08-02.

- On close: tag [[issues-162-agent-start-cli]] with `ready-for-agent` (it is blocked only by this ticket).
- 2026-08-02: Implemented and closed.
