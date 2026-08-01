---
id: issues-147
title: "File gaps found while running full dogfood tour"
description: "After dogfood.sh / full_tour gate: file any new product bugs or docs gaps as ready-for-agent issues; close this if none."
status: reviewing
issue-type: observation
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
---

# File gaps found while running full dogfood tour

## Description

The full capability demo is also a verification tool. After [[issues-145-core-desk-dogfood-script]] and [[issues-146-core-desk-full-tour-gate]], run the tour and file residual bugs.

## Affected

- Product crates / docs as discovered
- This issue’s Comments log

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-145-core-desk-dogfood-script]]
- [[issues-146-core-desk-full-tour-gate]]

## Agent Brief

**Category:** chore  
**Summary:** Run full dogfood + tour gate; for each unexpected failure or honesty gap, create a new `ready-for-agent` issue under docs/planning/issues with Agent Brief; link from Comments here. If none, close this issue to `closed/` with Answer “no new gaps.”

**Bindings:**

- Parent: [[issues-121-full-capability-demo-map]]
- Issue tracker: `docs/agents/issue-tracker.md`
- Do not duplicate 119–136 already filed

**Desired behavior:**

1. Run `examples/core-desk/scripts/dogfood.sh` and `cargo test -p odm --test core_desk_full_tour`.
2. Classify failures: test flake vs product bug vs missing demo asset.
3. File only real product/docs bugs not already ticketed.
4. Append list of new issue wikilinks under Comments.
5. Set status closed and move to closed/ when done (even if zero new issues).

**Acceptance criteria:**

- [ ] Tour + gate executed
- [ ] New issues filed or explicit “none”
- [ ] This issue closed with log

**Out of scope:** Implementing the new bugs in the same session (optional).

## Acceptance

- [ ] Agent Brief acceptance criteria all met
