---
id: spr_01KY7S6Q69YJ6HATZB48SZBRRM
sequence: 7
kind: sprint
status: closed
created: 2026-07-23
closed: 2026-07-25
---

# Sprint 7: Managed decisions and the daily loop

## Goal

Land four small, independent, unambiguous vertical slices chosen for
immediate utility: make decisions the fifth managed collection
(task 32), make `strata doctor` a commit gate in `scripts/check.sh`
(task 33), wire a session-start orientation hook into this repository
as a deliberate desire-path instrument (task 34), and ship shell
completions (task 35).

## Rationale

Sprint 5's retrospective named managed decisions the next-collection
candidate; sprint 6 deferred it behind the incident hold, which is now
released. The corpus is already conformant — all fifteen decisions
carry managed-style front matter with uniform `accepted` status — so
the slice is command coverage and discovery, not migration.

The other three items convert existing capability into daily-loop
pressure. Doctor inside `check.sh` makes archaeology validity a
commit-gate fact instead of a remembered manual step. The session-start
hook is an instrument as much as a feature: every session it runs
either confirms the orientation ritual is served by existing commands
or generates concrete friction evidence for or against
[[ide_01KY7S6GHMQ8ZWNXPX7TX21X7N|idea 24]]. Completions lower the cost
of human CLI use, which is where desire-path data comes from.

Deliberately not smuggled in: the spec-engine extraction
([[idea-declarative-collection-specs|idea 10]]) is not required by any
task here. If task 32's implementation shows that a fifth collection
means a wholesale further copy of collection mechanics, the implementer
surfaces that as a decision point rather than copying silently or
extracting silently — adoption of idea 10 is Henry's call, made on
that evidence.

A fifth managed collection widens the exposure of
[[drg-bootstrap-branch-collisions|dragon 1]] again; accepted unchanged,
as sprints 4 and 5 accepted it.
[[drg_01KY3C0S3JQKEMEB9BH6NVJ35F|Dragon 4]] is unaffected in kind.

## Success criteria

- `strata new decision`, `strata list decisions` (with `--json`), and
  `strata show decision:N` (with `--json`) work, and `doctor` validates
  decisions under the same invariants as other collections, over the
  unmodified existing corpus of fifteen files.
- `scripts/check.sh` fails when `strata doctor` reports problems in
  this repository.
- A fresh Claude Code session in this repository opens with
  active-sprint status and one fortune line produced by strata
  commands, and the friction observed is recorded in task 34's result.
- `strata completions <shell>` emits a completion script that loads
  cleanly in zsh at minimum.
- Every task closes with its result recorded; `scripts/check.sh` and
  `strata doctor` are green at close.

## Non-goals

- Decision lifecycle beyond creation as `accepted`: supersession and
  deprecation wait for the first real supersession event.
- Managed logs or comments collections.
- The spec-engine extraction (idea 10) as an end in itself; see
  Rationale for the only path by which it may enter.
- Relevance, ranking, or projection work: ideas 8, 12, 24, 25, and 27
  remain parked.
- The standing bootstrap non-goals: daemon, watcher, index,
  embeddings, MCP, GraphQL.

## Amendment: final pre-release dogfood sprint (2026-07-23)

This is the final pre-release dogfood sprint. It does not replace the
next sprint, "First Contact", whose expected scope is: an installable
v0.1.0, a quickstart, clean-machine verification, and a deterministic
60-second demo. Sprint 7's slices are chosen to be finished and lived
with before that release sprint opens; nothing here should grow toward
packaging or presentation work.

The same amendment commit strengthens the acceptance criteria of
tasks 32, 33, and 34 before implementation begins: task 32 gains
identity-format, addressing-matrix, lifecycle-refusal, global-invariant,
and README-projection criteria; task 33's gate claim is narrowed from
arbitrary corruption to doctor-detectable errors in managed
collections; task 34 gains hook-source, working-directory,
stdout-intent, `/next`-coexistence, failure-notice, and verification
criteria. Task 35 is unchanged.

## Retrospective (2026-07-25)

Eight tasks closed: the four planned slices (32–35) and four minted
mid-sprint from observed friction (36, 37, 38, 39). Every success
criterion holds: decisions are the fifth managed collection over the
unmodified fifteen-file corpus, `scripts/check.sh` fails on
doctor-detectable corruption (demonstrated against a corrupted
scratch copy), a fresh session opens with strata-built orientation
plus one fortune line, and `strata completions zsh` loads cleanly
under `zsh -f`. Doctor is green over 95 artifacts with zero
advisories.

Durable learnings, recorded where they belong:

- The idea 10 stop-condition in task 32 resolved *against*
  extraction: the fifth flat collection cost one `Collection` value,
  one create wrapper, and enumeration entries, with the interpreting
  machinery reused unmodified (task 32 result). Flat collections are
  already spec-driven in practice; the divergence that resisted
  data-ification in sprint 5 lives entirely in the containment
  layouts. Extraction pressure went down, not up.
- The session-start hook earned its "instrument" framing on day one:
  composing "active sprints with pending tasks" required stitching
  two `list` calls with `awk` on a column position, which is the
  concrete evidence [[ide_01KY7S6GHMQ8ZWNXPX7TX21X7N|idea 24]] was
  waiting for (task 34 result).
- The desire-path loop closed inside a single sprint: task 37's
  grep-for-`id:` friction minted task 38, `strata resolve` shipped,
  and its first real invocation answered the exact query that had
  been manual. This is the workflow the case study exists to prove.
- Lifecycle refusals as guidance paid off again: decisions refuse
  every verb with supersession guidance rather than not-found, at
  the cost of one best-effort probe that never blocks other
  collections (task 32 result).
- An agent implementing hook wiring is correctly permission-blocked
  from registering hooks itself — self-granting execution is a
  boundary. Future tasks whose deliverable is hook configuration
  should plan the human settings edit into the loop (task 34
  result).

Friction to fix next: idea 24 (`strata status`) now has a written
consumer — the entire body of `scripts/session-start.sh` is the
query it would replace, and `strata fortune | head -n 1` shows the
card format wants a one-line mode. Committing remains ceremony
beside the tool (idea 9). The pre-release runway is clear: the next
sprint is "First Contact" per this sprint's amendment — installable
v0.1.0, quickstart, clean-machine verification, and a deterministic
60-second demo, for which completions and the doctor gate were the
groundwork.
