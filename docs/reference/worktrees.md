# Worktree slots (v1 implemented + deferred)

**v1 implemented** — slot lifecycle (`list` / `add` / `rm` / `prune`) and `--wt` path binding for `project git` (and actions cwd). Items under **Deferred** remain out of scope. Domain terms: root `CONTEXT.md`. Placement: `architecture.md`. CLI: `cli.md`. Primary checkouts: `multi-git.md`.

## Intent

Parallel human or agent working trees on one **Project** without touching that Project’s **Primary checkout**. Agent and human work land in known slot paths, not ad-hoc clones.

## Placement and ownership

- **Path:** `worktrees/<project-name>/<slot-name>/` at Workspace root — **not** under `.odm/`.
- **Git:** ignored when the Workspace is a repo; created on first use (`architecture.md`).
- **Binding:** each slot is a git worktree bound to a branch; branch stays plain git vocabulary.
- **ODM owns:** path convention, create/remove orchestration, `--wt` resolution.
- **Git owns:** worktree and branch mechanics.
- **User owns:** commits and branch policy inside the slot.

## CLI (v1)

```text
odm project worktree list <project>
odm project worktree add <project> <slot> [--branch <b>]
odm project worktree rm <project> <slot> [--force]
odm project worktree prune <project> [--force]
odm project worktree prune --all [--force]
```

- **`list <project>`:** slots under `worktrees/<project>/` that are registered git worktrees (from `git worktree list`, filtered to the slot prefix). Human: one slot name per line. `--json`: `{ "project", "slots": [ { "name", "path" } ] }` where `path` is `worktrees/<project>/<slot>`.
- **`add <project> <slot> [--branch <b>]`:** create a git worktree at the slot path from the Project primary. Optional `--branch` creates and checks out a new branch (`git worktree add -b`). Prefer `--branch` when the primary already has the default branch checked out (plain `worktree add` without a new branch fails in that case). Fails if slot path exists or primary is not a git repo. `--json`: `{ "project", "slot", "path" }`.
- **`rm <project> <slot> [--force]`:** `git worktree remove` on the slot; `--force` maps to git force. Best-effort remove of empty `worktrees/<project>/`. `--json`: `{ "project", "slot", "path" }`.
- **`prune <project> [--force]`:** manual GC for **orphan** slot dirs (same definition as doctor: valid slot-name directory under `worktrees/<project>/` not in the registered worktree set). Default removes **empty** orphans only; non-empty orphans are skipped and the command exits `3` after removing empties (partial OK). `--force` recursively deletes orphan dirs even if non-empty. Never deletes registered worktree paths or Primary. Best-effort remove of empty `worktrees/<project>/`. Human: pruned count/names (and skipped non-empty names when applicable). `--json`: `{ "project", "pruned": [ { "name", "path" } ] }` (`path` is `worktrees/<project>/<slot>`). No orphans → exit `0`, `pruned: []`.
- **`prune --all [--force]`:** same orphan rules for **every** configured Project (sorted name order). Missing primary / non-git projects are skipped (no hard fail). Exit `3` if any non-empty orphan remains without `--force`. Human: qualified `project/slot` names. `--json`: `{ "all": true, "pruned": [ { "project", "name", "path" } ], "skipped_nonempty": [ … ] }` (`skipped_nonempty` always present). Mutually exclusive with positional `<project>`.
- **`--wt <slot>`** on `project git` (and `run` with `--project`): resolve working tree to `worktrees/<project>/<slot>/`. Requires Project context. **Does not** auto-create a missing slot (missing → exit `4`). Pin auto-maintain stays **Primary-only**.

## Rules

- Slot only on a **Project** whose Primary checkout is a **git** repo. Non-git Project → hard error on `worktree add` / list / rm / prune (exit `3` operation).
- Slot name: non-empty name token (no path separators, no `.` / `..`); names only, not filesystem paths.
- **Primary checkout is never a slot.** Omit `--wt` → Primary.
- `project rm` does **not** delete `worktrees/<project>/` by default (same keep-tree spirit as Project trees). Orphan slot dirs are possible; `odm doctor` **warns** on configured-project slot dirs that are not registered git worktrees (`fixable: false` — does **not** delete on `--fix`). Cleanup is **`project worktree prune`** (explicit manual GC). `odm status` lists **registered** slots only (same filter as `project worktree list`); orphans remain doctor-warn + prune.
- `odm doctor` also **warns** on **dirty registered** worktree slots (`worktree_dirty:<project>:<slot>`, `fixable: false` — `--fix` does not clean or stash). Clean registered slots produce no dirty check. Probe errors soft-skip. Primary dirty remains status/entity observation, not this check.

## Deferred

- Worktree slot declarations in Workspace config
- Branch naming templates
- Auto prune on `doctor --fix` (prune stays an explicit command)
- Pin file interaction with slots (pin stays Primary-oriented unless later decided)
- Multi-Project or Workspace-level slots (prune `--all` landed)
- `status` obligations for orphans or dirty slots (registered slots already in status; doctor dirty-slot **warn** landed)
- Global `--wt` deep behavior beyond path binding on `project git` / `run`

## Non-goals

- Not a substitute for Primary checkout lifecycle (`multi-git.md`)
- Not under `.odm/`
- No `odm serve` / MCP / daemon involvement

## Related

- Architecture layout: `architecture.md`
- CLI matrix: `cli.md`
- Multi-git Primary: `multi-git.md`
- Agent packs (v1 local) + prompt (v1 thin) + start (sketch): `env-gen-packs.md`
- Phased delivery: `phased-delivery.md`
