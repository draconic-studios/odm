---
id: issues-49
title: "Generate integration tests and docs"
description: "CLI integration tests, optional core-desk generator fixture, promote generate from sketch in reference docs."
status: open
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
  - ready-for-agent
---

# Generate integration tests and docs

## Description

After CLI works, lock behavior with integration tests and update reference docs / dogfood so generate is honest v1 (local template only), not a sketch stub.

## Affected

- `crates/odm/tests/`
- `examples/core-desk/` (optional small generator)
- `docs/reference/cli.md`, `env-gen-packs.md`, maybe `CHANGELOG.md` / README mention
- Parent map close

## Impact

Without proof + docs, generate remains half-secret and regressions slip.

## Proposed Fix

See Agent Brief.

## Blocked by

- ~~[[issues-48-generate-cli]]~~ (closed)

## Agent Brief

**Category:** test + docs  
**Summary:** End-to-end tests and doc promotion for local `odm generate` v1; close the generators map when done.

**Bindings:**

- Parent map [[issues-45-generators-map]] destination + deferred list
- Implemented CLI/core from 47–48
- Prior art: `crates/odm/tests/cli_worktree.rs`, `actions_run.rs`, core-desk layout

**Desired behavior:**

1. **Integration test** (temp workspace, real binary via assert_cmd or existing harness):
   - Write config + generator bundle + template dir with ≥1 nested file
   - `odm --root … generate` lists the name
   - `odm --root … generate <name> --dest out/x` creates files; assert contents
   - Second run without `--force` fails; with `--force` succeeds
   - Unknown generator exit `1`
   - url-only generator in bundle → run exit `1` with clear stderr (optional if awkward in same file)
2. **Dogfood (preferred):** add `examples/core-desk/generators/…` + config `generators:` pointer + tiny template (e.g. `hello.txt`); document one command in core-desk README
3. **Docs:**
   - `cli.md`: mark `odm generate` as **v1 local template** (not pure sketch); document `--dest` / `--force` / JSON; keep remote deferred
   - `env-gen-packs.md`: note local copy v1 landed; keep deferred list honest
   - `CHANGELOG.md` under Unreleased or 0.1.x note: generate local + worktree if still listed as stub only — fix any **false** “project worktree stub” claims left from 0.1.0 notes if still wrong
4. **Map:** append Decisions + Answer on [[issues-45-generators-map]], set map `status: closed`, move map to `closed/` when checklist met
5. `cargo test` green

**Acceptance criteria:**

- [ ] Integration test covers list, generate, force, unknown name
- [ ] Reference docs no longer call generate a pure unimplemented sketch
- [ ] Deferred remote/templating still explicit
- [ ] Map 45 closed with Answer when this ticket completes
- [ ] `cargo test` green

**Out of scope:**

- Agent packs
- Remote generators
- Graph

## Acceptance

- [ ] Agent Brief acceptance criteria all met
