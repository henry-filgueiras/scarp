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

Apply to `henry-filgueiras/scarp` exactly the settings
[[tsk_01KYX1WHRPEXG8Z8EBPQJRHHFH|task 49]] recommended and
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] permitted, so that a
proposal pull request has something to wait on and something to merge
after.

Nothing is designed here. If task 49's recommendation seems wrong at
execution time, that is a finding to record and route back, not a
change to improvise. If task 49 recommended the fallback instead of
auto-merge, this task applies whatever that fallback needs and says so.

## Why this is its own task

It changes the repository's governance for every contributor including
Henry, and it is the one task in the sprint whose effect is invisible
in the working tree — nothing in a `git diff` records that `main`
became protected. That makes it exactly the kind of work
[[idea-capability-constrained-work|idea 15]] describes: it requires
GitHub repository-administration access, held by the owner, and it is
the sort of change task 12's closure showed an agent can partly perform
through the API while confidently mis-describing the parts it cannot
see.

## Acceptance criteria

- The applied settings match task 49's recommendation exactly, or every
  deviation is recorded with its reason.
- The before state is captured before anything changes, so the change
  is reversible and the record shows what was actually true rather than
  what this sprint assumed. The 2026-07-31 snapshot in task 49 is a
  starting point, not the record.
- Each setting is verified after application by reading it back from
  the API, not by trusting the write.
- Whether direct pushes to `main` still work for Henry is verified
  empirically and recorded, because CLAUDE.md's commit policy depends
  on the answer. If the answer is no, CLAUDE.md's commit policy is
  updated in the same state rather than left describing a workflow the
  repository no longer allows.
- Required status check names are confirmed against a real pull request
  that reports them, not against the workflow file's job names, since a
  mismatch blocks every future pull request silently.
- Per CLAUDE.md's first-performance policy, the exact commands or API
  calls used are recorded in this Result as dated provenance. They are
  not turned into a script.
- Any repository-settings change an agent could not make, or could not
  see to verify, is named as such rather than reported as done.
- A rollback path is recorded: what to change back, and how.

## Result
