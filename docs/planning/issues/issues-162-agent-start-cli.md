---
id: issues-162
title: "CLI odm agent start"
description: "Wire odm agent start --project/--wt + argv over start lib; human inherit + --json; exit passthrough; integration tests."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ready-for-agent
---

# CLI odm agent start

## Description

Replace `AgentCmd::Start` not-implemented stub with real CLI over the start lib. Global `--project` / `--wt` / `--json`; trailing program argv; exit-code passthrough like `odm run`.

## Affected

- `crates/odm/src/cli.rs` — typed Start argv (drop pure stub)
- `crates/odm/src/main.rs` — dispatch; stop `not_implemented("agent start")`
- `crates/odm/src/commands/` — start handler + DTO (new or beside run)
- Tests: `cli_exit_code_matrix.rs` (`agent_start_not_implemented` must change), `cli_agent_pack.rs` / `progen_vault.rs` stub expects, new `cli_agent_start.rs` (or extend existing)
- File size limits apply

## Impact

Lib-only start is invisible to users/agents.

## Proposed Fix

See Agent Brief.

## Blocked by

None (unblocked — [[issues-161-agent-start-lib]] closed)

## Agent Brief

**Category:** feat  
**Summary:** Thin CLI adapter for `odm agent start` over start lib. TDD.

**Bindings:**

- Parent map: [[issues-158-agent-start-map]] Decisions
- Lib API from [[issues-161-agent-start-lib]]
- Patterns: `commands/run.rs` (`finish_run`, inherit vs JSON capture, exit passthrough)
- Global flags already on binary: `--project`, `--wt`, `--json`, `--root`

**CLI shape (v1 lock):**

```text
odm --project <name> [--wt <slot>] [--json] agent start -- <program> [args…]
odm --project <name> agent start <program> [args…]
```

- **`--project` required** (missing → usage exit `1`). Prefer global flag; do not invent a second project flag unless clap forces it.
- **`--wt` optional**; requires project (existing global resolve); missing slot dir → exit `4`.
- **argv:** at least one token (program); remainder are args. Empty → usage `1`.
- Prefer requiring `--` only when args look like flags; trailing_var_arg + allow_hyphen_values OK (match `project git` / `run` spirit).

**Behavior:**

1. Resolve workspace via existing `Ctx`.
2. Call start lib with project/wt/program/args; stdio Capture when `--json`, else Inherit.
3. **Human:** inherit stdio; process exit = child exit (no required success banner).
4. **`--json`:** print one object then exit with child code:
   - `{ "cwd", "program", "args", "exitCode", "stdout", "stderr" }`
   - field names stable; `exitCode` camelCase like `ActionRunDto`
5. Pre-exec ODM errors use normal envelope / exit spine (`1`/`2`/`3`/`4`) — do **not** use not-implemented.
6. **Update tests** that expect `agent start` always exit 1 not-implemented (matrix case, pack/prompt stub tests, progen_vault). Add integration coverage: happy `true`, fail `false` passthrough, missing project, missing wt, `--json` shape.
7. Docs/dogfood honesty deferred to [[issues-163-agent-start-docs-dogfood]] — but **do not leave cargo test red** (fix stub asserts here).

**Acceptance criteria:**

- [ ] `odm --project <p> agent start -- true` exit 0 in a temp workspace with that project on disk
- [ ] Child non-zero exit passthrough
- [ ] Missing `--project` → exit 1
- [ ] Missing wt slot → exit 4
- [ ] `--json` valid object with cwd/program/args/exitCode
- [ ] No remaining `not_implemented("agent start")` success path
- [ ] Exit matrix + stub tests updated; `cargo test` green
- [ ] No pack/prompt/runtime-matrix scope creep

**Out of scope:**

- Reference docs / README / website / dogfood script expect-success flip ([[issues-163-agent-start-docs-dogfood]])
- Changing pack or prompt behavior
- Default agent binary / config-declared runtimes

## Comments

Minted from [[issues-158-agent-start-map]] 2026-08-02.

- 2026-08-02: Unblocked — [[issues-161-agent-start-lib]] closed; tagged `ready-for-agent`.
- On close: tag [[issues-163-agent-start-docs-dogfood]] with `ready-for-agent`.
