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

Build the smallest good operator-driven realization path: from a trusted
local machine, list the open proposals and turn a chosen one into a
canonical idea, then commit through the ordinary workflow.

*Replaced 2026-08-01 by [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s
Option B adjudication.* This task was a GitHub Actions workflow —
checkout, pinned binary, doctor gate, branch, pull request, receipt,
eight partial-state recoveries, late authorization. None of it survives.
There is no workflow, no token, no branch, and no pull request. What
survives is the useful core plus two of the review findings, which turn
out to matter under any design.

## The product interface

The raw pipeline works today and is the desire path, not the product:

```sh
gh issue view 42 --json body -q .body > /tmp/b.md
scarp new idea "$(gh issue view 42 --json title -q .title)" --body-file /tmp/b.md
```

Nobody should be asked to assemble that. It fetches title and body
separately, leaves a temporary file to clean up, and — worst — records
nothing, so running it twice silently creates a duplicate idea.

Deliver instead a bounded Scarp surface, naming notwithstanding:

```console
$ scarp proposal list
$ scarp proposal realize 42
```

or an equivalently narrow GitHub-aware operation. Two verbs is the
target; a third needs to argue for itself.

## Architectural constraints

Follow [[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|idea 37]]'s direction: **`gh`
plays the role for GitHub that the installed `git` already plays for
Git.** Shell out to an authenticated `gh`. Scarp acquires none of:

- its own GitHub credential storage;
- an HTTP client for this feature;
- GitHub SDK machinery;
- token handling;
- a speculative generic forge abstraction.

**Failure must be clean and local.** If `gh` is absent, unauthenticated,
offline, or the repository has no GitHub remote, this feature is
unavailable with a typed error naming what is missing and what to do.
Ordinary Scarp commands are unaffected in every one of those cases —
`new`, `list`, `show`, `doctor`, and every transition keep working, and
the repository stays fully usable. This is the "Git is optional at the
core" property, one layer out.

**Realization reuses the existing core.** It composes `gh` with the same
creation path `scarp new --body-file` already uses. It must not grow a
parallel canonicalization route, and it must not accept a collection, a
flag, or a path from the proposal.

**Implement only the smallest subset of idea 37** that makes this
usable. Richer detection, scaffolding beyond what the sprint needs, and
generalized forge-aware surfaces grow from observed use.

## The duplicate-realization guard

Realizing the same proposal twice must **refuse clearly**, not silently
create a second idea. This survived from the automated design because it
was never really about automation — an operator can run a command twice
just as easily as a workflow can fire twice.

Proposal identity is durable and derived from transport: **repository
plus issue number.** Not the title, not a label.

Prefer the least new state that is reliable. Candidate evidence, in
rough order of preference:

- provenance recorded in the realized artifact's own prose, which task
  51 already established as where provenance lives;
- commit history;
- a marker or comment on the originating issue;
- some other durable surface that already exists.

**Do not invent a receipt subsystem** unless simpler evidence proves
insufficient. If a scan of existing artifacts answers "has issue 42 been
realized", that is the answer. Record which evidence was chosen and what
it costs — in particular whether it still works when the realized idea
has since been adopted or rejected, and when the operator is on a branch
that does not yet contain the earlier realization.

## Snapshot semantics

Realization consumes an explicit snapshot of the proposal taken at
invocation. Later edits to the issue never mutate an already-created
artifact.

There is no synchronization, in either direction, and the issue is never
canonical. Closing or deleting it invalidates nothing. Under Option B
this is close to free — the operator fetches once and creates once — but
it must be stated and true rather than incidental, because a future
reader will ask what happens when the issue changes.

## Acceptance criteria

- An idea filed as a proposal issue from a phone becomes a canonical
  Scarp artifact through one bounded operator command, with no manual
  transcription and no hand-assembled shell pipeline.
- The realized artifact is compared against one created locally by hand
  from the same payload and matches in form: same front-matter fields,
  same section structure, same line endings, and a Scarp-allocated
  sequence, identity, slug, and path.
- `scarp doctor` is green after realization, and the operator commits
  through the ordinary workflow. Nothing in this task commits or pushes.
- Realizing the same issue twice refuses with a diagnostic naming the
  existing artifact, verified by doing it. The chosen evidence for "has
  this been realized" is recorded with its limitations.
- Snapshot semantics are demonstrated: editing the issue after
  realization leaves the artifact unchanged.
- Hostile proposal content is refused by Scarp's existing task 51
  validation rather than by new code here, verified with at least one
  payload from [[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|task 48]]'s injection
  inventory. Note task 51's finding that a title beginning with `-`
  needs `--`, and that flags must precede the separator.
- The feature fails cleanly and specifically when `gh` is missing,
  unauthenticated, and when the repository has no GitHub remote. Each is
  induced deliberately, and ordinary Scarp commands are confirmed
  unaffected in each case.
- The command surface is bounded: no field of a proposal reaches an
  argument position that could select a different command, collection,
  flag, or path.
- Nothing hardcodes this repository — no owner login, no path
  assumption, no dependence on the Scarp source being present.
- Per CLAUDE.md's first-performance policy, the exact `gh` invocations
  used are recorded in this Result as dated provenance.
- Ideas created through this channel are ordinary parked ideas, subject
  to `adopt` and `reject` like any other. Nothing marks them as
  second-class, and no lifecycle state was added.
- `scripts/check.sh` passes.

## Result
