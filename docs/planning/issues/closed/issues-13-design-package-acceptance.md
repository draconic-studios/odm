---
id: issues-13
title: "Design package acceptance"
description: "Define when the docs package is review-complete and this map can close."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
---

# Design package acceptance

## Question

What checklist means the design package is done for human review (required files present, no unresolved conflicts with Decisions so far, open questions listed, ready for implement map) — and does this map close when that checklist passes?

## Blocked by

- [[issues-5-config-schema-spine]]
- [[issues-6-progen-scope-and-federation]]
- [[issues-7-multi-git-and-pins]]
- [[issues-8-odm-dot-directory-contract]]
- [[issues-9-cli-surface-v1]]
- [[issues-11-migration-and-repo-home]]
- [[issues-12-sketch-sections-depth]]

## Answer

Checklist is necessary and sufficient as the definition of done. Closing this ticket (then the parent map) is the human gate — no extra ceremony.

### Acceptance checklist (all must be true)

1. **Files present**
   - root `CONTEXT.md`
   - full-spec: `docs/reference/vision.md`, `architecture.md`, `config.md`, `cli.md`, `progen.md`, `multi-git.md`, `phased-delivery.md`
   - sketches: `docs/reference/worktrees.md`, `graph.md`, `env-gen-packs.md`
   - **not** required: `concepts.md` (folded into CONTEXT), ADRs, research notes

2. **Depth**
   - full-spec files: enough that **Implement core** can start without reopening fundamentals (ownership, non-goals, CONTEXT-aligned names, consistent cross-links). Not full flag tables or implement-only detail already deferred
   - sketch files: bar from [[issues-12-sketch-sections-depth]] (intent, placement/ownership, CLI names reserved, explicit deferred/non-goals; not a Ship gate)
   - CONTEXT: every product noun used in the package; no “brain”; no implementation detail

3. **No unresolved conflicts**
   - each closed decision on the map matches its cited reference file
   - required files do not contradict locked choices
   - CONTEXT vocabulary is what reference docs use
   - map **Out of scope** and **Not yet specified** are not silently promoted to full-spec

4. **Open questions**
   - canonical register = map **Not yet specified** (may be non-empty)
   - nothing that still blocks the Design package left unnamed
   - local deferred bullets in refs must not invent design blockers missing from the map list

5. **Ready for implement map means**
   - safe to chart a later **Implement core** map only
   - this map does not chart or start implementation
   - sketches and Not-yet-specified items are not implement-core prerequisites unless a later map pulls them in

### Close sequence

1. Checklist recorded here + Done-means in `phased-delivery.md`
2. Run checklist once
3. If green: close this ticket → Decisions-so-far on [[issues-1-odm-design-docs-map]] → close the map
4. If red: fix docs or file a gap; do not close

### Run (2026-08-01)

- **Files:** all required paths present; `concepts.md` absent (folded); `docs/adr/` empty (OK)
- **Depth:** full-spec + sketches + CONTEXT meet bars above (spot-check; #12 sketches written)
- **Conflicts:** Decisions so far align with cited refs; config under `.odm/`; plain clones; progen façade; no serve/MCP as product; no dual “brain” vocabulary in product docs
- **Open questions:** map Not yet specified non-empty and non-blocking for design close
- **Ready:** design fundamentals locked for a later Implement core map only

**Result: PASS** — close this ticket and the parent map.

## Comments

Parent map: [[issues-1-odm-design-docs-map]]

Grilled with maintainer; checklist locked and run green.
