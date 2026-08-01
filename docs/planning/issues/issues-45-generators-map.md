---
id: issues-45
title: "Generators map"
description: "Wayfinder map: implement odm generate from local template bundles (v1 copy materialize)."
status: open
issue-type: feature-request
tags:
  - planning
  - issue
  - wayfinder-map
---

# Generators map

## Destination

Productize **Generators** per `docs/reference/config.md`, `cli.md`, and `env-gen-packs.md`: named scaffolds from Workspace generator bundles; CLI `odm generate` lists and materializes a **local** `template` directory into a destination path under the Workspace. Config load/merge already exists (`GeneratorDef`); finish the vertical slice through CLI, dogfood, tests, and docs.

## Notes

- **Domain:** root `CONTEXT.md` (Generator).
- **Authority:** `config.md` (bundle pointer + `template`/`url`), `cli.md` (reserved `odm generate`), `env-gen-packs.md` (sketch intent), `architecture.md` (thin module until crate earned).
- **Prereqs already landed:** `Workspace.generators` + `load_generator_bundles` (merge, duplicate error, require template and/or url); CLI stub `Generate { name }` → `not_implemented("generate")`; stub test exit 1.
- **Execution override:** ticket resolution = decision recorded **and** code/tests land. Prefer TDD.
- **Standing prefs (seeded 2026-08-01, AFK best-default):**
  - **No new crate** for v1 — `odm-core` module (e.g. `generate`) owns materialize; CLI thin adapter + JSON. Earn a crate only if file size / depth demands it later.
  - **CLI:**
    - `odm generate` — list merged generator names (human one-per-line sorted; JSON below).
    - `odm generate <name> --dest <rel-path> [--force]` — materialize local template.
  - **Local template only in v1:** `template` is a path **relative to Workspace root** pointing at an existing **directory**. Recursive copy of all files/dirs (including dotfiles except `.` / `..`). **No** `template.toml`, **no** variable substitution, **no** interactive prompts.
  - **`url`:** may appear in config/list; **run** of a generator that has only `url` (no usable `template`) → clear not-implemented / usage error exit `1` (message mentions remote generators deferred). If both set, **prefer `template`**.
  - **`--dest`:** required when generating; relative to Workspace root; must not escape root (reuse path policy). Create parent dirs as needed. Destination is the **root** of the copied tree (contents of template land under dest).
  - **Overwrite policy:** if `dest` exists and is non-empty (or is a file), fail with operation/usage error unless `--force`. With `--force`, overwrite files in place (copy over); do not delete unrelated extra files under dest.
  - **Unknown name:** exit `1` usage. Missing/invalid workspace/config: existing exit spine (`2` etc.).
  - **JSON:**
    - list: `{ "generators": [ { "name", "template", "url" } ] }` sorted by name; missing fields `null`.
    - run: `{ "generator", "dest", "copied" }` where `copied` is file count written.
  - **Proof:** unit tests for copy/force/path escape; integration test via real CLI + temp workspace; optional core-desk sample generator.
- **Deferred (map out of scope):** remote fetch/cache, template.toml / prompts / vars, dry-run, Nx/schematics interop, inline generators in Workspace config, agent packs, graph.

## Decisions so far

- Config wiring locked in design package; load already in core.
- Slice tickets: [[issues-47-generator-materialize-core]], [[issues-48-generate-cli]], [[issues-49-generate-integration-and-docs]].
- Hardening parallel (not on this map): [[issues-46-clippy-clean-tests]].
- **[[issues-47-generator-materialize-core]] closed:** `odm_core::{generate_local, generator, GenerateOutcome}` — local template recursive copy, force/empty-dest/escape/url-only semantics; unit tests; no CLI.
- Empty template dir → success `copied: 0` (confirmed in core).
- Symlink in template → copy as symlink when platform allows (unix tested).

## Out of scope

- Agent packs / `odm agent …`
- Graph
- Env injection
- Changing Action run semantics

## Blocked by

None — worktree map closed; generator config load exists.

## Comments

Seeded by swarm 2026-08-01 after empty frontier post worktree map close.
