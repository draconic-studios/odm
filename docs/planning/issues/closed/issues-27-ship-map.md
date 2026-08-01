---
id: issues-27
title: "Ship map"
description: "Wayfinder map: phase 5 — static odm binary, GitHub Releases v1, consumer install docs."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Ship map

## Destination

Phase **5. Ship** per `docs/reference/phased-delivery.md`: single static `odm` binary as the distributeable; primary channel GitHub Releases; v1 only when Implement core + Progen integration + Actions are far enough to match the vision one-liner; install/build docs updated for consumers.

## Notes

- **Authority:** `phased-delivery.md` (phase 5), `vision.md`, root README.
- **Execution override:** decisions + artifacts land (scripts, docs, verified release build).
- **Standing prefs (charted 2026-08-01, best-default autonomous):**
  - **No GitHub Actions CI** (repo policy) — release is a **manual** script + documented `gh release` upload.
  - **Targets day one:** host triple; script builds available locally; artifact names include triple.
  - **Artifact name:** `odm-<version>-<target>.tar.gz`
  - **Version:** crate `odm` 0.1.0; git tag `v0.1.0`
  - **Install docs:** README + `docs/reference/install.md`
  - **Release script:** `scripts/release-build.sh` → `dist/`
  - **Proof:** `cargo test` green; release binary smokes; dogfood gate green.

## Decisions so far

- [[issues-32-ship-slice-order-and-acceptance]] — slices + phase gate.
- [[issues-33-release-matrix-and-docs]] — matrix, naming, README, CHANGELOG, script.
- **2026-08-01 implement land:** CHANGELOG, release-build.sh, README/install.md, dist packaging, release smoke green. Post-ship polish: global flags, sketch stubs, progen body/tree/backlinks.

## Phase gate checklist

- [x] Single `odm` binary is the distributeable (release tarball contains it)
- [x] Primary channel documented as GitHub Releases
- [x] v1 spine present: core + progen + actions (prior maps closed)
- [x] Install/build docs updated for consumers (README)
- [x] Release build script lands; `dist/` gitignored
- [x] CHANGELOG records 0.1.0
- [x] `cargo test` green; release binary smokes

## Out of scope

- GitHub Actions workflows
- `serve` / MCP
- Legacy Go config readers
- Homebrew / notarization / crates.io (follow-on)

## Blocked by

- [[issues-25-progen-integration-map]] (closed)
- [[issues-26-actions-map]] (closed)

## Comments

**2026-08-01 close:** Phase gate complete. Publish when ready:

```bash
bash scripts/release-build.sh
gh release create v0.1.0 dist/odm-0.1.0-*.tar.gz --title "v0.1.0" --notes-file CHANGELOG.md
```
