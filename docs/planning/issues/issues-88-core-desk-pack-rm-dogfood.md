---
id: issues-88
title: "core-desk dogfood: agent pack rm"
description: "Document and integration-gate odm agent pack rm on examples/core-desk demo pack."
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

# core-desk dogfood: agent pack rm

## Description

core-desk already dogfoods pack install/list. After pack rm CLI lands, README + integration gate should install → list → rm → list empty (and optionally doctor clean of pack_missing).

## Affected

- `examples/core-desk/README.md`
- `crates/odm/tests/core_desk.rs` (or adjacent integration module if that is the dogfood pattern)

## Impact

Without dogfood, pack rm regresses silently relative to the example Workspace story.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-84-agent-pack-rm-core]]
- [[issues-85-agent-pack-rm-cli]]

## Agent Brief

**Category:** test  
**Summary:** core-desk docs + integration prove `odm agent pack rm` against `agent-packs/demo`.

**Bindings:**

- Parent map: [[issues-82-post-v1-pack-lifecycle-hardening-map]]
- Existing pack dogfood in core-desk README and any pack steps in `core_desk.rs`
- CLI from [[issues-85-agent-pack-rm-cli]]

**Desired behavior:**

1. **README:** after install/list sample, show rm + list empty (temp `--home` path as today).
2. **Integration:** one focused test (or extend existing pack gate) that on a temp copy of core-desk (or minimal workspace with demo pack path):
   - `agent pack install agent-packs/demo --home <tmp>`
   - `agent pack list` shows `demo`
   - `agent pack rm demo` exit 0
   - `agent pack list` empty / `(no agent packs)`
3. Optional: after rm, `doctor` has no `pack_missing:demo` (only if cheap with existing harness).
4. Do not require doctor pack_missing ticket if 86 is still open — rm dogfood alone is enough; if 86 already merged, asserting no pack_missing after rm is nice.
5. `cargo test` green. No product feature work beyond test/docs in example.

**Acceptance criteria:**

- [ ] core-desk README documents pack rm dogfood
- [ ] Integration test covers install → rm → empty list
- [ ] `cargo test` green

**Out of scope:**

- Reference docs pass ([[issues-87-pack-lifecycle-docs-honesty]])
- New pack features
- Worktree dogfood changes

## Acceptance

- [ ] Agent Brief acceptance criteria all met
