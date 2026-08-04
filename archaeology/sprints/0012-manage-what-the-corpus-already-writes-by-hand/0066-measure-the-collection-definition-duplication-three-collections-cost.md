---
id: tsk_01KZ738BQTR4H7Z7YBKPPCXGHT
sequence: 66
kind: task
status: pending
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
---

# Measure the collection-definition duplication three collections cost

## Objective

Measure what adding three collections actually cost, and write the
number back into the idea that has been waiting for it since sprint 2.

[[idea-declarative-collection-specs|Idea 10]] asked for exactly this and
named the discipline: derive the spec shape from the second and third
concrete collections, not ahead of them; extract from working duplicated
code or the abstraction encodes guesses. This sprint supplies three at
once, and two of them break the mould in opposite directions — `log` has
neither lifecycle nor template, `principle` and `maintenance` have both.
That asymmetry is the most useful thing the measurement can report,
because a spec derived only from collections that resemble each other
would not have survived the log.

This is a checkpoint, not a mandate. The default outcome is evidence
recorded and the idea left parked.

## Acceptance criteria

- A per-collection inventory of what adding `log`, `principle`, and
  `maintenance` touched: which files, and for each site whether it was a
  pure constant or enum arm mechanically implied by the collection's
  existence, or a genuine per-collection semantic choice. The
  `Collection` static, the directory constant, the CLI enum arm and its
  `FromStr`, the `verb_guidance` arm, the `doctor` scan array, the
  `create_*` function, and the `probe_reachability` arm are each
  classified, not merely counted.
- The count distinguishes duplication that a data-driven spec would
  erase from duplication that encodes meaning a spec would have to carry
  anyway.
- The measurement is recorded as a dated section in idea 10's Evidence,
  in concrete terms — files, sites, and which of them a spec would
  actually remove — not as a verdict.
- An extraction is performed only if it is small, falls out of the work
  rather than being designed for it, and removes repeated arms or
  constants without collapsing a per-collection semantic difference. The
  log's absent lifecycle and absent template are the test any candidate
  extraction has to pass.
- If no extraction is performed, the reason is recorded and idea 10
  stays `parked`. That is a legitimate and expected result.
- Whatever this sprint learned about the log's statelessness and empty
  template is also recorded on
  [[ide_01KYZY233Z7GAKFPFSKEAF89ZD|idea 41]], since a collection with no
  managed sections has no ordering or ownership conflict to resolve and
  therefore bounds what that idea still has to answer.
- `scripts/check.sh` passes.
