# core-desk bare fixtures

`alpha.git` and `beta.git` are **bare** git repositories with at least one commit on `main` (a root `README.md`). They are committed so offline `git clone` and `cargo test` need no network.

## Verify

From the monorepo root:

```bash
git clone examples/core-desk/fixtures/alpha.git /tmp/core-desk-alpha
git -C /tmp/core-desk-alpha branch --show-current   # main
git -C /tmp/core-desk-alpha log -1 --oneline
rm -rf /tmp/core-desk-alpha
```

Same for `beta.git`.

## Rebuild

From the monorepo root (replace `alpha` with `beta` for the other fixture):

```bash
NAME=alpha
rm -rf "examples/core-desk/fixtures/${NAME}.git"
git init --bare "examples/core-desk/fixtures/${NAME}.git"
git -C "examples/core-desk/fixtures/${NAME}.git" symbolic-ref HEAD refs/heads/main

WORK=$(mktemp -d)
git clone "examples/core-desk/fixtures/${NAME}.git" "$WORK"
printf '# %s\n\nFixture project for core-desk dogfood.\n' "$NAME" > "$WORK/README.md"
git -C "$WORK" checkout -b main
git -C "$WORK" add README.md
git -C "$WORK" -c user.name='ODM Fixtures' -c user.email='fixtures@odm.local' \
  commit -m "Initial commit"
git -C "$WORK" push -u origin main
rm -rf "$WORK"
```

Repeat with `NAME=beta`.
