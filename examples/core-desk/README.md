# core-desk

Offline dogfood Workspace for ODM **core**, multi-**Progen** vaults + groups, shell **Actions** (including project-scoped cwd), and a tiny local **Generator**.

## Layout

```text
examples/core-desk/
  README.md
  scripts/dogfood.sh   # full offline tour (source of truth)
  fixtures/
    README.md
    alpha.git/    # bare fixture
    beta.git/     # bare fixture
  progens/
    notes/        # path-only Progen = Obsidian vault (DeskUniqueToken / id welcome+readme)
    ops/          # second Progen (OpsUniqueToken / id ops-note)
  actions/
    core.yaml     # hello / fail / chain / in-alpha / in-alpha-dir
  generators/
    core.yaml     # hello → templates/hello
  templates/
    hello/        # local generate template
  .odm/
    odm.config.yaml
```

Managed checkouts (`projects/alpha`, `projects/beta`) and `odm.lock.yaml` are **not** committed — they appear after `odm sync`. Progen indexes live under `.odm/progen/<name>/` after `odm progen reindex` (gitignored). Worktree slots and `out/` are also gitignored.

**Assets note:** two progens (`notes`, `ops`), groups `default` → notes and `all-docs` → notes+ops, vault note ids for wikilink/backlink demos, and `in-alpha` for `run --project` / `--wt`.

Config URLs are relative (`fixtures/alpha.git`). Integration harnesses should rewrite them to absolute `file://` paths against a temp copy when needed. Plain `git clone` from this directory root works as-is.

## Quick start

From the monorepo root:

```bash
cargo build -p odm
ODM=target/debug/odm examples/core-desk/scripts/dogfood.sh
```

The script copies core-desk to a temp dir (does not modify this tree), runs the full shipped-CLI tour fail-fast, and cleans up.

## Full tour

**Full tour:** `scripts/dogfood.sh` — sync → pin → status → doctor → project git → worktree → progen façade → find groups → context → run → generate. No network; relative fixtures only.

## Fixtures

See [fixtures/README.md](fixtures/README.md) to rebuild the bare repos. Committed fixtures are enough for offline clone and tests — no network.
