# Env and generators

**Mixed depth** — local generate is **v1 landed**; env remains sketch. Not a Ship gate for deferred items. Domain terms: root `CONTEXT.md`. Config pointers: `config.md`. CLI: `cli.md`. Worktrees: `worktrees.md`.

## Env

**Intent (future):** Workspace may eventually inject or declare env for actions so `run` shares known vars.

**Not** a secrets manager.

**v1 design package:**

- No env config keys
- No `odm env` CLI
- No env profile entity in `CONTEXT.md`

**Deferred entirely** until an implement slice needs it. Recorded so nobody invents `.odm/env` ad hoc.

---

## Generators

**Intent:** Named scaffolds from a template package (local path or remote URL). Not an Action; not Nx/schematics as the entity.

**Config (already locked):** `generators` map → bundle files; each Generator has `template` and/or `url`; names merge across bundles; duplicates error (`config.md`).

**CLI (v1 local template landed):**

```text
odm generate                                    # list
odm generate <generator-name> --dest <rel-path> [--force] [--dry-run]
```

- Resolve `<generator-name>` from merged bundles; unknown → exit `1`.
- **v1 behavior:** recursive copy of a local `template` directory (relative to Workspace root) into `--dest` under the Workspace. No variable substitution, no interactive wizard.
- **`--force`:** overwrite when dest is non-empty; without force → exit `3`.
- **`--dry-run`:** same validation as a real run; no filesystem writes; reports the file count that would be copied (`cli.md`).
- **Url-only** entries may appear in list; run fails with a clear deferred-remote message (exit `1`, with or without `--dry-run`). Both set → prefer `template`.
- JSON list/run shapes: `cli.md` (run includes `dry_run` bool).

**Deferred (still explicit):**

- `template.toml` (or equivalent) schema
- Prompt / variable contract
- Remote fetch and cache layout
- Nx / schematics interop
- Inline generator bodies in Workspace config (still forbidden)

**Landed (not deferred):** `--dry-run` no-write preview on local generate.

---

## Shared absences

- **`odm serve` / MCP / long-running daemon** — out of the design package and out of v1 Ship unless a later map reopens it. Not upstream progen `serve` (not re-exported).
- **Plugin host** (legacy Go go-plugin / npm installers) — dead path.
- **No** top-level `pack` or `env` commands.
- **No** cross-store graph API (see `graph.md`, `progen.md`).
- **Env / generator cache on status:** no obligation to report env or generator cache on `status`. Core remains correct when sketch features are absent.

---

## Related

- Config generators map: `config.md`
- CLI full vs sketch matrix: `cli.md`
- Worktree slots: `worktrees.md`
- Code↔doc index: `graph.md`
- Architecture / `.odm/`: `architecture.md`
- Vision jobs: `vision.md`
- Phased delivery: `phased-delivery.md`
