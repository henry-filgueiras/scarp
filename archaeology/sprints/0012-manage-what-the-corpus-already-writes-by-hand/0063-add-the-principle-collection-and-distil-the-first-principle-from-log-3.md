---
id: tsk_01KZ738BJ5MXNBDWECX8REA391
sequence: 63
kind: task
status: pending
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
---

# Add the principle collection and distil the first principle from log 3

## Objective

Adopt [[ide_01KYDZVN858BK52A35KJ3ZY5BP|idea 28]] and give durable
engineering heuristics first-class identity, then distil the first one
out of [[log_01KYK8RC0YEY51YP37RGV7M7N4|log 3]] without disturbing it.

Idea 28's own gate was "no collection work is justified until the corpus
demonstrates need". It is met twice over: the heuristic recurred four
times in a single sprint, and it was drafted in exactly the schema the
idea proposed — statement, rationale, application ordering,
counterpressure, failure signals — with every field carrying something
non-obvious, counterpressure most of all. Log 3 names the gap that
remains after writing it as a log: a log cannot be cited by a decision
as `applies` or `overrides`.

The log stays. It is the dated account of how the shape was found, with
its four instances and its honest closing admission that three of the
four were found after the work they invalidated had been recorded as
verified. The principle is the reusable claim extracted from it and
cites it as provenance. Nothing is deleted, replaced, or relocated.

`active -> retired` rather than a single permanent state: a decision
records a choice already made and is therefore permanent, while a
principle makes a live claim about what to do next, so it must be
possible to stop making it — and the alternatives, deleting or
rewriting, are both barred by preserve-history.

## Acceptance criteria

- A `principle` collection at `archaeology/principles/`, flat, created
  `active`, transitioning `active -> retired` and no further; terminal
  states are permanent, as ideas already are.
- The template is idea 28's, in order: `Statement`, `Rationale`,
  `Application ordering`, `Counterpressure`, `Failure signals`.
- `new`, `list`, `show`, `doctor`, and the retire transition all work,
  and `--body-file` behaves as it does for every other templated
  collection.
- `doctor` validates a principle exactly as it validates any artifact —
  identity, sequence, filename agreement, front matter, title — and
  makes no judgement whatsoever about whether any artifact conforms to
  any principle.
- Principle 1 states the verification blind-spot heuristic and its
  companion (*a passing check is not evidence that its documented
  mechanism ran*), filling all five sections, and cites log 3 with a
  bound marker as the account of how it was found.
- Log 3's own text is unchanged. If it gains a forward pointer to the
  principle, it is a dated appended section and nothing above it moves;
  the invocation used to append it is recorded as desire-path evidence
  in [[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|idea 26]].
- The cost of `applies` and `overrides` typed edges is measured and
  recorded: how many `EDGE_KINDS` entries, and what authoring surface
  would be needed given that decisions have no lifecycle verb to hang a
  flag on. They are implemented only if a decision written during this
  sprint genuinely cites a principle; absent a real instance they are
  not added, and the measurement is written into the idea.
- [[ide_01KYDZVN858BK52A35KJ3ZY5BP|Idea 28]] transitions to `adopted`
  with an `adopted-by` edge naming this task, and records what of its
  sketch was not built.
- `scripts/check.sh` passes.
