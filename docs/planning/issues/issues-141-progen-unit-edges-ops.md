---
id: issues-141
title: "Progen unit tests: index/note/vault/ops edges"
description: "ops.rs untested; index/note/vault thin — add unit coverage for formatters, doctor_progens, vault walk, note edges."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# Progen unit tests: index/note/vault/ops edges

## Description

`ops.rs` has zero unit tests. `index`/`note`/`vault` only happy-path. Need edges independent of CLI (FTS/dup/freshness may land in 123/131 with their own tests).

## Affected

- `crates/odm-progen/src/ops.rs`
- `note.rs`, `vault.rs`, `index.rs` edges not covered by bug tickets

## Proposed Fix

See Agent Brief.

## Blocked by

None (coordinate with 122/123/131 so tests don’t duplicate)

## Agent Brief

**Category:** test  
**Summary:** Unit tests for human formatters, doctor_progens, vault walk skip rules, note parsing edges already stable.

**Bindings:**

- Parent: [[issues-120-test-coverage-map]]

**Desired behavior:**

1. `format_find_human` / `format_context_human` empty and non-empty.
2. `doctor_progens`: missing vault fail/warn; index present path.
3. vault: nested dirs walked; `.git`/`.obsidian` skipped (current behavior).
4. note: heading title fallback; aliases if supported; basic wikilink + header strip if any.
5. Do not re-implement 122/123/131 tests here — only gaps those tickets leave.

**Acceptance criteria:**

- [ ] ops.rs has direct unit tests
- [ ] vault/note edge cases covered
- [ ] `cargo test -p odm-progen` green

**Out of scope:** CLI progen-group (142); full FTS product changes.

## Acceptance

- [ ] Agent Brief acceptance criteria all met
