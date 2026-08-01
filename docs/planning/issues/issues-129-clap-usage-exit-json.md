---
id: issues-129
title: "Clap parse errors exit 1 and honor --json"
description: "Clap usage failures exit 2 with human stderr; docs say usage exit 1; --json ignored on parse errors."
status: open
issue-type: bug
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# Clap parse errors exit 1 and honor --json

## Description

Two usage worlds: clap parse → exit 2, never JSON; `OdmError::Usage` → exit 1 + JSON. Docs only describe library mapping. Tests sometimes lock clap exit 2 (`cli_worktree` prune mutual exclusion).

## Affected

- `crates/odm/src/main.rs` / `cli.rs`
- `docs/reference/cli.md` exit codes
- Integration tests that expect exit 2 for clap

## Impact

Agents cannot machine-parse flag mistakes; CI expecting exit 1 gets 2.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Prefer docs: map clap parse failures to exit 1; when `--json` is present on argv, emit standard error envelope on stdout.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Error envelope: same shape as `output::print_error` / `{ ok: false, error: { code, message, detail? } }`
- `code`: `"usage"`

**Desired behavior:**

1. Unknown command / bad flags → process exit **1** (not 2).
2. If `--json` appears anywhere before parse failure is reported, stdout is one JSON error object; human message may still go stderr or only in JSON message field — match existing error printer.
3. Successful parse path unchanged.
4. Update tests that assert clap exit 2 → exit 1.
5. Update cli.md if any residual ambiguity; lock “parse and library usage both exit 1”.
6. Implementation sketch: `Cli::try_parse()` in main, or clap error hook — keep it small.

**Acceptance criteria:**

- [ ] `odm notacommand` → exit 1
- [ ] `odm --json project worktree prune` (missing required) → exit 1 + JSON error when applicable
- [ ] Tests updated; no remaining “clap → 2” lock-in unless documented exception removed
- [ ] `cargo test -p odm` green

**Out of scope:** validating unused global flags on every command (YAGNI).

## Acceptance

- [ ] Agent Brief acceptance criteria all met
