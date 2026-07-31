---
id: tsk_01KYX1WHTGXMBCBA7NE27RM9CF
sequence: 50
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
---

# Adjudicate whether CI may author canonical commits

## Objective

Settle, as a recorded decision, whether automation may author commits
containing canonical archaeology — and if so, under exactly what
constraints. This task is the sprint's gate: no implementation task
starts until it closes.

The project as written forbids the sprint's central mechanic.
[[dec-bootstrap-interaction-surfaces|Decision 7]] states that "Tooling
never rewrites prose outside an explicit, user-initiated operation
whose diff the user can review. No save hooks, no CI commits (automatic
commits are already a recorded non-goal), no batch rewrites hidden
inside unrelated commands." CLAUDE.md's explicit non-goals list
"automatic commits", and its change discipline requires a new recorded
decision plus evidence before any of them is implemented. Building the
workflow first and recording the decision afterwards would be exactly
the ceremony this repository exists to prevent.

## The question, sharpened

Decision 7's prohibition has a rationale — hidden rewrites the user
never sees — and this sprint's mechanic may honour that rationale while
violating the literal words. The adjudication should engage the
distinction rather than assume it:

- The issue *is* an explicit, user-initiated operation. Does a proposal
  filed deliberately by an authorized human satisfy "user-initiated",
  or does decision 7 mean initiated from the working tree?
- The pull request *is* a reviewable diff. Does that satisfy "whose
  diff the user can review" when auto-merge means nobody is required to
  look at it before it lands? This is the sharpest edge in the sprint:
  auto-merge is precisely the step that converts a reviewable diff into
  an unreviewed one, and it is worth asking whether reviewability
  after the fact — a merged pull request, permanently inspectable and
  revertible — is the same guarantee or a weaker one.
- The commit contains only a new file that Scarp generated from
  declared input. Is "authoring a new artifact" the same act decision 7
  banned, or is the ban about *rewriting existing prose*? Note that
  nothing in this sprint modifies an existing artifact.
- [[idea-single-invocation-commits|Idea 9]] already proposes an opt-in
  `--commit` flag and names the automatic-commits non-goal as its own
  adoption gate. Whatever is decided here should be consistent with
  what idea 9 would need, so the two do not settle the same question
  differently.

## Scope

Adjudication and, if the answer is yes, one decision artifact created
with `scarp new decision`. The decision must also fix the vocabulary
the rest of the sprint and
[[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]] use: **proposal authority**
(who may express mutation intent) versus **mutation authority** (what
may realize canonical state), and why they are separate.

This is Henry's call. An agent may research, frame the options, and
draft, but the choice and its reasoning are owner judgment — the same
boundary [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] drew around
publication.

## Acceptance criteria

- The decision is adjudicated by Henry, not inferred from the sprint
  existing. A recorded refusal is a legitimate outcome; if it is the
  outcome, the sprint's implementation tasks are transitioned rather
  than left pending, and the sprint closes with the research kept.
- If permitted, a decision artifact records: the exact scope of the
  permission, its constraints, and what remains forbidden. At minimum
  it should address whether the permission is limited to creating new
  artifacts rather than modifying existing ones, limited to
  non-load-bearing collections, conditioned on a passing `scarp doctor`
  and on landing through a pull request, and whether auto-merge
  specifically is inside or outside the grant.
- The decision states its relationship to
  [[dec-bootstrap-interaction-surfaces|decision 7]] explicitly —
  narrowing it, amending it, or carving an exception — and does not
  leave two artifacts giving contradictory guidance. Decision 7 is not
  silently rewritten; supersession or amendment is stated in the new
  decision, as
  [[dec-canonical-representation|the project's amendment practice]]
  requires.
- CLAUDE.md's non-goal list is reconciled with whatever is decided. If
  "automatic commits" is now qualified rather than absolute, CLAUDE.md
  says so and points at the decision.
- The proposal-authority versus mutation-authority distinction is
  defined in terms a later reader can apply to a collection this sprint
  never touched.
- The evidence CLAUDE.md's change discipline demands is cited: the
  concrete recurring need, not the attractiveness of the design.
- Consistency with [[idea-single-invocation-commits|idea 9]] is stated,
  and idea 9 is left parked unless the decision genuinely adopts it.
- No workflow file, issue form, or repository setting changes here.

## Result
