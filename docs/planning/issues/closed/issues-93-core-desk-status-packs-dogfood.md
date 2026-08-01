---
id: issues-93
title: "core-desk dogfood: status packs + doctor pack_missing"
description: "examples/core-desk README + gate test for status agent_packs and doctor pack_missing."
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

# core-desk dogfood: status packs + doctor pack_missing

## Description

core-desk already dogfoods agent pack install→list→rm ([[issues-88-core-desk-pack-rm-dogfood]]). Extend dogfood so operators see **status pack inventory** and **doctor pack_missing** after a deleted dest (then clean up with rm).

## Affected

- `examples/core-desk/README.md` — pack section commands
- `crates/odm/tests/core_desk.rs` — new or extended gate (follow `core_desk_agent_pack_rm_gate` style)

## Impact

Without dogfood, status packs + doctor pack_missing stay unit-only and drift from the example desk.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-90-status-agent-packs]]
- [[issues-91-doctor-pack-missing-cli]]

## Agent Brief

**Category:** test  
**Summary:** core-desk documents and integration-gates status `agent_packs` plus doctor `pack_missing` on a missing pack dest; cleanup via `agent pack rm`.

**Bindings:**

- Parent map: [[issues-89-post-v1-status-packs-map]]
- Existing pack rm dogfood in core-desk README + `core_desk_agent_pack_rm_gate`
- Status JSON `agent_packs` from ticket 90; doctor JSON from ticket 91 / 86

**Desired behavior:**

1. **README** (pack section, after install/list or as a short subsection):
   - `odm status` / `odm status --json` — note `agent_packs` includes demo after install (`missing: false`).
   - Optional comment: delete dest under agent home → `odm doctor` warns `pack_missing:demo`; `odm status --json` shows `missing: true`.
   - Then `odm agent pack rm demo` → empty list / no pack_missing (keep existing rm dogfood).
   - Keep commands copy-pasteable; use same `--home` path convention as existing pack dogfood.
2. **Integration gate** (prefer one focused test, e.g. `core_desk_status_packs_doctor_gate`):
   - init/sync path as other core_desk gates (reuse helpers).
   - install demo pack → `status --json` has agent_packs entry name demo, missing false.
   - delete dest → `doctor --json` has pack_missing:demo; status agent_packs missing true.
   - `agent pack rm demo` → list empty; doctor no pack_missing for demo.
3. Do not require network. Do not expand unrelated core-desk surfaces.
4. Docs-only README + tests; no odm-core feature work unless a trivial bug blocks the gate.
5. `cargo test` green.

**Acceptance criteria:**

- [x] core-desk README covers status packs + doctor pack_missing + rm cleanup
- [x] Integration gate asserts status missing false→true and doctor pack_missing, then rm clears
- [x] `cargo test` green

**Out of scope:**

- Reference docs (ticket 92)
- New pack product features
- Worktree dogfood changes

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

core-desk dogfood for status packs + doctor pack_missing:

- **README** pack section: `odm status` / `status --json` after install; optional missing-dest comments (`pack_missing:demo`, `missing: true`); rm cleanup note.
- **Gate** `core_desk_status_packs_doctor_gate`: install → `agent_packs` demo `missing: false` → delete dest → doctor `pack_missing:demo` warn + status `missing: true` → `agent pack rm demo` clears list and doctor check.
- Docs + tests only; full `cargo test` green.
