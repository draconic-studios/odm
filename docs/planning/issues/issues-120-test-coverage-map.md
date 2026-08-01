---
id: issues-120
title: "100% test coverage suite map"
description: "Behavior-seam coverage toward a complete suite: unit matrix, CLI exit codes, progen edges, coverage tooling — not vanity line %."
status: open
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - wayfinder-map
---

# 100% test coverage suite map

## Destination

Every shipped CLI verb and public lib seam has success + primary error coverage; exit codes 1–4 are table-tested; progen/git thin modules gain unit tests; optional local `cargo llvm-cov` script (no CI — AGENTS policy).

## Notes

- Static audit 2026-08-01: ~332 tests, strong on worktree/packs/generate/run/status; thin on pin/sync/rm CLI, error/io/ops, FTS edges, `--progen-group`.
- “100%” = **seam checklist complete**, not forced 100% llvm lines on `main.rs` match arms.
- Respect file size ≤1000 / hard 1250 — split new test files rather than grow `core_desk.rs` / `cli_worktree.rs`.

## Decisions so far

- Tooling last (after suite is broad enough that % is meaningful).
- Reuse existing harness patterns in `crates/odm/tests/*`.

## Fog / tickets

- [[issues-137-coverage-tooling]]
- [[issues-138-error-io-exit-unit-matrix]]
- [[issues-139-cli-pin-sync-rm-integration]]
- [[issues-140-cli-exit-code-matrix]]
- [[issues-141-progen-unit-edges-ops]]
- [[issues-142-progen-group-cli-integration]]
- [[issues-143-odm-git-worktree-real-git]]

## Related

- [[issues-119-swarm-audit-hardening-map]] — bugs that need regression tests in their own tickets
- [[issues-121-full-capability-demo-map]] — end-to-end dogfood gate
