---
id: issues-101
title: "CLI odm generate --dry-run"
description: "Wire --dry-run on generate run; human would-generate line; JSON dry_run field."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# CLI odm generate --dry-run

## Description

After core dry-run lands, expose `--dry-run` on `odm generate <name> --dest <path>` with honest human/JSON output.

## Affected

- `crates/odm/src/cli.rs` — generate args
- `crates/odm/src/main.rs` — dispatch
- `crates/odm/src/commands/generate.rs` — `GenerateRunDto` + human formatters
- `crates/odm/tests/cli_generate.rs` — integration coverage
- File size limits apply

## Impact

Dry-run only in core is invisible to users/agents.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-100-generate-dry-run-core]]

## Agent Brief

**Category:** feat  
**Summary:** Wire `odm generate <name> --dest <path> [--force] [--dry-run]` through core dry-run; stable human + JSON.

**Bindings:**

- Parent map: [[issues-99-post-v1-generate-dry-run-map]]
- Core API from [[issues-100-generate-dry-run-core]]
- Existing: `GenerateRunDto { generator, dest, copied }`, `format_generate_run_human`, `cli_generate.rs` tests

**Desired behavior:**

1. **Clap:** add `--dry-run` flag on the generate-run path (name + dest required as today). Compatible with `--force` and `--json`.
2. **Dispatch:** call core with dry_run true/false; map errors to existing exit codes unchanged.
3. **JSON (`GenerateRunDto`):** always include `"dry_run": bool` (`true` when flag set, `false` on real run). Keep `generator`, `dest`, `copied`. Additive field — update unit tests that assert exact key sets.
4. **Human:**
   - Dry-run success: `would generate <name> -> <dest> (<n> files)` (trailing newline like today).
   - Real run: keep `generated <name> -> <dest> (<n> files)`.
5. **List path:** bare `odm generate` / list unchanged; `--dry-run` without a generator name is usage error (clap or exit `1`) — do not dry-run list.
6. **Integration tests** (`cli_generate.rs` or equivalent):
   - Dry-run success JSON: `dry_run: true`, `copied` > 0, dest path still absent on disk.
   - Dry-run human contains `would generate`.
   - Real run JSON: `dry_run: false` and files exist.
   - Dry-run + non-empty dest without `--force` → exit `3`.
   - Url-only + dry-run still exit `1` if already covered pattern exists; add if cheap.
7. No reference docs/CHANGELOG/core-desk (102–103).
8. `cargo test` green; clippy `-D warnings` clean; file sizes within limits.

**Acceptance criteria:**

- [ ] `odm generate <name> --dest <p> --dry-run` exits 0, writes nothing, prints would-generate / JSON `dry_run: true`
- [ ] Real run JSON includes `dry_run: false`; human still `generated …`
- [ ] Force/non-empty/error exit codes unchanged vs non-dry-run
- [ ] Integration tests cover dry-run success + no-write + real-run regression
- [ ] `cargo test` green; clippy clean; LOC limits held

**Out of scope:**

- Docs / core-desk
- Listing files that would be copied (count only)
- Remote / template.toml

## Acceptance

- [ ] Agent Brief acceptance criteria all met
