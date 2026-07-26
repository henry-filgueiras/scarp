---
id: ide_01KYE386E7T9AZW4Z6MW39JB0R
sequence: 30
kind: idea
status: parked
created: 2026-07-25
---

# Terminal narratives ride the close transition

## Problem

Every terminal transition in this repository carries a hand-appended
narrative section — `## Result` on closed tasks, dated
`## Resolution` on closed dragons, dated `## Retrospective` on closed
sprints — and nothing but discipline connects the state to the story.
The corpus census at parking time: 34 of 34 closed tasks, 2 of 2
closed dragons, and 6 of 6 closed sprints conform, 42 specimens with
zero violations, all hand-performed. `strata close` transitions the
front matter and asks for nothing; a closure with an empty or missing
narrative stays green through `close`, `doctor`, and the commit gate
alike. The convention lives only in CLAUDE.md's workflow text — the
prompt-file protocol registry
[[ide_01KYE05MPV39B4FYQCTA013PRR|idea 29]] names — and the risk
concentrates exactly where unsupervised runs do their closing.

Two halves, and their order matters. Enforcement without a
transition-time mechanism is hostile: doctor complains after the
fact and the user materializes a section by hand to appease it. The
mechanism belongs first, so the check arrives as a backstop for a
path the tool already paves.

## Sketch

Mechanism: `close` (and only `close`; creation stubs must not grow
empty terminal sections, per task 37's audit) accepts the terminal
narrative and appends the kind's section in the same safe write as
the front-matter transition — one intent, one mutation, extending
the atomicity argument of
[[idea-single-invocation-commits|idea 9]]. Input surfaces, in
adoption order: a non-interactive argument or stdin first, since
agents are the demonstrated closer; a bare interactive `close` may
later open `$EDITOR` through [[idea-strata-edit|idea 3]]'s validated
round-trip rather than a parallel implementation. Prior art for the
feel: `git commit`, where the transition command demands the message
at transition time and an empty message aborts the commit.

Check: a terminal-state artifact whose narrative section is missing
or whitespace-only becomes a doctor finding — advisory at the base
tier, failing in an opt-in strict tier that `scripts/check.sh` and
CI would run. The tier split follows decision 10's precedent for
absent provenance edges ("enforcing presence would turn every bare
`strata close` into an instant red doctor"), and the promotion
question is the same one already parked with
[[idea-strict-doctor|idea 13]]; this check and the provenance-edge
absence check form a natural strict-tier family — terminal
completeness: the state, the story, and the edge.

The set, by specimen count: task/`Result`, dragon/`Resolution`,
sprint/`Retrospective`, with the dated forms
(`## Resolution (2026-07-21)`) matching. Idea terminal states are
deferred: zero adopted or rejected specimens exist, and their
provenance edges may prove to be the whole story.

## Boundaries

- Presence of non-whitespace text only; doctor never judges narrative
  quality ("semantic systems advise; they do not define truth").
- The base tier stays advisory so a repository is never unhealthy
  between the two commits of today's write-section-then-close flow;
  strictness is asked for at gates, not ambient.
- No new lifecycle states, section vocabulary beyond the three
  specimen kinds, or reopening semantics (a reopened dragon keeps its
  historical `Resolution`; decision 10's stale-edge advice already
  models the analogous state).
- The `$EDITOR` path is idea 3's machinery or nothing; no bespoke
  editor loop inside `close`.
- Adoption needs no new evidence for the convention itself — the
  census above is the evidence — but implementation waits for a
  sprint that owns it; parking is not a task.

## Evidence

Origin: Henry, 2026-07-25, in conversation, including the
mechanism-before-check coupling. Corpus census same day: 42
conforming specimens, zero violations, all hand-appended — the
strongest desire path yet recorded, per
[[ide_01KY7S6GG3NAA35KBJTC6CA1TM|idea 23]]'s promotion logic.
Named as a cross-cutting observation in task 37's result (the
close-time section family) and felt from the tool side in this
sprint's tablestakes review: `close task:37` would have succeeded
with an empty Result, and only CLAUDE.md's workflow text said
otherwise. Prior art: `git commit` aborting on an empty message;
issue trackers requiring a resolution comment on state transitions
(Jira workflow validators); `visudo`-style validated editor
round-trips (idea 3's lineage).
