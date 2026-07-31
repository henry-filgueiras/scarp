---
id: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
sequence: 10
kind: sprint
status: active
created: 2026-07-31
---

# Remote proposals

## Goal

Let an authorized person, away from the development machine, cause a
durable idea to appear in this repository — without hand transcription,
and without any conversational agent holding write access.

The vertical slice is deliberately one collection wide: a structured
GitHub issue is realized as a canonical idea artifact by Scarp itself,
validated by `scarp doctor`, and landed through a branch and a pull
request. Ideas only. No generic mutation endpoint.

## Rationale

The motivating instance is mundane and recurring: an idea arrives while
Henry is on a phone, talking to an AI. Today the only durable path runs
through a later session that re-reads a chat transcript and re-derives
the artifact by hand — exactly the hand-performed operation
[[ide_01KY7S6GG3NAA35KBJTC6CA1TM|idea 23]] argues is the strongest
promotion evidence Scarp produces. Ideas are the right first collection
precisely because they are never load-bearing: no typed edge may target
one, and a bad idea landing costs a `reject` transition, not an
invariant.

The architecture exists to keep one boundary sharp. A GitHub issue
carries **mutation intent**; it is a proposal, not the artifact. Scarp
alone realizes canonical state — sequence allocation, stable identity,
slug, path, template, front matter. Nothing outside Scarp manufactures
a canonical artifact, which is why the Actions job must invoke the
binary rather than write Markdown. That separation is the point of the
sprint, not an implementation detail of it.

This sprint does **not** adopt
[[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|idea 22]]. Idea 22 proposes a
transport-neutral, repository-local mutation-intent envelope and gates
itself on repeated constrained-agent handoffs. This is a narrower,
GitHub-native channel for one collection, built because a concrete
recurring need exists now. Its results are evidence for or against
idea 22's envelope; idea 22 stays parked and unamended.
[[idea-capability-constrained-work|Idea 15]] supplies the framing for
why this is a real gap: the constraint is the invoking session's
affordances — a phone with no checkout — not who is asking.

Two obstacles were identified before the sprint opened, and the task
order exists to confront them rather than discover them halfway:

- **The project currently forbids what this sprint proposes.**
  [[dec-bootstrap-interaction-surfaces|Decision 7]] says in terms: "No
  save hooks, no CI commits", and CLAUDE.md lists automatic commits as
  an explicit bootstrap non-goal. Under the project's own change
  discipline that is an adoption gate requiring a new decision, which
  is why [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] blocks all
  implementation. If the adjudication goes the other way, the sprint
  stops there having produced the research and a recorded refusal.
- **The desired end state is not currently reachable.** Observed
  2026-07-31 against the live repository: `allow_auto_merge` is
  `false`, `main` has no branch protection, and the repository has no
  rulesets. Auto-merge is reported to require a blocking requirement to
  wait on, so success criterion 8 likely cannot exist until `main` is
  governed — a change that also touches how Henry commits today.
  [[tsk_01KYX1WHRPEXG8Z8EBPQJRHHFH|Task 49]] verifies that and prices
  it.

## Success criteria

- An authorized user creates a structured proposal issue and a
  canonical idea artifact appears on `main`, created by Scarp, without
  anyone editing a file by hand.
- The realized artifact is indistinguishable in form from one created
  locally: Scarp assigned its sequence, identity, slug, path, and
  template, and `scarp doctor` is green after the merge.
- Authorization is enforced on the requester, not assumed. An
  unauthorized issue produces a refusal with a useful diagnostic and no
  branch, no commit, and no pull request.
- A malformed payload is refused the same way, and the diagnostic
  reaches the person who filed the proposal.
- Canonical state lands through a branch and a pull request that runs
  the repository's normal checks. No path in this sprint pushes
  directly to `main`.
- The proposal remains linked to the artifact it produced, so the
  provenance of a remotely-created idea is recoverable later.
- The trust boundary — proposal authority versus mutation authority,
  and why canonical state stays repository-owned — is documented for a
  reader who arrives without this sprint's conversation.

## Non-goals

- Any collection but ideas. Dragons, decisions, evidence, typed edges,
  and task closure are named extension points only; adding one to this
  sprint is scope creep with a governance cost.
- A generic mutation endpoint, a capability manifest, or anything that
  accepts an arbitrary Scarp command. The workflow invokes one
  operation with structured arguments; it must never grow a field whose
  value becomes a command, a flag, or a shell fragment.
- Arbitrary shell or remote command execution, internet-facing
  services, and direct write access for conversational agents. The
  agent on the phone drafts prose; a human files it and GitHub
  authorizes it.
- MCP, an external proposal API, and idea 22's envelope.
- A `proposals/` collection, a `proposed` lifecycle status, or a doctor
  finding for pending proposals. Idea 22's boundaries hold: doctor
  remains structural validation, and an issue is not an artifact.
- Retrofitting existing hand-transcribed ideas.
- The standing bootstrap non-goals: daemon, watcher, index, embeddings,
  semantic search, GraphQL, TUI.
