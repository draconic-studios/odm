---
id: issues-18
title: "CLI error and exit-code spine"
description: "Lock library error types and mapping to exit codes 0–4 plus human/--json error envelopes."
status: closed
tags:
  - planning
  - issue
  - wayfinder
  - wayfinder-grilling
  - ready-for-agent
---

# CLI error and exit-code spine

## Question

How do `odm-core` / `odm-git` errors map onto CLI exit codes `0`–`4` from `cli.md`, and what are the human stderr vs `--json` error envelope shapes for core commands?

## Blocked by

_(none — frontier)_

## Answer

**Recommended lock:**

### Ownership

- **`odm-core`**: typed `OdmError` (+ `thiserror`); no process exit.
- **`odm` bin**: maps `OdmError` / clap failures → exit code; prints human or JSON.

### `OdmError` kinds (core)

- **`Usage`** — unknown entity name, bad args from core validation, not-implemented stub
- **`Workspace`** — not a Workspace, discovery miss, invalid/missing config, bundle path miss, serde/validation
- **`Operation`** — git failures, pin apply fail, materialize fail, doctor hard fail, dirty-without-force
- **`NotFound`** — resource missing when distinct from usage (e.g. named path absent on pin apply when name is valid)

Map `GitError`: `GitNotFound` / `Failed` / `Parse` / `NotARepo` / `OriginMissing` / `NotAbsolute` / `EmptyArgs` → **`Operation`** (wrap with context). Origin-mismatch policy errors raised in core as **`Operation`**.

### Exit codes (bin)

| Code | When |
|------|------|
| `0` | success |
| `1` | clap parse fail; `Usage`; not-implemented stubs |
| `2` | `Workspace` |
| `3` | `Operation` |
| `4` | `NotFound` |

### Human (stderr)

```text
error: <message>
```

Optional second line(s) for detail (e.g. git stderr) without `error:` prefix. No color required day one.

### `--json` error (stdout, non-zero exit)

```json
{
  "ok": false,
  "error": {
    "code": "usage" | "workspace" | "operation" | "not_found",
    "message": "<string>",
    "detail": null
  }
}
```

- `detail`: optional string (git stderr, path); use JSON `null` when absent.
- Success responses are **bare command objects** (no `{ok:true}` wrapper) — matches `cli.md` init shape.
- Diagnostics stay on stderr even under `--json` only if needed for progress; structured error body is on **stdout**.

### Stubs

Unbuilt core verbs: exit `1`, message `not implemented: <verb>` (human) / JSON `code: "usage"`.

## Comments

Parent map: [[issues-14-implement-core-map]]

Recommended decision locked for agent implement 2026-08-01.

Landed 2026-08-01: `OdmError` + `exit_code` in `odm-core`; bin `output.rs` human/JSON envelopes; stubs exit 1.
