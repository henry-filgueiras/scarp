---
id: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
sequence: 11
kind: sprint
status: active
created: 2026-08-01
---

# Proposal reconciliation

## Goal

Close the remote proposal lifecycle. After a realized idea lands on
`main`, the GitHub issue that proposed it stops being stale: it names the
canonical artifact that now supersedes it, and it closes.

No automation acquires authority over canonical state. Reconciliation
only reports a fact the repository already established.

## Rationale

[[ide_01KYZXGDY8YAFXMP1FV931ZB0M|Idea 40]] arrived through the channel
[[spr_01KYX1WAD7CC0RHVZY0V7VE4X1|sprint 10]] built — the second proposal
ever filed, and the first one filed *about* the channel. Its complaint is
the desire path sprint 10 left behind: realize locally, commit, push, and
then a human has to notice the issue is still open, find the artifact,
write the reference by hand, and close it.

**This repository is the live instance, right now.** Issue
[#2](https://github.com/henry-filgueiras/scarp/issues/2) was realized as
[[ide_01KYZVJ6XCK11DP67GVMC3M23C|idea 38]], shipped in `0.2.0`, and is
still open. A reader arriving at the issue list cannot tell it apart from
a proposal nobody has looked at.

### Half of what the idea asks for already shipped

Idea 40 sketches a prose marker, conceptually `GitHub-Proposal:
owner/repo#42`, and warns against inventing "hidden front matter that
doctor ignores". Scarp already has the managed `proposal:` front-matter
field: written by `scarp proposal realize`, parsed into the artifact
summary, surfaced by `scarp proposal list`, and checked by `doctor`,
which reports `duplicate-proposal` when two artifacts claim the same one.
Explicit, machine-readable, non-heuristic linkage is not work this sprint
does. It is the input this sprint starts from.

That is worth recording rather than quietly skipping, because it is a
predictable property of the channel and not a one-off. Idea 40 was
drafted remotely, from a session with no checkout — the constraint
[[idea-capability-constrained-work|idea 15]] names — so it reasons from
what the author remembered rather than from the tree. Proposals filed
this way will routinely re-propose things that already exist, and
realization deliberately does not check. **Reconciling a proposal against
what shipped is part of realizing it**, and this sprint is the first
worked example.

### The unsettled question

*Settled 2026-08-01 by [[tsk_01KYZXTN3AMPNJ482J4Q13ACTW|task 58]]:
**declined**, against a recorded promotion criterion. Reconciliation
stays an operator command; no workflow, no token, nothing unattended,
and none of the policy sites below amended. The reasoning below is
retained as written, including the prediction it got wrong — see
"Adjudicated" after this section.*

Idea 40 prefers a post-merge GitHub Actions workflow that observes
newly-landed ideas, and leaves the door open to a local command "if
evidence shows that is simpler and equally reliable".

Sprint 10's headline lesson argues for finding out rather than choosing
up front: it planned an automated channel, built the one genuinely
missing primitive first, and then saw the remaining machinery was serving
a threat model rather than the use case. This sprint is sequenced the
same way — primitive first, automation adjudicated with it in hand.

But sprint 10 does **not** settle it, and must not be cited as if it
did. Idea 40's central argument is that the authority differs in kind:
by the time reconciliation runs, the canonical mutation has already
passed through the trusted local path and landed. The workflow would hold
`issues: write` and never `contents: write`. "Sprint 10 declined
automation" is a precedent about automation that *writes canon*, and it
does not transfer.

### The landing invariant is the hard part

Idea 40 states it:

> A proposal issue is closed only because canonical `main` contains an
> artifact explicitly claiming realization of that exact proposal.

Read it precisely: **`main`, not the working tree.** Realization creates
an artifact on the operator's machine; landing is a separate later event
that `realize` knows nothing about.

The repository demonstrates the gap as this sprint opens. Issue #2's idea
38 is on `origin/main` and its issue should close. Issue #3's idea 40 was
realized and committed minutes ago and is on no remote at all — closing
it would advertise an artifact that exists on exactly one disk. A
reconciler that reads the working tree gets #3 wrong.

So reconciliation is Git-aware in a way realization is not, and a local
implementation has to *prove* landing rather than assume it. That cost is
the strongest argument the workflow has, and the adjudication must answer
it rather than route around it.

### The policy gate

The local path needs no amendment, exactly as Option B needed none.

The automated path is different, and the difference should be recorded
before it is built, not after. It would be the project's first automation
that acts outward under a write-scoped token. CLAUDE.md lists "GitHub
Issues synchronization" as a bootstrap non-goal, and
[[dec-bootstrap-interaction-surfaces|decision 7]] says "No save hooks, no
CI commits". A one-way terminal projection is arguably neither of those
things — and "arguably" is what a recorded decision exists to settle.

Underneath sits a claim general enough to outlive this feature, which
idea 40 states and asks to be proved narrowly first: *automation that
projects established canonical truth outward can be granted more freely
than automation that changes canon.* If it survives this sprint it is
durable architecture; if it does not, that is worth knowing before
anything else leans on it.

## Adjudicated: operator-driven, declined against a criterion (2026-08-01)

[[tsk_01KYZXTN3AMPNJ482J4Q13ACTW|Task 58]] declined automation. `scarp
proposal reconcile <number>` is the whole feature. **No policy site is
amended** — decision 7, the "GitHub Issues synchronization" non-goal, and
the commit-and-push policy stand as they were, because the operator path
implicates none of them.

**Two of the charter's predictions above were wrong, and are worth more
than a tidy record.**

*The landing proof was supposed to be the hard part.* The section above
calls it "the strongest argument the workflow has", reasoning that a
post-merge workflow knows for free what the operator's machine must go
and find out. [[tsk_01KYZXTN1EV8KKTK3Q75B8HSYR|Task 57]] found the
opposite, for a reason not noticed when this was written:
**reconciliation is inherently online**, since it must reach GitHub to
comment and close. The landing proof therefore costs two API calls on a
command already making them. The workflow's advantage is approximately
two HTTP requests, which deletes the correctness argument and leaves
convenience.

*The deciding evidence came from somewhere else entirely.* Not cost —
judgment. [[tsk_01KYZXTN71EDDR370MD3F00CK9|Task 60]] published the first
real reconciliation comment, and reading it found a defect every passing
test had missed: a bare forty-character sha, unlinked, in a table
otherwise made of links. Had that run unattended, it would have been
published to every proposal before anyone read one. The judgment
automation removes is exactly the judgment that caught it, on the first
performance.

That is an argument about young code, not about automation forever,
which is why task 58 records a **promotion criterion** rather than a
prohibition: reconsider when five reconciliations have been performed (or
two await at once) *and* the comment body has gone unchanged across three
consecutive runs.

**The general claim stays untested.** Idea 40's outward-projection
principle asked to be proved narrowly here first. It was not — the
automation was never granted, so the experiment never ran. That is a null
result, not support, and the principle stays parked in idea 40 rather
than accreting as settled architecture.

One thing this sprint got right and should not lose: **idea 40 was right
about the authority.** A reconciling workflow would hold `issues: write`
and never `contents: write`. Declining is about value, not permission,
and nothing here forbids outward-projecting automation.

## Success criteria

- After a realized idea is on `main`, its originating issue carries one
  comment that names the canonical artifact — display sequence, stable
  id, and repository path — links the commit that landed it, and states
  plainly that the Scarp artifact is now the canonical record. The issue
  is then closed as completed.
- **Nothing closes an issue whose artifact is not on `main`.** The
  repository's own two proposals are the acceptance case: #2 reconciles,
  #3 does not until it lands.
- Linkage comes only from the managed `proposal:` field. No title
  matching, body similarity, timestamp proximity, or sequence adjacency,
  in any code path.
- Idempotency is boring and demonstrated: an already-closed issue is a
  no-op; an existing reconciliation comment is not duplicated; a re-run
  produces no second side effect; missing or malformed provenance does
  nothing and says why; several landed ideas reconcile independently, and
  one failure does not strand the others.
- Reconciliation never creates, edits, transitions, or deletes a
  canonical artifact, and holds no authority to. Whatever runs it cannot
  write repository contents.
- The GitHub-aware surface stays optional in the established sense: `gh`
  absent, unauthenticated, offline, or no GitHub remote is a typed
  unavailability naming what is missing and what to do, and every
  ordinary Scarp command keeps working.
- The automation question is adjudicated on evidence with the primitive
  in hand. Whichever way it lands, the policy sites above are either
  genuinely untouched or amended by a recorded decision — not stretched
  in prose.
- The lifecycle is documented end to end for a reader who arrives without
  this sprint's conversation: propose, realize, land, reconcile, and what
  each step may and may not do.
- Reproducible in a repository that is not this one, needing no
  repository settings configured.

## Non-goals

- Any collection but ideas. Idea 40 asks for this explicitly, and no
  other proposal kind has a consumer.
- Synchronization in either direction. Reconciliation is one-shot and
  terminal: adopting or rejecting the idea later does not reopen,
  reword, or reclose anything, and editing the issue afterwards reaches
  nothing.
- Automated realization. Sprint 10's deferral and its promotion
  criterion stand untouched; nothing here shortens the GitHub → Scarp
  path.
- Any automation authority over repository contents: no commits, no
  pushes, no `contents: write`, no canonical mutation from CI under any
  justification.
- Reopening the auto-merge, branch protection, and ruleset questions.
  Nothing here opens a pull request.
- A generic outward-projection framework, an event bus, a webhook
  endpoint, or a capability manifest. The principle is proved on one
  narrow lifecycle before anything generalizes it.
- A forge abstraction with one implementation, and any GitHub credential
  storage, HTTP client, or SDK inside Scarp. `gh` keeps playing the role
  `git` plays.
- The full [[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|idea 37]] surface and
  [[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|idea 22]]'s envelope. Both stay
  parked.
- Retrofitting anything but the two proposals this repository has
  actually filed.
- The standing bootstrap non-goals: daemon, watcher, index, embeddings,
  semantic search, GraphQL, TUI, MCP.
