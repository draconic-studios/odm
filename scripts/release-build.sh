#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' crates/odm/Cargo.toml | head -1)"
if [[ -z "${VERSION}" ]]; then
  echo "error: could not parse version from crates/odm/Cargo.toml" >&2
  exit 1
fi

TARGET="${ODM_TARGET:-$(rustc -vV | sed -n 's/^host: //p')}"
if [[ -z "${TARGET}" ]]; then
  echo "error: could not determine target triple (set ODM_TARGET)" >&2
  exit 1
fi

BUILD_ARGS=(-p odm --release)
if [[ -n "${ODM_TARGET:-}" ]]; then
  BUILD_ARGS+=(--target "$TARGET")
fi

echo "Building odm ${VERSION} for ${TARGET}..."
cargo build "${BUILD_ARGS[@]}"

if [[ -n "${ODM_TARGET:-}" ]]; then
  BIN="target/${TARGET}/release/odm"
else
  BIN="target/release/odm"
fi

if [[ ! -f "$BIN" ]]; then
  # cross / explicit target without ODM_TARGET env still lands under target/$TARGET
  if [[ -f "target/${TARGET}/release/odm" ]]; then
    BIN="target/${TARGET}/release/odm"
  else
    echo "error: binary not found at ${BIN}" >&2
    exit 1
  fi
fi

if command -v strip >/dev/null 2>&1; then
  strip "$BIN" || true
fi

mkdir -p dist
STAGE="$(mktemp -d "${TMPDIR:-/tmp}/odm-release.XXXXXX")"
cleanup() { rm -rf "$STAGE"; }
trap cleanup EXIT

cp "$BIN" "$STAGE/odm"
chmod +x "$STAGE/odm"

cat >"$STAGE/README-release.txt" <<EOF
ODM ${VERSION} (${TARGET})

Install:
  1. Extract this archive somewhere on your PATH, e.g.:
       tar xzf odm-${VERSION}-${TARGET}.tar.gz -C /usr/local/bin odm
     or keep the folder and add it to PATH.
  2. Ensure git is on PATH.
  3. Run: odm --version

Docs: https://github.com/hembrow-innovations/odm (see README and docs/reference/)

Unix shell required for Actions (odm run).
EOF

if [[ -f LICENSE ]]; then
  cp LICENSE "$STAGE/LICENSE"
fi

ARCHIVE="dist/odm-${VERSION}-${TARGET}.tar.gz"
tar czf "$ARCHIVE" -C "$STAGE" .
echo "Wrote ${ARCHIVE}"

echo
echo "Next steps (manual publish):"
echo "  gh release create v${VERSION} dist/odm-${VERSION}-*.tar.gz --title \"v${VERSION}\" --notes-file CHANGELOG.md"
echo "  # or attach specific archives after building each target"
echo

if [[ "${ODM_RELEASE_PUBLISH:-}" == "1" ]]; then
  if ! command -v gh >/dev/null 2>&1; then
    echo "error: ODM_RELEASE_PUBLISH=1 but gh is not available" >&2
    exit 1
  fi
  echo "Publishing GitHub release v${VERSION}..."
  gh release create "v${VERSION}" "$ARCHIVE" \
    --title "v${VERSION}" \
    --notes-file CHANGELOG.md
fi
