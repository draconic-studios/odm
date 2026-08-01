# core-desk

Offline dogfood Workspace for ODM **core** + a path-only **Progen** vault (Obsidian-compatible). No actions or generators.

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

# Progen / Obsidian vault
odm progen list
odm progen reindex
odm find DeskUniqueToken
odm context welcome
# Open progens/notes in Obsidian or: obsidian-cli … against that folder
```

## Fixtures

See [fixtures/README.md](fixtures/README.md) to rebuild the bare repos. Committed fixtures are enough for offline clone and tests — no network.
