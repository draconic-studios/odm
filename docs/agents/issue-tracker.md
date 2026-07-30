# Issue tracker: vault issues

There is **no GitHub Issues**. Issues and specs live as markdown notes under
`docs/planning/issues/` in this repo's docs vault.

## Location

- **Live issues:** `docs/planning/issues/issues-<N>-<slug>.md`
- **Done / rejected:** `docs/planning/issues/closed/`
- Specs/PRDs are issue notes (same folder), not a separate tree

No `plans/`, `tasks/`, or `ideas/` folders — one kind only: **issue**.

## Filename and id

- Pattern: `issues-<N>-<slug>.md` (e.g. `issues-1-cli-json-output.md`)
- `<N>` is a positive integer from a **single sequence** across live + `closed/`
- **Never eyeball** the next id from `ls` of the live folder alone — closed notes
  move out and free-looking numbers are still taken

Allocate the next id:

```bash
# high-water mark across live + closed
find docs/planning/issues -name 'issues-*.md' -print \
  | sed -E 's/.*issues-([0-9]+)-.*/\1/' \
  | sort -n | tail -1
# next = that number + 1 (or 1 if none)
```

Re-run immediately before writing if another session may be filing too.

## Frontmatter

```yaml
---
id: issues-1
title: "Short title"
description: "One-sentence summary"
status: open
issue-type: bug          # optional: bug | feature-request | observation
severity: medium           # optional: critical | high | medium | low
tags:
  - planning
  - issue
  # triage roles (see triage-labels.md), e.g. ready-for-agent
---
```

**Status:** `open` | `reviewing` | `closed` | `wontfix`

- `open` — live, unclaimed (or waiting on triage tags)
- `reviewing` — claimed; someone/agent is working it
- `closed` — done; move to `closed/`
- `wontfix` — will not action; move to `closed/`

Triage roles ride on `tags` (and optionally `issue-type` / `severity`). See
`docs/agents/triage-labels.md`.

## Body template

```markdown
# {Title matching frontmatter title}

## Description

What's wrong, missing, or wanted.

## Affected

Areas, commands, crates, or flows impacted.

## Observed

Repro steps, output, or unexpected behaviour (bugs).

## Impact

Why it matters.

## Proposed Fix

Best known approach, or leave blank.

## Comments

Append discussion here over time.
```

Required in practice: **Description**. Add **Agent Brief** when tagging
`ready-for-agent` (see triage skill).

## Filing rules

- Create under `docs/planning/issues/`
- Link related notes with `[[wikilinks]]`
- Append history under `## Comments` (or `## Log`) — don't delete discussion
- When `closed` or `wontfix`: set `status`, then move the file to
  `docs/planning/issues/closed/` (keep the same filename)
- Don't delete closed issues — they are the audit trail and keep wikilinks stable
  if you use path-stable moves; prefer updating links when renaming

## When a skill says "publish to the issue tracker"

Create `docs/planning/issues/issues-<N>-<slug>.md` with the frontmatter + body
above (allocate `<N>` first).

## When a skill says "fetch the relevant ticket"

Read the file at the given path or resolve `issues-<N>` via:

```bash
find docs/planning/issues -name 'issues-<N>-*.md'
```

## Wayfinding (issues only)

Used by `/wayfinder`. Everything is still an **issue** note — no plan files.

- **Map:** an issue with `tags` including `wayfinder-map` (body: Destination /
  Notes / Decisions so far / Fog)
- **Ticket:** an issue with `tags` including `wayfinder` and
  `wayfinder-<type>` where type is `research` | `prototype` | `grilling` | `task`
- **Blocking:** a `## Blocked by` section of `[[wikilinks]]` to other issue notes
- **Frontier:** open, unblocked, unclaimed wayfinder tickets; lowest `id` first
- **Claim:** set `status: reviewing` before work
- **Resolve:** append `## Answer`, set `status: closed`, move to `closed/`, then
  add a one-line gist + `[[wikilink]]` on the map's Decisions so far
