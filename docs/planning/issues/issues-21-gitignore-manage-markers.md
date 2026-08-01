---
id: issues-21
title: "Gitignore manage markers"
description: "Lock how ODM maintains managed-path ignore blocks when manage_gitignore is true."
status: reviewing
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
  - ready-for-agent
---

# Gitignore manage markers

## Question

How does ODM identify, insert, and update its managed sections in Workspace-root and ancestor-checkout `.gitignore` files when `manage_gitignore` is true — marker format, idempotency, and what is left alone?

## Blocked by

- [[issues-17-config-and-pin-serde-model]]

## Answer

**Recommended lock:**

### Markers

```gitignore
# >>> ODM managed (do not edit between markers)
.odm/cache/
.odm/log/
.odm/progen/
worktrees/
projects/alpha/
projects/beta/
# <<< ODM managed
```

- Begin: `# >>> ODM managed (do not edit between markers)`
- End: `# <<< ODM managed`
- Exactly **one** managed block per `.gitignore` file; if markers missing, append a new block at EOF (ensure trailing newline before block).
- Content **inside** markers is **fully rewritten** each update (idempotent). User edits inside markers are clobbered. Lines **outside** markers are never modified.

### What goes in the Workspace-root block

Always (ephemeral layout):

- `.odm/cache/`
- `.odm/log/`
- `.odm/progen/`
- `worktrees/`

Plus every **managed** checkout path relative to Workspace root, with trailing `/`.

Do **not** ignore all of `.odm/` (config/pin stay trackable).

### Ancestor managed checkouts

When managed path `child` is nested under managed path `parent`, update `parent`’s checkout-root `.gitignore` with a block listing `child` relative to `parent` (trailing `/`). Ephemeral `.odm/…` lines only at Workspace root, not inside project checkouts unless that checkout is also a Workspace (it is not).

### When

- `manage_gitignore == true` (default) **and** Workspace root is a git repo.
- On: `init` (seed), `project add`/`rm`, `sync`/`materialize` success paths that change managed set, `doctor --fix`.
- When false: never touch ignore files.

### File create

If `.gitignore` missing at a path that needs updates → create with only the managed block + trailing newline.

### Remove

On un-declare (`project rm`): rewrite block without that path; if block would only have ephemerals at root, keep ephemerals.

## Comments

Parent map: [[issues-14-implement-core-map]]

Recommended decision locked for agent implement 2026-08-01.
