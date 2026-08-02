---
id: tsk_01KYZXTN3AMPNJ482J4Q13ACTW
sequence: 58
kind: task
status: closed
sprint: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
created: 2026-08-01
closed: 2026-08-01
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

## Result

**Declined. Reconciliation stays an operator command.** No GitHub Actions
workflow, no token, nothing unattended. A promotion criterion is recorded
below.

No decision artifact was created, and no policy site is amended:
[[dec-bootstrap-interaction-surfaces|decision 7]], CLAUDE.md's "GitHub
Issues synchronization" non-goal, and the commit-and-push policy all
stand exactly as they were. Under the operator path none of them is
implicated, which is the outcome that costs the project nothing.

### This is not sprint 10's precedent, and must not be read as one

The charter warned against collapsing the two, and the warning holds
against *this* conclusion as much as against the opposite one. So,
plainly: **idea 40 was right about the authority.** Reconciliation writes
nothing in the repository. A workflow doing it would hold `issues: write`
and never `contents: write`, and by the time it ran the canonical
mutation would already have passed through the operator's trusted machine
and landed. That is a materially cheaper grant than the one
[[spr_01KYX1WAD7CC0RHVZY0V7VE4X1|sprint 10]] declined, and if this had
come down to authority the answer would have been yes.

It did not come down to authority. It came down to the automation not
being worth building, which is a different reason and a weaker precedent.
Nothing here says outward-projecting automation is forbidden.

### Task 57's finding, engaged directly

The charter predicted the pivot would be the cost of proving landing:
a post-merge workflow knows for free what the operator's machine must go
and find out, so if that proof were awkward the workflow would be *buying
correctness* rather than convenience.

[[tsk_01KYZXTN1EV8KKTK3Q75B8HSYR|Task 57]] found the opposite, for a
reason the charter missed. **Reconciliation is inherently online** — it
must reach GitHub to comment and close — so the landing proof costs two
API calls on a command already making them. There is no offline path
being sacrificed and no network cost being introduced. The workflow's
advantage over the operator path is therefore approximately two HTTP
requests.

That deletes the correctness argument entirely and leaves convenience,
against a command that takes about a second.

### The evidence that actually decided it

Not the cost. [[tsk_01KYZXTN71EDDR370MD3F00CK9|Task 60]] published the
first real reconciliation comment, and reading it found a defect every
passing test had missed: the `Landed in` row was a bare forty-character
sha, unlinked, in a table otherwise made of links. The assertions all
passed, because `body.contains("abc1234")` cannot notice that a human
would not read it.

**Had this run unattended, that defect would have been published to every
proposal before anyone read one.** The judgment an automated reconciler
removes — *is this good enough to publish under the repository's name* —
is exactly the judgment that caught it, on the very first performance.

That is an argument about young code rather than about automation
forever, which is what makes it a promotion criterion rather than a
prohibition.

### What declining costs

Stated, not waved away: **nothing reminds the operator.** A landed
proposal stays open until someone thinks to run the command, which is the
same class of staleness this sprint exists to end — moved one step, not
eliminated. Today the prompt is `scarp proposal list`, which nobody is
obliged to run.

The cheap answer is a reminder rather than an actor, and it already has a
home: [[ide_01KY7S6GHMQ8ZWNXPX7TX21X7N|idea 24]]'s one-screen
orientation is where "two landed proposals await reconciliation" belongs.
Deliberately not built here, and deliberately not a `doctor` finding —
sprint 10 named that a non-goal, and an unreconciled proposal is not
repository corruption.

### Promotion criterion

Reconsider automated reconciliation when **both** hold. Either alone is
insufficient, and the conjunction is the point: high volume with an
unstable comment is precisely when automation is worst.

1. **Volume.** Five or more reconciliations have been performed, or two
   or more landed proposals are awaiting reconciliation at the same
   moment.
2. **Stability.** `comment_body` in `src/proposal.rs` has gone unchanged
   across the last three consecutive reconciliations — that is, the
   operator's read-before-publish has stopped finding anything.

Both are checkable rather than arguable: the first from closed proposal
issues and `scarp proposal list`, the second from `git log -- src/proposal.rs`.

When it is met, the work starts from this task and from task 60's
observed procedure, and it would need what any unattended writer needs
and the operator path gets for free: a trigger, a token scoped to
`issues: write` alone, replay behaviour, and a decision artifact settling
the three policy sites *before* anything is built.

### The principle stays untested

Idea 40 proposes a claim broader than the feature: *automation that
projects established canonical truth outward can be granted more freely
than automation that changes canon.* It asked to have that proved
narrowly, on this lifecycle, before anything generalizes it.

**It was not proved, because the automation was never granted.** The
sprint found it did not need to run the experiment. That is a good
outcome for the sprint and a null result for the principle, and the two
should not be confused: nothing here is evidence for the claim, and
nothing is evidence against it. It stays in idea 40, unpromoted, and a
later reader should not find this sprint and think the question was
settled.
