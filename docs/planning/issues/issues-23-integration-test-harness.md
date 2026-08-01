---
id: issues-23
title: "Integration test harness"
description: "Lock how cargo tests drive the odm binary against temp core-desk + fixtures."
status: reviewing
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
  - ready-for-agent
---

# Integration test harness

## Question

How do automated integration tests invoke the `odm` binary against a temporary copy of `examples/core-desk` and its bare fixtures (setup/teardown, PATH to git, assertion style), and what minimum scenarios gate “core done”?

## Blocked by

- [[issues-15-vertical-slice-order-and-core-acceptance]]
- [[issues-19-core-desk-example-layout]]

## Answer

**Recommended lock:**

### Location / deps

- Integration tests in **`crates/odm/tests/`** (e.g. `core_desk.rs`, helpers module).
- Dev-deps: `assert_cmd`, `predicates`, `tempfile`, `serde_json`.
- Binary: `Command::cargo_bin("odm")` / `assert_cmd`.
- Require **`git` on PATH**; if `which git` fails, **`#[ignore]` or early return skip** with message (same spirit as odm-git tests).

### Setup

1. Copy `examples/core-desk` tree into a **unique tempdir** (include `fixtures/*.git` bare repos).
2. Optionally `git init` + user.name/email in temp Workspace when testing pin/gitignore.
3. Invoke with `--root <temp>` (explicit; no reliance on cwd discovery for harness stability).
4. Offline only — no network.

### Assertion style

- Exit codes per issues-18.
- Human: stderr/stdout `predicates` contains.
- JSON: `serde_json::Value` path assertions (not giant golden files day one).
- One tempdir per test; parallel-safe.

### Minimum gate scenarios (map close)

1. `init` on empty temp dir → config created, exit 0; `--json` has `root` + `git`
2. `project add` managed from fixture url OR use pre-seeded core-desk config
3. `sync` → clones alpha/beta from bare fixtures
4. `pin status` / `pin apply` (after pin file auto-created)
5. `status --json` → on_disk / pin_state populated
6. `doctor` → exit 0 or only warns; `doctor --fix` repairs gitignore if drifted

Additional recommended (not all required for first green): origin mismatch fails; unknown project name exit 1; dirty pin apply fails without `--force`.

### Policy

- No GitHub Actions; `cargo test` local only.
- Harness grows with slices; unbuilt verbs not asserted until implemented.

## Comments

Parent map: [[issues-14-implement-core-map]]

Recommended decision locked for agent implement 2026-08-01.
