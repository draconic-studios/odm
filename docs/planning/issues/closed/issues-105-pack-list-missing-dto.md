---
id: issues-105
title: "pack list DTO includes missing"
description: "PackEntryDto gains missing bool (doctor/status path rule); list + install/link/rm JSON share fields."
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

# pack list DTO includes missing

## Description

`odm status` already reports `agent_packs[].missing`. `odm agent pack list --json` still omits `missing`, so agents must probe the filesystem or call status. Add `missing` to the shared pack entry DTO used by list and install/link/rm JSON.

## Affected

- `crates/odm/src/commands/agent_pack.rs` — `PackEntryDto`, `From<&PackEntry>`, formatters, unit tests
- Possibly thin probe helper (keep in CLI command module or tiny core helper — prefer DTO layer probe on `entry.path`)
- No registry format change; no doctor/status changes

## Impact

List JSON cannot answer “is the pack dest still on disk?” without a second command.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Extend `PackEntryDto` with `missing: bool` aligned with status/doctor pack path rules; populate for list and single-entry install/link/rm DTOs.

**Bindings:**

- Parent map: [[issues-104-post-v1-pack-list-missing-map]]
- Status rule ([[issues-90-status-agent-packs]]): `missing = path.symlink_metadata().is_err()`
- Doctor ([[issues-86-doctor-pack-missing]]): dangling symlink present is **not** missing
- cli.md: install/link/rm JSON = same fields as list items

**Desired behavior:**

1. **`PackEntryDto`** fields: `name`, `source`, `path`, `mode`, **`missing`** (bool).
2. **Probe:** when building DTO from `PackEntry`, set `missing` from `entry.path.symlink_metadata().is_err()`.
3. **`pack_list_dto` / `pack_entry_dto`:** both include `missing`.
4. **Human list (`format_pack_list_human`):** one line per pack; bare `name` when present; `name missing` (space + word `missing`) when `missing` — empty list still `(no agent packs)\n`. Do **not** change install/link/rm human success lines.
5. **Unit tests (TDD):**
   - DTO JSON includes `"missing": false` for a normal path that exists (use temp dir path that exists).
   - `"missing": true` when path has no metadata entry.
   - Dangling symlink path → `missing: false` if feasible in unit test without full pack install (create temp dangling symlink).
   - Human formatter: present → `a\n`; missing → `a missing\n`; empty unchanged.
   - install/link/rm human strings unchanged.
6. File size ≤1000 / ≤1250. No docs/CHANGELOG (107). No bin integration (106). No core-desk (108).
7. `cargo test` green; prefer clippy `-D warnings` clean on touched crates.

**Acceptance criteria:**

- [x] `PackEntryDto` serializes `missing` bool
- [x] Present path → false; absent path → true; dangling symlink → false
- [x] `pack_list_dto` and `pack_entry_dto` both set `missing`
- [x] Human list suffix only when missing; empty message unchanged
- [x] Unit tests cover above; `cargo test` green

**Out of scope:**

- Bin/CLI integration tests (106)
- Reference docs / CHANGELOG (107)
- core-desk (108)
- Doctor or status changes
- Pack marketplace/manifest

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

`PackEntryDto` gains `missing: bool` via `entry.path.symlink_metadata().is_err()` (status/doctor parity). List + install/link/rm JSON share the field; human list suffixes ` missing` only when absent; empty list and install/link/rm human lines unchanged. Unit tests cover present/absent/dangling symlink + human formatter. `cargo test -p odm` green; clippy `-D warnings` clean.
