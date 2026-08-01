---
id: issues-122
title: "find snippet panics on non-ASCII bodies"
description: "odm find can abort the process when building snippets around multi-byte UTF-8 matches."
status: open
issue-type: bug
severity: critical
tags:
  - planning
  - issue
  - ready-for-agent
---

# find snippet panics on non-ASCII bodies

## Description

`snippet` in `odm-progen` slices `body[start..end]` using byte offsets from `str::find` ± 40. Offsets need not be char boundaries; CJK/emoji vaults panic the binary.

## Affected

- `crates/odm-progen/src/store.rs` — `fn snippet`
- `odm find` / `ProgenStore::find`

## Observed

Match near multi-byte chars → `byte index is not a char boundary` panic.

## Impact

Process abort on normal multilingual Progen content — highest severity find bug.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Make `snippet` UTF-8 safe; add regression test with CJK/emoji around the match.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- Code: `crates/odm-progen/src/store.rs` ~315–332

**Desired behavior:**

1. Floor `start` / ceil `end` to char boundaries (`floor_char_boundary` / `ceil_char_boundary` on Rust 1.73+, or `char_indices` walk).
2. Never panic on any valid UTF-8 `body` + `query`.
3. Empty query still returns `None`.
4. Unit test: body with CJK prefix (e.g. many `你`) + ASCII query match → `Some` snippet without panic; emoji case optional.
5. Existing find tests stay green.

**Acceptance criteria:**

- [ ] No panic on non-ASCII bodies when building snippets
- [ ] Unit test covers multi-byte boundary case
- [ ] `cargo test -p odm-progen` green

**Out of scope:** FTS escaping (123), index freshness (131), CLI changes.

## Acceptance

- [ ] Agent Brief acceptance criteria all met
