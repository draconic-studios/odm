---
id: issues-166
title: "CI policy: allow release publish workflow"
description: "Extend AGENTS.md (optional ADR) so GitHub Actions may run a release publish workflow plus Pages; still forbid product CI test matrix."
status: closed
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ci
  - release
---

# CI policy: allow release publish workflow

## Description

`AGENTS.md` currently allows GitHub Actions only for GitHub Pages deploy of `website/`. Multi-platform Releases need a **release publish** workflow. Policy must explicitly allow that without opening the door to a product CI test matrix on every PR.

## Affected

- `AGENTS.md` (Actions / CI exception line)
- Optional short ADR under `docs/adr/` if a durable decision record is preferred alongside the one-liner

## Impact

Without the policy update, agents and humans will keep treating any non-Pages workflow as forbidden.

## Proposed Fix

See Agent Brief.

## Blocked by

None

## Agent Brief

**Category:** chore  
**Summary:** Document that Pages + **release publish** workflows are allowed; product CI test matrix remains forbidden.

**Bindings:**

- Parent map: [[issues-165-multiplatform-github-releases-curl-install]] Decisions (AFK defaults)
- Current `AGENTS.md` line: no product CI test matrix; GitHub Actions allowed only for GitHub Pages deploy of `website/`

**Map Decisions (v1 lock — do not reopen):**

- **Allowed Actions:** GitHub Pages deploy of `website/` **and** a **release publish** workflow (tag `v*` / `workflow_dispatch` builds that upload release assets)
- **Still forbidden:** product CI test matrix on PRs / every push (no full `cargo test` matrix as gate CI)
- Platforms for the eventual workflow (context only): `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`
- This ticket is **policy only** — do not add the workflow YAML here (that is [[issues-167-multi-target-release-workflow]])

**Desired behavior:**

1. Update `AGENTS.md` so the Actions exception is clearly: Pages + release publish; still no product CI test matrix.
2. Optional: add a short ADR under `docs/adr/` capturing the same decision (why release publish is allowed, why test matrix is not).
3. Do not implement workflow, install.sh, docs/site copy, or version bump.

**Acceptance criteria:**

- [x] `AGENTS.md` allows release publish workflow in addition to Pages
- [x] `AGENTS.md` still forbids product CI test matrix
- [x] Optional ADR matches policy if filed
- [x] No workflow YAML / install script / version cut in this ticket

**Out of scope:**

- Implementing `.github/workflows/*` release job
- Product test matrix on PRs
- Windows / musl / signing policy changes

## Answer

`AGENTS.md` Actions exception is now Pages + **release publish** (tag `v*` / `workflow_dispatch` asset upload); product CI test matrix still forbidden. Durable record: `docs/adr/0001-github-actions-pages-and-release-publish.md`. No workflow YAML.

## Comments

Minted from [[issues-165-multiplatform-github-releases-curl-install]] 2026-08-02.

