---
id: issues-169
title: "Install docs + website (curl primary)"
description: "Make curl|sh the primary install path in install.md, README, and website; link release assets; source secondary; honest Windows/signing. E2e if HTML changes."
status: open
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - docs
  - website
  - install
  - ready-for-agent
---

# Install docs + website (curl primary)

## Description

Flip install honesty surfaces so end users get a working `odm` via curl one-liner / release downloads first; build-from-source becomes the contributor path. Stay honest about Windows and unsigned binaries.

## Affected

- `docs/reference/install.md`
- Root `README.md` Install section
- `website/install.html` (+ index/quickstart CTAs as needed)
- Website Playwright e2e if HTML assertions break
- Optional: `docs/reference/cli.md` or other install pointers only if they still claim source-only

## Impact

Docs/site still say Releases are optional / source-primary; that blocks website-driven onboarding once assets exist.

## Proposed Fix

See Agent Brief.

## Blocked by

None hard. Prefer coordinating with [[issues-168-install-sh]] (script URL) and assets from [[issues-167-multi-target-release-workflow]] / [[issues-170-first-cut-v0-1-1]]. If landing before first tag, copy must say so honestly (“after first multi-platform release” / link latest when present).

## Agent Brief

**Category:** docs  
**Summary:** Curl one-liner primary; GitHub Releases asset links; source secondary; Windows/signing honesty. Run website e2e if HTML changes.

**Bindings:**

- Parent map: [[issues-165-multiplatform-github-releases-curl-install]] Decisions
- Install script: [[issues-168-install-sh]] → `scripts/install.sh` raw URL on `main`
- Releases: `https://github.com/hembrow-innovations/odm/releases`
- Defaults: four triples; install to `~/.local/bin`; SHA256 verify in script; first cut `v0.1.1` human-gated

**Map Decisions (v1 lock — do not reopen):**

- **Primary:** terminal curl|sh installing from GitHub Releases
- **Also primary-adjacent:** direct download links for the four platform tarballs (latest release)
- **Secondary:** build from source (Rust toolchain) for contributors
- **Not v1 / honest non-goals:** Windows primary channel, Homebrew, signed/notarized macOS
- **Integrity messaging:** SHA256 verified by install.sh; no cosign claim
- Do not retcon closed residual docs pass ([[issues-164-update-docs-and-website]]) beyond install/release surfaces

**Desired behavior:**

1. `docs/reference/install.md` — curl one-liner first; document `ODM_VERSION`, `ODM_INSTALL_DIR`, supported triples, checksum behavior; source build section demoted; Windows/signing honesty.
2. `README.md` Install — same priority; short and accurate.
3. `website/install.html` (+ index/quickstart CTAs) — curl primary; asset links to latest release (or honest “when published” only if assets truly absent — prefer real links once [[issues-170-first-cut-v0-1-1]] exists).
4. Run website Playwright e2e if HTML/copy assertions change; fix tests to match new honesty.
5. Do not implement workflow/install.sh/version bump here unless already present and only docs need wiring.

**Acceptance criteria:**

- [ ] install.md + README: curl primary, source secondary
- [ ] Website install (+ CTAs) match; asset links or honest pre-release wording
- [ ] Windows / unsigned binary honesty present
- [ ] Four triples / `~/.local/bin` / SHA256 reflected where install is documented
- [ ] Website e2e green if HTML changed
- [ ] No tag/publish in this ticket

**Out of scope:**

- Implementing release workflow or install.sh from scratch
- Homebrew / Windows installer
- Unrelated residual docs (closed 164 scope)
- Cutting `v0.1.1`

## Comments

Minted from [[issues-165-multiplatform-github-releases-curl-install]] 2026-08-02.
