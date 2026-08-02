---
id: issues-155
title: "Align website with current project truth"
description: "Audit and correct website/ pages so claims, install, CLI, guides, and concepts match the shipped product."
status: closed
issue-type: observation
severity: medium
tags:
  - planning
  - issue
  - docs
  - website
  - ready-for-agent
---

# Align website with current project truth

## Description

The static site under [`website/`](../../../../website/) should match current product truth: status, install path, quickstart, CLI surface, concepts/features, and guides. Audit every public page against CLI help, `CHANGELOG.md`, root/README, `docs/reference/*`, and examples; fix stale, wrong, or aspirational claims written as if shipped.

Related: [[issues-154-update-readme-correct-info]] (README honesty). Prior website delivery: [[issues-109-project-website-github-pages-map]] (closed).

## Affected

- `website/index.html`
- `website/install.html`, `website/quickstart.html`
- `website/cli.html`, `website/config.html`
- `website/concepts.html`, `website/features.html`, `website/guides.html`
- `website/guide-*.html` (workspace, projects, worktrees, progen, agents, actions)
- Possibly `website/README.md` and e2e expectations if copy/nav changes

## Observed

Triage audit (2026-08-02): site is **mostly accurate** after closed website work (#109–#118, #136). Not significantly stale; residual wrong/stale lines are wording bugs and omissions (see Agent Brief). No invented shipped commands (`agent start`, graph, serve correctly sketch/deferred).

## Impact

Public docs mislead visitors and agents on federation, pack paths, and exit codes if left unfixed.

## Proposed Fix

1. Apply the concrete copy fixes in the Agent Brief (highest severity first)
2. Keep honesty on sketch vs shipped
3. Smoke-check internal links and existing Playwright e2e after copy edits

## Agent Brief

**Category:** enhancement
**Summary:** Fix residual website copy drift so federation, pack sources, exit codes, and a few CLI omissions match shipped product truth — not a site rewrite.

**Current behavior:**
Public pages under the project website largely match v0.1.0 (install primary path, command tree, worktrees/generate/agent packs shipped, `agent start` sketch). Prior honesty/publish work held. Remaining defects from triage audit:

1. **Progen guide** — text treats `find` **and** `context` as federated. Product truth: only `find` federates; `context` is single-root / in-store (with progen selection as documented in reference CLI/progen docs).
2. **Agents guide** — pack source described as relative-under-Workspace only. Product truth: relative (no escape) **or absolute** is allowed (`odm agent pack install --help` / reference CLI).
3. **Workspace guide** — exit code `3` tied to action passthrough. Product truth: executed actions pass through the **action’s** exit code; pre-exec failures use ODM codes (see reference CLI).
4. **Omissions:** FTS whole-token (not substring) caveat missing near find examples; CLI page omits `find --limit` (default 200); features/guides soft-lump find/context into progen “façade”; pack install/link `--force` unmentioned; quickstart dogfood omits core-desk full-tour script; index status chip thinner than root README (optional align).

**Desired behavior:**
- `context` never described as federated; federation language reserved for `find`
- Pack source docs allow absolute paths; relative stays no-escape under Workspace
- Exit-code docs: `3` is ODM operation failure where applicable; `run` documents action exit passthrough separately
- Find docs mention `--limit` and whole-token FTS semantics (match reference CLI honesty)
- Features/guides distinguish top-level find/context from `odm progen` façade verbs
- Agents guide mentions pack `--force` where install/link overwrite behavior is described
- Optional: quickstart → core-desk dogfood script; index status closer to README one-liner
- Sketch/deferred list unchanged in spirit: `agent start`, interactive init, remote generate, marketplace, serve/MCP stay not-shipped
- Internal links remain unbroken; Playwright smoke still passes (or update selectors only if copy changes break them)

**Key interfaces:**
- Website public HTML copy (guides, CLI, features, quickstart, index)
- Product truth: live `odm` help, root README status, CHANGELOG, reference CLI/progen/install/env-gen-packs docs
- Existing website Playwright suite (smoke after edits)

**Acceptance criteria:**
- [x] No page claims `context` is federated; federation attributed to `find` only
- [x] Agents guide documents absolute **or** workspace-relative pack sources
- [x] Workspace (and CLI if present) exit-code text does not claim action failures always exit `3`; `run` passthrough noted
- [x] CLI (or progen guide) documents `find --limit` and FTS whole-token match semantics
- [x] Features/guides do not list find/context as progen façade subcommands
- [x] Pack `--force` mentioned where install/link overwrite is discussed
- [x] No new aspirational-as-shipped commands
- [x] Internal link crawl / existing Playwright e2e still green (adjust tests only if assertions pin old wrong copy)

**Out of scope:**
- Root README polish ([[issues-154-update-readme-correct-info]])
- Full site redesign, new pages, visual/theme work
- Changing CLI or runtime behavior
- Re-opening a full website rebuild (#109-era delivery)

## Comments

Filed from request to keep the website aligned and correct with the current project; triage should list concrete wrong lines before `ready-for-agent`.

### 2026-08-02 triage

> *This was generated by AI during triage.*

- **Category:** enhancement (docs honesty); kept `issue-type: observation`
- **State:** `needs-triage` → `ready-for-agent`
- **Redundancy check:** closed #109–#118 / #136 delivered site + prior honesty; residual wrong lines remain (not fully implemented)
- **Prior rejection:** no matching `.out-of-scope/` entry
- **Verify:** audited public HTML vs `odm 0.1.0` help/README/CHANGELOG/reference — mostly accurate; top defects listed in Agent Brief

### 2026-08-02 closed

Residual website copy drift fixed across guides/CLI/features/quickstart/index. Playwright e2e 33/33 green.
