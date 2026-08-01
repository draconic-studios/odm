---
id: issues-142
title: "CLI integration: --progen-group on find"
description: "Shipped --progen-group is unit-tested in scope.rs only; zero binary integration coverage."
status: reviewing
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# CLI integration: --progen-group on find

## Description

Flag wired in cli.rs/main.rs; scope unit-tested; `crates/odm/tests/**` has no progen-group cases. Wiring regressions would not fail CI.

## Affected

- `crates/odm/tests/progen_vault.rs` (or new file)
- Multi-progen fixture (synthetic or extended)

## Proposed Fix

See Agent Brief.

## Blocked by

None (pairs with [[issues-144-core-desk-assets-full-surface]] later for dogfood)

## Agent Brief

**Category:** test  
**Summary:** Binary tests for find scoped by --progen-group.

**Bindings:**

- Parent: [[issues-120-test-coverage-map]]
- cli.md find scope flags

**Desired behavior:**

1. Workspace with ≥2 progens and a group that includes only one.
2. `find <token> --progen-group <g>` returns only that progen’s hits.
3. Unknown group → exit 1 (or 2 per current code — lock actual).
4. Union: `--progen` + `--progen-group` if cheap.
5. `--json` hit `progen` field asserted.

**Acceptance criteria:**

- [ ] Group narrows hits at CLI
- [ ] Unknown group fails with stable exit
- [ ] `cargo test -p odm` green

**Out of scope:** core-desk asset changes (144).

## Acceptance

- [ ] Agent Brief acceptance criteria all met
