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

The project as written forbids the sprint's central mechanic in four
places, and all four must be reconciled or the repository is left
giving contradictory guidance:

1. [[dec-bootstrap-interaction-surfaces|Decision 7]]: "Tooling never
   rewrites prose outside an explicit, user-initiated operation whose
   diff the user can review. No save hooks, no CI commits (automatic
   commits are already a recorded non-goal), no batch rewrites hidden
   inside unrelated commands."
2. CLAUDE.md's non-goal list: "automatic commits".
3. CLAUDE.md's non-goal list: "GitHub Issues synchronization".
4. CLAUDE.md's commit policy: "Never push. Pushing is always a human
   decision." The workflow pushes a proposal branch, so this is a
   literal violation even though nothing reaches `main` without a
   human.

CLAUDE.md's change discipline requires a new recorded decision plus
evidence before any non-goal is implemented. Building the workflow
first and recording the decision afterwards would be exactly the
ceremony this repository exists to prevent.

## The grant, as directed

Henry fixed the shape on 2026-07-31; this task writes it down properly
and reconciles the four sites above. The grant:

> Automation may cause Scarp to **create** a new artifact, in
> collections that are never load-bearing. It may never modify or
> delete an existing artifact.

with the conditions that creation is realized through the Scarp CLI,
that `scarp doctor` is green before a pull request opens, and that
canonical state reaches the default branch only through a pull request
a human merges.

Two things about that wording are deliberate and should survive
drafting. **Creation-only** is the clause that tracks decision 7's
actual rationale: decision 7 prohibits *rewriting prose*, and the ban's
three examples are all rewrites. Authoring a file that did not exist is
not the act it banned. It is also the only clause checkable from the
diff alone — a conforming proposal changes exactly one added file under
a managed collection — so the grant is enforceable rather than trusted.
**Ideas-only** is the deliberately tighter dial: the broader
creation-only rule was available and was not taken, so extending the
channel to a second collection costs an amendment on purpose.

Auto-merge is **outside** the grant. Declining it is what lets decision
7's "diff the user can review" clause be satisfied literally rather
than argued around, and the decision should say so, since that is the
clause a future reader will test any extension against.

## The other three sites

- **CLAUDE.md non-goal "automatic commits"** is qualified, not
  removed, and points at this decision. The list is already conditional
  on "a new recorded decision and evidence"; this is that decision.
- **CLAUDE.md non-goal "GitHub Issues synchronization"** stays, with
  the distinction stated: this channel is one-shot *realization*, not
  synchronization. Nothing mirrors state back, the issue is never
  canonical, and deleting the issue invalidates nothing. Without that
  sentence a future reader will reasonably conclude the project shipped
  something on its own non-goal list.
- **CLAUDE.md "Never push"** is scoped rather than deleted. Its real
  content — nothing reaches `main` without a human — survives intact;
  what changes is that the proposal channel may push its own proposal
  branch. Sessions still never push.

## Scope

Adjudication, one decision artifact created with `scarp new decision`,
and the reconciling edits to CLAUDE.md and decision 7 in the same
state. The decision must also fix the vocabulary the rest of the sprint
and [[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]] use: **proposal
authority** (who may express mutation intent) versus **mutation
authority** (what may realize canonical state), and why they are
separate.

The shape above is owner direction, but the drafted decision still
returns to Henry before the sprint proceeds — the same boundary
[[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] drew around publication. An
agent drafting a decision is not the same as the decision being made.

Consistency check that must not be skipped:
[[idea-single-invocation-commits|Idea 9]] proposes an opt-in `--commit`
flag and names the automatic-commits non-goal as its own adoption gate.
Whatever is written here changes what idea 9 would still need, so the
two must not settle the same question differently.

## Acceptance criteria

- The drafted decision is ratified by Henry, not inferred from the
  sprint existing. A recorded refusal remains a legitimate outcome; if
  it is the outcome, the sprint's implementation tasks are transitioned
  rather than left pending, and the sprint closes with the research
  kept.
- The decision records the grant, its four conditions, and what remains
  forbidden — with modification, deletion, load-bearing collections,
  and auto-merge each named as outside it rather than merely unmentioned.
- The creation-only clause is stated in a form that can be checked
  against a diff, and the decision says so, so a later reader knows the
  grant is enforceable rather than trusted.
- The decision states its relationship to
  [[dec-bootstrap-interaction-surfaces|decision 7]] explicitly —
  narrowing it, not superseding it — and decision 7 gains a pointer so
  neither artifact can be read alone and come away wrong. In-place
  dated amendment is established practice here; decision 11 carries
  `## Amendment: narrowed to lifecycle authority (2026-07-22)` as the
  model.
- All three CLAUDE.md sites are reconciled in the same state: the
  "automatic commits" non-goal qualified and pointing at the decision,
  the "GitHub Issues synchronization" non-goal kept with the
  realization-versus-synchronization distinction stated, and the
  "Never push" policy scoped to preserve its real content.
- The proposal-authority versus mutation-authority distinction is
  defined in terms a later reader can apply to a collection this sprint
  never touched, and to a repository that is not this one.
- The decision is written to be true of any Scarp repository, not only
  this one. A consumer adopting the channel adopts this grant, so a
  clause that only makes sense here is a defect.
- The evidence CLAUDE.md's change discipline demands is cited: the
  concrete recurring need, not the attractiveness of the design.
- Consistency with [[idea-single-invocation-commits|idea 9]] is stated,
  and idea 9 is left parked unless the decision genuinely adopts it.
- No workflow file, issue form, or repository setting changes here.

## Result
