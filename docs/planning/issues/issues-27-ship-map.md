---
id: issues-27
title: "Ship map"
description: "Wayfinder map: phase 5 — static odm binary, GitHub Releases v1, consumer install docs."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
  - needs-triage
---

# Ship map

## Destination

Phase **5. Ship** per `docs/reference/phased-delivery.md`: single static `odm` binary as the distributeable; primary channel GitHub Releases; v1 only when Implement core + Progen integration + Actions are far enough to match the vision one-liner; install/build docs updated for consumers.

## Description

Chart and execute the release map after Actions ([[issues-26-actions-map]]). Core-only is not v1 Ship.

## Done means (phase gate)

- Single **static `odm` binary** as the distributeable
- Primary channel: **GitHub Releases**
- **v1** requires Implement core + Progen integration + Actions far enough to match the vision one-liner (poly-repo desk, multi-Progen, one CLI)
- Install/build docs updated for consumers

## Out of scope (this phase unless decided otherwise)

- Concrete OS/arch matrix as a design-time lock (choose at ship time)
- Homebrew or other secondary channels
- `serve` / MCP
- Legacy Go config readers or submodule migration tools

## Authority

- `docs/reference/phased-delivery.md` (phase 5)
- `docs/reference/vision.md`
- root README (consumer install story)

## Blocked by

- [[issues-14-implement-core-map]] (closed)
- [[issues-25-progen-integration-map]]
- [[issues-26-actions-map]]

## Decisions so far

_(none — map not charted yet)_

## Not yet specified

- OS/arch matrix at ship time
- Release automation vs manual (`cargo` build + upload); repo policy historically no GitHub Actions — confirm at charting
- Versioning / semver policy and changelog location
- Whether secondary channels (brew, etc.) get a follow-on issue

## Comments

Filed as remaining delivery phase after Implement core close. Dogfood builds during phases 2–4 are not “ODM v1 shipped.”
