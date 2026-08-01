---
id: issues-146
title: "Integration gate: core_desk_full_tour"
description: "Thin Rust integration test covering the biggest core-desk composition holes (groups, context/prompt, project git, run --project, pack link, store façade)."
status: open
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - ready-for-agent
---

# Integration gate: core_desk_full_tour

## Description

Focused core_desk gates miss composition. One new test file should lock the full sample Workspace as a regression net without duplicating every script phase.

## Affected

- `crates/odm/tests/core_desk_full_tour.rs` (new)
- Shared harness extract from `core_desk.rs` if needed (LOC)

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-144-core-desk-assets-full-surface]]
- Prefer [[issues-145-core-desk-dogfood-script]] for phase list alignment (soft)

## Agent Brief

**Category:** test  
**Summary:** One integration test (or small set) on temp core-desk covering composition gaps.

**Bindings:**

- Parent: [[issues-121-full-capability-demo-map]]
- Reuse `setup_temp_core_desk` — **extract** to `crates/odm/tests/common/mod.rs` or `harness.rs` if `core_desk.rs` would exceed 1000 LOC

**Must cover:**

1. sync + reindex  
2. `find` token in notes; `find --progen-group` narrows  
3. `context welcome` + `agent prompt welcome` JSON anchor id  
4. `progen get` / `body` / `tree` or `ls` / `backlinks` on seeded ids  
5. `project git alpha -- rev-parse HEAD` (or status)  
6. worktree add + `run <in-alpha> --project alpha` (and optional `--wt`)  
7. `agent pack link` + list  
8. `generate --force` after first materialize  

Keep existing focused gates; this is additive.

**Acceptance criteria:**

- [ ] New test file green in `cargo test -p odm`
- [ ] Covers items 1–8 above
- [ ] File size limits; harness shared if extracted
- [ ] Does not require network

**Out of scope:** Every negative exit path (140); website.

## Acceptance

- [ ] Agent Brief acceptance criteria all met
