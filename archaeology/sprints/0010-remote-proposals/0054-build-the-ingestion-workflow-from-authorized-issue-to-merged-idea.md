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
    checkout
        ↓
    install pinned published scarp
        ↓
    scarp new idea --body-input
        ↓
    scarp doctor + scripts/check.sh
        ↓
    branch + commit
        ↓
    pull request
        ↓
    Henry taps merge

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

**The checks run inline.** A pull request opened with `GITHUB_TOKEN` is
expected not to trigger `ci.yml` at all, so the pull request would
otherwise carry no checks and a human would be tapping merge on an
unverified diff. The workflow therefore runs the repository's own
checks itself before opening the pull request, and the pull request
body carries their result. Confirm against
[[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|task 48]]'s finding whether `ci.yml`
subsequently runs on the `push` to `main` after the human merge; if it
does, post-merge coverage is intact and should be stated, and if it
does not, that gap is recorded rather than assumed away.

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
- The pull request carries the result of the repository's own checks,
  run inline before it opened, and whether `ci.yml` additionally runs
  post-merge is recorded as fact rather than assumed.
- Two proposals filed simultaneously do not collide on display
  sequence, verified by firing them concurrently rather than by
  inspecting the workflow.
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
- The diff of a conforming proposal pull request is exactly one added
  file under `archaeology/ideas/`. This is the checkable form of
  [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s creation-only grant, and
  the workflow enforces it rather than trusting it: a proposal whose
  diff modifies or deletes anything aborts before opening a pull
  request. Verified by inducing one.
- The workflow contains nothing specific to this repository beyond its
  own name — no owner login, no path assumption, no dependence on the
  Scarp source being present. Confirmed by reading it as though copying
  it elsewhere, and any residual coupling is named in the Result for
  [[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]]'s recipe to address.
- `scripts/check.sh` passes.

## Result
