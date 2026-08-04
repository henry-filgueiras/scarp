---
id: tsk_01KZ738BG7HDGBJDM57TW40ED5
sequence: 62
kind: task
status: pending
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
---

# Carry the terminal narrative on the close transition

## Objective

Let the transition that produces a terminal narrative also carry it, so
the section every closure in this repository already has stops arriving
by `cat >>`.

This is the mechanism half of
[[ide_01KYE386E7T9AZW4Z6MW39JB0R|idea 30]] and nothing more. That idea
parked with a 42-specimen census and zero violations; WitnessGlass
supplied the second-repository confirmation from the consumer side,
where a workflow contractually requires a `## Result` that
`new --body-file` correctly refuses and `close` declines to write.

The section name and its dated-or-not form are read off the corpus, not
chosen: 58 of 60 closed tasks carry a bare `## Result`, while both
closed dragons carry `## Resolution (YYYY-MM-DD)` and all ten closed
sprints carry `## Retrospective (YYYY-MM-DD)`. The date the dated forms
want is the transition date, which `perform_with_edge` already computes
for the `closed:` stamp.

The check half of idea 30 — a doctor finding for a terminal artifact
with an empty narrative — is deliberately not in this task. Idea 30
argues the mechanism must land first so the check arrives as a backstop
for a path the tool already paves, and its promotion question belongs to
the strict tier parked as idea 13.

## Acceptance criteria

- `scarp close <ref> --body-file <path>` appends the collection's
  terminal section, populated from the file, in the same safe write that
  rewrites `status` and stamps `closed:`. A failure at any point leaves
  the artifact byte-identical, as the existing atomic replace already
  guarantees.
- The section written matches the corpus per collection: `## Result` for
  tasks, `## Resolution (<transition date>)` for dragons,
  `## Retrospective (<transition date>)` for sprints.
- A body file that itself contains the terminal section's heading is
  refused, before any mutation, with a message saying Scarp writes that
  heading and the file should contain only what goes beneath it.
- `close` without `--body-file` behaves exactly as before and appends
  nothing: a creation stub must never grow an empty terminal section.
- On dragons, `--body-file` composes with `--resolved-by`; both land in
  the one write.
- `adopt` and `reject` gain nothing. Idea terminal states have zero
  specimens, and idea 30 defers them explicitly.
- Every already-closed artifact is untouched; this changes no existing
  file.
- [[ide_01KYE386E7T9AZW4Z6MW39JB0R|Idea 30]] transitions to `adopted`
  with an `adopted-by` edge naming this task.
- `scripts/check.sh` passes.
