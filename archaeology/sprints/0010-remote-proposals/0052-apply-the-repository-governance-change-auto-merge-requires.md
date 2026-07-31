---
id: tsk_01KYX1WHY82P2WNW9RG5KWVGYA
sequence: 52
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
---

# Apply the repository governance change auto-merge requires

## Objective

Apply to `henry-filgueiras/scarp` the small settings the one-tap
proposal channel needs — realistically branch hygiene and nothing else
— exactly as [[tsk_01KYX1WHRPEXG8Z8EBPQJRHHFH|task 49]] recommended.

The task's title is now larger than its content, and that is left
uncorrected on purpose. It was created on 2026-07-31 to apply branch
protection and enable auto-merge; Henry declined auto-merge the same
day, and the governance change went with it. Renaming the task would
erase the fact that this sprint deliberately chose the cheaper path,
which is one of the more useful things about it.

Nothing is designed here. If task 49's recommendation seems wrong at
execution time, that is a finding to record and route back, not a
change to improvise.

## Why this is still its own task

It changes repository settings, whose effect is invisible in the
working tree — nothing in a `git diff` records that
`delete_branch_on_merge` flipped. That makes it the kind of work
[[idea-capability-constrained-work|idea 15]] describes: it requires
GitHub repository-administration access, and task 12's closure showed
an agent can perform part of such work through the API while
confidently mis-describing the parts it cannot see.

It also carries the consumer question. A setting we flip here is a
setting a consumer must be *told* to flip, and one they will not
discover from copying the workflow.

## Acceptance criteria

- The applied settings match task 49's recommendation exactly, or every
  deviation is recorded with its reason. If the recommendation is that
  nothing needs changing, this task closes having changed nothing and
  says so — that is a result, not a failure.
- The before state is captured before anything changes, so the change
  is reversible and the record shows what was actually true rather than
  what this sprint assumed. The 2026-07-31 snapshot in task 49 is a
  starting point, not the record.
- Each setting is verified after application by reading it back from
  the API, not by trusting the write.
- Any setting applied here is recorded as a consumer prerequisite for
  [[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]]'s recipe. A channel that
  silently depends on a repository setting is a channel that breaks on
  the first repository that copies it.
- Direct pushes to `main` are confirmed still to work, since CLAUDE.md's
  commit policy depends on it and this sprint's premise is that nothing
  about that changes.
- Per CLAUDE.md's first-performance policy, the exact commands or API
  calls used are recorded in this Result as dated provenance. They are
  not turned into a script.
- Any repository-settings change an agent could not make, or could not
  see to verify, is named as such rather than reported as done.
- A rollback path is recorded: what to change back, and how.

## Result
