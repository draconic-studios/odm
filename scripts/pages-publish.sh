#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

SITE="$ROOT/website"
if [[ ! -f "$SITE/index.html" ]]; then
  echo "error: website/index.html missing — nothing to publish" >&2
  exit 1
fi

BRANCH="${ODM_PAGES_BRANCH:-gh-pages}"
MSG="${ODM_PAGES_MESSAGE:-chore(pages): publish website}"

STAGE="$(mktemp -d "${TMPDIR:-/tmp}/odm-pages.XXXXXX")"
cleanup() { rm -rf "$STAGE"; }
trap cleanup EXIT

if command -v rsync >/dev/null 2>&1; then
  rsync -a --delete --exclude '.DS_Store' --exclude 'README.md' "$SITE"/ "$STAGE"/
else
  find "$STAGE" -mindepth 1 -maxdepth 1 -exec rm -rf {} +
  cp -R "$SITE"/. "$STAGE"/
  rm -f "$STAGE/README.md" "$STAGE/.DS_Store" 2>/dev/null || true
fi

if [[ ! -f "$STAGE/index.html" ]]; then
  echo "error: staged tree missing index.html" >&2
  exit 1
fi

rm -rf "$STAGE/.git"

git -C "$STAGE" init -q
git -C "$STAGE" checkout -q -b "$BRANCH"
git -C "$STAGE" add -A
if git -C "$STAGE" diff --cached --quiet; then
  echo "error: nothing staged to commit" >&2
  exit 1
fi
git -C "$STAGE" -c user.email="${GIT_AUTHOR_EMAIL:-pages@odm.local}" \
  -c user.name="${GIT_AUTHOR_NAME:-odm-pages}" \
  commit -q -m "$MSG"

# Import objects + update branch ref in this repo (orphan history)
if git show-ref --verify --quiet "refs/heads/$BRANCH"; then
  git fetch -q --force "$STAGE" "refs/heads/$BRANCH:refs/heads/$BRANCH"
else
  git fetch -q "$STAGE" "refs/heads/$BRANCH:refs/heads/$BRANCH"
fi

echo "Updated local branch '${BRANCH}' from website/"
echo "  tip: $(git rev-parse --short "$BRANCH")"

if [[ "${ODM_PAGES_PUSH:-}" == "1" ]]; then
  REMOTE="${ODM_PAGES_REMOTE:-origin}"
  PUSH_ARGS=(push)
  if [[ "${ODM_PAGES_FORCE:-}" == "1" ]]; then
    PUSH_ARGS+=(--force)
  fi
  PUSH_ARGS+=("$REMOTE" "refs/heads/$BRANCH:refs/heads/$BRANCH")
  echo "Pushing to ${REMOTE} ${BRANCH}..."
  git "${PUSH_ARGS[@]}"
  echo "Pushed. Site URL (after Pages build): https://hembrow-innovations.github.io/odm/"
else
  echo
  echo "Next steps:"
  echo "  ODM_PAGES_PUSH=1 $0"
  echo "  # force only gh-pages if history rewrite needed:"
  echo "  ODM_PAGES_PUSH=1 ODM_PAGES_FORCE=1 $0"
  echo
  echo "Pages settings: branch ${BRANCH}, folder / (root)"
  echo "Expected URL: https://hembrow-innovations.github.io/odm/"
fi
