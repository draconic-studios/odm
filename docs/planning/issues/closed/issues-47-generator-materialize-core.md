---
id: issues-47
title: "Generator materialize core"
description: "Core API to list generators and copy a local template directory to a workspace-relative dest."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# Generator materialize core

## Description

Generator bundles already load into `Workspace.generators`. There is no core API to validate dest paths or copy a local `template` tree. CLI cannot implement `odm generate` without this seam.

## Affected

- `crates/odm-core` (new module or extend existing)
- Downstream: generate CLI ticket

## Impact

Without core materialize, generate stays a stub.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feature  
**Summary:** Add a small `odm-core` generate/materialize API used by the CLI. Pure filesystem; no network.

**Bindings:**

- Parent map [[issues-45-generators-map]] standing prefs
- `docs/reference/config.md` Generators section
- Path policy: `paths::resolve_under_root` / existing relative-path rules
- `CONTEXT.md` Generator term

**Current behavior:**

- `GeneratorDef { template: Option<String>, url: Option<String> }` loaded and merged
- No generate/copy helpers

**Desired behavior:**

1. **Public helpers** (names flexible if clear):
   - Resolve generator by name from `&Workspace` → `&GeneratorDef` or usage error if missing
   - **`generate_local(ws, name, dest_rel, force) -> Result<GenerateOutcome, OdmError>`** (or equivalent):
     - Look up generator; if no non-empty `template`, return usage/not-implemented style error suitable for exit `1` (remote-only deferred)
     - Resolve `template` under workspace root; must exist and be a **directory** → else operation/usage error
     - Resolve `dest_rel` under workspace root (no escape); create parent directories of dest as needed
     - If dest exists:
       - file → error unless you treat as fail always (dest should be directory root)
       - non-empty directory or any existing path without `force` → error
       - with `force`: allow overwrite of files during copy
     - Recursively copy template contents into dest (files + dirs). Count files written.
     - Return outcome with at least `copied: u32` (or `usize`) and resolved dest path if useful
2. **Empty template dir:** success, `copied == 0`
3. **Unit tests** with tempdirs: happy copy, nested paths, force overwrite, reject escape/`..`, missing template, unknown name, url-only generator error
4. Do **not** wire CLI in this ticket
5. Keep module ≤1000 LOC; YAGNI — no templating engine

**Acceptance criteria:**

- [x] Public generate/materialize API on core callable without CLI
- [x] Local template directory recursive copy with file count
- [x] `--force` semantics as map (fail if exists/non-empty without force; overwrite files with force)
- [x] Path escape rejected
- [x] url-only generator errors clearly
- [x] Unit tests cover above
- [x] `cargo test` green
- [x] No CLI changes in this ticket

**Out of scope:**

- CLI / JSON DTOs
- Remote url fetch
- Variable substitution / template.toml
- core-desk fixtures

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Added `odm_core::generate` with `generator`, `generate_local`, and `GenerateOutcome { copied, dest }`.

- Local `template` dir recursive copy (files + dirs + symlinks); empty template → `copied == 0`
- Prefer `template` when both template and url set; url-only → usage exit 1 (remote deferred)
- Dest under workspace root via `resolve_under_root`; create parents; empty dest OK without force; non-empty/file fails unless force (overwrite files, keep extras)
- Unit tests in `crates/odm-core/src/generate.rs`; no CLI wiring
