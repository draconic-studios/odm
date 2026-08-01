---
id: issues-58
title: "Agent prompt thin v1 (CLI over context)"
description: "Implement odm agent prompt <id> as thin Progen context work-package; keep agent start stub."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# Agent prompt thin v1 (CLI over context)

## Description

`odm agent prompt` is reserved as a thin wrap of progen prompt / agent work-package output (`env-gen-packs.md`, `cli.md`). There is no separate prompt engine in-tree — **v1** packages existing **context** for one note id to stdout for agents.

## Affected

- `crates/odm/src/cli.rs` — replace trailing-var `Prompt` with real args
- `crates/odm/src/main.rs` — wire handler; stop `not_implemented("agent prompt")`
- Possibly tiny helper next to context formatting in `odm-progen` (optional; may call `context_notes` + `format_context_human` from bin)
- Unit/CLI tests updated so prompt is no longer always exit 1
- Stub tests in `cli_agent_pack.rs` / `progen_vault.rs` that expect prompt not-implemented

## Impact

Agents lack a dedicated prompt entrypoint; only `odm context` exists.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Implement `odm agent prompt <id>` as context packaging. Leave `agent start` stubbed.

**Bindings:**

- `docs/reference/env-gen-packs.md` — agent prompt intent
- `docs/reference/cli.md` — sketch section (behavior becomes v1 thin)
- Existing: `context_notes`, `format_context_human`, `ContextHit` JSON via serde
- Exit codes: unknown note → `4`; usage/scope → `1`; workspace → `2`
- Parent map: [[issues-55-post-v1-hardening-map]]

**CLI shape:**

```text
odm agent prompt <id> [--progen <name>] [--json]
```

- Global `--progen` / `--json` / `--root` already on the binary — wire consistently with `odm context`.
- **Do not** require `--project` / `--wt` for v1 (prompt is Progen-scoped, not worktree-scoped).
- Drop trailing `rest: Vec<String>` on `Prompt` in favor of typed `id: String`.

**Behavior lock:**

1. Resolve note context exactly like `odm context <id>` (same multi-progen / `name:id` / single `--progen` rules already in `main.rs` for context). Prefer **sharing** the same code path rather than forking logic.
2. **Human:** markdown work package. Minimum: same content as `format_context_human` (anchor + outgoing + incoming). Optional header tweak `# agent prompt <id>` — either reuse context formatter or thin wrapper; do not invent task/plan engines.
3. **`--json`:** serialize the same `ContextHit` (or identical field set) as `odm context --json`.
4. Missing note → exit `4` with clear message.
5. `odm agent start` remains `not_implemented` exit `1`.
6. TDD: red→green; update tests that asserted prompt always not-implemented.
7. Full `cargo test` green.

**Acceptance criteria:**

- [x] `odm agent prompt <existing-id>` exit 0, stdout includes note id/body or context sections
- [x] `odm --json agent prompt <id>` valid JSON with anchor/outgoing/incoming (or equivalent ContextHit fields)
- [x] Unknown id → exit 4
- [x] `odm agent start` still exit 1 not-implemented
- [x] No new crate; no marketplace/start runtime
- [x] `cargo test` green

**Out of scope:**

- Full upstream `task prompt` / plan/log node types
- `agent start`
- Integration dogfood + reference doc promotion (ticket [[issues-59-agent-prompt-integration-docs]])
- Changing `odm context` behavior

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Implemented `odm agent prompt <id>` as a thin alias of `odm context`.

- **CLI:** `AgentCmd::Prompt { id: String }` (typed id; no trailing rest).
- **Handler:** shared `run_context_prompt` used by both `context` and `agent prompt` — same `one_progen_flag` / `context_notes` / `format_context_human` / `ContextHit` JSON path.
- **Human header:** reuses `format_context_human` (`# context <id>`).
- **`agent start`:** still `not_implemented` exit 1.
- **Tests:** `agent_prompt_is_thin_context_alias` (happy human/json, exit 4, start stub); stub-only tests no longer expect prompt not-implemented.
- Docs honesty deferred to [[issues-59-agent-prompt-integration-docs]].
