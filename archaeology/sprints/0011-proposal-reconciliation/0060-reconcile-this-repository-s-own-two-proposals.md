---
id: tsk_01KYZXTN71EDDR370MD3F00CK9
sequence: 60
kind: task
status: pending
sprint: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
created: 2026-08-01
---

# Reconcile this repository's own two proposals

## Objective

Run the finished lifecycle against the two proposals this repository has
actually filed, and close the loop on both.

This is the sprint's proof that the thing works for its purpose rather
than merely works — the same distinction sprint 10's retrospective drew
when idea 38 arrived through the channel instead of being tested through
it.

### The two instances

They are not the same case, which is why both are worth doing:

- **Issue [#2](https://github.com/henry-filgueiras/scarp/issues/2)** was
  realized as [[ide_01KYZVJ6XCK11DP67GVMC3M23C|idea 38]] and shipped in
  `0.2.0`. It has been landed and stale for a sprint. Reconciling it is
  the ordinary case, run against real history rather than a fixture.
- **Issue [#3](https://github.com/henry-filgueiras/scarp/issues/3)** was
  realized as [[ide_01KYZXGDY8YAFXMP1FV931ZB0M|idea 40]] — the proposal
  that asked for this sprint. It becomes reconcilable only after this
  sprint's own work reaches `main`, so it exercises the refusal first
  and the success path second, in that order, without contriving either.

Issue #3 completes a path with no hand transcription anywhere in it:
drafted on a phone, filed as a structured issue, realized by Scarp,
landed by an ordinary commit, and closed by Scarp against its own
canonical record.

### Sequencing and authority

Both reconciliations post publicly under the repository's identity, and
issue #3's requires this sprint's commits to be on `main` first. **Only
Henry pushes.** This task therefore waits on a human action it must not
attempt, and that dependency is the point rather than an inconvenience:
it is the same boundary the whole channel is built around.

### What to watch for

Record friction as first-class output, not as an aside — the comment's
wording read cold, anything the refusal failed to explain, whether the
landing proof behaved against real history the way it did against tests,
and any step still performed by hand.

## Acceptance criteria

- Issue #2 carries a reconciliation comment naming idea 38 and its
  landing commit, and is closed as completed.
- Issue #3 refuses reconciliation before this sprint's work lands,
  observed rather than assumed, and is reconciled and closed after.
- `scarp proposal list` reports no open realized proposals afterwards.
- Nothing about the canonical artifacts changed: `scarp doctor` is green,
  and ideas 38 and 40 are byte-identical to before.
- Observed friction is recorded in the Result, including anything that
  argues for or against task 58's outcome.
