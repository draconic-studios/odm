# core-desk

Offline dogfood Workspace for ODM **core**, path-only **Progen** vault, shell **Actions**, a tiny local **Generator**, and a demo **Agent pack**.

## Layout

```text
examples/core-desk/
  README.md
  fixtures/
    README.md
    alpha.git/    # bare fixture
    beta.git/     # bare fixture
  progens/
    notes/        # path-only Progen = Obsidian vault (plain Markdown)
  actions/
    core.yaml     # hello / fail / chain
  generators/
    core.yaml     # hello → templates/hello
  templates/
    hello/        # local generate template
  agent-packs/
    demo/         # local pack source for install/link dogfood
  .odm/
    odm.config.yaml
```

Managed checkouts (`projects/alpha`, `projects/beta`) and `odm.lock.yaml` are **not** committed — they appear after `odm sync`. Progen index lives under `.odm/progen/notes/` after `odm progen reindex` (gitignored).

Config URLs are relative (`fixtures/alpha.git`). Integration harnesses should rewrite them to absolute `file://` paths against a temp copy when needed. Plain `git clone` from this directory root works as-is.

## Dogfood (once `odm` exists)

From the monorepo root:

```bash
cargo build -p odm
cd examples/core-desk
```

Optional (pin / gitignore materialize paths):

```bash
git init
```

Then:

```bash
odm --root . sync
# or, once root discovery works from cwd:
# odm sync

odm pin status
odm status
odm doctor

# optional
odm project list
odm pin apply

# Worktree slots (git primary after sync; --branch avoids double-checkout of main)
odm project worktree add alpha dogfood --branch odm-dogfood
odm project worktree list alpha
odm status                 # human lists slot names when non-empty (dirty suffix when dirty)
odm status --json          # projects[].worktree_slots: [{ "name", "path", "dirty" }]
# list --json: { "project", "slots": [{ "name", "path", "dirty" }] }
# dirty is true / false / null (null = cleanliness probe failed)
# path shape: worktrees/alpha/dogfood
# optional dirty demo (then clean up):
#   echo x > worktrees/alpha/dogfood/dirty.txt
#   odm status --json      # dogfood.dirty == true
#   odm project worktree list alpha --json
#   odm project worktree rm alpha dogfood --force
# optional cleanup (clean slot): odm project worktree rm alpha dogfood

# Orphan / dirty doctor + prune (empty dirs under worktrees/<project>/ that are not registered)
mkdir -p worktrees/alpha/stale-orphan
odm doctor                 # warn worktree_orphan:alpha:stale-orphan (not fixable)
# optional dirty registered slot (doctor warn only — does not auto-prune or clean):
#   echo x > worktrees/alpha/dogfood/dirty.txt
#   odm doctor             # warn worktree_dirty:alpha:dogfood (not fixable)
# doctor --fix does NOT delete orphans or clean dirty slots
odm project worktree prune alpha
# removes empty orphan dirs for one project; non-empty orphans need --force
# exit 3 when non-empty orphans remain without --force
# odm project worktree prune alpha --force
# workspace-wide orphan GC (every configured project; same empty/--force rules):
odm project worktree prune --all
# odm project worktree prune --all --force
# --json: { "all": true, "pruned": [{ "project", "name", "path" }], "skipped_nonempty": [...] }

# Progen / Obsidian vault
odm progen list
odm progen reindex
odm find DeskUniqueToken
odm find DeskUniqueToken --limit 5
odm context welcome
odm agent prompt welcome
# Open progens/notes in Obsidian or: obsidian-cli … against that folder

# Actions
odm run                 # list: hello, fail, chain
odm run hello           # prints hello-desk
odm run chain           # step1 then step2
odm run fail            # exit 7
odm --json run hello    # {"action":"hello","exitCode":0}

# Generators (local template copy)
odm generate            # list: hello
odm generate hello --dest out/hello
# out/hello/hello.txt

# Agent packs (local install into a home dir)
mkdir -p /tmp/odm-agent-home
odm agent pack install agent-packs/demo --home /tmp/odm-agent-home
odm agent pack list
# demo
odm status                 # lists agent packs (demo)
odm status --json          # agent_packs: [{ "name": "demo", …, "missing": false }]
# optional missing-dest demo (then clean up):
#   rm -rf /tmp/odm-agent-home/demo
#   odm doctor             # warn pack_missing:demo (not fixable)
#   odm status --json      # demo.missing == true
odm agent pack rm demo
odm agent pack list
# (no agent packs)
# after rm: doctor has no pack_missing:demo
```

## Fixtures

See [fixtures/README.md](fixtures/README.md) to rebuild the bare repos. Committed fixtures are enough for offline clone and tests — no network.
