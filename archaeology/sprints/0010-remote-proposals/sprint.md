---
id: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
sequence: 10
kind: sprint
status: closed
created: 2026-07-31
closed: 2026-08-01
---

# Remote proposals

## Goal

Let an authorized person, away from the development machine, cause a
durable idea to appear in this repository — without hand transcription,
and without any conversational agent holding write access.

The north star, fixed by [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s
adjudication on 2026-08-01:

> Remote capture becomes durable immediately; canonicalization remains
> local, explicit, trusted, and cheap.

The vertical slice is deliberately one collection wide: a structured
GitHub issue makes the proposal durable, and later — from a trusted
development machine — the operator realizes it into a canonical idea
through Scarp and commits normally. Ideas only. No generic mutation
endpoint, no automation holding write authority.

The channel is built to be a *recipe*, not a fixture. Scarp is a tool
other people's repositories will use, and a proposal channel that only
works here would be a demo. Where practical Scarp should scaffold the
GitHub-specific files rather than leaving a consumer to reconstruct them
from prose.

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
slug, path, template, front matter. Nothing outside Scarp manufactures a
canonical artifact. That separation is the point of the sprint, not an
implementation detail of it, and Option B sharpens it rather than
softening it: mutation authority is never delegated to a token at all,
it stays with the operator.

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
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] gates the automation work.

*Resolved 2026-08-01.* **None of the four is amended.** Task 50 chose
Option B, under which no automation commits, nothing pushes, and no
policy site needs qualifying. The sprint proceeds with decision 7 and
the commit policy exactly as they were — which is the strongest
available evidence that the channel did not need the authority it was
originally designed around.

## Adjudicated shape (2026-08-01)

[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|Task 50]] settled the design after
[[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task 51]] shipped `--body-file` and
revealed that the only genuinely missing primitive was already in hand.
The remote side makes a proposal durable; the operator canonicalizes it
later from a trusted machine.

The load-bearing observation is that an **open GitHub issue is already
durable**. Persistence was never the problem — canonicalization was —
and canonicalization benefits from the operator's judgment rather than
suffering from the delay. Everything the sprint had accumulated beyond
that point served the threat model of a workflow holding a write token,
not the use case.

Deferred, not rejected. Automated realization keeps an explicit
promotion criterion recorded in task 50's Result: reconsider it when
operator-driven realization demonstrates that the manual step is
recurring material friction — proposals accumulating, being abandoned,
being meaningfully delayed, or needing burdensome batching. Tasks 48 and
49 are preserved as the evidence any such reconsideration would start
from.

The GitHub-aware surface stays minimal. [[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|Idea
37]] holds the larger direction and stays parked; this sprint implements
only the smallest subset that makes operator-driven realization usable,
and the raw `gh` plus `scarp new` pipeline is treated as the desire path
that would justify more.

## Superseded: shape fixed by owner direction (2026-07-31)

*Retained as history. Every bullet below assumed the automated channel
and was overtaken by the adjudication above.* Kept because the sprint
assumed automation, built the one primitive that was actually missing,
and then found the automation optional — and that sequence is worth more
than a tidy charter.

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

## Superseded: review amendments (2026-08-01, pre-adjudication)

*Retained as history.* These findings were raised against the automated
channel. Under Option B most of their subject matter no longer exists —
there is no pull request, so nothing carries or fails to carry a check,
and there are no distributed side effects to recover. **Two survive in
altered form and are live requirements on
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]]**: realizing the same issue
twice must be refused rather than silently duplicated, and realization
must consume a snapshot so later issue edits cannot mutate an
already-created artifact.

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

- An idea drafted in remote conversation becomes a canonical Scarp
  artifact with **no manual transcription** and no fishing through a
  chat transcript: the proposal is filed structured, and realization
  consumes it directly.
- Filing the proposal makes it durable immediately. Nothing is lost if
  canonicalization happens hours or days later.
- The operator realizes a proposal from a trusted local machine through
  a bounded Scarp operation, and the resulting artifact is
  indistinguishable from one created locally by hand — Scarp assigned
  its sequence, identity, slug, path, and template, and `scarp doctor`
  is green.
- Realizing the same proposal twice refuses clearly rather than silently
  creating a second idea.
- Realization consumes a snapshot of the proposal taken at invocation.
  Later edits to the issue never mutate an already-created artifact, and
  nothing synchronizes in either direction.
- Canonical state reaches `main` only through the operator's ordinary
  commit and push. No automation commits, nothing pushes on its own, and
  no CI holds a write token.
- The GitHub-aware surface fails cleanly when `gh` is absent,
  unauthenticated, or offline, and when the repository has no GitHub
  remote. Ordinary Scarp commands are unaffected in every such case.
- The channel is reproducible in a repository that is not this one, and
  needs no repository settings configured to work.
- The trust boundary — proposal authority versus mutation authority, and
  why canonical state stays repository-owned — is documented for a
  reader who arrives without this sprint's conversation.

## Non-goals

- Any collection but ideas. Dragons, decisions, evidence, typed edges,
  and task closure are named extension points only.
- **Automated realization.** No GitHub Actions workflow that creates
  canonical artifacts, no CI-held write token, no automated commit or
  push. Deferred against a recorded promotion criterion, not rejected in
  principle.
- Amending [[dec-bootstrap-interaction-surfaces|decision 7]] or the
  commit and push policy. Option B needs neither, and a sprint that
  amends them anyway would be spending authority it does not use.
- Auto-merge, branch protection, rulesets, and any repository governance
  change. Nothing in the chosen design opens a pull request.
- A generic mutation endpoint, a capability manifest, or anything that
  accepts an arbitrary Scarp command, flag, or path from a proposal.
- Issue-to-artifact synchronization in either direction. Realization is
  one-shot; the issue is never canonical, and closing or deleting it
  invalidates nothing.
- Arbitrary shell or remote command execution, internet-facing services,
  and direct write access for conversational agents. The agent on the
  phone drafts prose; a human files it and a human realizes it.
- Scarp acquiring its own GitHub credential storage, an HTTP client for
  this feature, a GitHub SDK, or token handling. It shells out to an
  authenticated `gh`, as it would to `git`.
- A forge abstraction with one implementation. GitHub has a consumer;
  GitLab and Forgejo do not.
- The full [[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|idea 37]] surface. Only the
  smallest subset that makes operator-driven realization usable.
- An in-repository `inbox/`, a `proposals/` collection, a `proposed`
  lifecycle status, or a doctor finding for pending proposals. Idea 22's
  boundaries hold, and GitHub Issues already provide the durable
  noncanonical staging surface this use case needs.
- MCP, an external proposal API, and idea 22's envelope.
- Retrofitting existing hand-transcribed ideas.
- Shipping a reusable GitHub Action, a published composite action, or
  prebuilt binaries. Packaging is
  [[ide_01KYX31AG163NY0EQPCTXAQ066|idea 36]]; the install cost that
  makes it attractive is [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]].
  Both stay parked.
- The standing bootstrap non-goals: daemon, watcher, index, embeddings,
  semantic search, GraphQL, TUI.

## Retrospective (2026-08-02)

Shipped in `scarp 0.2.0`. Issue
[#2](https://github.com/henry-filgueiras/scarp/issues/2) — filed from a
phone — is idea 38 in this corpus, created by Scarp, with no manual
transcription anywhere in the path.

### The sprint's own headline

**It planned an automated channel and shipped an operator-driven one.**
The pivot came from building the single genuinely missing primitive
first: once `--body-file` existed, the remaining complexity was visible
as serving a *threat model* — a workflow holding a write token — rather
than the use case. Every mechanism the plan had accumulated (replay
guard, receipt, partial-state recovery, late authorization,
postcondition proof, four amended policy sites) belonged to the token,
and the token turned out to be optional.

The sprint therefore ends with **decision 7 and the commit-and-push
policy unamended**, and with no new decision artifact. A sprint that
sought a permission and finished without needing it is a better outcome
than one that got the permission.

### What the criteria actually got

Met: no-transcription capture; immediate durability; a bounded operator
command producing an artifact indistinguishable from a local one;
refusal on double realization; canonical state reaching `main` only
through an ordinary commit; clean unavailability with every other
command unaffected; no repository setting configured; the trust boundary
documented.

Partially met, stated rather than rounded up:

- **Snapshot semantics are true by construction, not demonstrated.**
  Realization fetches once, never re-reads, and refuses a second run —
  so a later issue edit cannot reach an existing artifact. That is
  sound, but no test edits an issue and re-checks, and "no code path
  exists" is a weaker claim than "we tried it".
- **The consumer proof covered everything except GitHub.** The recipe
  was followed into a genuinely alien repository (Python, no Rust, no
  `scripts/`, no corpus) and worked. The GitHub half was not exercised
  in a second repository because creating one needs `delete_repo`
  scope, which this environment lacks — so it could be created and not
  cleaned up. Narrow, expected to pass, unverified.

### What was learned

**Real payloads find what synthetic ones cannot.** A dozen hand-written
bodies were used to build `--body-file`; not one contained a fenced code
block. Henry's first real proposal did, and it exposed a parser that
would refuse a shell snippet containing `# comment`. Issue 2 escaped it
only because its JSON has no `#` lines — one snippet away from breaking
the first proposal ever filed.

**Managed beats conventional, and the correction came from outside.**
This sprint's own reasoning had declined a front-matter provenance field
because doctor treats unknown keys as inert. That objection applied to
an *unmanaged* key; Henry's push-back for a managed one was right, and
it bought something prose-scanning could not: duplicate realization is
now a doctor finding, catching the merge-time case where two branches
each realized once and neither run could have known.

**Research gates earn their cost even when they change the answer.**
[[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|Task 48]] corrected a premise the
sprint was built on — `GITHUB_TOKEN`-created pull requests *do* trigger
`ci.yml`, in an approval-required state — which had been recorded
confidently in two places.
[[tsk_01KYX1WHRPEXG8Z8EBPQJRHHFH|Task 49]] closed without performing its
research and is explicitly marked *not a finding*, so nobody later reads
it as "auto-merge was investigated and rejected".

### Promotion evidence carried forward

- The `gh`-as-`git` shell-out stance and the `proposal:` field are new
  architecture living only in [[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|idea 37]]
  and task Results. A second producer or a second forge makes them
  decision-worthy rather than something to let accrete.
- Automated realization is deferred against a recorded criterion, with
  tasks 48 and 49's research preserved as its starting point.
- Four ideas parked by this sprint: prebuilt binaries
  ([[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|35]]), a reusable action
  ([[ide_01KYX31AG163NY0EQPCTXAQ066|36]]), the GitHub integration
  ([[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|37]]), and trusted publishing
  ([[ide_01KYZWC2NBHCSPHFSWZZTB25MH|39]]).
- Idea 38 arrived *through the channel this sprint built*, which is the
  first evidence that it works for its purpose rather than merely works.
