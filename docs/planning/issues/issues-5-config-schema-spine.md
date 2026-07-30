---
id: issues-5
title: "Config schema spine"
description: "Lock odm.config.yaml shape: layout, projects, progens, named combos, actions, generators hooks."
status: open
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Config schema spine

## Question

What is the v1 `odm.config.yaml` schema for the design docs — maps vs lists, layout templates, `projects`, `progens` (paths anywhere), named progen combinations, actions, and what is explicitly deferred — written to `docs/reference/config.md`?

## Blocked by

- [[issues-2-domain-glossary]]
- [[issues-3-research-progenitor-surface]]

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Standing prefs: every path explicit or templated; projects keyed by name; no submodule fields; progens named and referenced; config is sole layout source of truth.
