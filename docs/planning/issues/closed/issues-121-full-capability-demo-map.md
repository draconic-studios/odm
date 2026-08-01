---
id: issues-121
title: "Full ODM capability demo map"
description: "Extend core-desk + dogfood script + tour integration test to exercise all shipped odm tools and surface product gaps."
status: closed
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - wayfinder-map
---

# Full ODM capability demo map

## Destination

One offline Workspace (`examples/core-desk`) plus `scripts/dogfood.sh` and a thin `core_desk_full_tour` integration gate that exercises **all shipped** CLI surfaces (not sketches). Gaps found while building the demo are filed back onto [[issues-119-swarm-audit-hardening-map]] or new issues.

## Notes

- Today core-desk is strong on sync/pin/worktree/packs/generate/find; weak on composition (`--progen-group`, `run --project/--wt`, store façade, `project git`, pack link, multi-progen).
- Prefer extend core-desk in place — do not invent `examples/full-desk` unless clarity forces a split.
- `core_desk.rs` is ~732 LOC — extract shared harness before adding a large tour file.

## Decisions so far

- Second path-only Progen + real `progen_groups` usage is required for federation demo.
- Sketch commands (`agent start`, `init -i`) only appear as “exit 1 honesty” smokes, not full features.

## Fog / tickets

- [[issues-144-core-desk-assets-full-surface]]
- [[issues-145-core-desk-dogfood-script]]
- [[issues-146-core-desk-full-tour-gate]]
- [[issues-147-demo-gap-followups]] — file bugs found while running the tour (may be empty if none)

## Related

- [[issues-120-test-coverage-map]]
- Open pack dogfood: [[issues-108-core-desk-pack-list-missing-dogfood]]

## Answer

Destination met 2026-08-02. Children 144–147 closed. Re-verified before map close:

- `cargo test -p odm --test core_desk_full_tour` → ok
- `examples/core-desk/scripts/dogfood.sh` → `dogfood: OK`
- [[issues-147-demo-gap-followups]] Answer: no new gaps
