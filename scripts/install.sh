#!/usr/bin/env sh
# Install prebuilt odm from GitHub Releases.
# Usage: curl -fsSL https://raw.githubusercontent.com/hembrow-innovations/odm/main/scripts/install.sh | sh
# Env: ODM_VERSION (tag or bare version), ODM_INSTALL_DIR (default: $HOME/.local/bin)

set -eu

REPO="${ODM_REPO:-hembrow-innovations/odm}"
INSTALL_DIR="${ODM_INSTALL_DIR:-${HOME}/.local/bin}"
GITHUB_API="${GITHUB_API:-https://api.github.com}"
GITHUB_URL="${GITHUB_URL:-https://github.com}"

err() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || err "required command not found: $1"
}

detect_triple() {
  os="$(uname -s 2>/dev/null || true)"
  arch="$(uname -m 2>/dev/null || true)"

  case "${OS:-}" in
    Windows_NT) err "Windows is not supported; install on macOS or Linux (see docs)" ;;
  esac

  case "$os" in
    MINGW*|MSYS*|CYGWIN*|Windows_NT)
      err "Windows is not supported; install on macOS or Linux (see docs)"
      ;;
    Darwin)
      case "$arch" in
        arm64|aarch64) printf '%s\n' "aarch64-apple-darwin" ;;
        x86_64) printf '%s\n' "x86_64-apple-darwin" ;;
        *) err "unsupported macOS architecture: ${arch} (supported: arm64, x86_64)" ;;
      esac
      ;;
    Linux)
      case "$arch" in
        x86_64|amd64) printf '%s\n' "x86_64-unknown-linux-gnu" ;;
        aarch64|arm64) printf '%s\n' "aarch64-unknown-linux-gnu" ;;
        *) err "unsupported Linux architecture: ${arch} (supported: x86_64, aarch64)" ;;
      esac
      ;;
    *)
      err "unsupported OS: ${os:-unknown} (supported: macOS, Linux)"
      ;;
  esac
}

normalize_version() {
  # Accept "0.1.1" or "v0.1.1" → bare version without leading v
  v="$1"
  case "$v" in
    v*|V*) printf '%s\n' "${v#?}" ;;
    *) printf '%s\n' "$v" ;;
  esac
}

latest_version() {
  need_cmd curl
  json="$(curl -fsSL "${GITHUB_API}/repos/${REPO}/releases/latest")" || \
    err "failed to fetch latest release from ${GITHUB_API}/repos/${REPO}/releases/latest"
  tag="$(printf '%s\n' "$json" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -1)"
  [ -n "$tag" ] || err "could not parse tag_name from latest release JSON"
  normalize_version "$tag"
}

download() {
  url="$1"
  dest="$2"
  if ! curl -fsSL -o "$dest" "$url"; then
    err "download failed: ${url}"
  fi
  [ -s "$dest" ] || err "download empty: ${url}"
}

file_sha256() {
  f="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$f" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$f" | awk '{print $1}'
  else
    err "need sha256sum or shasum to verify download"
  fi
}

lookup_expected_sha() {
  # Prefer release-wide SHA256SUMS; fall back to per-asset .sha256
  archive_name="$1"
  sums_file="$2"
  per_asset="$3"

  if [ -f "$sums_file" ] && [ -s "$sums_file" ]; then
    # GNU coreutils format: "<hash>  <filename>" or "<hash> *<filename>"
    line="$(grep -E "[[:space:]](\*?)${archive_name}\$" "$sums_file" | head -1 || true)"
    if [ -n "$line" ]; then
      printf '%s\n' "$line" | awk '{print $1}'
      return 0
    fi
  fi

  if [ -f "$per_asset" ] && [ -s "$per_asset" ]; then
    # first field is the hash (file may be "HASH" or "HASH  name")
    awk '{print $1; exit}' "$per_asset"
    return 0
  fi

  return 1
}

main() {
  need_cmd curl
  need_cmd tar
  need_cmd uname
  need_cmd mktemp

  if [ -z "${ODM_INSTALL_DIR:-}" ] && [ -z "${HOME:-}" ]; then
    err "HOME is unset; set ODM_INSTALL_DIR to choose an install path"
  fi

  triple="$(detect_triple)"
  if [ -n "${ODM_VERSION:-}" ]; then
    version="$(normalize_version "$ODM_VERSION")"
  else
    printf 'Resolving latest release for %s...\n' "$REPO"
    version="$(latest_version)"
  fi
  [ -n "$version" ] || err "empty version"
  tag="v${version}"

  archive_name="odm-${version}-${triple}.tar.gz"
  base_url="${GITHUB_URL}/${REPO}/releases/download/${tag}"
  archive_url="${base_url}/${archive_name}"
  sums_url="${base_url}/SHA256SUMS"
  per_sha_url="${base_url}/${archive_name}.sha256"

  tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/odm-install.XXXXXX")"
  cleanup() { rm -rf "$tmpdir"; }
  trap cleanup EXIT INT HUP TERM

  archive_path="${tmpdir}/${archive_name}"
  sums_path="${tmpdir}/SHA256SUMS"
  per_sha_path="${tmpdir}/${archive_name}.sha256"

  printf 'Downloading %s...\n' "$archive_url"
  download "$archive_url" "$archive_path"

  # Checksums are best-effort fetch; verify requires at least one source
  curl -fsSL -o "$sums_path" "$sums_url" 2>/dev/null || true
  curl -fsSL -o "$per_sha_path" "$per_sha_url" 2>/dev/null || true

  expected="$(lookup_expected_sha "$archive_name" "$sums_path" "$per_sha_path" || true)"
  if [ -z "${expected:-}" ]; then
    err "no SHA256 checksum found for ${archive_name} (looked for SHA256SUMS and ${archive_name}.sha256 on release ${tag})"
  fi

  actual="$(file_sha256 "$archive_path")"
  if [ "$actual" != "$expected" ]; then
    err "SHA256 mismatch for ${archive_name}: expected ${expected}, got ${actual}"
  fi
  printf 'SHA256 OK (%s)\n' "$actual"

  printf 'Extracting odm...\n'
  tar xzf "$archive_path" -C "$tmpdir" odm 2>/dev/null || \
    tar xzf "$archive_path" -C "$tmpdir" ./odm 2>/dev/null || \
    err "archive does not contain odm binary at root: ${archive_name}"

  bin_src="${tmpdir}/odm"
  [ -f "$bin_src" ] || err "extracted binary missing: ${bin_src}"
  chmod +x "$bin_src"

  mkdir -p "$INSTALL_DIR"
  bin_dst="${INSTALL_DIR}/odm"
  # atomic-ish replace
  cp "$bin_src" "${bin_dst}.tmp"
  chmod +x "${bin_dst}.tmp"
  mv -f "${bin_dst}.tmp" "$bin_dst"

  printf 'Verifying %s --version...\n' "$bin_dst"
  if ! "$bin_dst" --version; then
    err "installed binary failed: ${bin_dst} --version"
  fi

  printf '\nInstalled odm %s (%s) to %s\n' "$version" "$triple" "$bin_dst"

  case ":${PATH}:" in
    *":${INSTALL_DIR}:"*) ;;
    *)
      printf 'Note: %s is not on PATH. Add it, e.g.:\n' "$INSTALL_DIR"
      printf '  export PATH="%s:$PATH"\n' "$INSTALL_DIR"
      ;;
  esac
}

main "$@"
