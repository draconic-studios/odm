---
id: issues-128
title: "run --json includes captured stdout/stderr"
description: "With --json, action stdio is captured then discarded; envelope only has action+exitCode so agents cannot debug failures."
status: open
issue-type: bug
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# run --json includes captured stdout/stderr

## Description

`cli.md` says capture keeps envelope clean. Binary captures via `StdioMode::Capture` but JSON only emits `{ "action", "exitCode" }`. Captured streams are dropped.

## Affected

- `crates/odm/src/main.rs` run path
- `crates/odm/src/commands/run.rs` DTOs
- `crates/odm/tests/actions_run.rs`
- `docs/reference/cli.md`

## Impact

Agent automation cannot see failure output without a second non-JSON run.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Extend run JSON envelope with captured `stdout` and `stderr` (strings); keep `action` + `exitCode`; update docs + tests.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- `odm-actions` already fills `TaskResult.stdout`/`stderr` under Capture
- Minimum fields stay; streams are stable extras

**Desired behavior:**

1. `--json run <action>` → `{ "action", "exitCode", "stdout", "stderr" }` (always present; empty string when empty).
2. Multi-task: concatenate in run order with clear separators **or** last-task-only — pick **concatenate all captured task streams in order** (simple, debuggable). Document choice in cli.md one line.
3. Without `--json`, inherit behavior unchanged (no capture).
4. Human success/fail unchanged.
5. Tests: `run hello --json` has stdout containing hello marker; `run fail --json` has exitCode 7 and any stderr/stdout from the fail action.
6. Update `cli.md` wrapper minimum + stream fields.
7. CHANGELOG Unreleased one bullet optional (honesty preferred).

**Acceptance criteria:**

- [ ] JSON envelope includes stdout/stderr strings
- [ ] Capture still prevents interleaved pollution of the JSON object
- [ ] Integration tests assert stream content
- [ ] cli.md updated
- [ ] `cargo test -p odm` green

**Out of scope:** per-task JSON array (YAGNI unless already trivial); exit code changes.

## Acceptance

- [ ] Agent Brief acceptance criteria all met
