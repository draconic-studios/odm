# Architecture

System shape for ODM v1 design. Domain terms: root `CONTEXT.md`. Product framing: `vision.md`. Config keys: `config.md`. Git lifecycle: `multi-git.md`.

## ODM state directory (`.odm/`)

Every Workspace has an **ODM state directory** at `<workspace>/.odm/`. It holds **ODM config and runtime state only**. It does **not** own Project checkouts, Progen stores, Worktree slot trees, or agent pack payloads.

### Layout

```text
<workspace>/
  .odm/
    odm.config.yaml           # tracked — Workspace config (layout source of truth)
    odm.lock.yaml             # tracked when present — Pin file
    cache/                    # gitignored — generic ODM caches
    log/                      # gitignored — optional CLI / diagnostic logs
    progen/
      <progen-name>/          # gitignored — ODM-side index/cache per Progen name
  worktrees/                  # NOT under .odm/
    <project-name>/
      <slot-name>/            # Worktree slot working tree (v1; deferred items in worktrees.md)
  # Project / Progen paths — only where Workspace config declares them
```

| Path | Role | Git (when Workspace is a repo) |
|------|------|--------------------------------|
| `.odm/odm.config.yaml` | Workspace config | **tracked** |
| `.odm/odm.lock.yaml` | Pin file | **tracked** when present |
| `.odm/cache/` | Disposable caches | **ignored** |
| `.odm/log/` | Logs | **ignored** |
| `.odm/progen/<name>/` | ODM-managed progen indexes/caches | **ignored** |
| `worktrees/...` | Worktree slots | **ignored**; created on first use |
| Config `path`s | Primary checkouts / Progen stores | never under `.odm/`; usually ignored as managed clones (`multi-git.md`) |

### Hard exclusions

**Never** place under `.odm/`:

- Project **Primary checkouts**
- **Progen** store roots (Markdown / content)
- **Worktree slot** working trees (use `worktrees/<project>/<slot>/`)
- **Agent pack** payloads or agent-home clones (link/install to agent-native paths or other Workspace conventions)
- User application source “for convenience”

The progen engine may keep its own index next to a store by upstream default. `.odm/progen/<name>/` is only for **ODM-owned** workspace-level cache/index material — not the store itself.

### Tracked vs ignored enforcement

When `manage_gitignore` is enabled (default **true**), ODM maintains **explicit** ignore entries for ephemeral paths, for example:

- `.odm/cache/`
- `.odm/log/`
- `.odm/progen/`
- `worktrees/`

plus managed checkout paths per `multi-git.md`.

ODM does **not** ignore all of `.odm/` with negation exceptions. New files under `.odm/` are tracked by default until classified. Users may add their own files under `.odm/`; ODM leaves unknowns alone (no v1 local-config merge file).

### Workspace root discovery

| Case | Behavior |
|------|----------|
| `--root <path>` | That directory is the Workspace. It must contain `.odm/odm.config.yaml`. No walk. |
| cwd is inside a `.odm/` directory | Start search from the parent of that `.odm/`. |
| otherwise | Walk up from cwd looking for a directory that contains `.odm/odm.config.yaml`. Stop at `$HOME` (or filesystem root if the walk is outside home). First match wins. |
| no match | Not a Workspace → clear error and stop. |

**Exception:** `odm init` (and equivalent bootstrap) may run without an existing Workspace; it creates `.odm/` and config at the target root.

Presence of an empty `.odm/` without `odm.config.yaml` does **not** count as a Workspace for discovery.

## System narrative

Outside-in flow for a Workspace:

1. **Human or agent** invokes the `odm` binary.
2. **CLI** parses globals and routes commands (`cli.md`).
3. **Workspace core** loads `.odm/odm.config.yaml` (and pin when present), resolves config **names** to paths, enforces discovery (`--root` or walk).
4. **Subsystems** (all name-driven from config; none invent undeclared Projects or Progens):
   - **Git lifecycle** — plain clones at Project/Progen paths; pin file; Worktree slots under `worktrees/<project>/<slot>/` (`multi-git.md`).
   - **Progen façade** — scope union → N× single-root progen engine calls; ODM-owned index/cache only under `.odm/progen/<name>/` (`progen.md`).
   - **Actions** — load Action bundle files → shell-out to command bodies.
   - **Agent packs** (v1 local) — install/link/list into agent-native homes; registry under `.odm/`.
5. **On disk, not owned as ODM content** — Primary checkouts, Progen stores, worktree slot trees (paths may be managed clones; content is the user’s).

**Edge rule:** Workspace config is the only layout source of truth.

```text
human / agent
    │
    ▼
odm (CLI)
    │
    ▼
workspace core  ── reads .odm/odm.config.yaml (+ pin)
    │
    ├── git lifecycle ──► primary checkouts, worktrees/
    ├── progen façade ──► progen stores (+ .odm/progen/<name>/ indexes)
    ├── actions ────────► shell-out (user/Nx/scripts)
    └── agent packs ────► agent config homes (v1 local)
```

## Ownership boundaries

- **ODM owns**
  - Workspace discovery, Workspace config, pin file
  - `.odm/` layout and `worktrees/` placement
  - Multi-git lifecycle (clone/fetch/pin/status/doctor orchestration)
  - Federation and query scope (`--progen`, `--progen-group`, default-all)
  - CLI surface, exit codes, `--json` shapes
  - Action and Generator dispatch (load bundles and invoke; not necessarily template-engine guts)
  - Agent-pack install/link into agent homes
  - Gitignore maintenance when `manage_gitignore` is enabled
- **Progen (crates) owns**
  - Single-store content model, index, query/context internals
  - In-store paths and engine defaults beside a store
  - Store verbs re-exported under `odm progen …`
- **Shell-out / external owns**
  - `git` for VCS operations (ODM orchestrates; does not reimplement git)
  - Action command bodies (user scripts, Nx targets, etc.)
  - Agent runtimes and their config homes
  - Optional remote template fetch for generators (sketch)
- **User owns**
  - Auth, commit policy, content of Projects and Progens

Product-level summary: `vision.md`.

## Crate layout (design intent)

Rust monorepo shape for ownership and tests — not a promise of empty crates on day one. **One binary** ships.

```text
odm (bin)              # CLI only: parse, UX, exit codes
odm-core               # Workspace, config, pin, discovery, paths
odm-git                # multi-git lifecycle (shells git)
odm-progen             # federation/scope + façade over progen crates
odm-actions            # load/run Action bundles
# no odm-agent crate yet — agent_pack v1 local lives in odm-core;
# agent prompt is thin CLI over progen context; agent start remains stub
# progen upstream crates (path or vendored) — store / index / query
# no odm-serve in v1
```

v0.1 ships an **in-repo** Obsidian-compatible vault engine inside `odm-progen` (not external progenitor crates yet); the façade stays swap-ready.

Rules:

- **Depend inward** — `odm` → feature crates → `odm-core`. Progen crates never depend on ODM.
- **One product binary** — crates are boundaries, not multiple distributeables.
- **Thin modules until crate earned** — `generate` (v1 local template) and `agent_pack` (v1 local install/link/list) live in `odm-core`; `agent prompt` is a thin CLI alias of context (no separate crate); `agent start` remains a stub until depth demands a crate.
- **Non-goal** — deep Serve/MCP (`odm serve`) is out of the v1 design package.

## Related

- Vision: `vision.md`
- Config schema: `config.md`
- Multi-git + pin semantics: `multi-git.md`
- Progen federation: `progen.md`
- Worktree slots (v1 + deferred): `worktrees.md`
- Code↔doc index (sketch): `graph.md`
- Env / generators / packs: `env-gen-packs.md`
- CLI (`--root`, init): `cli.md`
