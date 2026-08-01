---
id: issues-33
title: "Release matrix and docs"
description: "Decision: OS/arch naming, script behavior, README/CHANGELOG content."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Release matrix and docs

## Question

Exact release artifact matrix, script behavior, and consumer docs?

## Answer

### Version

- Source of truth: `crates/odm/Cargo.toml` `version`
- Workspace members may stay `0.1.0` in lockstep for the bin; libraries need not be published
- Git tag: `v0.1.0`
- CHANGELOG: Keep a Changelog style, section `## [0.1.0] - 2026-08-01`

### Artifact

```text
dist/
  odm-0.1.0-aarch64-apple-darwin.tar.gz
  # contents:
  #   odm
  #   README-release.txt
```

- Build: `cargo build -p odm --release`
- Strip when `strip` available
- Triple from `rustc -vV` host or `ODM_TARGET` env for cross
- Script fails clearly if build fails

### Script `scripts/release-build.sh`

- Reads version from `crates/odm/Cargo.toml`
- Builds release binary
- Packs tarball into `dist/`
- Prints next steps for `gh release create v$VER dist/*`
- If `ODM_RELEASE_PUBLISH=1`: runs `gh release create` (optional; not tested in CI)

### README consumer sections

- What ODM is (vision one-liner; status: usable spine)
- Install: GitHub Releases binary **or** build from source
- Quickstart: `odm init`, point at design docs + `examples/core-desk`
- Requirements: `git` on PATH; Unix shell for actions
- Link CHANGELOG, docs/reference

### Explicit non-goals this ship

- Homebrew formula
- crates.io
- Notarization
- Windows primary (may build if host is windows; not marketed day one)

## Comments

Locks ship implement detail.
