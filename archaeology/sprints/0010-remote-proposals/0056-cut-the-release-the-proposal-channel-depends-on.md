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

Publish the Scarp release carrying
[[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task 51]]'s body-input surface, so the
proposal workflow has a published binary to pin.

This task exists because of a choice, not an accident. The workflow
installs a published `scarp` rather than building the checkout, which
makes this repository's channel identical to a consumer's and settles
which binary realized any given artifact. The cost of that choice is
this sequencing constraint: **task 51 → this release →
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] can go live.** An unreleased
flag is an unusable one.

## Relationship to sprint 9

[[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|Task 47]] in sprint 9 is the crates.io
quickstart anchor defect frozen into `0.1.0`. Its notes say explicitly
that spending a version number on one dead link is a poor trade, and
that holding it until another release-worthy change appears is a
reasonable choice. This is that change.

That does not make this task the owner of task 47. Batching it in is
Henry's call, recorded in task 47's own Result either way. What this
task owes is the coordination: ask the question before publishing
rather than after, since after is a version number too late.

## Acceptance criteria

- The release carries task 51's body-input surface, and its version
  number is chosen deliberately — whether an additive CLI surface is a
  minor bump or rides a patch is a judgment recorded here, not a
  default.
- Task 47's batching question is put to Henry before publication and
  its answer recorded, in task 47's Result if it ships and here either
  way.
- The full package reverification applies as in
  [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]] and
  [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]]: package construction,
  file list, unpack outside the checkout, and `cargo publish --dry-run
  --locked`, with neither `--allow-dirty` nor `--no-verify`.
- Publication follows the human-owned boundary
  [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] established: an agent
  prepares the exact command and stops.
- The published binary is verified to provide the body-input surface by
  installing it from crates.io into a clean environment and running it
  — not by trusting that what was tested locally is what shipped. This
  is the specific failure that would strand
  [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] with a pin to a release
  that cannot do the job.
- The install is timed in that clean environment, because the number is
  what a consumer pays on every proposal and it is the evidence
  [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]] needs.
- The exact version the proposal workflow will pin is recorded here, so
  task 54 does not have to infer it.
- Nothing in the `0.1.0` record is rewritten to suggest it was clean.
- Per CLAUDE.md's first-performance policy, any non-obvious invocation
  is recorded as dated provenance. The recurring form of a release
  remains a chore ledger ([[idea-chore-artifacts|idea 7]]), not a
  script.

## Result
