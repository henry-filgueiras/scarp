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
    build the pinned Scarp
        ↓
    scarp new idea --body-input
        ↓
    scarp doctor
        ↓
    branch + commit
        ↓
    pull request
        ↓
    checks
        ↓
    merge

**Which Scarp runs is a question this repository makes strange.** The
tool that mutates the archaeology is built from the repository being
mutated, so "pinned" has to mean something specific. Decide and record
whether the workflow builds from the checked-out commit, from a tag, or
installs a published version, and what happens the day a proposal
arrives while `main` is mid-change. A proposal must not be realized by
a Scarp whose behaviour nobody has verified.

**`scarp doctor` is a gate, not a report.** It runs after creation and
before the pull request opens, and a non-green result aborts without
opening one. A repository that was already unhealthy before the
proposal arrived must fail loudly and distinguishably from a proposal
that broke something, so the filer is not blamed for a pre-existing
fault.

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
- No path in the workflow pushes to `main`. The mutation lands through
  a pull request that ran the repository's normal checks; the run
  showing those checks reporting on the proposal pull request is
  recorded, since a pull request opened by the wrong identity may
  trigger nothing at all.
- Whichever merge behaviour [[tsk_01KYX1WHRPEXG8Z8EBPQJRHHFH|task 49]]
  recommended — auto-merge or the fallback — is implemented and
  demonstrated end to end.
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
- `scripts/check.sh` passes.

## Result
