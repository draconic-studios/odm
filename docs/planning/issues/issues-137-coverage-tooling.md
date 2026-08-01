---
id: issues-137
title: "Local coverage tooling script (no CI)"
description: "Add scripts/coverage.sh using cargo-llvm-cov for local seam measurement; document in README or docs; no GitHub Actions test CI."
status: reviewing
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
---

# Local coverage tooling script (no CI)

## Description

No llvm-cov/tarpaulin/grcov in repo. Need a **local** script to measure coverage while building toward the coverage map. AGENTS: no product CI.

## Affected

- `scripts/coverage.sh` (new)
- brief mention in README or docs/reference (optional)

## Proposed Fix

See Agent Brief.

## Blocked by

Prefer after [[issues-138-error-io-exit-unit-matrix]]–[[issues-142-progen-group-cli-integration]] land, but can land anytime.

## Agent Brief

**Category:** chore  
**Summary:** Add optional local coverage script; do not add GitHub Actions test workflow.

**Bindings:**

- Parent: [[issues-120-test-coverage-map]]
- AGENTS.md: no CI test matrix

**Desired behavior:**

1. `scripts/coverage.sh` runs `cargo llvm-cov --workspace` (or documents install of cargo-llvm-cov if missing).
2. Writes HTML or lcov under `target/coverage/` or `coverage/` (gitignored).
3. `.gitignore` updated for coverage artifacts.
4. One short README or docs note: local only, not CI.
5. Script exits non-zero if llvm-cov missing with install hint.

**Acceptance criteria:**

- [ ] Script exists and is executable
- [ ] Artifacts gitignored
- [ ] No new CI workflow for tests
- [ ] Docs one-liner

**Out of scope:** Coverage gates in CI; 100% line enforcement.

## Acceptance

- [ ] Agent Brief acceptance criteria all met
