---
id: issues-90
title: "status reports registered agent packs"
description: "odm status includes registry-backed agent_packs with name/path/mode/missing."
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

# status reports registered agent packs

## Description

`env-gen-packs.md` deferred “status pack reports”. Doctor already warns `pack_missing:<name>` when a registry path is gone. Operators still need a single workspace snapshot that lists registered packs (like `worktree_slots` on projects). Add top-level `agent_packs` on `odm status`.

## Affected

- `crates/odm-core/src/status.rs` — `StatusSnapshot`, `build_status`, `format_status_human`
- `crates/odm-core/src/agent_pack.rs` — reuse `pack_list` / `PackEntry` only (no new registry format)
- Unit tests in status (and/or agent_pack helpers); keep files ≤1000 target / ≤1250 hard (extract status tests if needed)
- Thin CLI only if status path needs DTO wiring (prefer core snapshot already serialized)

## Impact

Agents cannot see pack inventory or missing dests without separate `agent pack list` + filesystem probes; doctor warn alone is not a full inventory.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** feat  
**Summary:** Extend `StatusSnapshot` so `odm status` / `status --json` includes registered agent packs from `.odm/agent-packs.json`, each with a `missing` flag aligned with doctor pack_missing path rules.

**Bindings:**

- Parent map: [[issues-89-post-v1-status-packs-map]]
- `pack_list` → `PackEntry { name, source, path, mode }`
- Doctor path rule ([[issues-86-doctor-pack-missing]]): missing when neither path nor symlink entry present (`symlink_metadata` Err); dangling symlink present → **not** missing
- Prior additive status pattern: `worktree_slots` on projects ([[issues-65-status-worktree-slots]])

**Desired behavior:**

1. **JSON:** `StatusSnapshot` gains `agent_packs: [ StatusPackInfo… ]` (always present; empty array when no registry / empty list / load soft-fails).
   - Each item: `{ "name": string, "source": string, "path": string, "mode": "install"|"link", "missing": bool }`
   - `path` serialized as the registry destination string (same as pack list JSON path display — absolute or as stored; match `pack_list` / existing pack list DTO stringification).
   - Sorted by `name` ascending (same as `pack_list`).
2. **`missing`:** `true` iff destination has no path/symlink entry (same rule as `pack_missing_checks`). Present dir/file/symlink (including dangling symlink) → `missing: false`.
3. **build_status:** fill packs after entity status; registry/list errors → `agent_packs: []` without failing whole status (soft-fail like worktree list).
4. **Human (`format_status_human`):**
   - If `agent_packs` non-empty, print an `Agent packs:` section (after projects/progens) with one line per pack: name, mode, and ` missing` suffix when `missing`.
   - If projects and progens are both empty **and** packs non-empty, do **not** early-return only `(no projects or progens)` — still show workspace root + packs section.
   - If everything empty (no projects, progens, packs) keep a single empty workspace message (update wording only if needed so packs-only desks are honest).
5. **TDD:** unit tests — install-like registry entry present → pack in snapshot `missing: false`; delete dest → `missing: true`; empty registry → `[]`; human shows packs / missing suffix; progen/project shapes unchanged aside from new top-level field.
6. File sizes: any touched `.rs` ≤1000 preferred, ≤1250 hard. If `status.rs` would exceed 1000, extract `status_tests.rs` via `#[path]` like worktree.
7. No docs/CHANGELOG in this ticket (see [[issues-92-status-packs-docs-honesty]]). No core-desk (see 93). No doctor changes.
8. `cargo test` green; `cargo clippy --workspace --all-targets -- -D warnings` clean.

**Acceptance criteria:**

- [x] `status --json` includes top-level `agent_packs` array with name/source/path/mode/missing
- [x] Present pack path → `missing: false`; absent path (no symlink entry) → `missing: true`
- [x] Dangling symlink dest → `missing: false` (aligned with doctor)
- [x] Empty / missing registry → `agent_packs: []` (status still succeeds)
- [x] Human lists packs when non-empty; missing suffix when missing; packs-only workspace not swallowed by empty early-return
- [x] File sizes within limits; `cargo test` green; clippy `-D warnings` clean

**Out of scope:**

- Doctor check changes
- `agent pack list` JSON shape changes (optional later)
- Pack marketplace/manifest
- core-desk / reference docs (separate tickets)
- `project info` pack fields

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

`StatusSnapshot` now always includes top-level `agent_packs: Vec<StatusPackInfo>` (`name`, `source`, `path`, `mode`, `missing`).

- **Fill:** `build_status` maps `pack_list` entries; `missing = path.symlink_metadata().is_err()` (doctor-aligned); registry errors soft-fail to `[]`.
- **Human:** `Agent packs:` section with `  {name}\t{mode}[ missing]`; packs-only desks no longer early-return on empty projects/progens.
- **Tests:** extracted to `status_tests.rs`; present/absent/dangling/empty/corrupt/sort/JSON/human coverage.
- **Files:** `status.rs`, `status_tests.rs`, `lib.rs` re-export; project/progen test literals updated. Docs deferred to issues-92.
