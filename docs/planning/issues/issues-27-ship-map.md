---
id: issues-27
title: "Ship map"
description: "Wayfinder map: phase 5 — static odm binary, GitHub Releases v1, consumer install docs."
status: reviewing
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
  - ready-for-agent
---

# Ship map

## Destination

Phase **5. Ship** per `docs/reference/phased-delivery.md`: single static `odm` binary as the distributeable; primary channel GitHub Releases; v1 only when Implement core + Progen integration + Actions are far enough to match the vision one-liner; install/build docs updated for consumers.

## Notes

- **Authority:** `phased-delivery.md` (phase 5), `vision.md`, root README.
- **Execution override:** decisions + artifacts land (scripts, docs, verified release build).
- **Standing prefs (charted 2026-08-01, best-default autonomous):**
  - **No GitHub Actions CI** (repo policy) — release is a **manual** script + documented `gh release` upload.
  - **Targets day one:** host triple (dev/mac) + `x86_64-unknown-linux-musl` / `aarch64-apple-darwin` / `x86_64-apple-darwin` as documented build matrix; script builds whatever is available locally and records the triple in artifact names.
  - **Artifact name:** `odm-<version>-<target>.tar.gz` containing `odm` binary + `LICENSE` if present + short `README-release.txt`.
  - **Version:** crate `odm` version is source of truth (`crates/odm/Cargo.toml`); git tag `v<version>`.
  - **Semver:** 0.1.0 = first public v1-capable spine (core+progen+actions); CHANGELOG at repo root.
  - **Install docs:** README consumer path — download release **or** `cargo install --path crates/odm` / `cargo install --git …`.
  - **Release script:** `scripts/release-build.sh` produces `dist/` artifacts; does not push or create GitHub release unless `ODM_RELEASE_PUBLISH=1` and `gh` authenticated.
  - **Proof:** `cargo test` green; release build produces runnable binary; `odm --version` matches; smoke `odm init` + core-desk gate still green.
- **Skills:** none special.

## Decisions so far

- [[issues-32-ship-slice-order-and-acceptance]] — slices + phase gate.
- [[issues-33-release-matrix-and-docs]] — matrix, naming, README, CHANGELOG, script.

## Not yet specified

- Homebrew / secondary channels (follow-on)
- Signed/notarized macOS binaries (follow-on)
- crates.io publish (not required for binary-first ship)

## Out of scope

- GitHub Actions workflows
- `serve` / MCP
- Legacy Go config readers
- Sketch features as ship blockers

## Blocked by

- [[issues-25-progen-integration-map]] (closed)
- [[issues-26-actions-map]] (closed)

## Comments

Charted 2026-08-01 after Actions close.
