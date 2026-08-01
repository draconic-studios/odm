---
id: issues-64
title: "odm find --limit flag"
description: "Expose find hit limit (default 200) as --limit instead of hardcoding in main."
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

# odm find --limit flag

## Description

`odm find` always passes `200` into `find_notes_dto` / `find_notes`. Callers cannot raise or lower the cap. Core already takes `limit_per: usize`.

## Affected

- `crates/odm/src/cli.rs` — `Commands::Find`
- `crates/odm/src/main.rs` — find dispatch
- Tests: unit and/or `crates/odm/tests/progen_vault.rs` (or adjacent find coverage)
- `docs/reference/cli.md` find section — document `--limit`

## Impact

Large vaults silently truncate; agents cannot request more/fewer hits.

## Proposed Fix

See Agent Brief.

## Blocked by

None (docs ticket 62 may race; this ticket owns the flag docs once implemented)

## Agent Brief

**Category:** feat  
**Summary:** Add `odm find --limit <n>` with default **200**, wired to existing `find_notes` limit parameter.

**Bindings:**

- `odm_progen::find_notes` / `find_notes_dto` limit args
- Exit codes: bad limit → usage exit `1`
- Parent map: [[issues-60-post-v1-polish-map]]

**Behavior lock:**

1. Clap: optional `--limit <u32 or usize>` on **find only** (not global). Default **200** when omitted (same as today).
2. Reject `0` (and negative is impossible for unsigned) with clear usage error exit `1`.
3. Pass limit through to existing find path unchanged otherwise (scope flags, JSON shape, empty query).
4. Semantics: same as today’s `limit_per` / per-call limit already used by `find_notes` — **do not** invent a new global merge cap unless code already has one; keep one parameter end-to-end.
5. TDD: test default still 200 behavior via existing tests; add test that small `--limit` caps results (temp multi-note vault or unit on dto if easier); test `--limit 0` → exit 1.
6. Update `cli.md` find synopsis + bullet for `--limit` (default 200).
7. `cargo test` green.

**Acceptance criteria:**

- [x] `odm find` without `--limit` behaves as before (default 200)
- [x] `odm find --limit N` respects N for N≥1
- [x] `--limit 0` → exit 1 usage
- [x] cli.md documents `--limit`
- [x] `cargo test` green

**Out of scope:**

- Facet flags
- context `--depth`
- Changing FindHit JSON field names
- FTS ranking changes

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Shipped `odm find --limit <n>` (clap default 200, find-only). `main` rejects `--limit 0` via `OdmError::usage` (exit 1) and passes `limit` into existing `find_notes_dto` → per-store `limit_per`. Tests: `find_limit_zero_is_usage`, `find_limit_caps_hits` in `progen_vault.rs`. `cli.md` documents synopsis + per-store semantics.
