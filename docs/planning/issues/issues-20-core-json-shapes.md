---
id: issues-20
title: "Core JSON shapes"
description: "Lock stable --json object shapes for core verbs agents will parse."
status: reviewing
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
  - ready-for-agent
---

# Core JSON shapes

## Question

What are the stable `--json` success (and error, if not fully covered elsewhere) object shapes for core verbs: `init`, `status`, `doctor`, `pin status` / `pin apply` summary, `project list` / `info` (and any other core command that supports `--json` in `cli.md`)?

## Blocked by

- [[issues-18-cli-error-and-exit-code-spine]]

## Answer

**Recommended lock** (errors → [[issues-18-cli-error-and-exit-code-spine]]; success = bare objects, **snake_case** fields):

### `odm init --json`

```json
{ "root": "/abs/path", "git": true }
```

### `odm status --json`

```json
{
  "root": "/abs/path",
  "projects": [
    {
      "name": "alpha",
      "path": "projects/alpha",
      "url": "…",
      "managed": true,
      "on_disk": true,
      "is_git": true,
      "head": "abc…",
      "pin_rev": "abc…",
      "pin_state": "in_sync",
      "dirty": false
    }
  ],
  "progens": []
}
```

`pin_state`: `none` | `missing_path` | `unpinned` | `in_sync` | `drift` | `missing_pin_file`.

### `odm doctor --json`

```json
{
  "ok": true,
  "checks": [
    {
      "id": "config_load",
      "status": "pass",
      "message": "config ok",
      "fixable": false
    }
  ]
}
```

`status`: `pass` | `warn` | `fail`. `ok` is false if any `fail`.

### `odm pin status --json`

```json
{
  "pin_file": ".odm/odm.lock.yaml",
  "present": true,
  "entries": [
    {
      "name": "alpha",
      "pin_rev": "…",
      "head": "…",
      "state": "in_sync"
    }
  ]
}
```

### `odm pin apply --json`

```json
{
  "results": [
    { "name": "alpha", "status": "applied", "rev": "…" }
  ]
}
```

`status`: `applied` | `skipped` | `failed`.

### `odm project list --json`

```json
{
  "projects": [
    { "name": "alpha", "path": "projects/alpha", "url": "…", "branch": "main", "type": null }
  ]
}
```

### `odm project info --json`

```json
{
  "name": "alpha",
  "path": "projects/alpha",
  "url": "…",
  "branch": "main",
  "type": null,
  "on_disk": true,
  "is_git": true,
  "head": "…",
  "origin": "…",
  "dirty": false,
  "pin_rev": "…",
  "pin_state": "in_sync"
}
```

### Commands without required JSON day one

`sync`, `project add`/`rm`/`git`: human-only OK; if `--json` passed, minimal `{ "ok": true }` or per-command summary is fine but not gated.

No `schema_version` field day one.

## Comments

Parent map: [[issues-14-implement-core-map]]

Recommended decision locked for agent implement 2026-08-01.
