---
id: issues-167
title: "Multi-target release workflow (four triples)"
description: "GitHub Actions on tag v* + workflow_dispatch: build four host triples, tarball + SHA256, upload to GitHub Releases. No PR test matrix."
status: open
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ci
  - release
  - ready-for-agent
---

# Multi-target release workflow (four triples)

## Description

Ship a release-only GitHub Actions workflow that builds `odm` for four supported host triples, packages matching tarballs + checksums, and uploads them to GitHub Releases. No product CI test matrix on PRs.

## Affected

- `.github/workflows/` (new release publish workflow)
- `scripts/release-build.sh` (reuse or extend asset naming / packaging patterns)
- GitHub Releases via `gh release` or `softprops/action-gh-release` (or equivalent)

## Impact

Without multi-target CI publish, Releases stay empty and curl/website install cannot work.

## Proposed Fix

See Agent Brief.

## Blocked by

- ~~[[issues-166-release-ci-policy]]~~ — closed; policy allows release publish + Pages

## Agent Brief

**Category:** feat  
**Summary:** Tag `v*` + `workflow_dispatch` release workflow: four triples → tarballs + SHA256 → GitHub Release assets. No PR test matrix.

**Bindings:**

- Parent map: [[issues-165-multiplatform-github-releases-curl-install]] Decisions
- Policy: [[issues-166-release-ci-policy]] / `AGENTS.md` after policy lands
- Asset naming today: `scripts/release-build.sh` → `odm-<version>-<triple>.tar.gz` containing `odm` binary

**Map Decisions (v1 lock — do not reopen):**

- **Triggers:** `push` tags matching `v*` **and** `workflow_dispatch` (manual / pre-release). Not on every PR.
- **Triples (exactly these four):**
  - `aarch64-apple-darwin`
  - `x86_64-apple-darwin`
  - `x86_64-unknown-linux-gnu`
  - `aarch64-unknown-linux-gnu`
- **Not v1:** musl, Windows
- **Artifacts:** `odm-<version>-<triple>.tar.gz` + `SHA256SUMS` (or per-asset `.sha256`)
- **No product CI test matrix** on PRs — this workflow is release-only
- Integrity v1 = SHA256 only; cosign/minisign deferred

**Desired behavior:**

1. Add a release publish workflow under `.github/workflows/`.
2. Matrix (or equivalent jobs) builds the four triples; use appropriate runners (macOS for apple triples; Linux for gnu triples). Cross-compile only if reliable; prefer native runners when available.
3. Package each binary as `odm-<version>-<triple>.tar.gz` with `odm` at archive root (match `release-build.sh`).
4. Produce release-wide or per-asset SHA256 checksums and upload with the tarballs.
5. On tag `v*`, create/update the GitHub Release for that tag and attach assets. `workflow_dispatch` may build and optionally publish (document inputs; prefer dry-run or explicit publish flag if ambiguous).
6. Reuse/extend `scripts/release-build.sh` patterns where it reduces drift; do not require a PR test matrix.
7. Do not implement `install.sh`, website copy, or cut `v0.1.1` in this ticket.

**Acceptance criteria:**

- [ ] Workflow triggers on `v*` tags and `workflow_dispatch` only (no PR test matrix)
- [ ] Four triples produce `odm-<version>-<triple>.tar.gz`
- [ ] SHA256 sums ship with the release assets
- [ ] Assets upload to GitHub Releases on tag publish path
- [ ] Policy still respected: no product CI test matrix
- [ ] `AGENTS.md` / policy child already allows this workflow

**Out of scope:**

- `scripts/install.sh`
- Docs/website honesty (except minimal workflow comments if needed)
- Version bump / CHANGELOG / first cut tag
- Windows, musl, signing/notarization
- Full `cargo test` matrix on PRs

## Comments

Minted from [[issues-165-multiplatform-github-releases-curl-install]] 2026-08-02.
