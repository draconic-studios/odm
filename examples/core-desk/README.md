# core-desk

Offline dogfood Workspace for ODM **core** only (multi-git sync, pin, status, doctor). No progens, actions, or generators.

## Layout

```text
examples/core-desk/
  README.md
  fixtures/
    README.md
    alpha.git/    # bare fixture
    beta.git/     # bare fixture
  .odm/
    odm.config.yaml
```

Managed checkouts (`projects/alpha`, `projects/beta`) and `odm.lock.yaml` are **not** committed — they appear after `odm sync`.

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
```

## Fixtures

See [fixtures/README.md](fixtures/README.md) to rebuild the bare repos. Committed fixtures are enough for offline clone and tests — no network.
