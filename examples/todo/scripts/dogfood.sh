#!/usr/bin/env bash
# Full network tour of shipped odm commands against a temp copy of examples/todo.
# Clones real GitHub repos; never commit/push/reset those checkouts.
# Usage (from monorepo root after cargo build -p odm):
#   ODM=target/debug/odm examples/todo/scripts/dogfood.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TODO_SRC="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

die() {
  echo "todo-dogfood: FAIL: $*" >&2
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

assert_match() {
  local haystack="$1"
  local pattern="$2"
  local msg="$3"
  if ! grep -Eq -- "$pattern" <<<"$haystack"; then
    die "$msg (pattern=$pattern)"
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

# Refuse mutating git verbs against project checkouts.
assert_read_only_git() {
  local repo="$1"
  local dirty
  dirty="$(git -C "$repo" status --porcelain 2>/dev/null || true)"
  # pin apply may leave detached HEAD; dirty tree is a fail
  if [[ -n "$dirty" ]]; then
    die "checkout became dirty (read-only violated): $repo"
  fi
}

resolve_odm
require_cmd git
require_cmd cp
require_cmd mktemp
require_cmd curl

echo "todo-dogfood: ODM=$ODM"
echo "todo-dogfood: source=$TODO_SRC"

# Smoke network before long clone work
if ! curl -fsSIL --max-time 15 https://github.com/jared-hembrow/tip-top >/dev/null 2>&1; then
  die "network/GitHub unreachable (needed for real-repo dogfood)"
fi

TMP="$(mktemp -d "${TMPDIR:-/tmp}/odm-todo-dogfood.XXXXXX")"
cleanup() {
  rm -rf "$TMP"
}
trap cleanup EXIT

DESK="$TMP/todo"
AGENT_HOME="$TMP/agent-home"
mkdir -p "$AGENT_HOME"
cp -R "$TODO_SRC" "$DESK"
# Drop prior local debris if source was already dogfooded in-tree
rm -rf "$DESK/projects" "$DESK/progens/sheets" "$DESK/worktrees" "$DESK/out" \
  "$DESK/.odm/odm.lock.yaml" "$DESK/.odm/progen" "$DESK/.odm/cache" \
  "$DESK/.odm/log" "$DESK/.odm/agent-packs.json"

git -C "$DESK" init -q
git -C "$DESK" config user.email "todo-dogfood@example.com"
git -C "$DESK" config user.name "todo-dogfood"

odm() {
  "$ODM" --root "$DESK" "$@"
}

# --- phases -----------------------------------------------------------------

phase "sync (real GitHub clones)"
odm sync
for p in tip-top cheat-key portfolio rss-td-game; do
  [[ -d "$DESK/projects/$p/.git" ]] || die "projects/$p missing after sync"
done
[[ -d "$DESK/progens/sheets/.git" ]] || die "progens/sheets missing after sync"
[[ -f "$DESK/.odm/odm.lock.yaml" ]] || die "lock file missing after sync"

phase "pin status / apply (local detached only)"
odm pin status
odm pin apply
out="$(odm --json pin status)"
assert_match "$out" 'in_sync' "pin status JSON missing in_sync"
for p in tip-top cheat-key portfolio rss-td-game; do
  assert_read_only_git "$DESK/projects/$p"
done
assert_read_only_git "$DESK/progens/sheets"

phase "status --json"
odm status
out="$(odm --json status)"
assert_match "$out" '"name": "tip-top"' "status JSON missing tip-top"
assert_match "$out" '"name": "sheets"' "status JSON missing sheets progen"

phase "doctor"
odm doctor

phase "project list / info / git (read-only)"
odm project list
odm project info tip-top
out="$(odm project git tip-top -- rev-parse HEAD)"
assert_match "$out" '^[0-9a-f]{40}$' "project git rev-parse failed"
out="$(odm project git cheat-key -- log -1 --oneline)"
[[ -n "$out" ]] || die "empty git log"
out="$(odm project git portfolio -- status --porcelain)"
[[ -z "$out" ]] || die "portfolio dirty after read-only ops: $out"

phase "worktree add / list / prune (local slots; no push)"
odm project worktree add tip-top dogfood --branch odm-todo-dogfood
out="$(odm project worktree list tip-top)"
assert_match "$out" 'dogfood' "worktree list missing dogfood"
# orphan dir
mkdir -p "$DESK/worktrees/tip-top/stale-orphan"
odm project worktree prune tip-top
[[ ! -d "$DESK/worktrees/tip-top/stale-orphan" ]] || die "orphan not pruned"
# run action in worktree slot cwd
odm run in-tip-top --project tip-top --wt dogfood
assert_read_only_git "$DESK/projects/tip-top"
# remove slot cleanly
odm project worktree rm tip-top dogfood --force

phase "progen reindex + façade"
odm progen list
odm progen reindex
out="$(odm progen get welcome --progen desk)"
assert_match "$out" 'TodoWelcomeToken' "progen get welcome"
out="$(odm progen body welcome --progen desk)"
assert_match "$out" 'TodoWelcomeToken' "progen body welcome"
out="$(odm progen tree --progen desk)"
assert_match "$out" 'Welcome.md' "progen tree"
out="$(odm progen backlinks projects-map --progen desk)"
assert_match "$out" 'welcome' "progen backlinks"
out="$(odm progen ls --progen desk)"
assert_match "$out" 'welcome' "progen ls"
odm progen doctor
# sheets is a real clone — reindex and list
out="$(odm progen ls --progen sheets)"
[[ -n "$out" ]] || true # may be empty if repo has no frontmatter ids

phase "find + --progen-group"
out="$(odm find TodoWelcomeToken)"
assert_match "$out" 'welcome' "find TodoWelcomeToken"
out="$(odm find TodoDeskToken --limit 5)"
assert_match "$out" 'desk-readme' "find --limit"
out="$(odm find TodoRulesToken --progen desk)"
assert_match "$out" 'rules' "find --progen desk"
out="$(odm find token --progen-group all-docs)"
assert_match "$out" 'welcome|desk-readme|rules|projects-map' "find --progen-group all-docs"

phase "context / agent prompt"
out="$(odm context welcome --progen desk)"
assert_match "$out" 'welcome' "context"
out="$(odm agent prompt welcome --progen desk)"
assert_match "$out" 'welcome' "agent prompt"
out="$(odm context desk:welcome)"
assert_match "$out" 'welcome' "context name:id"

phase "run hello / fail / chain / --project"
out="$(odm run)"
assert_match "$out" 'hello' "run list missing hello"
out="$(odm run hello)"
assert_match "$out" 'hello-todo' "run hello output"
expect_exit 7 odm run fail
out="$(odm run chain)"
assert_match "$out" 'step1' "run chain step1"
assert_match "$out" 'step2' "run chain step2"
odm run in-tip-top --project tip-top
odm run in-tip-top-dir
odm run read-only-log
out="$(odm --json run hello)"
assert_match "$out" '"exitCode":[[:space:]]*0' "run hello --json"

phase "generate dry-run / real / force / remote-deferred"
out="$(odm generate)"
assert_match "$out" 'note' "generate list"
odm generate note --dest out/note --dry-run
[[ ! -e "$DESK/out/note/note.md" ]] || die "dry-run wrote files"
odm generate note --dest out/note
[[ -f "$DESK/out/note/note.md" ]] || die "generate did not write note.md"
odm generate note --dest out/note --force
expect_exit 1 odm generate remote-deferred --dest out/remote

phase "agent pack install / list / link / rm"
odm agent pack install agent-packs/todo-desk --home "$AGENT_HOME"
out="$(odm agent pack list)"
assert_match "$out" 'todo-desk' "pack list after install"
odm agent pack rm todo-desk
out="$(odm agent pack list)"
assert_no_match "$out" '^todo-desk([[:space:]]|$)' "pack list still has todo-desk after rm"
odm agent pack link agent-packs/todo-desk --home "$AGENT_HOME"
out="$(odm agent pack list)"
assert_match "$out" 'todo-desk' "pack list after link"
odm agent pack rm todo-desk

phase "agent start (one-shot)"
expect_exit 0 odm --project tip-top agent start -- true
expect_exit 1 odm --project tip-top agent start -- false

phase "final cleanliness (no dirty remotes)"
for p in tip-top cheat-key portfolio rss-td-game; do
  assert_read_only_git "$DESK/projects/$p"
done
assert_read_only_git "$DESK/progens/sheets"

phase "done"
echo "todo-dogfood: OK (temp workspace cleaned on exit)"
