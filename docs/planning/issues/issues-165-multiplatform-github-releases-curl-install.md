---
id: issues-165
title: "Multi-platform GitHub Releases + curl/website install"
description: "Build odm for supported host platforms, publish assets on GitHub Releases, and offer one-command terminal install (curl|sh) plus website download links."
status: open
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - wayfinder-map
  - release
  - install
  - ci
  - website
---

# Multi-platform GitHub Releases + curl/website install

## Destination

A visitor can get a working `odm` binary **without cloning or installing Rust**:

1. **GitHub Releases** — versioned release with **one archive per supported platform** (host triple).
2. **Terminal one-liner** — `curl … | sh` (or equivalent) detects OS/arch, downloads the matching asset, installs onto `PATH`, verifies `odm --version`.
3. **Website** — `install.html` (and index CTA) links to latest release assets and documents the curl path as primary for end users; build-from-source remains a contributor path.

## Description

Today install is **build-from-source primary**. `scripts/release-build.sh` packages the **current host** only; docs/site say Releases are “when published.” There is no multi-target CI publish and no install script.

User ask: build each platform, push to GitHub Releases, setup completely from the terminal via curl, or download from the website.

## Affected

- `scripts/release-build.sh` (and/or new `scripts/install.sh` published raw or via release)
- GitHub Actions — **policy:** extend `AGENTS.md` exception for a **release publish** workflow only (no product test matrix). See Decisions.
- `docs/reference/install.md`, root `README.md` Install section
- `website/install.html` (+ index/quickstart CTAs; e2e if copy assertions break)
- `CHANGELOG.md` when the first multi-platform release ships
- Crate/`Cargo.toml` version policy vs tag (`vX.Y.Z`) — deliberate cut off rolling Unreleased (overrides closed [[issues-157-release-hygiene-rolling-unreleased]] for this map only)

## Observed

- No assets at https://github.com/hembrow-innovations/odm/releases (or empty / source-only).
- Install docs: “No published release assets are required to install today.”
- Non-goals today: Homebrew, crates.io binary publish, signed/notarized macOS, Windows as primary channel (`docs/reference/install.md`).
- Local packaging exists: `./scripts/release-build.sh` → `dist/odm-<version>-<triple>.tar.gz`; optional `ODM_RELEASE_PUBLISH=1`.

## Impact

Without binary releases + curl install, adoption requires Rust toolchain. Blocks “download and setup completely from the terminal” and website-driven onboarding.

## Decisions so far

### AFK triage defaults (2026-08-02 self-grill)

Fog locked without further maintainer interview:

- **Platforms (v1):**
  - `aarch64-apple-darwin`
  - `x86_64-apple-darwin`
  - `x86_64-unknown-linux-gnu`
  - `aarch64-unknown-linux-gnu`
  - **Not v1:** musl (revisit if static/glibc pain appears); **Windows** deferred (honest non-goal)
- **CI triggers:** tag `v*` **and** `workflow_dispatch` (manual / pre-release). Still **no** product CI test matrix on PRs.
- **AGENTS.md policy:** extend the Actions exception from “Pages only” to “Pages + **release publish** workflow” — release-only, no test matrix. Done via [[issues-166-release-ci-policy]] → `AGENTS.md` + `docs/adr/0001-github-actions-pages-and-release-publish.md`.

- **Install default path:** `~/.local/bin` (create if missing); no root required. Override via env (e.g. `ODM_INSTALL_DIR`) acceptable.
- **`install.sh` hosting:** canonical in-repo `scripts/install.sh` on `main` (raw.githubusercontent.com URL in docs); optional copy/sync onto Pages later is nice-to-have, not blocking. Script downloads assets from **GitHub Releases** (latest or `ODM_VERSION=`).
- **First cut version:** prepare **`v0.1.1`** (bump crate + CHANGELOG section from Unreleased) — human-gated tag/publish. Do not retag `v0.1.0`. Rolling Unreleased decision (157) is overridden **only** when cutting this binary release.
- **Integrity v1:** ship `SHA256SUMS` (or per-asset `.sha256`) with the release; verify in `install.sh`. Cosign/minisign deferred.
- **Asset name:** keep `odm-<version>-<triple>.tar.gz` containing `odm` binary (match `release-build.sh` today).
- **Children minted (2026-08-02):**
  - [[issues-166-release-ci-policy]] — AGENTS.md (+ optional ADR): release publish allowed + Pages; no product CI test matrix
  - [[issues-167-multi-target-release-workflow]] — tag `v*` + workflow_dispatch; four triples; tarballs + SHA256 (blocked by 166)
  - [[issues-168-install-sh]] — OS/arch→triple; Releases download; `~/.local/bin` / `ODM_INSTALL_DIR`; checksum + `odm --version`
  - [[issues-169-install-docs-website]] — curl primary; asset links; source secondary; Windows/signing honesty
  - [[issues-170-first-cut-v0-1-1]] — version/CHANGELOG prepare; tag/publish **human-gated**
- [[issues-169-install-docs-website]] — closed: curl primary docs/site/README; source secondary; honest pre-asset wording

## Fog / open questions

- _(cleared by AFK defaults above)_

## Delivery slices (children to mint)

1. **CI policy** — ADR or AGENTS.md one-liner + short note: release-only workflow allowed; still no product test matrix.
2. **Multi-target build + release workflow** — matrix of four triples → upload tarballs + checksums on tag `v*` / `workflow_dispatch`.
3. **`install.sh`** — OS/arch → triple → download → `~/.local/bin` → checksum → `odm --version`.
4. **Docs + website** — curl primary; Releases links; source secondary; Windows/signing honesty.
5. **First cut release** — version bump to 0.1.1 + notes; **tag/publish may be `ready-for-human`** if credentials/approval needed.

## Out of scope (unless expanded later)

- Homebrew / apt / scoop / winget
- crates.io publish of the binary crate
- Signed/notarized macOS, Windows code signing
- Full product CI test matrix on every PR
- Changing core CLI behavior
- musl or Windows v1 targets

## Blocked by

None for minting. First publish may need human for tag + GitHub Release permissions.

## Agent Brief

> *This was generated by AI during triage.*

**Category:** enhancement  
**Summary:** Operationalize this map: mint five child issues from locked AFK defaults, refresh Index frontier, stop — do not cut the release or implement the full matrix in the minting session unless a child is explicitly in scope.

**Current behavior:**

- Host-only `scripts/release-build.sh`; Actions = Pages only (`AGENTS.md`).
- No `install.sh`; install docs/site = build-from-source primary; Releases empty/unpublished.
- AFK platform/CI/install/version defaults recorded under Decisions so far.

**Desired behavior:**

1. **Mint exactly five children** (high-water id across live + `closed/`; never reuse). Each: vault frontmatter, `status: open`, parent wikilink to this map, wayfinder tags, and `ready-for-agent` + Agent Brief when AFK-executable.
   - **CI policy** — `wayfinder-task`: update `AGENTS.md` (and optional short ADR under `docs/adr/`) so GitHub Actions may run a **release publish** workflow in addition to Pages; still forbid product CI test matrix.
   - **Release workflow** — `wayfinder-task`: workflow on `v*` tags + `workflow_dispatch`; build matrix of the four triples; produce `odm-<version>-<triple>.tar.gz` + SHA256 sums; upload via `gh release` / equivalent. Reuse or extend `release-build.sh` patterns; no PR test matrix.
   - **`install.sh`** — `wayfinder-task`: detect OS/arch → triple map → download from GitHub Releases (latest or `ODM_VERSION`) → install to `~/.local/bin` (or `ODM_INSTALL_DIR`) → verify checksum + `odm --version`. Fail clearly on unsupported platform (including Windows).
   - **Docs + website** — `wayfinder-task`: install.md, README Install, `website/install.html` (+ index/quickstart CTAs): curl one-liner primary; asset links; source secondary; honesty on Windows/signing. Run website e2e if HTML changes. May land before first tag if copy says “after first release” honestly — prefer coordinating with first cut.
   - **First cut `v0.1.1`** — version bump + CHANGELOG; prepare notes. **Tag and `gh release` publish are human-gated** unless the environment already has permission and maintainer policy allows agent tag — default: agent prepares, leaves publish steps for human (`ready-for-human` on publish-only child or section).
2. **Index** — this map under Maps; frontier = open unblocked children (lowest id first).
3. **Link back** — append child wikilinks under Decisions; blocking edges only if real (docs may soft-depend on first assets existing).
4. **Stop** after mint (+ optional policy child if trivial). Do not implement all slices in one session.

**Key interfaces:**

- `AGENTS.md` Actions policy line
- `scripts/release-build.sh` asset naming
- GitHub Releases API / `gh release`
- Install honesty surfaces (README, `docs/reference/install.md`, `website/install.html`)
- Crate version in workspace `Cargo.toml` files for 0.1.1 cut

**Acceptance criteria:**

- [x] Five children filed with correct ids, wikilinks, wayfinder tags
- [x] Defaults above reflected in each child’s brief (four triples; `~/.local/bin`; SHA256; v0.1.1 human-gated publish)
- [x] CI policy child forbids product test matrix
- [x] Index frontier lists new children
- [x] This map stays open until children complete
- [x] Minting session does not silently tag/publish without human gate

**Out of scope:**

- Windows / musl / Homebrew / signing
- Product CI on every PR
- Closing this map before children complete
- Docs residual pass unrelated to install ([[issues-164-update-docs-and-website]])

## Comments

### Filed

Filed from maintainer request: multi-platform builds → GitHub Releases; curl terminal setup; website download path. Related: [[issues-157-release-hygiene-rolling-unreleased]], [[issues-164-update-docs-and-website]].

### 2026-08-02 triage

> *This was generated by AI during triage.*

- Category: enhancement (`feature-request`)
- State: `needs-triage` → `ready-for-agent`
- Redundancy: not implemented (host-only package script; Pages-only Actions; no install.sh; Releases empty). Closed 33/136/157 are related policy/hygiene only.
- Prior rejection: `.out-of-scope/` empty
- Self-grill: locked platforms, CI triggers, install dir, hosting, v0.1.1 human-gated cut, SHA256 v1 — see Decisions so far
- Agent Brief: mint five children from defaults; do not publish in mint session

### 2026-08-02 mint

- Claimed → minted [[issues-166-release-ci-policy]], [[issues-167-multi-target-release-workflow]], [[issues-168-install-sh]], [[issues-169-install-docs-website]], [[issues-170-first-cut-v0-1-1]].
- Map stays `open`; `ready-for-agent` dropped (children hold the frontier). No tag/publish.
- Index: drop stale 164; frontier = 166–170 (167 blocked by 166).
