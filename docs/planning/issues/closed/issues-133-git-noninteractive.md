---
id: issues-133
title: "Git runner non-interactive (no auth hang)"
description: "ProcessRunner can hang on interactive git auth; agents need fail-fast."
status: closed
issue-type: bug
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# Git runner non-interactive (no auth hang)

## Description

`odm-git` `ProcessRunner` does not set `GIT_TERMINAL_PROMPT=0` (or equivalent). clone/fetch against auth-required remotes can block forever with inherited stdin.

## Affected

- `crates/odm-git/src/runner.rs`
- All materialize/sync/fetch paths

## Impact

Agent/CI sessions hang instead of failing fast.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Set non-interactive git env on captured/process runner ops used by ODM lifecycle.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]

**Desired behavior:**

1. For runner invocations used by clone/fetch/rev-parse/etc., set `GIT_TERMINAL_PROMPT=0`.
2. Optionally set `GIT_ASKPASS` to a failing helper or `echo` only if needed — prefer minimal env change first.
3. `Git::run` passthrough for `project git` may keep inherit stdin (user-facing); document if lifecycle ops differ.
4. Test: assert env is set via mock runner recording env, **or** unit-test helper that builds Command — do not require network.
5. No behavior change for already-working local file:// fixtures.

**Acceptance criteria:**

- [x] Lifecycle git ops set non-interactive env
- [x] Test proves env (mock) or documents contract
- [x] `cargo test -p odm-git` green

**Out of scope:** credential helpers productization; SSH agent config.

## Acceptance

- [x] Agent Brief acceptance criteria all met
