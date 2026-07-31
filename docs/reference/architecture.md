# Architecture

System shape for ODM v1 design. Domain terms: root `CONTEXT.md`. Product framing: `vision.md` (when present). Config keys: `config.md`. Git lifecycle: `multi-git.md`.

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
      <slot-name>/            # Worktree slot working tree (lazy; sketch details in worktrees.md)
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

## Ownership boundaries (stub)

Full product narrative and crate layout: **Vision and architecture narrative**. Locked here only as it touches the state directory:

| Owner | Responsibility |
|-------|----------------|
| **ODM** | Workspace config, pin file, CLI UX, multi-git lifecycle, federation scope, paths under `.odm/` listed above, `worktrees/` placement |
| **Progen (crates)** | Single-store content, in-store index/query (engine defaults may live beside the store) |
| **User / git** | Auth, commit policy, content of Projects and Progens |
| **Agent tools** | Their own config homes; ODM may link packs into them |

## Related

- Config schema: `config.md`
- Multi-git + pin semantics: `multi-git.md`
- Progen federation: `progen.md`
- Worktree slot behavior (sketch): `worktrees.md`
- CLI (`--root`, init): `cli.md`
