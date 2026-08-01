---
id: issues-103
title: "core-desk dogfood generate --dry-run"
description: "core-desk README + integration gate: dry-run writes nothing then real generate materializes."
status: open
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# core-desk dogfood generate --dry-run

## Description

Dogfood Workspace should document and gate dry-run: `generate hello --dest … --dry-run` leaves tree clean → real generate writes files.

## Affected

- `examples/core-desk/README.md` — Generators section
- `crates/odm/tests/core_desk.rs` — new or extended gate (watch file size ≤1000 / ≤1250)
- No product feature work beyond dogfood harness

## Impact

Without dogfood, dry-run can rot relative to the sample Workspace.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-100-generate-dry-run-core]]
- [[issues-101-generate-dry-run-cli]]

## Agent Brief

**Category:** test  
**Summary:** Document and integration-test generate dry-run against `examples/core-desk` sample `hello` generator.

**Bindings:**

- Parent map: [[issues-99-post-v1-generate-dry-run-map]]
- Existing generate dogfood lines in core-desk README; `cli_generate` / prior `core_desk` gates for harness patterns
- Generator: `hello` → `templates/hello` (already in example)

**Desired behavior:**

1. **README:** under Generators, show:
   - `odm generate hello --dest out/hello --dry-run` (would generate / no files)
   - then `odm generate hello --dest out/hello` (real materialize)
   - optional `--json` note for `dry_run: true`
2. **Integration gate** in `core_desk.rs` (name e.g. `core_desk_generate_dry_run_gate`):
   - Use existing core-desk temp-copy harness.
   - Run dry-run to a dest under the temp root → exit 0; assert dest still missing (or empty / not created per core semantics); JSON `dry_run: true` and `copied` ≥ 1 if using `--json`.
   - Run real generate to same dest → exit 0; expected template file(s) exist (e.g. `hello.txt` or whatever the template contains).
3. Do not require force/error matrix here (CLI tests own that).
4. Keep `core_desk.rs` ≤1000 preferred / ≤1250 hard — extract helpers if needed rather than blow the file.
5. No reference-doc epic (102 owns docs). No new generators in the example unless required.
6. `cargo test` green.

**Acceptance criteria:**

- [ ] core-desk README documents dry-run then real generate for `hello`
- [ ] Integration gate asserts dry-run no-write + real generate writes
- [ ] `core_desk.rs` within LOC limits
- [ ] `cargo test` green

**Out of scope:**

- Product changes to generate beyond what’s already landed
- Pack/worktree dogfood changes
- Docs reference tree (ticket 102)

## Acceptance

- [ ] Agent Brief acceptance criteria all met
