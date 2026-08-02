---
id: tsk_01KYZXTN3AMPNJ482J4Q13ACTW
sequence: 58
kind: task
status: pending
sprint: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
created: 2026-08-01
---

# Adjudicate whether reconciliation may run unattended

## Objective

With the primitive in hand, decide whether reconciliation may run
unattended after changes land on `main`, or stays an operator command.

This is an adjudication, in the shape of task 50: it may decline, and
declining is a result. It is deliberately sequenced *after* task 57
because sprint 10's headline lesson is that building the missing
primitive first can make the surrounding machinery visible as optional.

### What is actually being asked

[[ide_01KYZXGDY8YAFXMP1FV931ZB0M|Idea 40]] prefers a post-merge workflow
that observes newly-landed ideas and reconciles them, and leaves the door
open to the local path "if evidence shows that is simpler and equally
reliable". Task 57 supplies exactly that evidence, from the direction
that matters: **how expensive and how reliable it turned out to be to
prove locally that an artifact had landed.** A workflow triggered by a
push to `main` knows the answer for free; the operator's machine has to
go and find out. If task 57 found that cheap, the case for automation is
mostly convenience. If it found it awkward or unreliable, the workflow is
buying correctness, which is a different and much better argument.

Weigh at least:

- what task 57's landing proof costs, and how it fails;
- whether the manual step recurs enough to be friction — two proposals
  exist, which is thin evidence, and saying so is a legitimate outcome;
- whether an unattended reconciler can be made idempotent against
  re-runs, replayed events, and concurrent pushes as cheaply as the
  operator path can;
- what a stale or wrong comment costs, given it is public, terminal, and
  posted under the repository's own identity.

### The precedent question

Sprint 10 declined automation. That must not be cited as if it settled
this. Its deferral was about automation that **writes canon** under a
`contents: write` token; this automation would hold `issues: write` and
would write nothing in the repository. The authority differs in kind, and
the adjudication should say so explicitly whichever way it lands, so a
later reader does not collapse the two.

### The policy sites

Under the operator path, nothing needs amending — the same outcome Option
B reached, and worth stating rather than assuming.

Under the workflow path, three sites are implicated and must be settled
before anything is built, not after:

- CLAUDE.md lists "GitHub Issues synchronization" as a bootstrap
  non-goal. A one-way terminal projection is arguably not
  synchronization; "arguably" is the problem.
- [[dec-bootstrap-interaction-surfaces|Decision 7]] says "No save hooks,
  no CI commits". This commits nothing, and it is still CI acting on its
  own.
- CLAUDE.md's commit policy and "never push" are untouched by either
  path, and the decision should confirm that rather than leave it
  inferred.

If the workflow path is chosen, record a decision that names the general
claim idea 40 makes and scopes it to what this sprint actually proves:
*automation that projects established canonical truth outward can be
granted more freely than automation that changes canon.* Give it the
boundary that makes it falsifiable — what would have to happen for it to
be wrong — rather than stating it as a maxim.

If the operator path is chosen, record the promotion criterion that would
reopen this, as task 50 did for automated realization, and preserve this
task's reasoning as its starting point.

## Acceptance criteria

- A recorded outcome that a reader can act on without this sprint's
  conversation: which path, on what evidence, and what it costs.
- Task 57's landing-proof finding is engaged with directly, not
  summarized.
- The distinction between this authority and sprint 10's is stated
  explicitly, in either direction.
- If automation is adopted: a decision artifact exists before the
  workflow is built, and each of the three policy sites above is either
  amended or shown not to need it.
- If automation is declined: a promotion criterion is recorded, specific
  enough to be met or not met rather than argued about.
- The sprint's charter is amended to match the outcome, in place and
  dated, with the superseded reasoning retained rather than rewritten.
