# Env, generators, and agent packs

**Mixed depth** — generators and agent packs have **v1 local** CLI; env and agent start/prompt remain sketch. Not a Ship gate for deferred items. Domain terms: root `CONTEXT.md`. Config pointers: `config.md`. CLI: `cli.md`. Worktrees: `worktrees.md`.

## Env

**Intent (future):** Workspace may eventually inject or declare env for actions/agents so `run` and agent start share known vars.

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
odm generate <generator-name> --dest <rel-path> [--force]
```

- Resolve `<generator-name>` from merged bundles; unknown → exit `1`.
- **v1 behavior:** recursive copy of a local `template` directory (relative to Workspace root) into `--dest` under the Workspace. No variable substitution, no interactive wizard.
- **`--force`:** overwrite when dest is non-empty; without force → exit `3`.
- **Url-only** entries may appear in list; run fails with a clear deferred-remote message (exit `1`). Both set → prefer `template`.
- JSON list/run shapes: `cli.md`.

**Deferred (still explicit):**

- `template.toml` (or equivalent) schema
- Prompt / variable contract
- Remote fetch and cache layout
- Nx / schematics interop
- Dry-run mode
- Inline generator bodies in Workspace config (still forbidden)

---

## Agent packs

**Intent:** Portable bundle of agent-facing assets (skills, prompts, conventions, Workspace links). Install or link into **agent-native homes** so agents share the desk — not ad-hoc copies under Projects.

**Not** a Progen and **not** a Project (`CONTEXT.md`).

**Placement and ownership:**

- **Targets:** agent-native config/skill homes (user/machine paths) — **not** under `.odm/`, **not** inside Project trees by default.
- **Operations:** Workspace-scoped (`odm agent pack …` run in a Workspace).
- **Registry:** Workspace-local `.odm/agent-packs.json` (list is registry-backed; does not scan arbitrary homes).
- **No** pack **payloads** under `.odm/` (only the registry); no global pack-registry product; marketplace/fetch cache deferred.

**CLI (v1 local install/link/list landed):**

```text
odm agent pack list
odm agent pack install <source> --home <path> [--force]
odm agent pack link <source> --home <path> [--force]
```

- **Source:** local directory; relative under Workspace root (no escape); name = basename. No remote/marketplace; no `pack.toml` required in v1.
- **`--home`:** required; pack at `<home>/<name>/`.
- **`install`:** recursive copy. Exists without `--force` → exit `3`. Missing source → exit `4`.
- **`link`:** symlink to absolute source; same force policy; no silent copy fallback when symlinks unavailable.
- **`list`:** human one name per line (empty → `(no agent packs)`); JSON shapes in `cli.md`.

**Deferred (still explicit):**

- Pack manifest schema and on-disk pack layout convention
- Windows junction policy beyond honest error
- Marketplace protocol / remote fetch
- First-class agent matrix (Cursor, Claude, etc.)
- Pack declarations in Workspace config spine
- `status` / `doctor` pack reports

---

## Agent start and prompt

**CLI names reserved (still sketch / not implemented):**

```text
odm agent start [--project …] [--wt …] …
odm agent prompt <id> --progen … 
```

- **`start`:** launch an agent **runtime** against a Project Primary or `--wt` slot via **shell-out**. No runtime matrix in the design package.
- **`prompt`:** thin wrap of progen prompt with ODM Progen scope — not a second prompt engine. Primary home is `odm agent prompt`, not a duplicate full surface under `odm progen`.

Honors `--project`, `--wt`, `--progen`, `--json` where relevant when implemented (`cli.md`).

**Deferred:** full start flags, runtime detection, session lifecycle, prompt flag parity tables.

---

## Shared absences

- **`odm serve` / MCP / long-running daemon** — out of the design package and out of v1 Ship unless a later map reopens it. Not the same as `agent start` (one-shot/shell-out) or upstream progen `serve` (not re-exported).
- **Plugin host** (legacy Go go-plugin / npm installers) — dead path.
- **No** top-level `pack` or `env` commands — agent UX under `odm agent …` only.
- **No** cross-store graph API (see `graph.md`, `progen.md`).
- **`status` / `doctor`:** no obligation to report packs, env, or generator cache in this package; core remains correct when sketch features are absent.

---

## Related

- Config generators map: `config.md`
- CLI full vs sketch matrix: `cli.md`
- Worktree slots: `worktrees.md`
- Code↔doc index: `graph.md`
- Architecture / `.odm/`: `architecture.md`
- Vision jobs: `vision.md`
- Phased delivery: `phased-delivery.md`
