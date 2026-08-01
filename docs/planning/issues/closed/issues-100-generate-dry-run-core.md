---
id: issues-100
title: "generate_local dry-run (no write, count files)"
description: "Core API: dry-run materialize validates like real run, writes nothing, returns would-copy file count."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# generate_local dry-run (no write, count files)

## Description

`env-gen-packs.md` Deferred lists **dry-run mode** for generators. Operators/agents need to preview how many files a local template would write without touching the destination tree.

## Affected

- `crates/odm-core/src/generate.rs` — `generate_local` (or sibling) + unit tests
- `crates/odm-core/src/lib.rs` only if new public symbols need re-export
- File size: ≤1000 target / ≤1250 hard

## Impact

Only way to know materialize impact is to run for real (or inspect template by hand). Dry-run is the deferred, AFK-ready next generator slice.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Add dry-run support to local generator materialize in `odm-core`: same validation as a real copy, zero filesystem writes, return the file count that would be copied.

**Bindings:**

- Parent map: [[issues-99-post-v1-generate-dry-run-map]]
- Authority: `docs/reference/env-gen-packs.md` Generators Deferred (dry-run); `cli.md` generate v1 local rules
- Landed: `generate_local(ws, name, dest, force) -> GenerateOutcome` (or equivalent) in `generate.rs`; recursive copy; force overwrite policy; escape checks

**Desired behavior:**

1. **API shape (pick one; prefer minimal churn):**
   - Extend `generate_local(…, force: bool, dry_run: bool)` **or** add `generate_local_dry_run(…)` that shares validation/count helpers with the real path.
   - Return type must expose at least: generator name, dest (as given/normalized consistently with today), `copied: u32` (would-write count), and enough for CLI to know it was dry-run (bool on outcome **or** caller-known flag).
2. **Validation (identical to real run):**
   - Resolve generator; local `template` required (url-only → same error as today, exit-class usage/`1` at CLI later).
   - `dest` relative under Workspace root; reject escape; reject when dest path is an existing **file**.
   - Dest exists and is **non-empty** without `force` → same operation error as today (exit `3` at CLI).
   - Empty existing dest dir OK without force (same as today).
3. **Dry-run writes:** when `dry_run`:
   - Do **not** create parent dirs, dest, or any files/symlinks.
   - Do **not** delete or overwrite anything even if `force` is true.
   - Still compute `copied` by walking the template the same way real copy counts written files (regular files; match existing symlink/file counting behavior in `copy_tree`).
4. **Real run:** `dry_run: false` preserves today’s write semantics exactly (including force overwrite-in-place, keep unrelated extras).
5. **TDD unit tests:**
   - Dry-run happy: template with N files → `copied == N`; dest path absent after call; no new files under root.
   - Dry-run with existing non-empty dest + force → still no writes; count is template file count.
   - Dry-run non-empty dest without force → errors; no partial writes.
   - Dry-run dest is a file → error.
   - Real run still writes (regression).
6. No CLI/docs/CHANGELOG/core-desk in this ticket (101–103).
7. `cargo test` green; `cargo clippy --workspace --all-targets -- -D warnings` clean.
8. Touched `.rs` files ≤1000 LOC preferred, ≤1250 hard.

**Acceptance criteria:**

- [x] Core dry-run path returns would-copy file count without creating/modifying dest tree
- [x] Validation parity with real run (escape, file-dest, non-empty without force, missing/url-only template)
- [x] Real run behavior unchanged when not dry-run
- [x] Unit tests cover happy dry-run, force+existing, error cases, real-run regression
- [x] File sizes within limits; `cargo test` green; clippy `-D warnings` clean

**Out of scope:**

- CLI flag wiring (ticket 101)
- Docs / core-desk
- Remote generators, `template.toml`, variable substitution
- Partial file listing in output (count only is enough)

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Extended `generate_local(..., force, dry_run)` in `odm-core`. Dry-run shares validation with the real path, uses private `count_tree` (files + symlinks, dirs recurse only — same rules as `copy_tree`), and never creates parents/dest or writes/overwrites. `GenerateOutcome` stays `{ copied, dest }` (caller-known dry-run). CLI `main.rs` passes `false` only (flag is ticket 101). Six new unit tests; full `cargo test` green; clippy `-D warnings` clean; `generate.rs` 673 LOC.
