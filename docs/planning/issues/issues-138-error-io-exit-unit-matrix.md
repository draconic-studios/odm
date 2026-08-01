---
id: issues-138
title: "Unit tests: error, io, exit_code matrix"
description: "odm-core error.rs and io.rs have zero direct unit tests; foundation for CLI exit honesty."
status: reviewing
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
---

# Unit tests: error, io, exit_code matrix

## Description

`error.rs` (`exit_code`, `code`, `detail`, From impls) and `io.rs` (`atomic_write`) are untested directly. Regressions only show at CLI.

## Affected

- `crates/odm-core/src/error.rs`
- `crates/odm-core/src/io.rs`

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** test  
**Summary:** Table-driven unit tests for exit codes, error codes/details, atomic_write success and failure cleanup.

**Bindings:**

- Parent: [[issues-120-test-coverage-map]]
- cli.md exit table: 1 usage, 2 workspace, 3 operation, 4 not_found

**Desired behavior:**

1. Usage→1, Workspace→2, Operation→3, NotFound→4.
2. `code()` stable strings match CLI JSON error codes.
3. `detail()` for multiline git stderr cases if applicable.
4. `atomic_write`: create new; replace existing; content correct; on write failure no torn final file (and temp cleaned if feasible).
5. Keep tests in-file or `error_tests`/`io_tests` if size pressure.

**Acceptance criteria:**

- [ ] Direct unit coverage for exit_code/code/detail
- [ ] atomic_write round-trip + failure hygiene
- [ ] `cargo test -p odm-core` green

**Out of scope:** CLI integration envelope (140); clap (129).

## Acceptance

- [ ] Agent Brief acceptance criteria all met
