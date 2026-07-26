---
id: tsk_01KY7S6QB3EZY0A441WRG301FX
sequence: 34
kind: task
status: closed
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-23
closed: 2026-07-25
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

## Result

The checked-in project settings register a SessionStart hook matching
`startup|resume` only — `clear` keeps `/next` as the explicit
reorientation path — running `scripts/session-start.sh`, which cds to
`$CLAUDE_PROJECT_DIR`, builds quietly, and composes curated lines
from `strata list sprints`, `strata list tasks --active`, and
`strata fortune`. Both failure paths (build failure, scan failure)
collapse to one controlled notice; raw cargo noise is suppressed in
success and failure alike, verified by running with cargo removed
from PATH.

Verified in a real fresh session (2026-07-25): a headless
`claude -p` run in this repository — a genuine `startup` source —
was asked to quote the injected orientation and returned the fortune
line verbatim. Script latency: ~90 ms warm (build no-op plus three
strata invocations); a cold first build after a clean checkout would
add its full compile time, accepted for a dev-only repository.

Composition friction (the desire-path evidence this task was
instrumented to collect, for [[ide_01KY7S6GHMQ8ZWNXPX7TX21X7N|idea
24]]):

- "Active sprints with their pending tasks" is not one query. The
  script stitches two `list` invocations and filters each with
  `awk '$2 == ...'`, keying on the *column position* of status in
  human output — an informal contract no test pins for external
  consumers. The robust alternative (`--json` + jq) would add a jq
  dependency to a checked-in hook.
- `strata fortune` emits a multi-line card; honoring the one-line
  spec meant `head -n 1`, discarding the age/path line the card
  format considers essential.
- Nothing aggregates across collections; every line of orientation
  is hand-formatted shell. A `strata status` would replace the
  entire body of this script.

One hook-shaped friction outside strata itself: the agent
implementing this task was correctly permission-blocked from editing
`.claude/settings.json` (hook registration is self-granting
execution), so the settings edit was performed by Henry — worth
remembering for any future task whose deliverable is hook wiring.
