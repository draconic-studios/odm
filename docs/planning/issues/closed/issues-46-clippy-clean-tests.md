---
id: issues-46
title: "Clippy clean test code"
description: "Fix workspace clippy -D warnings failures (field_reassign_with_default, useless_format in tests)."
status: closed
issue-type: bug
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
---

# Clippy clean test code

## Description

`cargo clippy --workspace --all-targets -- -D warnings` fails on odm-core test code: `field_reassign_with_default` and `useless_format`. Hardening debt — not product behavior, but blocks a clean lint gate.

## Affected

- `crates/odm-core` tests (config, gitignore, pin, and any other sites clippy reports)

## Impact

Agents and humans cannot treat clippy-deny as a green gate; noise hides real issues.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** chore / fix  
**Summary:** Make `cargo clippy --workspace --all-targets -- -D warnings` pass with no code behavior change outside tests (or trivial test-only cleanups).

**Bindings:**

- Repo standards: TDD not required for pure lint fix; keep diffs minimal
- Do not weaken workspace clippy config; fix call sites

**Current behavior:**

- Clippy fails with multiple `field_reassign_with_default` on `WorkspaceConfig::default()` then field assigns in tests
- `useless_format` in `pin.rs` tests (format of a string literal)

**Desired behavior:**

- Prefer `WorkspaceConfig { field: …, ..Default::default() }` (or struct update) instead of mut reassign after default
- Replace useless `format!("literal")` with `"literal".to_string()` or plain `&str` as appropriate
- Fix **all** current clippy `-D warnings` errors under workspace all-targets (re-run until clean)
- No production logic changes unless a lint forces an equivalent rewrite; no new features

**Acceptance criteria:**

- [x] `cargo clippy --workspace --all-targets -- -D warnings` exits 0
- [x] `cargo test` still green
- [x] Diff limited to lint fixes (no drive-by refactors)

**Out of scope:**

- Enabling clippy in CI (repo has no CI)
- Pedantic clippy groups beyond what `-D warnings` already implies for default lints
- Generators / agent packs

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Fixed four clippy `-D warnings` in odm-core tests only:

- `config.rs` / `gitignore.rs`: `WorkspaceConfig { field, ..Default::default() }` instead of mut reassign after `default()`
- `pin.rs`: plain string literal instead of useless `format!`

`cargo clippy --workspace --all-targets -- -D warnings` and `cargo test` both green. No production logic changes.
