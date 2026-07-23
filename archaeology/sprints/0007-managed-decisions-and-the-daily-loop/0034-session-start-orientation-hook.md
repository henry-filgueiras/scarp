---
id: tsk_01KY7S6QB3EZY0A441WRG301FX
sequence: 34
kind: task
status: pending
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-23
---

# Session-start orientation hook

## Objective

Wire a SessionStart hook into this repository's checked-in Claude Code
settings that opens each session with orientation — active sprints
with their pending tasks, plus one `strata fortune` line — built only
from existing strata commands. The hook is deliberately an instrument:
its recorded friction is the evidence base for or against
[[ide_01KY7S6GHMQ8ZWNXPX7TX21X7N|idea 24]]'s `strata status`.

## Acceptance criteria

- A checked-in project settings SessionStart hook matches only the
  `startup` and `resume` sources — not `clear`, `compact`, or `fork` —
  and prints active-sprint status with pending tasks plus one fortune
  line, produced by strata commands.
- The hook runs relative to `$CLAUDE_PROJECT_DIR`, not the incidental
  working directory.
- The hook's stdout is written as intentional orientation context for
  the session that receives it — curated lines, not raw command
  spillover.
- `/next` remains the explicit post-`/clear` reorientation path; the
  hook does not fire on `clear` and does not duplicate the `/next`
  ceremony.
- On build or run failure the hook emits at most one controlled
  one-line notice; raw cargo build noise is suppressed in both the
  success and failure paths.
- Observed latency and composition friction (aggregation gaps,
  formatting, anything hand-stitched around missing tool support) are
  recorded in this task's result as desire-path evidence.
- `scripts/check.sh` passes, and the hook is verified once in a real
  fresh session with the result recorded.
