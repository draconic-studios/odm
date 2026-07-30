---
id: issues-4
title: "Research legacy Go ODM capabilities"
description: "AFK research: what current Go ODM implements (actions, submodules, plugins) for migration.md."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-research
---

# Research legacy Go ODM capabilities

## Question

What does this repo's Go ODM implement today (CLI surface, config shape, submodules, plugins, actions) that migration docs must call out as replace, drop, or optionally map — facts from this codebase only?

## Blocked by

None.

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

## Answer

Legacy Go ODM is a single-binary CLI (`src/main.go`) that loads root `odm.config.{yaml,json}`, runs core `add`/`remove`/`install` plus named action pipelines (`cmd`/`copy`/`env` + unfinished HashiCorp plugins), and models multi-repo as git submodules. Docs/`build-docs`, plugin manager init, and config persistence are incomplete or dead.

Full findings: `docs/reference/research/legacy-go-odm.md` (branch `research/legacy-go-odm`).
