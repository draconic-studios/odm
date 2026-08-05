# todo

Network dogfood Workspace for ODM against **real GitHub repos** (jared-hembrow public). Clones only; dogfood is **read-only** on those checkouts (no commit/push/reset against remotes).

Companion to offline [core-desk](../core-desk/README.md). After probing, see [REVIEW.md](REVIEW.md).

## Layout

```text
examples/todo/
  README.md
  REVIEW.md              # post-probe findings (written after dogfood)
  scripts/dogfood.sh     # full tour (temp copy; network)
  scripts/probe.sh       # exhaustive surface + edge cases → review notes
  progens/
    desk/                # path-only Progen (Todo*Token notes)
  actions/todo.yaml
  generators/todo.yaml
  templates/note/
  .odm/odm.config.yaml
```

Managed checkouts (`projects/*`, `progens/sheets`) and `odm.lock.yaml` are **not** committed — they appear after `odm sync`. Indexes, worktrees, and `out/` are gitignored.

## Projects (real remotes)

- **tip-top** — https://github.com/jared-hembrow/tip-top
- **cheat-key** — https://github.com/jared-hembrow/cheat-key
- **portfolio** — https://github.com/jared-hembrow/portfolio
- **rss-td-game** — https://github.com/jared-hembrow/rss-td-game

## Progens

- **desk** — local vault under `progens/desk` (path-only)
- **sheets** — https://github.com/jared-hembrow/cheat-key-sheets (git-managed Progen)

Groups: `default` → desk; `all-docs` → desk+sheets; `personal` → desk.

## Quick start

From the monorepo root (needs network + git):

```bash
cargo build -p odm
ODM=target/debug/odm examples/todo/scripts/dogfood.sh
```

Dogfood copies this tree to a temp dir, runs the full shipped-CLI tour, never modifies the committed example or pushes to remotes, then cleans up.

Exhaustive edge-case probe (temp copy recommended):

```bash
TEMP=1 ODM=target/debug/odm examples/todo/scripts/probe.sh
# in-place also works after sync; writes out/probe-log.txt
```

Interactive (mutates a local working copy under `examples/todo/`):

```bash
cargo build -p odm
export ODM="$PWD/target/debug/odm"
cd examples/todo
"$ODM" --root . sync
"$ODM" --root . doctor --fix
"$ODM" --root . status
"$ODM" --root . progen reindex
"$ODM" --root . find TodoWelcomeToken
```

## Review

Post-probe findings: [REVIEW.md](REVIEW.md) (every CLI surface, exit codes, real-repo footguns).

## Rules

See `progens/desk/Rules.md`. Summary: sync/pin/status/doctor/find/run/generate/worktree are fine; **no** commits or pushes inside cloned remotes.
