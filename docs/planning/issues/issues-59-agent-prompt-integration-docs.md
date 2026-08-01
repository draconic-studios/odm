---
id: issues-59
title: "Agent prompt integration tests and docs"
description: "CLI integration coverage for agent prompt; promote docs from sketch; CHANGELOG; keep start stub."
status: open
issue-type: feature-request
severity: low
tags:
  - planning
  - issue
  - ready-for-agent
  - wayfinder
  - wayfinder-task
---

# Agent prompt integration tests and docs

## Description

After thin prompt lands, lock behavior with integration tests and make reference docs / CHANGELOG honest: prompt v1 thin context package; start still sketch.

## Affected

- `crates/odm/tests/cli_agent_pack.rs` and/or new `cli_agent_prompt.rs`
- `docs/reference/cli.md`
- `docs/reference/env-gen-packs.md`
- `docs/reference/architecture.md` (stub sentence for start/prompt if still absolute)
- `CHANGELOG.md` [Unreleased]
- Optional one-liner in `examples/core-desk/README.md` dogfood (`odm agent prompt welcome` after reindex)
- Parent map [[issues-55-post-v1-hardening-map]] Decisions on close

## Impact

Without proof + docs, prompt stays invisible and sketch matrix lies.

## Proposed Fix

See Agent Brief.

## Blocked by

- [[issues-58-agent-prompt-thin]]

## Agent Brief

**Category:** test + docs  
**Summary:** Integration tests + docs honesty for agent prompt v1; start remains stub.

**Bindings:**

- Behavior from [[issues-58-agent-prompt-thin]] and implemented CLI
- Patterns: `crates/odm/tests/cli_agent_pack.rs`, `progen_vault.rs` workspace fixtures
- Parent map: [[issues-55-post-v1-hardening-map]]

**Tests (minimum):**

1. Workspace with one progen + note → `agent prompt <id>` exit 0, stdout contains id or title/body
2. `--json` → parseable JSON with anchor id
3. Unknown id → exit 4
4. `agent start` still exit 1 not-implemented
5. Reuse temp workspace helpers; no network

**Docs:**

1. **cli.md:** move `agent prompt` from pure sketch to **v1 thin** (context work-package); document args and JSON; keep `agent start` sketch/not-implemented
2. **env-gen-packs.md:** Agent start/prompt section — prompt landed thin; start still reserved
3. **Full vs sketch matrix** updated
4. **CHANGELOG** [Unreleased] Added bullet for `odm agent prompt`
5. Optional core-desk README one-liner after progen reindex
6. Close map destination notes when this ticket closes (append Decisions / Answer on map if last child)

**Acceptance criteria:**

- [ ] Integration tests cover success, json, missing id, start stub
- [ ] cli.md + env-gen-packs.md honest about prompt v1 vs start stub
- [ ] CHANGELOG Unreleased mentions agent prompt
- [ ] `cargo test` green
- [ ] Map [[issues-55-post-v1-hardening-map]] updated when destination met (if 56–58 already closed)

**Out of scope:**

- agent start implementation
- New progen node types
- Graph

## Acceptance

- [ ] Agent Brief acceptance criteria all met
