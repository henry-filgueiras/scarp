---
id: ide_01KY7S6GG3NAA35KBJTC6CA1TM
sequence: 23
kind: idea
status: parked
created: 2026-07-23
---

# Desire-path ledger: hand-performed operations as promotion evidence

## Problem

The strongest roadmap evidence Strata produces is the operation someone
had to perform by hand. The project already reasons this way, but only
retrospectively and only when a retrospective author happens to
remember: sprint 4's retrospective named sprint and task closure as the
last recurring hand-performed archaeology "for the third consecutive
sprint", which pitched sprint 5; sprint 6's retrospective named the
repeated manual front-matter-plus-`git mv` thread mechanics as
promotion evidence for [[idea-comment-threads|comment threads]]. When
the observation is not made at retrospective time, it is recoverable
only by re-reading diffs. Hand-performances that happen mid-task —
minting a decision by hand, appending a dated amendment section,
writing a log entry — leave no aggregated trace at all.

## Sketch

A convention before any machinery: when a session performs by hand an
operation Strata should conceptually own, it appends one dated row to a
single ledger artifact naming the operation shape (not the content) —
for example `2026-07-22 amend decision in place with dated section`.
Rows accumulate across sessions; at sprint-pitch time the ledger is
read and recurrence counts become promotion evidence, with the
project's working rule of three as the presumptive threshold for
sprint candidacy.

Relations. The [[idea-chore-artifacts|chore ledger]] shares the
one-row-per-performance grain but tracks recurring *maintenance*; this
ledger tracks *tool gaps*, and a row's ideal fate is that its
operation becomes a command and the rows stop. The CLAUDE.md
first-performance policy records the exact external command as dated
provenance inside the task that ran it; this ledger records only the
gap, aggregated in one place where recurrence is visible.

## Boundaries

- Not telemetry: rows are authored deliberately by the session that
  felt the gap; nothing writes the ledger automatically.
- The ledger is canonical prose or row-oriented data like any other
  artifact; projections may count it but must not replace it.
- Absence of a row proves nothing; the ledger only ever argues *for*
  promotion, never against it.
- Not a doctor concern: an out-of-date ledger is not corruption.

## Evidence

Sprint 4's retrospective (third consecutive sprint of hand-closure)
drove sprint 5's managed sprints and tasks. Sprint 6's retrospective
explicitly used "promotion evidence" for [[idea-comment-threads|ideas
11]] and [[ide_01KY64ZPXVR0XRZBHKERBXXJ0C|20]] — the pattern this idea
makes first-class was already performed twice, by hand, in
retrospectives. Decisions were minted by hand fifteen times before
sprint 7 proposed managing them; no ledger recorded that count — it
was reconstructed by listing a directory.

Proposed by Claude during the sprint 7 pitch, 2026-07-23.

### 2026-07-27, task 46: four rows in one task

[[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|Task 46]] hand-performed four
operations Scarp should conceptually own. They are recorded here as
the ledger would have recorded them — operation shape, not content —
because there is no ledger to write them to, which is the gap:

```text
2026-07-27  append dated erratum to a closed artifact's result
2026-07-27  amend a sprint's execution order after allocation
2026-07-27  edit downstream artifacts to recognise a newly inserted dependency
2026-07-27  snapshot tracked paths into a disposable git repo to satisfy a clean-tree check
```

The third is the one worth noticing. Inserting task 46 between 43 and
44 required hand-editing **three** other artifacts — the sprint's
order, task 44's introductory dependency, task 45's preconditions —
because ordering and dependency live only in prose. Scarp knows a task
belongs to a sprint; it does not know a task precedes another, so
nothing could have found those three sites or verified they were all
updated. Missing one would have left a task pointing at a superseded
predecessor with no structural trace.

The fourth is now a **second** performance: task 43 built the same
disposable-snapshot apparatus for the same reason (`cargo publish`
requires clean VCS state, implementing the task dirties the tree), and
task 46 rebuilt it from scratch the same day. By the project's rule of
three, one more performance makes it a candidate.

This entry is itself the hand-performance the idea describes: written
into an idea's Evidence section because the aggregating artifact does
not exist. Absent this note, the four rows would be recoverable only
by re-reading a diff — exactly the failure the Problem section names.
