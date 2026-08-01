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
request that Henry merges with one tap. Ideas only. No generic mutation
endpoint.

The channel is built to be a *recipe*, not a fixture. Scarp is a tool
other people's repositories will use, and a proposal channel that only
works here would be a demo. The workflow therefore installs a pinned
published `scarp` rather than building the checkout, so this
repository's channel is byte-identical to the one a consumer would
copy — the same binary, the same commands, the same failure modes.

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

**The project currently forbids what this sprint proposes**, in four
places. [[dec-bootstrap-interaction-surfaces|Decision 7]] says in
terms: "No save hooks, no CI commits". CLAUDE.md lists both "automatic
commits" and "GitHub Issues synchronization" as explicit bootstrap
non-goals, and its commit policy says "Never push. Pushing is always a
human decision" — which the workflow's proposal branch violates
literally. Under the project's own change discipline that is an
adoption gate requiring a new recorded decision, which is why
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] blocks all implementation.
If the adjudication goes the other way, the sprint stops there having
produced the research and a recorded refusal.

## Shape fixed by owner direction (2026-07-31)

Four choices were adjudicated before implementation planning, and the
tasks are written against them rather than re-deriving them:

- **The leniency is creation-only and ideas-only.** Automation may
  cause Scarp to create a new artifact, in collections that are never
  load-bearing, and may never modify or delete an existing one. The
  broader creation-only rule was available and was deliberately not
  taken: extending the channel to any further collection costs an
  amendment, on purpose.
- **Henry taps merge.** Auto-merge was declined for now. This keeps
  [[dec-bootstrap-interaction-surfaces|decision 7]]'s "diff the user
  can review" clause satisfied literally rather than argued around, and
  it removes the branch-protection change, the `allow_auto_merge` flip,
  and the second credential the auto-merge path would have needed. One
  tap is not transcription, so the motivating use case survives intact.
- **The workflow runs a pinned published `scarp`.** This makes the
  channel identical to a consumer's and retires the question of which
  binary realized an artifact. It costs a sequencing constraint:
  `--body-file` must ship in a release before the channel can go live,
  which is [[tsk_01KYX31ACH05NGA3GYH0TJA870|task 56]].
- **Authorization is a live repository-permission check**, not a login
  allowlist. Today that resolves to Henry alone; in a consumer's
  repository it works unchanged for a team, and it cannot go stale the
  way a hardcoded list does.

Observed against the live repository on 2026-07-31, and preserved
because the one-tap choice depends on it: `allow_auto_merge` is
`false`, `main` has no branch protection, and there are no rulesets.
The one-tap model needs none of that to change.

## Review amendments (2026-08-01)

Sprint review before implementation raised two findings, incorporated
into the tasks rather than left as commentary. Neither reopens the
adjudications above.

**Validation semantics were imprecise.** The sprint said the pull
request would carry a green check run, while the design creates that
pull request with `GITHUB_TOKEN`. Running validation and *showing* a
check are different claims, and only the first was ever guaranteed. The
invariant is now stated as ordering — nothing is published until the
exact resulting state has passed validation — and the wording about
what a reader sees must match observed behaviour.

*Corrected 2026-08-01 by [[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|task 48]].*
This amendment was written expecting the pull request to show no check
at all. That is wrong: `pull_request` with `opened` is a documented
exception to the `GITHUB_TOKEN` suppression rule, so `ci.yml` **is**
triggered — but the run is created in an approval-required state. The
correct description of what a reader sees is a workflow run awaiting
approval, not a green check and not nothing. Nothing blocks the merge,
since `main` has no required checks, and the ordering invariant above
is unaffected because inline validation still runs before publication.
The Check Runs API is consequently **not** built: task 48 found the
reason for considering it had evaporated.

**Replay was unaccounted for.** Serialization protects two different
proposals from racing on sequence allocation, but says nothing about
the same proposal arriving twice — through a re-run, a repeated event,
or a transient failure after partial side effects. One issue must
realize at most one idea, which needs a durable proposal identity, a
deterministic branch name, a machine-readable realization receipt, and
documented behaviour for each recoverable partial state.

Three clarifications of the existing trust boundary came with them, and
are treated as sharpening rather than new scope: authorization is
re-checked immediately before publication rather than trusted from
trigger time; the creation-only, ideas-only grant is proved from actual
repository state rather than inferred from the command invoked, as
defence against this project's own bugs as much as hostile input; and
proposal prose is snapshotted at an explicit realization event so later
issue edits cannot silently diverge from what was realized.

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
- No proposal branch or pull request is published until the exact
  resulting repository state has passed the required validation.
  Canonical state then lands through that pull request, merged by a
  human. No path in this sprint pushes to `main`, and no path merges
  without a person.
- What the pull request shows a reader is described accurately. If
  GitHub exposes no check run on a workflow-created pull request's head
  SHA, the realization run is durably linked from the pull request and
  the repository's prose says that, rather than claiming a check that
  does not exist.
- One proposal issue realizes at most one canonical idea, unless a
  human explicitly performs a recovery operation. Duplicate deliveries,
  re-runs, and partial failures do not produce a second artifact.
- The proposal remains linked to the artifact it produced by a durable,
  machine-readable receipt, so the provenance of a remotely-created
  idea — and the state of a half-finished one — is recoverable later.
- The channel is reproducible in a repository that is not this one: the
  workflow runs a published `scarp`, and a consumer can copy the form,
  the workflow, and the permissions block without editing anything but
  their own repository name.
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
- Auto-merge, branch protection, and rulesets. Declined by owner
  direction above, priced by
  [[tsk_01KYX1WHRPEXG8Z8EBPQJRHHFH|task 49]] so a later sprint can
  reconsider with real usage as evidence, and not built here.
- Shipping a reusable GitHub Action, a published composite action, or
  prebuilt binaries. The consumer story this sprint owes is a working
  recipe someone can copy; packaging it is
  [[ide_01KYX31AG163NY0EQPCTXAQ066|idea 36]], and the install cost that
  makes packaging attractive is
  [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]]. Both stay parked.
- The standing bootstrap non-goals: daemon, watcher, index, embeddings,
  semantic search, GraphQL, TUI.
