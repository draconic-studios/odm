---
id: issues-168
title: "install.sh one-liner (curl|sh)"
description: "Canonical scripts/install.sh: OS/arch→triple, download from GitHub Releases, install to ~/.local/bin or ODM_INSTALL_DIR, verify SHA256 + odm --version."
status: closed
issue-type: feature-request
severity: high
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - install
  - release
---

# install.sh one-liner (curl|sh)

## Description

Add in-repo `scripts/install.sh` so users can `curl … | sh` to install a prebuilt `odm` from GitHub Releases without a Rust toolchain.

## Affected

- `scripts/install.sh` (new; canonical on `main` for raw.githubusercontent.com URL)
- Optional tiny helper tests / shellcheck if the repo already has a pattern — YAGNI otherwise

## Impact

Without install.sh, terminal one-command setup does not exist even after multi-platform assets ship.

## Proposed Fix

See Agent Brief.

## Blocked by

None for authoring the script. Runtime success needs release assets from [[issues-167-multi-target-release-workflow]] (and first cut [[issues-170-first-cut-v0-1-1]]); script may still land first with clear errors when assets are missing.

## Agent Brief

**Category:** feat  
**Summary:** Portable install script: detect OS/arch → four-triple map → download + SHA256 verify → `~/.local/bin` (or `ODM_INSTALL_DIR`) → `odm --version`. Fail clearly on Windows/unsupported.

**Bindings:**

- Parent map: [[issues-165-multiplatform-github-releases-curl-install]] Decisions
- Asset names: `odm-<version>-<triple>.tar.gz` + SHA256 sums from Releases
- Repo: `hembrow-innovations/odm` (confirm remote if needed)

**Map Decisions (v1 lock — do not reopen):**

- **Install default:** `~/.local/bin` (create if missing); no root required
- **Override:** `ODM_INSTALL_DIR`
- **Version:** latest release by default; `ODM_VERSION=` for a specific tag/version
- **Triples supported:**
  - `aarch64-apple-darwin`
  - `x86_64-apple-darwin`
  - `x86_64-unknown-linux-gnu`
  - `aarch64-unknown-linux-gnu`
- **Integrity:** verify SHA256 from release checksums before install
- **Hosting:** canonical path `scripts/install.sh` on `main` (docs will point at raw URL later)
- **Unsupported:** Windows and unknown OS/arch → non-zero exit + clear message (no silent fallback to source build)

**Desired behavior:**

1. Detect OS (`uname -s`) and arch (`uname -m`); map to one of the four triples.
2. Resolve version: `ODM_VERSION` if set, else latest GitHub Release for the repo.
3. Download matching `odm-<version>-<triple>.tar.gz` and checksum metadata from GitHub Releases.
4. Verify SHA256; refuse to install on mismatch.
5. Extract `odm` binary; install to `"${ODM_INSTALL_DIR:-$HOME/.local/bin}"` (mkdir -p); ensure executable bit.
6. Run `"$install_dir/odm" --version` (or PATH-qualified) and fail if it does not run.
7. Print short success note including install path; mention adding dir to PATH if not already on PATH.
8. Fail clearly when: unsupported platform (incl. Windows), network/API errors, missing asset, checksum fail, binary won't run.
9. Prefer `curl` with fail flags; keep script POSIX-ish enough for macOS bash/zsh and common Linux sh.
10. Do not change website/docs in this ticket (see [[issues-169-install-docs-website]]); do not tag/publish.

**Acceptance criteria:**

- [x] `scripts/install.sh` exists and is executable in intent (mode + shebang)
- [x] OS/arch maps to the four triples only
- [x] Default install dir `~/.local/bin`; `ODM_INSTALL_DIR` honored
- [x] Downloads from GitHub Releases (latest or `ODM_VERSION`)
- [x] SHA256 verified before install
- [x] Post-install `odm --version` check
- [x] Clear failure on Windows / unsupported / missing assets
- [x] No docs/website/version-cut in this ticket

**Out of scope:**

- Homebrew / apt / scoop
- Windows install path
- Cosign/minisign
- Website/README copy (child 169)
- Cutting the release tag

## Comments

Minted from [[issues-165-multiplatform-github-releases-curl-install]] 2026-08-02.

### 2026-08-02 close

- Landed `scripts/install.sh` (POSIX sh): four-triple map, `ODM_VERSION` / latest, `ODM_INSTALL_DIR` / `~/.local/bin`, SHA256SUMS or `.sha256`, `odm --version`.
- Verified: `sh -n`; Windows/unsupported-arch fail; missing asset / checksum mismatch / no-sum fail; mock HTTP e2e install + PATH note.
- Runtime success still needs release assets ([[issues-167-multi-target-release-workflow]] / [[issues-170-first-cut-v0-1-1]]).
