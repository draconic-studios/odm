#!/usr/bin/env bash
# Exhaustive probe of odm surfaces against examples/todo (in-place or TEMP=1).
# Read-only on cloned remotes. Writes out/probe-log.txt under the workspace.
# Usage (monorepo root):
#   ODM=target/debug/odm examples/todo/scripts/probe.sh
#   TEMP=1 ODM=target/debug/odm examples/todo/scripts/probe.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TODO_SRC="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

LOG_BUF=""
PASS=0
FAIL=0
SKIP=0
NOTES=()

log() {
  echo "$*"
  LOG_BUF+="$*"$'\n'
}

note() {
  NOTES+=("$*")
  log "NOTE: $*"
}

die() {
  log "FATAL: $*"
  flush_log
  exit 1
}

flush_log() {
  mkdir -p "$DESK/out"
  printf '%s' "$LOG_BUF" >"$DESK/out/probe-log.txt"
  {
    echo ""
    echo "=== SUMMARY ==="
    echo "pass=$PASS fail=$FAIL skip=$SKIP"
    echo "notes=${#NOTES[@]}"
    for n in "${NOTES[@]+"${NOTES[@]}"}"; do
      echo "- $n"
    done
  } | tee -a "$DESK/out/probe-log.txt"
}

phase() {
  log ""
  log "######## $* ########"
}

record_pass() {
  PASS=$((PASS + 1))
  log "PASS: $*"
}

record_fail() {
  FAIL=$((FAIL + 1))
  log "FAIL: $*"
}

record_skip() {
  SKIP=$((SKIP + 1))
  log "SKIP: $*"
}

run_cap() {
  # run_cap VARNAME cmd...
  local _var="$1"
  shift
  set +e
  local _out
  _out="$("$@" 2>&1)"
  local _ec=$?
  set -e
  printf -v "$_var" '%s' "$_out"
  return "$_ec"
}

expect_exit() {
  local want="$1"
  local label="$2"
  shift 2
  set +e
  local out
  out="$("$@" 2>&1)"
  local got=$?
  set -e
  if [[ "$got" -eq "$want" ]]; then
    record_pass "$label (exit $want)"
    printf '%s\n' "$out"
    return 0
  fi
  record_fail "$label (want exit $want got $got): $out"
  printf '%s\n' "$out"
  return 0
}

expect_match() {
  local label="$1"
  local pattern="$2"
  local haystack="$3"
  if grep -Eq -- "$pattern" <<<"$haystack"; then
    record_pass "$label"
  else
    record_fail "$label (no match /$pattern/)"
  fi
}

expect_no_match() {
  local label="$1"
  local pattern="$2"
  local haystack="$3"
  if grep -Eq -- "$pattern" <<<"$haystack"; then
    record_fail "$label (unexpected /$pattern/)"
  else
    record_pass "$label"
  fi
}

assert_clean() {
  local repo="$1"
  local dirty
  dirty="$(git -C "$repo" status --porcelain 2>/dev/null || true)"
  if [[ -n "$dirty" ]]; then
    record_fail "dirty tree: $repo :: $dirty"
  else
    record_pass "clean: $repo"
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
  die "odm binary not found"
}

resolve_odm
command -v git >/dev/null || die "git required"

if [[ "${TEMP:-}" == "1" ]]; then
  TMP="$(mktemp -d "${TMPDIR:-/tmp}/odm-todo-probe.XXXXXX")"
  trap 'rm -rf "$TMP"' EXIT
  DESK="$TMP/todo"
  cp -R "$TODO_SRC" "$DESK"
  rm -rf "$DESK/projects" "$DESK/progens/sheets" "$DESK/worktrees" "$DESK/out" \
    "$DESK/.odm/odm.lock.yaml" "$DESK/.odm/progen" "$DESK/.odm/cache" \
    "$DESK/.odm/log"
  git -C "$DESK" init -q
  git -C "$DESK" config user.email "probe@example.com"
  git -C "$DESK" config user.name "todo-probe"
else
  DESK="$TODO_SRC"
fi

odm() {
  "$ODM" --root "$DESK" "$@"
}

log "probe: ODM=$ODM"
log "probe: DESK=$DESK"
log "probe: TEMP=${TEMP:-0}"
log "probe: date=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
log "probe: odm_version=$(odm --version 2>&1 || true)"

# ---------------------------------------------------------------------------
phase "0 bootstrap / discovery"
# ---------------------------------------------------------------------------
out="$(expect_exit 2 "init refuses existing workspace" odm init)" || true
out="$(expect_exit 1 "unknown command" odm not-a-command)" || true
out="$(expect_exit 1 "unknown global project" odm --project nope status)" || true

# ---------------------------------------------------------------------------
phase "1 sync"
# ---------------------------------------------------------------------------
if [[ ! -d "$DESK/projects/tip-top/.git" ]]; then
  expect_exit 0 "sync all" odm sync
else
  expect_exit 0 "sync (already present)" odm sync
fi
expect_exit 0 "sync one name" odm sync tip-top
expect_exit 1 "sync unknown name" odm sync does-not-exist
for p in tip-top cheat-key portfolio rss-td-game; do
  [[ -d "$DESK/projects/$p/.git" ]] && record_pass "exists projects/$p" || record_fail "missing projects/$p"
done
[[ -d "$DESK/progens/sheets/.git" ]] && record_pass "exists progens/sheets" || record_fail "missing progens/sheets"
[[ -f "$DESK/.odm/odm.lock.yaml" ]] && record_pass "lock file present" || record_fail "lock missing"
log "lock preview:"
log "$(head -40 "$DESK/.odm/odm.lock.yaml" 2>/dev/null || true)"

# ---------------------------------------------------------------------------
phase "2 pin"
# ---------------------------------------------------------------------------
out="$(expect_exit 0 "pin status" odm pin status)"
out="$(expect_exit 0 "pin status --json" odm --json pin status)"
expect_match "pin status json has entries" 'tip-top|in_sync|present|drift' "$out"
out="$(expect_exit 0 "pin apply" odm pin apply)"
out="$(expect_exit 0 "pin status after apply --json" odm --json pin status)"
expect_match "in_sync after apply" 'in_sync' "$out"
expect_exit 0 "pin apply named" odm pin apply tip-top
expect_exit 4 "pin apply unknown" odm pin apply nope-entity

# detached HEAD is expected after pin apply
head_state="$(git -C "$DESK/projects/tip-top" symbolic-ref -q HEAD 2>/dev/null || echo DETACHED)"
log "tip-top HEAD state: $head_state"
note "pin apply leaves detached HEAD on managed checkouts (by design)"

# ---------------------------------------------------------------------------
phase "3 status / doctor"
# ---------------------------------------------------------------------------
out="$(expect_exit 0 "status human" odm status)"
out="$(expect_exit 0 "status --json" odm --json status)"
expect_match "status projects" 'tip-top' "$out"
expect_match "status progens" 'desk|sheets' "$out"
out="$(expect_exit 0 "doctor" odm doctor)"
out="$(expect_exit 0 "doctor --json" odm --json doctor)"
out="$(expect_exit 0 "doctor --fix" odm doctor --fix)"

# ---------------------------------------------------------------------------
phase "4 project lifecycle read surfaces"
# ---------------------------------------------------------------------------
out="$(expect_exit 0 "project list" odm project list)"
out="$(expect_exit 0 "project list --json" odm --json project list)"
for p in tip-top cheat-key portfolio rss-td-game; do
  out="$(expect_exit 0 "project info $p" odm project info "$p")"
  out="$(expect_exit 0 "project info $p --json" odm --json project info "$p")"
  expect_match "info $p has path" 'path|worktree' "$out"
done
expect_exit 1 "project info unknown" odm project info nope

# read-only git passthrough
out="$(expect_exit 0 "git rev-parse" odm project git tip-top -- rev-parse HEAD)"
expect_match "rev-parse sha" '^[0-9a-f]{40}$' "$(echo "$out" | tail -1)"
out="$(expect_exit 0 "git log" odm project git cheat-key -- log -3 --oneline)"
out="$(expect_exit 0 "git branch -a" odm project git portfolio -- branch -a)"
out="$(expect_exit 0 "git remote -v" odm project git rss-td-game -- remote -v)"
expect_match "remote github" 'jared-hembrow' "$out"
out="$(expect_exit 0 "git status porcelain" odm project git tip-top -- status --porcelain)"
# empty porcelain = clean
clean_line="$(echo "$out" | tail -1 | tr -d '[:space:]')"
[[ -z "$clean_line" || "$clean_line" == *"PASS"* ]] && record_pass "porcelain empty-ish" || note "porcelain output: $out"

expect_exit 1 "git without --" odm project git tip-top rev-parse HEAD

# ---------------------------------------------------------------------------
phase "5 worktree slots"
# ---------------------------------------------------------------------------
# ensure primary is a real branch for worktree add (pin apply detached can complicate)
# re-attach primary to origin/main without rewriting remote — local checkout only
git -C "$DESK/projects/tip-top" checkout -B main origin/main 2>/dev/null \
  || git -C "$DESK/projects/tip-top" checkout main 2>/dev/null \
  || note "could not reattach tip-top to main; worktree add may still work"

out="$(expect_exit 0 "worktree list empty/ok" odm project worktree list tip-top)"
out="$(expect_exit 0 "worktree add dogfood" odm project worktree add tip-top probe-slot --branch odm-todo-probe)"
out="$(expect_exit 0 "worktree list" odm project worktree list tip-top)"
expect_match "list has probe-slot" 'probe-slot' "$out"
out="$(expect_exit 0 "worktree list --json" odm --json project worktree list tip-top)"
expect_match "json dirty field" 'dirty' "$out"

# status/info should show slots
out="$(expect_exit 0 "status with slots" odm --json status)"
expect_match "status worktree_slots" 'probe-slot|worktree_slots' "$out"
out="$(expect_exit 0 "info with slots" odm --json project info tip-top)"
expect_match "info worktree_slots" 'probe-slot' "$out"

# orphan
mkdir -p "$DESK/worktrees/tip-top/orphan-probe"
out="$(expect_exit 0 "status orphans" odm --json status)"
expect_match "orphan observed" 'orphan-probe|worktree_orphans' "$out"
out="$(expect_exit 0 "doctor orphan warn" odm doctor)"
note "doctor orphan warn expected for orphan-probe"
out="$(expect_exit 0 "prune orphans" odm project worktree prune tip-top)"
[[ ! -d "$DESK/worktrees/tip-top/orphan-probe" ]] && record_pass "orphan pruned" || record_fail "orphan remains"

# run --wt
out="$(expect_exit 0 "run --project --wt" odm run in-tip-top --project tip-top --wt probe-slot)"
expect_exit 4 "run --wt missing slot" odm run in-tip-top --project tip-top --wt no-such-slot

# git --wt
out="$(expect_exit 0 "git --wt rev-parse" odm project git tip-top --wt probe-slot -- rev-parse --is-inside-work-tree)"
expect_match "wt inside" 'true' "$out"

# prune --all
out="$(expect_exit 0 "prune --all dry path" odm project worktree prune --all)"
out="$(expect_exit 0 "worktree rm" odm project worktree rm tip-top probe-slot --force)"
out="$(expect_exit 0 "list after rm" odm project worktree list tip-top)"
expect_no_match "slot gone" 'probe-slot' "$out"

expect_exit 1 "worktree add bad project" odm project worktree add nope slot1

# ---------------------------------------------------------------------------
phase "6 progen lifecycle + store"
# ---------------------------------------------------------------------------
out="$(expect_exit 0 "progen list" odm progen list)"
out="$(expect_exit 0 "progen list --json" odm --json progen list)"
out="$(expect_exit 0 "progen info desk" odm progen info desk)"
out="$(expect_exit 0 "progen info sheets" odm progen info sheets)"
out="$(expect_exit 0 "progen info desk --json" odm --json progen info desk)"
expect_exit 1 "progen info unknown" odm progen info nope

out="$(expect_exit 0 "progen reindex" odm progen reindex)"
out="$(expect_exit 0 "progen reindex desk only" odm progen reindex --progen desk)"
out="$(expect_exit 0 "progen doctor" odm progen doctor)"

out="$(expect_exit 0 "progen get welcome" odm progen get welcome --progen desk)"
expect_match "get token" 'TodoWelcomeToken' "$out"
out="$(expect_exit 0 "progen body welcome" odm progen body welcome --progen desk)"
expect_match "body token" 'TodoWelcomeToken' "$out"
out="$(expect_exit 0 "progen tree desk" odm progen tree --progen desk)"
expect_match "tree Welcome" 'Welcome' "$out"
out="$(expect_exit 0 "progen ls desk" odm progen ls --progen desk)"
expect_match "ls welcome" 'welcome' "$out"
out="$(expect_exit 0 "progen backlinks" odm progen backlinks projects-map --progen desk)"
expect_match "backlink welcome" 'welcome' "$out"
out="$(expect_exit 0 "progen get --json" odm --json progen get welcome --progen desk)"

expect_exit 4 "progen get missing id" odm progen get no-such-id --progen desk
# multi-progen without --progen should error for single-root cmds
out="$(expect_exit 1 "progen get without --progen (multi)" odm progen get welcome)" || true

# sheets real repo index
out="$(expect_exit 0 "progen ls sheets" odm progen ls --progen sheets)"
log "sheets ls (first 20 lines):"
log "$(echo "$out" | head -20)"
out="$(expect_exit 0 "progen tree sheets" odm progen tree --progen sheets)"

# ---------------------------------------------------------------------------
phase "7 find / context"
# ---------------------------------------------------------------------------
out="$(expect_exit 0 "find welcome token" odm find TodoWelcomeToken)"
expect_match "find welcome" 'welcome' "$out"
out="$(expect_exit 0 "find --json" odm --json find TodoWelcomeToken)"
out="$(expect_exit 0 "find --limit 2" odm find Todo --limit 2)"
out="$(expect_exit 0 "find empty query lists" odm find)"
out="$(expect_exit 0 "find --progen desk" odm find TodoRulesToken --progen desk)"
expect_match "rules hit" 'rules' "$out"
out="$(expect_exit 0 "find --progen-group all-docs" odm find Todo --progen-group all-docs)"
out="$(expect_exit 0 "find --progen-group default" odm find TodoWelcomeToken --progen-group default)"
expect_match "default group" 'welcome' "$out"
out="$(expect_exit 0 "find --progen-group personal" odm find TodoDeskToken --progen-group personal)"
expect_exit 1 "find --limit 0" odm find x --limit 0
out="$(expect_exit 0 "find zero hits" odm find ZzzNoHitTokenEver12345)"
expect_exit 1 "find unknown progen" odm find x --progen nope

out="$(expect_exit 0 "context welcome" odm context welcome --progen desk)"
expect_match "context anchor" 'welcome' "$out"
out="$(expect_exit 0 "context name:id" odm context desk:welcome)"
out="$(expect_exit 0 "context --json" odm --json context welcome --progen desk)"
expect_match "context json neighborhood" 'anchor|outgoing|incoming' "$out"
expect_exit 4 "context missing" odm context missing-id --progen desk
expect_exit 1 "context multi without scope" odm context welcome
expect_exit 1 "context conflict prefix" odm context desk:welcome --progen sheets

# ---------------------------------------------------------------------------
phase "8 run actions"
# ---------------------------------------------------------------------------
out="$(expect_exit 0 "run list" odm run)"
expect_match "list hello" 'hello' "$out"
out="$(expect_exit 0 "run list --json" odm --json run)"
out="$(expect_exit 0 "run hello" odm run hello)"
expect_match "hello-todo" 'hello-todo' "$out"
out="$(expect_exit 0 "run hello --json" odm --json run hello)"
expect_match "exitCode 0" '"exitCode":[[:space:]]*0' "$out"
expect_exit 7 "run fail exit 7" odm run fail
out="$(expect_exit 0 "run chain" odm run chain)"
expect_match "step1" 'step1' "$out"
expect_match "step2" 'step2' "$out"
out="$(expect_exit 0 "run status-all" odm run status-all)"
out="$(expect_exit 0 "run in-tip-top --project" odm run in-tip-top --project tip-top)"
out="$(expect_exit 0 "run in-tip-top-dir" odm run in-tip-top-dir)"
out="$(expect_exit 0 "run read-only-log" odm run read-only-log)"
expect_exit 1 "run unknown action" odm run no-such-action
expect_exit 1 "run --project unknown" odm run hello --project nope

# ---------------------------------------------------------------------------
phase "9 generate"
# ---------------------------------------------------------------------------
out="$(expect_exit 0 "generate list" odm generate)"
expect_match "note gen" 'note' "$out"
out="$(expect_exit 0 "generate list --json" odm --json generate)"
expect_match "json generators" 'generators|note' "$out"
rm -rf "$DESK/out/note"
out="$(expect_exit 0 "generate dry-run" odm generate note --dest out/note --dry-run)"
[[ ! -e "$DESK/out/note/note.md" ]] && record_pass "dry-run no write" || record_fail "dry-run wrote"
out="$(expect_exit 0 "generate dry-run --json" odm --json generate note --dest out/note --dry-run)"
expect_match "dry_run true" 'dry_run' "$out"
out="$(expect_exit 0 "generate real" odm generate note --dest out/note)"
[[ -f "$DESK/out/note/note.md" ]] && record_pass "note.md written" || record_fail "note.md missing"
expect_exit 3 "generate exists without force" odm generate note --dest out/note
out="$(expect_exit 0 "generate --force" odm generate note --dest out/note --force)"
expect_exit 1 "generate remote deferred" odm generate remote-deferred --dest out/remote
expect_exit 1 "generate unknown" odm generate nope --dest out/x
expect_exit 2 "generate dest escape" odm generate note --dest ../outside

# ---------------------------------------------------------------------------
phase "10 json error envelope"
# ---------------------------------------------------------------------------
out="$(expect_exit 1 "json usage error" odm --json not-a-cmd)" || true
expect_match "json error envelope" '"ok":\s*false|"error"' "$out" || note "json error shape: $out"

# ---------------------------------------------------------------------------
phase "11 project add/rm roundtrip (local only fixture)"
# ---------------------------------------------------------------------------
# add a path-only project pointing at a tiny local git repo we create under out/
LOCAL_GIT="$DESK/out/local-fixture.git"
rm -rf "$LOCAL_GIT" "$DESK/projects/local-fixture"
git init --bare "$LOCAL_GIT" >/dev/null
git -C "$LOCAL_GIT" symbolic-ref HEAD refs/heads/main
WORK=$(mktemp -d)
git clone "$LOCAL_GIT" "$WORK" >/dev/null 2>&1
echo '# local' >"$WORK/README.md"
git -C "$WORK" checkout -b main >/dev/null 2>&1 || true
git -C "$WORK" add README.md
git -C "$WORK" -c user.email=p@e -c user.name=p commit -m init >/dev/null
git -C "$WORK" push -u origin main >/dev/null 2>&1
rm -rf "$WORK"

out="$(expect_exit 0 "project add local-fixture" odm project add local-fixture --path projects/local-fixture --url "$LOCAL_GIT" --branch main)"
[[ -d "$DESK/projects/local-fixture/.git" ]] && record_pass "local-fixture cloned" || record_fail "local-fixture missing"
out="$(expect_exit 0 "project info local" odm project info local-fixture)"
out="$(expect_exit 0 "project rm keep tree" odm project rm local-fixture)"
[[ -d "$DESK/projects/local-fixture" ]] && record_pass "rm kept tree" || record_fail "rm deleted tree unexpectedly"
# re-add and delete
out="$(expect_exit 0 "project add again" odm project add local-fixture --path projects/local-fixture --url "$LOCAL_GIT" --branch main --no-clone)"
out="$(expect_exit 0 "project rm --delete" odm project rm local-fixture --delete --force)"
[[ ! -d "$DESK/projects/local-fixture" ]] && record_pass "rm --delete removed" || record_fail "rm --delete left tree"

# progen add/rm path-only
mkdir -p "$DESK/out/extra-vault"
printf '%s\n' '---' 'id: extra' 'title: Extra' '---' '' '# Extra' >"$DESK/out/extra-vault/Extra.md"
out="$(expect_exit 0 "progen add extra" odm progen add extra --path out/extra-vault)"
out="$(expect_exit 0 "progen info extra" odm progen info extra)"
out="$(expect_exit 0 "progen reindex with extra" odm progen reindex --progen extra)"
out="$(expect_exit 0 "find in extra" odm find Extra --progen extra)"
out="$(expect_exit 0 "progen rm extra" odm progen rm extra)"
[[ -d "$DESK/out/extra-vault" ]] && record_pass "progen rm kept path" || record_fail "progen rm deleted path"

# ---------------------------------------------------------------------------
phase "12 cleanliness of real remotes"
# ---------------------------------------------------------------------------
for p in tip-top cheat-key portfolio rss-td-game; do
  assert_clean "$DESK/projects/$p"
done
assert_clean "$DESK/progens/sheets"

# ensure no unexpected remotes pushes configured as dirty
for p in tip-top cheat-key portfolio rss-td-game; do
  br="$(git -C "$DESK/projects/$p" rev-parse --abbrev-ref HEAD 2>/dev/null || echo DETACHED)"
  log "projects/$p branch=$br"
done

# ---------------------------------------------------------------------------
phase "13 help surfaces"
# ---------------------------------------------------------------------------
for cmd in "" project progen pin "project worktree" generate run find context; do
  if [[ -z "$cmd" ]]; then
    out="$(expect_exit 0 "help root" odm --help)"
  else
    # shellcheck disable=SC2086
    out="$(expect_exit 0 "help $cmd" odm $cmd --help)"
  fi
done

flush_log
log ""
log "probe complete → $DESK/out/probe-log.txt"
if [[ "$FAIL" -gt 0 ]]; then
  exit 1
fi
exit 0
