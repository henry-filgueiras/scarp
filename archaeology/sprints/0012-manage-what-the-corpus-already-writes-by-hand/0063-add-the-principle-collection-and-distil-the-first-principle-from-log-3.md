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

## Adjudicated: principles ship single-state (2026-08-04)

Owner direction, before execution. This supersedes the `active -> retired`
lifecycle in the Objective and the first acceptance criterion above; the
original text stands as the reasoning that was overruled.

The principle collection ships with **one admitted state and no
transitions**, exactly as `decision` does. The argument that a principle
makes a live claim and must therefore be retractable is sound about
principles and unsound about *now*: it settles lifecycle semantics on
zero specimens. There is evidence for durable principles — four
instances in one sprint — and none whatsoever for principle retirement.

Retirement is **deferred, not rejected**. The specific risk in choosing
now is that a future specimen may not be "retired" at all. A principle
that stops applying because the world changed, one that is replaced by a
sharper statement of the same force, and one that turns out to have been
wrong are three different events, and the last two look much more like
[[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|idea 26]]'s supersession-versus-erratum
distinction than like a lifecycle transition. Picking `retired` today
would name the first and quietly foreclose the other two — the same
mistake decision 11's original universal placement claim made, and the
same one its amendment had to undo.

The revised criterion: principles are created in a single admitted
state, `active`, with an empty transition table; no `retire` verb, no
new `Status` variant. `Collection::transitions` being empty already
makes every lifecycle verb refuse a principle with truthful guidance,
which is the decision collection's existing behaviour and needs no new
code. The first principle worth retiring is the evidence that reopens
this.
