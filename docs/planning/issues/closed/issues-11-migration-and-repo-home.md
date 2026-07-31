---
id: issues-11
title: "Migration and repo-home story"
description: "Lock how this repo becomes Rust ODM home; what Go dies; phased docs for migration.md."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Migration and repo-home story

## Question

What does `docs/reference/migration.md` and `phased-delivery.md` claim — this repo as Rust monorepo home, Go as inspiration then remove, binary distribution, doc-phase vs implement-phase boundary — without performing the code migration in this map?

## Blocked by

- [[issues-4-research-legacy-go-odm]]
- [[issues-10-vision-and-architecture-narrative]]

## Answer

Locked in `docs/reference/phased-delivery.md` (and thin cross-links in `vision.md` + map). **No `migration.md`.**

- **Greenfield** — Rust-first; not a Go port. Archive: tag `legacy-go-archive` + `research/legacy-go-odm.md`. No legacy config reader.
- **Repo home** — this repo is the permanent Rust monorepo home.
- **Design map** — docs only; no Cargo/crates/binary.
- **Phases** — Design → Implement core → Progen integration → Actions → Ship. Sketches optional, not ship gates.
- **After design** — later implement map(s) along that spine (core first).
- **Ship** — static `odm` + GitHub Releases intent; OS/arch/brew deferred. **v1** = core + progen + actions (dogfood builds earlier OK).
- **Per phase** — short done-means + out-of-phase; no estimates/ticket dumps.

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Grilled with maintainer; `phased-delivery.md` written; `migration.md` dropped from doc tree.
