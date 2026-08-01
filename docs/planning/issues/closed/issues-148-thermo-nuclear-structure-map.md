---
id: issues-148
title: "Thermo-nuclear structure deepen map"
description: "Wayfinder map from 2026-08-02 code quality review: inventory sample API, CLI Present/Ctx spine, membership split, fsutil, typed path errors."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
  - architecture
  - deepen
---

# Thermo-nuclear structure deepen map

## Destination

Clear the structural blockers from [[thermo-nuclear-code-quality-review]] (2026-08-02) without changing locked product JSON/exit contracts:

1. **Inventory sample API** — one worktree + pack sample plane; status, doctor, project info, prune name-sets consume it; pack missing owned once.
2. **CLI Present/Ctx spine** — finish the thin-bin story: one context, one present path, family dispatch; no dual print ritual / success `json!` monoculture in main.
3. **Membership split** — get `membership` module under ≤1000 LOC before the next feature.
4. **Shared fsutil** — one tree-copy / dest-prep engine for generate + agent pack.
5. **Typed path resolve errors** — stop remapping escape via English `contains`.

## Notes

- **Authority:** `docs/thermo-nuclear-code-quality-review.md`, AGENTS.md file-size target (≤1000 / hard 1250), prior deepen [[issues-34-workspace-observation-depth]], [[issues-38-cli-command-dtos]].
- **Standing prefs (AFK defaults):**
  - Preserve locked `--json` field names and exit codes unless a ticket explicitly migrates a type (e.g. prune rows already omit `dirty` in JSON).
  - No new product verbs; no remote generate; no `agent start`; no worktree deferred product.
  - TDD for code tickets; `cargo test` + `cargo clippy --workspace --all-targets -- -D warnings` green.
  - Prefer deleting concepts over rearranging them.
- **Execution order:** 149 unblocked first (highest leverage). 150–153 can proceed in parallel after or beside 149 where they do not fight the same files; 150 touches CLI heavily — coordinate if parallel with 149 only on CLI project_info attach. Prefer serial 149 → 150 if one agent.

## Decisions so far

- Child tickets: [[issues-149-workspace-inventory-sample]], [[issues-150-cli-present-ctx-spine]], [[issues-151-membership-module-split]], [[issues-152-fsutil-copy-tree]], [[issues-153-typed-path-resolve-errors]].
- Source review: `docs/thermo-nuclear-code-quality-review.md` + `.html`.

## Not yet specified

- Human formatter migration out of core (status/doctor) — opportunistic, not a map ticket.
- Soft-fail warning surface (“none” vs “could not sample”) — YAGNI unless a child needs it.

## Out of scope

- JSON schema redesign / field renames for agent comfort
- New crates
- Website / Playwright
- Coverage CI

## Blocked by

None

## Answer

Destination met 2026-08-02. Children 149–153 closed under `issues/closed/`. Re-verified before map close:

- `cargo test` → ok
- `cargo clippy --workspace --all-targets -- -D warnings` → clean
