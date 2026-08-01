---
id: issues-108
title: "core-desk dogfood pack list missing"
description: "core-desk README + gate: pack list missing false after install, true after dest delete, clear after rm."
status: closed
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# core-desk dogfood pack list missing

## Description

Dogfood Workspace should document and gate pack list `missing` observation using the demo pack, without relying only on `odm status`.

## Affected

- `examples/core-desk/README.md` — agent pack section
- `crates/odm/tests/core_desk.rs` — gate (watch LOC ≤1000 / ≤1250)
- No product feature work beyond dogfood harness

## Impact

Without dogfood, pack list missing can rot relative to the sample Workspace.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-105-pack-list-missing-dto]]
- [[issues-106-pack-list-missing-cli]]

## Agent Brief

**Category:** test  
**Summary:** Document and integration-test pack list missing against `examples/core-desk` demo pack.

**Bindings:**

- Parent map: [[issues-104-post-v1-pack-list-missing-map]]
- Existing pack dogfood in core-desk README / `core_desk` gates (status packs, pack rm)
- Demo pack: `agent-packs/demo`

**Desired behavior:**

1. **README:** under agent packs, show:
   - install demo into a temp home
   - `odm agent pack list` / `--json` → `missing: false`
   - delete dest dir → `list --json` → `missing: true` (and/or human ` missing`)
   - `odm agent pack rm demo` clears registry
   - Keep status/doctor pack_missing notes; list is complementary inventory.
2. **Integration gate** (e.g. `core_desk_pack_list_missing_gate`):
   - Temp-copy core-desk harness
   - install demo → list JSON `missing: false`
   - delete home dest → list JSON `missing: true`
   - rm → list empty / pack absent
3. Do not duplicate full doctor matrix (already gated elsewhere) unless one assert is cheap.
4. Keep `core_desk.rs` within LOC limits; extract helpers if needed.
5. No reference-doc epic (107 owns docs).
6. `cargo test` green.

**Acceptance criteria:**

- [x] core-desk README documents pack list missing observation
- [x] Integration gate asserts false → true → cleared via rm
- [x] `core_desk.rs` within LOC limits
- [x] `cargo test` green

**Out of scope:**

- Product changes beyond dogfood
- Reference docs (107)
- Worktree/generate dogfood changes

## Acceptance

- [x] Agent Brief acceptance criteria all met
