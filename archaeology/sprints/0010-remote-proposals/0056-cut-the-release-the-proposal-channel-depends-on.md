---
id: tsk_01KYX31ACH05NGA3GYH0TJA870
sequence: 56
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
---

# Cut the release the proposal channel depends on

## Objective

Publish a Scarp release carrying [[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task
51]]'s `--body-file` surface, and whatever else has earned a version
number by then.

*Removed from the critical path 2026-08-01 by
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s Option B adjudication.*
This task was created because Option A's workflow would have installed a
pinned published `scarp` in CI, making an unreleased flag an unusable
one. Option B has the operator run the Scarp they already have, so
nothing in the sprint is blocked by publication and
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] no longer waits on this.

The release keeps its own independent reasons, which are not
manufactured for this sprint:

- `--body-file` is shipped, tested, and useful to anyone who installs
  Scarp; leaving it unreleased means the published tool cannot do the
  thing this sprint is about.
- [[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|Task 47]] in sprint 9 holds a
  cosmetic defect frozen into `0.1.0`, and its notes say explicitly that
  it is waiting for another release-worthy change to batch with. That
  reason predates this sprint and survives it.

Ordering is therefore free. This task may run at any point, or slip past
the sprint entirely without blocking anything.

## Relationship to sprint 9

Batching [[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|task 47]] in is Henry's call,
recorded in task 47's own Result either way. What this task owes is the
coordination: ask the question before publishing rather than after,
since after is a version number too late.

## Acceptance criteria

- The release carries task 51's `--body-file` surface, and its version
  number is chosen deliberately — whether an additive CLI surface is a
  minor bump or rides a patch is a judgment recorded here, not a
  default.
- Task 47's batching question is put to Henry before publication and its
  answer recorded, in task 47's Result if it ships and here either way.
- The full package reverification applies as in
  [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]] and
  [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]]: package construction, file
  list, unpack outside the checkout, and `cargo publish --dry-run
  --locked`, with neither `--allow-dirty` nor `--no-verify`.
- Publication follows the human-owned boundary
  [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] established: an agent
  prepares the exact command and stops.
- The published binary is verified to provide `--body-file` by
  installing it from crates.io into a clean environment and running it,
  rather than trusting that what was tested locally is what shipped.
- The install is timed in that clean environment, because that number is
  the evidence [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]] needs.
- Nothing in the `0.1.0` record is rewritten to suggest it was clean.
- Per CLAUDE.md's first-performance policy, any non-obvious invocation is
  recorded as dated provenance. The recurring form of a release remains a
  chore ledger ([[idea-chore-artifacts|idea 7]]), not a script.
- The Result states plainly that this release was **not** a sprint 10
  dependency, so a later reader does not infer that Option B needed a
  publication.

## Result
