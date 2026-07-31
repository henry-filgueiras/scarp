---
id: tsk_01KYX1WJ3P25528P5YTXJAJA4P
sequence: 55
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
---

# Document the proposal and mutation authority boundary

## Objective

Write the durable explanation of what this channel is and what it
deliberately is not, for a reader who arrives with none of this
sprint's conversation — a future contributor, an agent reading the
repository to form an operational model, or Henry in a year.

The audience is not the person filing a proposal. Their instructions
belong in the Issue Form itself. This is the architectural account:
where the trust boundary sits, why canonical state is repository-owned,
and what an extension would have to satisfy.

## What it must convey

**The separation.** A GitHub issue carries mutation intent; Scarp
realizes canonical state. An issue is a proposal, not an artifact, and
it is never itself canonical — nothing in the repository depends on the
issue continuing to exist. Use the proposal-authority versus
mutation-authority vocabulary
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s decision fixed, rather
than a fresh set of terms.

**Why the workflow does not write Markdown.** The reason Actions
invokes the binary instead of generating a file is the whole design,
and it is the single most likely thing for a future contributor to
"simplify" — writing the file directly is obviously easier and looks
harmless. Say plainly what breaks: numbering, stable identity, slug
and path selection, template shape, and the guarantee that a
remotely-created artifact is indistinguishable from a local one. Two
authors of canonical form is the failure mode.

**Where the trust boundary actually is.** Be concrete about what is
trusted and what is not: the issue author is authenticated by GitHub
but not trusted; the issue content is untrusted; the workflow
definition on the default branch is trusted; the Scarp binary is
trusted; `main` is authoritative. Note explicitly that no
conversational agent holds write access anywhere in this design — the
agent drafts prose, a human files it, GitHub authorizes it, Scarp
realizes it.

**Why ideas went first.** Ideas are never load-bearing: no typed edge
may target one, and a bad idea landing costs a `reject` transition
rather than an invariant. That is a property of the collection, not a
convenience, and any proposal to extend the channel has to re-argue it
for the new collection.

**Extension points, named and unbuilt.** Dragons, evidence, decisions,
typed edges, task closure, capability manifests, external proposal
APIs, and MCP. For each, one line on what it would have to settle
first. This section's job is to be honest about what is deliberately
absent, not to design any of it. Where the ceiling of this approach
sits — and where [[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|idea 22]]'s
transport-neutral envelope would take over — should be stated, since
this sprint is evidence for or against that idea rather than an
implementation of it.

## Acceptance criteria

- The document exists at a location a reader finds without being told —
  the choice between `README.md`, `CONTRIBUTING.md`, a `docs/` page, or
  a comment header in the workflow is made deliberately and recorded,
  and whatever surfaces already describe contributing are updated to
  point at it.
- It reads correctly without this sprint's conversation, without the
  task files, and without the decision open in another tab, while
  citing the decision as the authority.
- The "why not just write the file" question is answered explicitly and
  early, in terms someone tempted to do it would find persuasive.
- The trust boundary is stated as an enumeration of what is and is not
  trusted, not as prose that gestures at security.
- Extension points are listed with their unresolved question each, and
  none is designed.
- Nothing in the document is aspirational. Every capability it
  describes exists and was demonstrated in
  [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]]; anything that was
  attempted and abandoned is described as absent rather than omitted.
- It survives the raw-diff test
  [[dec-bootstrap-interaction-surfaces|decision 7]] applies to
  everything else here: it reads well as source, not only rendered.
- CLAUDE.md is updated if this channel changes how a contributor or
  agent should work — in particular if a remotely-filed proposal is now
  a thing a session may encounter mid-sprint.
- `scripts/check.sh` passes.

## Result
