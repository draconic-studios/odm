# Worktree slots (sketch)

**Sketch** — not a Ship gate. Depth bar: intent, placement/ownership, CLI names reserved, explicit deferred. Full behavior is a later implement slice. Domain terms: root `CONTEXT.md`. Placement: `architecture.md`. CLI stubs: `cli.md`. Primary checkouts: `multi-git.md`.

## Intent

Parallel human or agent working trees on one **Project** without touching that Project’s **Primary checkout**. Agent and human work land in known slot paths, not ad-hoc clones.

## Placement and ownership

- **Path:** `worktrees/<project-name>/<slot-name>/` at Workspace root — **not** under `.odm/`.
- **Git:** ignored when the Workspace is a repo; created on first use (`architecture.md`).
- **Binding:** each slot is a git worktree bound to a branch; branch stays plain git vocabulary.
- **ODM owns:** path convention, create/remove orchestration, `--wt` resolution.
- **Git owns:** worktree and branch mechanics.
- **User owns:** commits and branch policy inside the slot.

## CLI names reserved

```text
odm project worktree list
odm project worktree add <slot> …
odm project worktree rm <slot> …
```

- **`list` / `add` / `rm`:** names only in this design package — no flag tables.
- **`add`:** create a git worktree at the slot path; branch is caller-supplied or plain git default. **No** ODM branch-naming template in the sketch.
- **`--wt <slot>`** (global): resolve working tree to `worktrees/<project>/<slot>/` for `project git`, `run`, and sketch `agent start`. Requires Project context. **Does not** auto-create a missing slot.

## Rules

- Slot only on a **Project** whose Primary checkout is a **git** repo. Non-git Project → hard error on `worktree add` / `--wt`.
- Slot name: name token (no path separators); charset aligned with Project names at implement time.
- **Primary checkout is never a slot.** Omit `--wt` → Primary.
- `project rm` does **not** delete `worktrees/<project>/` by default (same keep-tree spirit as Project trees). Orphan slot dirs are possible; cleanup is manual or a later `doctor` concern — **not** required of core `status`/`doctor` in this package.

## Deferred

- Worktree slot declarations in Workspace config
- Branch naming templates
- GC / prune policy
- Pin file interaction with slots (pin stays Primary-oriented unless later decided)
- Multi-Project or Workspace-level slots
- `status` / `doctor` obligations for orphans or dirty slots
- Full flag tables and JSON shapes

## Non-goals (sketch-wide)

- Not a substitute for Primary checkout lifecycle (`multi-git.md`)
- Not under `.odm/`
- No `odm serve` / MCP / daemon involvement

## Related

- Architecture layout: `architecture.md`
- CLI matrix: `cli.md`
- Multi-git Primary: `multi-git.md`
- Agent start + packs (sketch): `env-gen-packs.md`
- Phased delivery: `phased-delivery.md`
