---
id: issues-123
title: "FTS query escaping for safe find"
description: "User find queries with AND/OR/punctuation become FTS5 syntax errors instead of safe results."
status: open
issue-type: bug
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# FTS query escaping for safe find

## Description

`search_fts` passes the user string to FTS5 `MATCH` after only doubling `"`. Tokens like `AND`, `OR`, `NOT`, or odd punctuation cause `fts5: syntax error` as `OdmError::operation`. Agents/humans expect plain-text search.

## Affected

- `crates/odm-progen/src/index.rs` — `search_fts`
- `odm find`

## Impact

Common queries fail hard instead of returning hits/empty.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** fix  
**Summary:** Tokenize/escape user queries for FTS5 so plain text never surfaces as syntax errors; preserve useful multi-word search.

**Bindings:**

- Parent: [[issues-119-swarm-audit-hardening-map]]
- `crates/odm-progen/src/index.rs`

**Desired behavior:**

1. Escape or quote user terms so `AND`, `OR`, `NOT`, `@@@`, quotes, and punctuation do not fail MATCH.
2. Multi-word queries still find notes containing the terms (AND of terms is OK; document if phrase search is not supported).
3. FTS syntax failures must not leak as raw sqlite errors to users if any remain — map to empty hits or clear usage message.
4. Unit tests: `AND` alone or in query; punctuation; multi-word happy path; existing reindex/search still pass.
5. No CLI flag changes.

**Acceptance criteria:**

- [ ] Query `AND` does not return operation/syntax failure
- [ ] Multi-word find still works on indexed notes
- [ ] Unit tests cover escape cases
- [ ] `cargo test -p odm-progen` green

**Out of scope:** snippet unicode (122), auto-reindex (131), ranking changes.

## Acceptance

- [ ] Agent Brief acceptance criteria all met
