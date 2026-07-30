---
id: issues-6
title: "Progen scope and federation model"
description: "Lock multi-progen query scope, --progen flags, named combos, no in-store cross-links."
status: open
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Progen scope and federation model

## Question

How do multiple progens participate in query/context — default all candidates, `--progen` single or list, named combinations only in ODM config, no cross-store linking inside a progen store — and how is that documented in `docs/reference/progen.md`?

## Blocked by

- [[issues-2-domain-glossary]]
- [[issues-3-research-progenitor-surface]]

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Charting intent: unrelated products stay separate; within a product, impl vs marketing vs support can be separate progens with config-side combination only.
