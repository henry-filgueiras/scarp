---
id: ide_01KZC769HTAF6F7GDDZW4HQGH7
sequence: 43
kind: idea
status: parked
created: 2026-08-06
---

# Promote bug reports from GitHub issues into maintenance items or sprint tasks

## Problem

Scarp has exactly one promotion path from a GitHub issue to a canonical
artifact, and it can only produce ideas. `proposal realize` lists issues
carrying a single hardcoded label — `PROPOSAL_LABEL = "idea"` in
`src/proposal.rs` — and creates a parked idea whose `proposal:` front
matter records where it came from.

A bug report is the other obvious thing an outside contributor files, and
it has nowhere to land. An operator reading one today must hand-create a
maintenance item or a task and hand-copy the issue URL, which is exactly
the unrecorded hand-performed promotion [[ide_01KY7S6GG3NAA35KBJTC6CA1TM|Desire-path ledger: hand-performed operations as promotion evidence]] is about.

The asymmetry is more than missing convenience. An idea is never
load-bearing: no typed edge may target one, and rejecting it invalidates
nothing. That is what makes realizing an idea a cheap, reversible act an
operator can perform on a stranger's say-so. A promoted bug report is not
that. A maintenance item asserts that work exists; a task inside an
active sprint can be planned around and depended on. The safety argument
that justifies the existing path does not transfer to the path this idea
proposes, and the design has to earn it separately rather than inherit
it.

## Sketch

Generalize the label so the promotion path is many-to-many rather than
one constant to one collection. `idea` continues to realize an idea;
`bug` realizes a maintenance item by default.

`scarp proposal realize <number>` picks its target collection from the
issue's label. `--sprint <reference>` attaches the result to an active
sprint as a task instead of a maintenance item, mirroring the selection
`scarp new task --sprint` already performs — so a bug-fixing sprint, when
one is active, is where a promoted report can land. An issue whose labels
are absent or ambiguous is refused rather than guessed at, consistent
with how the rest of the tool treats underdetermined intent.

Provenance needs nothing new. The `proposal:` front-matter field already
carries the issue URL as one-shot provenance rather than a live link, and
a promoted bug wants precisely that field with precisely those semantics.
The "tag the task with the issue that caused it" requirement is already
solved; this idea only widens which collections can hold the field.

`proposal list` should say which collection each open proposal would
realize into, since with more than one label that is no longer inferable.

Reconciliation is where the analogy genuinely breaks. [[ide_01KYZXGDY8YAFXMP1FV931ZB0M|Reconcile realized idea proposals back to GitHub issues]] closes
a proposal when the artifact it asked for has landed, and for an idea
that is the whole story — the idea *is* the deliverable. For a bug the
filer is waiting on the fix, not on the existence of a tracking item, so
the honest reconciliation point is the terminal transition of the
maintenance item or task, not its creation. Either reconcile learns to
wait for that transition, or bug reconciliation is a separate and later
act with its own trigger.

## Boundaries

Operator-driven, one-shot, and never synchronization. The issue is not
canonical, nothing mirrors state back, and closing or deleting an issue
invalidates nothing — the same terms the idea path already runs on, and
the line that keeps this clear of the standing non-goal of GitHub Issues
synchronization.

Still `gh` shelled out from a machine that already holds the authority to
commit. No HTTP client, no token storage, no unattended run.

No triage automation, no severity or priority model, no auto-labelling,
no assignment. Promotion stays a judgement an operator with write access
makes and Scarp records.

## Evidence

Prior art is the existing proposal cycle — `list`, `realize`, `reconcile`
— and its module's own framing of a GitHub issue as *mutation intent*
rather than canonical state, which is the same authority-boundary
argument as [[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|Reviewable mutation intents across authority boundaries]]. [[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|Scarp-on-GitHub as a detected first-class integration]] and [[ide_01KYX31AG163NY0EQPCTXAQ066|A reusable proposal action for consumer repositories]] describe the
surrounding integration this would extend.

Two questions this idea does not answer, both load-bearing.

**A bug report that turns out not to be a bug has no terminal state.**
Ideas have `adopt` and `reject`, and rejection is a first-class outcome.
Maintenance runs `pending` to `closed`, so a promoted report that is
working-as-intended, unreproducible, or already fixed can only be closed
as though the work were done. Recording a non-bug as completed
maintenance is a false statement in the archaeology, and the current
lifecycle offers no honest alternative. Either maintenance gains a
terminal state for withdrawn work, or triage has to happen before
promotion rather than after.

**Promotion asserts the report is real, and nobody has checked.** An
unverified idea costs nothing; an unverified maintenance item is work the
project now believes it owes. Whether reproduction is a precondition of
promotion, or promotion explicitly means "worth investigating" rather
than "confirmed", changes what a maintenance item means — and that is a
lifecycle question, not a GitHub question.
