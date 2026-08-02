#!/usr/bin/env bash
# Full offline tour of shipped odm commands against a temp copy of core-desk.
# Usage (from monorepo root after cargo build -p odm):
#   ODM=target/debug/odm examples/core-desk/scripts/dogfood.sh
# Or rely on relative discovery of target/debug/odm | target/release/odm.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CORE_DESK="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

die() {
  echo "dogfood: FAIL: $*" >&2
  exit 1
}

phase() {
  echo ""
  echo "=== $* ==="
}

expect_exit() {
  local want="$1"
  shift
  set +e
  "$@"
  local got=$?
  set -e
  if [[ "$got" -ne "$want" ]]; then
    die "expected exit $want from: $* (got $got)"
  fi
}

# Capture stdout then grep (avoids pipefail+grep -q SIGPIPE).
assert_match() {
  local haystack="$1"
  local pattern="$2"
  local msg="$3"
  if ! grep -Eq -- "$pattern" <<<"$haystack"; then
    die "$msg"
  fi
}

assert_no_match() {
  local haystack="$1"
  local pattern="$2"
  local msg="$3"
  if grep -Eq -- "$pattern" <<<"$haystack"; then
    die "$msg"
  fi
}

resolve_odm() {
  if [[ -n "${ODM:-}" ]]; then
    if [[ ! -x "$ODM" && -f "$ODM" ]]; then
      chmod +x "$ODM" 2>/dev/null || true
    fi
    [[ -x "$ODM" ]] || die "ODM is not executable: $ODM"
    if [[ "$ODM" != /* ]]; then
      ODM="$(cd "$(dirname "$ODM")" && pwd)/$(basename "$ODM")"
    fi
    return
  fi
  local cand
  for cand in \
    "$REPO_ROOT/target/debug/odm" \
    "$REPO_ROOT/target/release/odm" \
    "$(command -v odm 2>/dev/null || true)"; do
    if [[ -n "$cand" && -x "$cand" ]]; then
      ODM="$cand"
      return
    fi
  done
  die "odm binary not found; build with cargo build -p odm or set ODM="
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "required command not on PATH: $1"
}

resolve_odm
require_cmd git
require_cmd cp
require_cmd mktemp

echo "dogfood: ODM=$ODM"
echo "dogfood: source=$CORE_DESK"

TMP="$(mktemp -d "${TMPDIR:-/tmp}/odm-core-desk-dogfood.XXXXXX")"
cleanup() {
  rm -rf "$TMP"
}
trap cleanup EXIT

DESK="$TMP/core-desk"
AGENT_HOME="$TMP/agent-home"
mkdir -p "$AGENT_HOME"
cp -R "$CORE_DESK" "$DESK"

git -C "$DESK" init -q
git -C "$DESK" config user.email "dogfood@example.com"
git -C "$DESK" config user.name "core-desk-dogfood"

odm() {
  "$ODM" --root "$DESK" "$@"
}

# --- phases -----------------------------------------------------------------

phase "sync"
odm sync
[[ -d "$DESK/projects/alpha" ]] || die "projects/alpha missing after sync"
[[ -d "$DESK/projects/beta" ]] || die "projects/beta missing after sync"
[[ -f "$DESK/.odm/odm.lock.yaml" ]] || die "lock file missing after sync"

phase "pin status / apply"
odm pin status
odm pin apply
out="$(odm --json pin status)"
assert_match "$out" 'in_sync' "pin status JSON missing in_sync"

phase "status --json"
odm status
out="$(odm --json status)"
assert_match "$out" '"name": "alpha"' "status JSON missing alpha"

phase "doctor"
odm doctor

phase "project list / info / git"
odm project list
odm project info alpha
out="$(odm project git alpha -- rev-parse HEAD)"
assert_match "$out" '^[0-9a-f]{40}$' "project git rev-parse failed"

phase "worktree add / list / prune"
odm project worktree add alpha dogfood --branch odm-dogfood
out="$(odm project worktree list alpha)"
assert_match "$out" 'dogfood' "worktree list missing dogfood"
mkdir -p "$DESK/worktrees/alpha/stale-orphan"
odm project worktree prune alpha
[[ ! -d "$DESK/worktrees/alpha/stale-orphan" ]] || die "orphan not pruned"

phase "progen reindex + façade"
odm progen list
odm progen reindex
out="$(odm progen get welcome --progen notes)"
assert_match "$out" 'DeskUniqueToken' "progen get welcome"
out="$(odm progen body welcome --progen notes)"
assert_match "$out" 'DeskUniqueToken' "progen body welcome"
out="$(odm progen tree --progen notes)"
assert_match "$out" 'Welcome.md' "progen tree"
out="$(odm progen backlinks readme --progen notes)"
assert_match "$out" 'welcome' "progen backlinks"
out="$(odm progen ls --progen notes)"
assert_match "$out" 'welcome' "progen ls"
odm progen doctor

phase "find + --progen-group"
out="$(odm find DeskUniqueToken)"
assert_match "$out" 'welcome' "find DeskUniqueToken"
out="$(odm find DeskUniqueToken --limit 5)"
assert_match "$out" 'welcome' "find --limit"
out="$(odm find OpsUniqueToken --progen ops)"
assert_match "$out" 'ops-note' "find --progen ops"
out="$(odm find token --progen-group all-docs)"
assert_match "$out" 'welcome' "find --progen-group missing notes hit"
assert_match "$out" 'ops-note' "find --progen-group missing ops hit"

phase "context / agent prompt"
out="$(odm context welcome --progen notes)"
assert_match "$out" 'welcome' "context"
out="$(odm agent prompt welcome --progen notes)"
assert_match "$out" 'welcome' "agent prompt"

phase "run hello / fail / chain / --project"
out="$(odm run)"
assert_match "$out" 'hello' "run list missing hello"
out="$(odm run hello)"
assert_match "$out" 'hello-desk' "run hello output"
expect_exit 7 odm run fail
out="$(odm run chain)"
assert_match "$out" 'step1' "run chain step1"
assert_match "$out" 'step2' "run chain step2"
odm run in-alpha --project alpha
out="$(odm --json run hello)"
assert_match "$out" '"exitCode":[[:space:]]*0' "run hello --json"

phase "generate dry-run / real / force"
out="$(odm generate)"
assert_match "$out" 'hello' "generate list"
odm generate hello --dest out/hello --dry-run
[[ ! -e "$DESK/out/hello/hello.txt" ]] || die "dry-run wrote files"
odm generate hello --dest out/hello
[[ -f "$DESK/out/hello/hello.txt" ]] || die "generate did not write hello.txt"
odm generate hello --dest out/hello --force
[[ -f "$DESK/out/hello/hello.txt" ]] || die "generate --force lost hello.txt"

phase "agent pack install / list / link / rm"
odm agent pack install agent-packs/demo --home "$AGENT_HOME"
out="$(odm agent pack list)"
assert_match "$out" 'demo' "pack list after install"
odm agent pack rm demo
out="$(odm agent pack list)"
assert_no_match "$out" '^demo([[:space:]]|$)' "pack list still has demo after rm"
odm agent pack link agent-packs/demo --home "$AGENT_HOME"
out="$(odm agent pack list)"
assert_match "$out" 'demo' "pack list after link"
odm agent pack rm demo

phase "agent start (one-shot)"
expect_exit 0 odm --project alpha agent start -- true
expect_exit 1 odm --project alpha agent start -- false

phase "done"
echo "dogfood: OK (temp workspace cleaned on exit)"
