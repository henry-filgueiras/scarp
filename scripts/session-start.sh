#!/usr/bin/env bash
# SessionStart orientation (task 34): open each session with active-sprint
# status, its pending tasks, and one fortune line, built only from existing
# scarp commands. This hook is deliberately an instrument — the friction of
# composing it is desire-path evidence for or against idea 24's
# `scarp status`. `/next` remains the explicit post-/clear reorientation;
# settings match startup and resume sources only.
set -uo pipefail

cd "${CLAUDE_PROJECT_DIR:?CLAUDE_PROJECT_DIR is unset}" 2>/dev/null || exit 0

# Every failure path emits exactly one controlled line; raw cargo or scarp
# noise never reaches the session in either path.
notice() {
  echo "scarp orientation unavailable: $1 — run scripts/check.sh to investigate"
  exit 0
}

cargo build --quiet >/dev/null 2>&1 || notice "cargo build failed"
scarp=target/debug/scarp
[ -x "$scarp" ] || notice "no scarp binary at $scarp"

sprints=$("$scarp" list sprints 2>/dev/null) || notice "scarp list sprints failed"
# Status is always the second column of list output; titles cannot reach it.
active=$(printf '%s\n' "$sprints" | awk '$2 == "active"')

echo "scarp orientation:"
if [ -z "$active" ]; then
  echo "  no active sprint — use /next to reorient and pitch the next one"
else
  printf '%s\n' "$active" | sed 's/^/  /'
  tasks=$("$scarp" list tasks --active 2>/dev/null) || notice "scarp list tasks failed"
  pending=$(printf '%s\n' "$tasks" | awk '$2 == "pending"')
  if [ -z "$pending" ]; then
    echo "  no pending tasks — the active sprint may be ready to close"
  else
    echo "  pending tasks:"
    printf '%s\n' "$pending" | sed 's/^/    /'
  fi
fi

fortune_out=$("$scarp" fortune 2>/dev/null) || fortune_out=""
fortune_line=$(printf '%s\n' "$fortune_out" | head -n 1)
if [ -n "$fortune_line" ]; then
  echo "  fortune: $fortune_line"
fi
exit 0
