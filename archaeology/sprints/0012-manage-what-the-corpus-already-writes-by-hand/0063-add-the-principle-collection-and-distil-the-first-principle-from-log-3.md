---
id: tsk_01KZ738BJ5MXNBDWECX8REA391
sequence: 63
kind: task
status: closed
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
closed: 2026-08-04
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

## Result

Delivered, with one adjudicated addition: sugar binding now applies to
`new --body-file` as well as `close --body-file`.

**The invariant, stated once.** Wherever Scarp accepts authored prose
that becomes canonical through a Scarp write, resolvable sugar in that
prose is bound at the write boundary. There was never a semantic reason
for a closure to bind `[[log:3]]` while a creation preserved it; that
distinction would only have recorded which command received the binder
first. The binder moved from `transition` to `edges`, where the marker
grammar, the claimant catalog, and addressability already live, and
`resolve_edge` now calls into it rather than owning it — one binder,
three callers, no parallel machinery. Creation binds before a sequence
is allocated or a path is touched, so unresolvable sugar refuses without
leaving an artifact, and a test asserts the directory is still empty
afterwards.

**The collection.** `archaeology/principles/`, flat, one admitted state
`active`, no transitions, template exactly [[ide_01KYDZVN858BK52A35KJ3ZY5BP|Principles: durable heuristics that shape decisions]]'s five fields in
its order. Added through the collection-data pattern the last two tasks
established — a `Collection` static, a directory constant, a section
list, an id prefix, a `create_*`, four dispatch arms, one `doctor` array
entry — and deliberately without extracting anything, so
[[tsk_01KZ738BQTR4H7Z7YBKPPCXGHT|Measure the collection-definition duplication three collections cost]] measures what is really there rather than what was tidied
ahead of it.

Single-state rather than stateless is load-bearing for the deferred
retirement question: the `status:` line already exists, so admitting a
terminal state later is a transitions-table change with no corpus
migration, where a stateless collection would need one.

**Principle 1** carries the verification blind-spot heuristic and its
companion, and cites [[log_01KYK8RC0YEY51YP37RGV7M7N4|Verification blind spots found while preparing the first release]] — which is unchanged, and remains the
canonical account of how the shape was found, with its four instances
and its admission that three of them were found after the work they
invalidated had been recorded as verified.

### Dogfood

Principle 1's citation was authored as `[[log:3]]` and bound on the way
in, against the real corpus, to log 3's ULID and its full title. No
stable id was transcribed by hand anywhere in this task except in the
dated section appended to [[ide_01KYDZVN858BK52A35KJ3ZY5BP|Principles: durable heuristics that shape decisions]], which is a hand-append to an
existing file and therefore outside every write boundary Scarp owns —
the residue this invariant does not reach, and the reason
[[idea-doctor-reference-graph|Doctor checks over the derived reference graph]]'s label check stays worth having.

This Result was itself supplied through `close --body-file`, so the
markers above were sugar when written.

### What was not built, and why

`applies` and `overrides` are deferred with the cost now measured rather
than estimated. Declaring an edge kind is two `EDGE_KINDS` entries;
*authoring* one is the expensive half, because the only authoring
surface is `resolve_edge`, reached exclusively from a lifecycle verb's
flag, and decisions have no lifecycle verb. No decision written in this
sprint cites a principle, so there is no specimen, and inventing a
decision mutation surface to complete a two-week-old sketch would be the
sketch driving the design. Recorded in idea 28 rather than left implicit.

Retirement stays deferred per the 2026-08-04 adjudication above. The
sketch's sixth field, "related decisions as they accumulate", is those
edges under another name and waits on them.

### One finding worth carrying forward

Writing the first principle immediately wanted a sixth section. Its
provenance — which log it came from, and that the log is not superseded
— is neither rationale nor a failure signal, and it was folded into
`Rationale`, where it reads acceptably rather than naturally. That is
[[ide_01KYZY233Z7GAKFPFSKEAF89ZD|Author-owned sections in managed templates]]'s tension appearing on the template's first day of use, in a
template designed from a worked example, which is about as favourable a
case as templates get. It is recorded rather than fixed: adding a
section on one specimen's evidence would be the same premature guess the
lifecycle deferral exists to avoid.
