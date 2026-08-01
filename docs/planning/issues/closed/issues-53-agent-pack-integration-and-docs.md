---
id: issues-53
title: "Agent pack integration and docs"
description: "CLI integration tests, optional core-desk pack fixture, promote agent pack docs from pure sketch."
status: closed
issue-type: feature-request
severity: medium
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-task
---

# Agent pack integration and docs

## Description

Prove `odm agent pack` end-to-end and make reference docs honest: v1 local install/link/list landed; start/prompt/marketplace still deferred.

## Affected

- `crates/odm/tests/` (new `cli_agent_pack.rs` or similar)
- `examples/core-desk/` optional tiny pack dir
- `docs/reference/cli.md`, `env-gen-packs.md`, maybe `architecture.md` / `CHANGELOG.md`
- Close parent map [[issues-50-agent-packs-map]] when done

## Impact

Without proof + docs, pack feature is untrusted and docs still say pure sketch.

## Proposed Fix

See Agent Brief.

## Blocked by

None — [[issues-52-agent-pack-cli]] closed.

## Agent Brief

**Category:** feature  
**Summary:** Integration tests + docs/CHANGELOG for agent pack v1.

**Bindings:**

- Map [[issues-50-agent-packs-map]]
- Behavior locked by 51/52
- Doc style: no markdown tables (`AGENTS.md`); match generate docs promotion pattern from issues-49

**Desired behavior:**

1. **Integration tests** (temp workspace + temp `--home`):
   - list empty
   - install then list (human + json)
   - link then list (`cfg(unix)` if needed)
   - force overwrite
   - missing source → non-zero
   - `agent start` still exit 1 not-implemented
2. **Optional dogfood:** small directory under `examples/core-desk` (e.g. `agent-packs/demo/`) referenced from example README one-liner — skip if awkward; tests alone OK.
3. **Docs:**
   - `cli.md`: `odm agent pack` full v1 local; start/prompt remain sketch
   - `env-gen-packs.md`: Agent packs section promoted like Generators (v1 local install/link/list; deferred list explicit)
   - Full vs sketch matrix updated
   - `CHANGELOG.md` [Unreleased] entry
4. **Close map 50** with Answer summarizing slice; move map to `closed/` when destination met.

**Acceptance criteria:**

- [x] Integration tests cover list/install/link/force/error/start-stub
- [x] Docs no longer call pack list/install/link pure unimplemented sketch
- [x] Deferred items (start, prompt, marketplace, manifest, config declarations) still explicit
- [x] CHANGELOG unreleased notes the feature
- [x] Map 50 closed with Answer
- [x] `cargo test` green

**Out of scope:**

- Implementing start/prompt
- Graph / env
- README quickstart overhaul beyond what [[issues-54-readme-post-010-docs-drift]] covers

## Acceptance

- [x] Agent Brief acceptance criteria all met

## Answer

Landed agent pack v1 proof + docs:

- **`crates/odm/tests/cli_agent_pack.rs`** — list empty, install+list human/json, force (exit 3), link (unix), missing source (exit 4), start/prompt stubs (exit 1)
- **core-desk dogfood** — `agent-packs/demo/skills/hello.md` + README one-liner
- **Docs** — `cli.md` / `env-gen-packs.md` / `architecture.md` promote pack to v1 local; start/prompt/marketplace deferred kept honest; `CHANGELOG.md` Unreleased
- **Map [[issues-50-agent-packs-map]]** closed with this ticket

`cargo test` green.

## Comments

Seeded with map issues-50.
