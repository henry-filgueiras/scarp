---
id: tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H
sequence: 54
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
---

# Build the ingestion workflow from authorized issue to merged idea

## Objective

Complete the channel: take the validated payload
[[tsk_01KYX1WJ03MD2WRNQBS3KGMXXA|task 53]] produces, have Scarp realize
it as a canonical idea, validate the repository, and land it on `main`
through a branch and a pull request.

Then prove it by actually doing it, from a phone.

## The path

    authorized, validated payload
        ↓
    inspect existing realization state   (replay guard)
        ↓
    snapshot the proposal prose          (immutable input)
        ↓
    checkout + install pinned published scarp
        ↓
    scarp new idea --body-input
        ↓
    scarp doctor + scripts/check.sh
        ↓
    prove the postcondition              (creation-only, ideas-only)
        ↓
    re-check requester permission        (load-bearing, late)
        ↓
    push branch + open PR + write receipt
        ↓
    Henry reviews and merges

The ordering is not incidental. Authorization is checked at trigger
time *and* immediately before publication, because everything between
them takes time during which permission can be revoked. Validation
happens before publication, not after, because of the invariant below.
The replay guard runs first, because the cheapest way to handle a
duplicate delivery is to not start.

**The workflow installs a published `scarp`; it does not build the
checkout.** This repository makes the question strange — the tool that
mutates the archaeology is built from the repository being mutated —
and installing a pinned release settles it: the binary that realizes a
proposal is one that was released, tested, and verified, not whatever
`main` happened to contain when the issue arrived. It also makes this
channel byte-identical to the one a consumer copies, which is the only
way the consumer story gets exercised rather than merely written down.

The version is pinned explicitly, and the pin is the release
[[tsk_01KYX31ACH05NGA3GYH0TJA870|task 56]] cuts. Record what happens
when a proposal arrives while `main` contains an unreleased CLI change:
the honest answer is that the channel keeps using the pinned release
and is unaffected, and that is a feature.

**`scarp doctor` is a gate, not a report.** It runs after creation and
before the pull request opens, and a non-green result aborts without
opening one. A repository that was already unhealthy before the
proposal arrived must fail loudly and distinguishably from a proposal
that broke something, so the filer is not blamed for a pre-existing
fault.

**The checks run inline, and the wording about them must be exact.**
The governing invariant is:

> No proposal branch or pull request is published until the exact
> resulting repository state has passed the required validation.

That is a statement about *ordering*, and it is satisfied by running
`scripts/check.sh` and `scarp doctor` against the realized state before
the branch is pushed. It is **not** a claim that GitHub displays a
check on the pull request, and the two must not be conflated.

A pull request opened with `GITHUB_TOKEN` is expected not to trigger
`ci.yml`, which would leave the pull request showing no check runs on
its head SHA at all. If [[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|task 48]]
confirms that, then this task must **not** describe the pull request as
"carrying a green check". The honest formulation is that the
realization run is *durably linked* from the pull request — a link to
the workflow run, plus its recorded result — and the sprint's language
is amended to match rather than the implementation being bent to
satisfy the original wording.

Publishing a real check run through the Check Runs API against the head
SHA is acceptable **only** if it falls naturally out of work already
here — the workflow already knows the SHA and already holds a token
that may have `checks: write`. It is not worth widening the sprint for,
and inventing machinery merely to make a sentence true is exactly the
wrong trade. If it is not done, say so plainly and record what a reader
of the pull request actually sees.

Separately, establish whether `ci.yml` runs on the `push` to `main`
that the human merge produces. If it does, post-merge coverage is
intact and should be stated as fact; if it does not, that is a real gap
in the repository's coverage and is recorded rather than assumed away.

## Replay and idempotency

Serialization stops two *different* proposals from racing. It does
nothing about the same proposal being delivered twice, and GitHub does
not promise exactly-once invocation. The invariant:

> One GitHub proposal issue realizes at most one canonical Scarp idea,
> unless a human explicitly performs a recovery operation.

**Proposal identity** is durable and derived from the transport:
repository plus issue number. Not the issue title, not the run id, not
a label.

**The branch name is deterministic** — `scarp/proposal-<issue-number>`
is a reasonable starting point if it fits repository conventions —
so a replay collides with its own prior attempt rather than silently
creating a second branch and a second idea.

**The realization receipt is durable and machine-readable**, written
where a later run can read it back: the originating issue, the pull
request, or both. It carries at least the proposal identity, the
realized artifact's stable id and reference, the branch, and the pull
request. A mutable label is not a receipt — it can be removed by
anyone with triage rights, it carries no artifact identity, and it
cannot distinguish "realized" from "realized then reverted". A label
may exist as a human-facing convenience on top of the receipt.

**The workflow inspects realization state before invoking Scarp**, and
the recoverable partial states are each given documented, intended
behaviour rather than being left to whatever happens. At minimum:

- nothing done yet;
- branch created, no commit;
- commit made, branch not pushed;
- branch pushed, no pull request;
- pull request open, no receipt written;
- receipt written, everything complete;
- pull request already merged;
- pull request closed unmerged by a human.

For each, state what a re-delivery does. "Resume" and "refuse and tell
a human" are both legitimate answers; "unknown" is not, and neither is
any answer that can produce a second idea from one issue. Note that the
last two states are deliberate human acts and a re-delivery must not
quietly undo them.

Recovery — genuinely re-realizing a proposal after something went
wrong — is an explicit human operation, not an automatic retry. This
task does not need to build a recovery command; it needs to make the
recovery path *possible* by writing down enough state, and to say what
the human would do.

## Snapshot semantics

Decide explicitly when proposal prose stops being editable input:

- creating or editing an issue does not by itself mutate canon;
- an explicit authorized realization event snapshots title, body, and
  parsed fields;
- later edits to the issue do not regenerate or alter an artifact that
  was already realized;
- a retry consumes the recorded snapshot, not whatever prose the issue
  now contains.

The failure this prevents is specific and quiet: a filer edits their
issue after realization, and the repository and the issue disagree
about what was proposed, with nothing recording which one the artifact
came from. If [[tsk_01KYX1WJ03MD2WRNQBS3KGMXXA|task 53]]'s form and
trigger design already give stronger guarantees than this, document
those instead of restating these.

## Authorization is re-checked late

The trigger-time check establishes who asked. It does not establish who
is still authorized minutes later, after a build, a test run, and a
network round trip. The workflow re-checks the requester's repository
permission immediately before publishing the branch and pull request,
and that late check is load-bearing: it is the one that gates the
mutation.

Both checks fail closed. An API error, a rate limit, an ambiguous
response, or an unresolvable actor refuses the proposal; none of them
proceeds on the assumption that the earlier answer still holds.

## Proving the postcondition

Intending to invoke `scarp new idea` is not evidence that only an idea
was created. The workflow inspects the actual resulting repository
state before publishing and proves:

- exactly one new canonical idea artifact exists;
- no existing managed artifact was modified;
- nothing was deleted;
- no second collection was touched;
- no unrelated tracked file changed, unless an already-adjudicated
  unavoidable generated file exists — and if one does, it is named in
  the decision rather than tolerated silently;
- the new artifact is an idea, and `scarp doctor` passes.

This is defence against this project's own bugs at least as much as
against hostile input. A Scarp defect that wrote two files, or touched
a neighbouring artifact, would otherwise reach `main` with the
creation-only grant intact on paper and violated in fact.

**Concurrent proposals collide.** Two proposals in flight each allocate
a display sequence against whatever `main` they saw, which is
[[drg_01KY169X7W0YXJ5QFV4D1MK4FB|dragon 1]] arriving in production
rather than a hypothetical. The workflow serializes — a `concurrency:`
group — and creates against a freshly fetched `main` rather than a
stale checkout. This must be tested by firing two proposals at once,
not reasoned about.

**Failure surfacing.** Every abort — build failure, Scarp refusal,
doctor finding, pull request creation failure, merge failure — reaches
the originating issue with a diagnostic naming which stage failed. A
proposal that silently produces nothing is the worst outcome available,
because the filer will assume it worked.

**Provenance.** The commit message, branch name, and pull request body
carry the link back to the originating issue, in whatever form
[[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task 51]] settled. The commit message
follows the project's `area: what changed` convention.

## Acceptance criteria

- An authorized, well-formed proposal issue results in a canonical idea
  artifact on `main`, created by Scarp, with no file edited by hand at
  any point.
- The realized artifact is compared against one created locally from
  the same payload and matches in form: same front-matter fields, same
  section structure, same line endings, and a Scarp-allocated sequence,
  identity, slug, and path.
- `scarp doctor` is green on `main` after the merge, and the run's
  own pre-pull-request doctor gate is demonstrated to block: a
  deliberately induced failure aborts before a pull request exists, and
  the abort is recorded.
- A pre-existing unhealthy repository produces a distinguishable
  failure from a proposal-induced one, verified.
- No path in the workflow pushes to `main`, and no path merges. The
  branch is pushed, the pull request is opened, and a human merges it.
- The publish-after-validation invariant holds and is demonstrated: no
  branch or pull request exists for a proposal whose realized state
  failed `scripts/check.sh` or `scarp doctor`.
- What a reader of the pull request actually sees is recorded exactly.
  If GitHub exposes no check run on the head SHA, the Result says so
  and the pull request instead carries a durable link to the
  realization run and its result. No artifact in this sprint claims the
  pull request "carries a green check" unless GitHub genuinely exposes
  one on that SHA.
- Whether `ci.yml` runs on the post-merge `push` to `main` is
  established empirically and recorded, and the resulting post-merge
  coverage is stated accurately — including, if it does not run, that
  this is a gap.
- Two proposals filed simultaneously do not collide on display
  sequence, verified by firing them concurrently rather than by
  inspecting the workflow.
- One issue yields at most one idea, verified by re-delivering the same
  proposal — a workflow re-run and a repeated trigger event at minimum
  — and confirming no second artifact, branch, or pull request results.
- Every recoverable partial state listed above has documented,
  intended behaviour, and at least the two most likely — branch pushed
  without a pull request, and pull request open without a receipt — are
  induced deliberately and their recovery observed rather than reasoned
  about.
- The realization receipt is machine-readable, durable, and sufficient
  to recover proposal identity, artifact stable id and reference,
  branch, and pull request. Its format is recorded. Removing a label
  does not destroy it.
- Authorization is re-checked immediately before publication, and that
  late check is demonstrated to block: permission revoked mid-run
  results in no branch and no pull request. Both checks fail closed on
  API error or ambiguity, verified by simulating a failed permission
  lookup.
- The snapshot semantics are implemented and demonstrated: editing the
  issue after realization does not alter the realized artifact, and a
  retry consumes the snapshot rather than the newer prose.
- Every abort stage is verified to reach the issue with a stage-naming
  diagnostic. At minimum, Scarp refusal and doctor failure are induced
  deliberately.
- The end-to-end performance is executed for real, from a phone,
  against the motivating use case: an idea drafted in conversation,
  filed as a proposal, landing without transcription. What was clumsy
  about it is recorded honestly — this is the sprint's own evidence
  about whether the channel is worth having.
- Per CLAUDE.md's first-performance policy, any non-obvious `gh` or API
  invocation used is recorded in this Result as dated provenance.
- Proposal branches do not accumulate: the chosen cleanup behaviour is
  implemented and confirmed on a real merge.
- Ideas created through this channel are ordinary parked ideas, subject
  to `adopt` and `reject` like any other. Nothing marks them as
  second-class, and no lifecycle state was added.
- The workflow invokes exactly one Scarp operation with fixed
  arguments. No field of the payload reaches an argument position that
  could select a different command, collection, flag, or path.
- The postcondition is proved from actual repository state, not
  inferred from the command that was invoked: exactly one new idea
  artifact, no modification, no deletion, no second collection, no
  unrelated tracked-file change, and a green `scarp doctor`. This is
  the checkable form of
  [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s creation-only,
  ideas-only grant, and the workflow enforces it rather than trusting
  it: a run whose resulting state violates any clause aborts before a
  pull request exists. Verified by inducing a violation — including one
  that simulates a Scarp defect rather than hostile input, since a bug
  is the likelier cause.
- The workflow contains nothing specific to this repository beyond its
  own name — no owner login, no path assumption, no dependence on the
  Scarp source being present. Confirmed by reading it as though copying
  it elsewhere, and any residual coupling is named in the Result for
  [[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]]'s recipe to address.
- `scripts/check.sh` passes.

## Result
