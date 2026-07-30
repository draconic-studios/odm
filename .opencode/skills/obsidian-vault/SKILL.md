---
name: obsidian-vault
description: Search, create, and manage notes in the repo docs/ folder — an Obsidian vault that is also this project's documentation. Use when the user wants to find, create, or organize docs, notes, ADRs, specs, or vault content under docs/.
---

# Obsidian Vault (`docs/`)

## Vault location

**Root:** `docs/` at the repository root (relative paths from repo root).

This folder is both:

1. **Project documentation** — committed, reviewed, linked from code and skills
2. **Obsidian vault** — open `docs/` as a vault for human browse/navigation

Do not use an external personal vault path. All notes for this project live under `docs/`.

Preserve existing special trees used by other skills:

- `docs/agents/` — agent config (issue tracker, triage, domain layout); do not reorganize
- `docs/adr/` — numbered ADRs (`0001-slug.md`); prefer `/domain-modeling` for new ADRs

## Layout

Prefer the hybrid brain layout (see `DESIGN.md`):

```
docs/
├── agents/                 # agent skill config (leave alone)
├── adr/                    # ADRs 0001-slug.md (domain-modeling)
├── planning/
│   └── issues/             # tracker (see docs/agents/issue-tracker.md)
│       └── closed/
├── reference/
│   ├── architecture/
│   ├── decisions/          # informal decisions; formal ADRs stay in adr/
│   ├── guides/
│   └── specs/
├── log/
│   └── changelog/YYYY/MM/
└── *.md                    # optional root index notes
```

Create subfolders lazily when the first note needs them.

| Kind | Path | Meaning |
|------|------|---------|
| issue | `docs/planning/issues/` | tracker notes (`issues-<N>-<slug>.md`) |
| architecture / guide / spec | `docs/reference/.../` | durable reference |
| changelog | `docs/log/changelog/YYYY/MM/` | dated log entries |
| ADR | `docs/adr/` | accepted architectural decisions |

No `plans/` or `tasks/` folders — issues only. Ops: `docs/agents/issue-tracker.md`.

## Naming

- **Kebab-case** filenames for repo docs: `offline-auth.md`, `cli-config-surface.md`
- **Index notes**: `Index.md` in a folder, or `Something Index.md` at vault root
- Title in the note H1 can be Title Case; filename stays kebab-case
- ADRs keep `NNNN-slug.md` numbering

## Note shape

Frontmatter when the note is graph-relevant (planning, specs, anything linked from code):

```yaml
---
title: Offline auth
description: "…"
tags: [reference, spec]
depends_on: []
blocks: []
related: []
---
```

Body:

1. H1 matching the topic
2. Content as one clear unit (problem, decision, guide, etc.)
3. Related links at the bottom

## Linking

- Obsidian `[[wikilinks]]`: `[[offline-auth]]` or `[[specs/offline-auth]]`
- Prefer wikilinks for note↔note edges; path links (`docs/reference/specs/foo.md`) when referring from code/`@references`
- Index notes are lists of `[[wikilinks]]`
- Frontmatter `depends_on` / `blocks` / `related` hold note ids or wikilink targets for typed edges

## Workflows

### Search

Use Grep/Glob on `docs/` (prefer tools over shell):

- by name: Glob `docs/**/*keyword*.md`
- by content: Grep path `docs/` pattern `keyword` include `*.md`
- indexes: Glob `docs/**/*[Ii]ndex*.md`
- backlinks to a note: Grep path `docs/` pattern `\[\[.*[Nn]ote-slug`

### Create a note

1. Pick the correct folder from the layout table
2. Kebab-case filename; add frontmatter if planning/reference
3. Write the unit of content
4. Add `[[wikilinks]]` to related notes
5. Update the nearest `Index.md` (or create one) if the folder is indexed
6. If code should point here, mention the path for `@references` (e.g. `docs/reference/specs/offline-auth.md`)

### ADRs

New formal ADRs → use `/domain-modeling` (or its ADR format) under `docs/adr/`.  
Informal decision write-ups can live in `docs/reference/decisions/` without numbers.

### Don't

- Don't store project docs outside `docs/`
- Don't rewrite `docs/agents/` as vault notes
- Don't invent a second vault root
- Don't create empty folder scaffolding with no notes
