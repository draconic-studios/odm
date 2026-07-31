# Env, generators, and agent packs (sketch)

**Sketch** — not a Ship gate. Depth bar: intent, placement/ownership, CLI names reserved, explicit deferred. Domain terms: root `CONTEXT.md`. Config pointers: `config.md`. CLI stubs: `cli.md`. Worktrees: `worktrees.md`.

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

**CLI name reserved:**

```text
odm generate <generator-name> [generator-flags…]
```

- Resolve `<generator-name>` from merged bundles; unknown → usage error.
- **Behavior one-liner:** materialize the template into a target path (default/flags at implement time).
- No interactive wizard spec in this design package.

**Deferred:**

- `template.toml` (or equivalent) schema
- Prompt / variable contract
- Remote fetch and cache layout
- Nx / schematics interop
- Dry-run and overwrite policy
- Inline generator bodies in Workspace config (still forbidden)

---

## Agent packs

**Intent:** Portable bundle of agent-facing assets (skills, prompts, conventions, Workspace links). Install or link into **agent-native homes** so agents share the desk — not ad-hoc copies under Projects.

**Not** a Progen and **not** a Project (`CONTEXT.md`).

**Placement and ownership:**

- **Targets:** agent-native config/skill homes (user/machine paths) — **not** under `.odm/`, **not** inside Project trees by default.
- **Operations:** Workspace-scoped (`odm agent pack …` run in a Workspace); may link assets that reference this Workspace.
- **No** global pack-registry product in this sketch.
- **No** pack payloads under `.odm/`; optional fetch cache under `.odm/cache/` only if implement needs it (detail deferred).

**CLI names reserved:**

```text
odm agent pack install …
odm agent pack link …
odm agent pack list …
```

- **`install`:** materialize/copy or fetch a pack into an agent home.
- **`link`:** symlink when the platform allows.
- **`list`:** what ODM knows is installed/linked.

**Deferred:**

- Pack manifest schema and on-disk pack layout
- Windows symlink vs copy default
- Marketplace protocol
- First-class agent matrix (Cursor, Claude, etc.)
- Pack declarations in Workspace config spine

---

## Agent start and prompt

**CLI names reserved:**

```text
odm agent start [--project …] [--wt …] …
odm agent prompt <id> --progen … 
```

- **`start`:** launch an agent **runtime** against a Project Primary or `--wt` slot via **shell-out**. No runtime matrix in the design package.
- **`prompt`:** thin wrap of progen prompt with ODM Progen scope — not a second prompt engine. Primary home is `odm agent prompt`, not a duplicate full surface under `odm progen`.

Honors `--project`, `--wt`, `--progen`, `--json` where relevant (`cli.md`).

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
- CLI sketch matrix: `cli.md`
- Worktree slots: `worktrees.md`
- Code↔doc index: `graph.md`
- Architecture / `.odm/`: `architecture.md`
- Vision jobs: `vision.md`
- Phased delivery: `phased-delivery.md`
