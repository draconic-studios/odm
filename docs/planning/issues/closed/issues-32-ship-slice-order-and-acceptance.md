---
id: issues-32
title: "Ship slice order and acceptance"
description: "Decision: vertical slice order and phase-5 map-close checklist."
status: closed
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Ship slice order and acceptance

## Question

What is the vertical implement order for phase 5, and when is the Ship map closable?

## Answer

**Slice order:**

1. CHANGELOG.md + version alignment (0.1.0) across bin crate
2. `scripts/release-build.sh` → `dist/odm-<ver>-<triple>.tar.gz`
3. Consumer README install/build + release channel docs
4. Optional publish path documented (`gh release create`) without requiring network in tests
5. Smoke: release-profile binary `--version`, `init`, core-desk integration still green
6. Close map when checklist green

**Map-close checklist (phase gate):**

- [ ] Single `odm` binary is the distributeable (release tarball contains it)
- [ ] Primary channel documented as GitHub Releases
- [ ] v1 spine present: core + progen + actions (prior maps closed)
- [ ] Install/build docs updated for consumers (README)
- [ ] Release build script lands; `dist/` gitignored
- [ ] CHANGELOG records 0.1.0
- [ ] `cargo test` green; release binary smokes

## Comments

Autonomous chart 2026-08-01. No GH Actions per AGENTS.md.
