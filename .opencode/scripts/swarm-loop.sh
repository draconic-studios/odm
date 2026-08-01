#!/usr/bin/env bash
# AFK swarm loop: fresh orchestrator session per iteration.
# Usage: .opencode/scripts/swarm-loop.sh [max_iters] [extra opencode args...]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

MAX_ITERS="${1:-20}"
if [[ $# -gt 0 ]]; then shift; fi

MAX_FAILS="${SWARM_MAX_FAILS:-3}"
OPENCODE_BIN="${OPENCODE_BIN:-opencode}"
AGENT="${SWARM_AGENT:-swarm-orchestrator}"
PROMPT_FILE="${SWARM_PROMPT:-.opencode/prompts/swarm-iteration.md}"

SWARM_DIR=".opencode/swarm"
LOG_DIR="$SWARM_DIR/log"
mkdir -p "$LOG_DIR"

if [[ ! -f "$SWARM_DIR/state.json" ]]; then
  cat >"$SWARM_DIR/state.json" <<'EOF'
{
  "iteration": 0,
  "current_ticket": null,
  "last_action": null,
  "seeded_at": null,
  "consecutive_failures": 0,
  "last_commit": null,
  "last_exit": null
}
EOF
fi

if ! command -v "$OPENCODE_BIN" >/dev/null 2>&1; then
  echo "error: opencode not found (OPENCODE_BIN=$OPENCODE_BIN)" >&2
  exit 127
fi

if [[ ! -f "$PROMPT_FILE" ]]; then
  echo "error: missing prompt $PROMPT_FILE" >&2
  exit 1
fi

fails=0
echo "swarm-loop: root=$ROOT max_iters=$MAX_ITERS max_fails=$MAX_FAILS agent=$AGENT"

for ((i = 1; i <= MAX_ITERS; i++)); do
  if [[ -f "$SWARM_DIR/STOP" ]]; then
    echo "swarm-loop: STOP file present — halting (130)"
    exit 130
  fi

  pad="$(printf '%04d' "$i")"
  stdout_log="$LOG_DIR/${pad}.stdout"
  title="swarm-$pad"

  echo "── iteration $i/$MAX_ITERS ──"

  set +e
  "$OPENCODE_BIN" run \
    --agent "$AGENT" \
    --auto \
    --title "$title" \
    --dir "$ROOT" \
    -f "$PROMPT_FILE" \
    "$@" \
    -- \
    "Iteration $i of $MAX_ITERS. Execute exactly one swarm pipeline cycle. End with SWARM_EXIT:N." \
    2>&1 | tee "$stdout_log"
  pipe_status=("${PIPESTATUS[@]}")
  oc_exit="${pipe_status[0]:-1}"
  set -e

  # Prefer explicit SWARM_EXIT from agent output over process exit code.
  swarm_exit=""
  if grep -qE 'SWARM_EXIT:0' "$stdout_log" 2>/dev/null; then
    swarm_exit=0
  elif grep -qE 'SWARM_EXIT:3' "$stdout_log" 2>/dev/null; then
    swarm_exit=3
  elif grep -qE 'SWARM_EXIT:2' "$stdout_log" 2>/dev/null; then
    swarm_exit=2
  fi

  code="${swarm_exit:-$oc_exit}"
  echo "swarm-loop: iteration=$i code=$code (opencode=$oc_exit swarm_exit=${swarm_exit:-none})"

  case "$code" in
    0)
      fails=0
      ;;
    3)
      echo "swarm-loop: DONE (empty backlog)"
      exit 0
      ;;
    130)
      echo "swarm-loop: stopped"
      exit 130
      ;;
    2|*)
      fails=$((fails + 1))
      echo "swarm-loop: fail streak $fails/$MAX_FAILS"
      if [[ "$fails" -ge "$MAX_FAILS" ]]; then
        echo "swarm-loop: too many consecutive failures — exit 2"
        exit 2
      fi
      ;;
  esac
done

echo "swarm-loop: reached max_iters=$MAX_ITERS"
exit 0
