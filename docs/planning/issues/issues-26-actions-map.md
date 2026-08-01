---
id: issues-26
title: "Actions map"
description: "Wayfinder map: phase 4 — Action bundles from config and odm run dispatch."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
  - needs-triage
---

# Actions map

## Destination

Phase **4. Actions** per `docs/reference/phased-delivery.md`: Action bundles loadable from Workspace config pointers; `odm run` (or locked CLI name) dispatches them; shell-out model for command bodies; enough support that the “one desk” story works without ad-hoc wrappers for common tasks.

## Description

Chart and execute the implement map for Actions after Progen integration ([[issues-25-progen-integration-map]]).

## Done means (phase gate)

- Action bundles loadable from Workspace config pointers; `odm run` (or locked CLI name) dispatches them
- Shell-out model for command bodies; Nx/user scripts remain outside ODM
- Enough action support that the “one desk” story is usable without ad-hoc wrappers for common tasks

## Out of scope (this phase)

- HashiCorp go-plugin / npm plugin installers (dropped with Go)
- Generator / `template.toml` full depth unless explicitly pulled from sketch into this slice
- Agent-pack and worktree productization unless explicitly pulled in

## Authority

- `docs/reference/phased-delivery.md` (phase 4)
- `docs/reference/config.md` (actions / bundles)
- `docs/reference/cli.md` (`run`)
- `docs/reference/architecture.md`
- root `CONTEXT.md`

## Blocked by

- [[issues-25-progen-integration-map]]

## Decisions so far

_(none — map not charted yet)_

## Not yet specified

- Vertical slice order
- Whether generators ride this map or a later deliberate pull
- Exit-code / `--json` details for `run` beyond `cli.md` minimum
- Dogfood Workspace actions examples

## Comments

Filed as remaining delivery phase after Implement core close. Depends on Progen phase for v1 spine ordering in `phased-delivery.md`.
