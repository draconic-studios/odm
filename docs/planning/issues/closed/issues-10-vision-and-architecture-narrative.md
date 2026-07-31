---
id: issues-10
title: "Vision and architecture narrative"
description: "Lock product one-liner, ownership boundaries, crate layout story for vision.md + architecture.md."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Vision and architecture narrative

## Question

What is the locked product definition, system diagram narrative, and ODM-owns vs progen-owns vs shell-out boundaries for `docs/reference/vision.md` and `docs/reference/architecture.md` (including crate layout as design intent, not implementation)?

## Blocked by

- [[issues-2-domain-glossary]]

## Answer

Locked in `docs/reference/vision.md` and expanded `docs/reference/architecture.md`.

- **One-liner** — poly-repo workspace OS for humans + AI agents; one config, one CLI; Projects + Progens; no submodules or second brain product.
- **Audience** — humans and agents equal on one desk.
- **Jobs** — many checkouts one desk; multi-store memory; agents on the desk; one binary UX.
- **Non-goals** — not build system / forge / second knowledge product / serve-MCP v1 / submodules / product-repo-as-workspace.
- **Ownership** — ODM: workspace, git lifecycle, federation, CLI, actions dispatch, packs, gitignore. Progen crates: single-store engine. Shell-out: git, action bodies, agent runtimes. User: auth, commits, content.
- **System narrative** — human/agent → CLI → core → git | progen façade | actions | packs; config sole layout truth.
- **Crates** — `odm` bin + `odm-core` / `odm-git` / `odm-progen` / `odm-actions` / `odm-agent`; progen upstream; depend inward; one binary.
- **Doc split** — vision = framing; architecture = `.odm/` + ownership + narrative + crates; CONTEXT = terms.

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Grilled with maintainer; docs written.
