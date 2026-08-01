---
id: issues-140
title: "CLI exit-code matrix integration tests"
description: "Table-driven bin tests locking exit codes 1–4 and JSON error envelope across primary failure modes."
status: open
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# CLI exit-code matrix integration tests

## Description

Exit codes are specified in cli.md but only ad hoc asserted. Need one table-driven file covering primary failure modes and `--json` error envelope.

## Affected

- New `crates/odm/tests/cli_exit_codes.rs`
- `crates/odm/src/output.rs` envelope

## Proposed Fix

See Agent Brief.

## Blocked by

- Prefer [[issues-127-wt-missing-exit-code]] and [[issues-129-clap-usage-exit-json]] first so matrix locks the correct contract
- [[issues-138-error-io-exit-unit-matrix]] helpful but not required

## Agent Brief

**Category:** test  
**Summary:** Table-driven CLI tests for exit codes and JSON `{ok:false,error:{code,message}}`.

**Bindings:**

- Parent: [[issues-120-test-coverage-map]]
- cli.md exit table

**Desired behavior (rows — adjust if blocked tickets change codes):**

1. Not a workspace → 2  
2. Invalid config YAML → 2  
3. Unknown project name → 1  
4. Unknown action → 1  
5. Missing pack source → 4  
6. Generate non-empty dest without force → 3  
7. Run action fail → passthrough N (e.g. 7)  
8. Worktree prune partial nonempty → 3  
9. Unknown note id context/get → 4  
10. agent start → 1  
11. Missing wt slot on run → 4 (after 127)  
12. Clap bad flags → 1 (after 129)

For a sample of rows with `--json`, assert stdout parses as error envelope with matching `error.code`.

**Acceptance criteria:**

- [ ] ≥10 distinct failure rows
- [ ] At least 4 JSON envelope assertions
- [ ] `cargo test -p odm --test cli_exit_codes` green
- [ ] File ≤1000 LOC

**Out of scope:** Implementing product fixes (file bugs if matrix finds wrong codes).

## Acceptance

- [ ] Agent Brief acceptance criteria all met
