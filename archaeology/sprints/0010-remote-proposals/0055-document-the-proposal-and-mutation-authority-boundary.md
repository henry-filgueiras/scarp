---
id: tsk_01KYX1WJ3P25528P5YTXJAJA4P
sequence: 55
kind: task
status: closed
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
closed: 2026-08-01
---

# Document the proposal and mutation authority boundary

## Objective

Write two things: the durable explanation of what this channel is and
what it deliberately is not, and the recipe that lets someone build the
same channel in a repository that is not this one.

The audience is not the person filing a proposal. Their instructions
belong in the Issue Form itself. This is the architectural account —
where the trust boundary sits, why canonical state is repository-owned,
what an extension would have to satisfy — and, because Scarp is a tool
other people's repositories will use, the copyable artifact that makes
the account actionable.

The two halves are one task because they fail together. A recipe
without the boundary account is a snippet someone will modify into
something unsafe; the account without the recipe is a design note about
a feature only this repository has.

## What it must convey

**The separation.** A GitHub issue carries mutation intent; Scarp
realizes canonical state. An issue is a proposal, not an artifact, and
it is never itself canonical — nothing in the repository depends on the
issue continuing to exist. Use the proposal-authority versus
mutation-authority vocabulary rather than a fresh set of terms.
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|Task 50]] chose Option B and therefore
created **no decision artifact**, so this document is where that
vocabulary becomes durable. It is not restating a decision; it is the
only place the distinction will live.

**Why realization invokes Scarp rather than writing Markdown.** This is
the whole design, and it is the single most likely thing for a future
contributor to "simplify" — writing the file directly is obviously
easier and looks harmless. Say plainly what breaks: numbering, stable
identity, slug and path selection, template shape, and the guarantee
that a remotely-proposed artifact is indistinguishable from a local
one. Two authors of canonical form is the failure mode.

**Where the trust boundary actually is.** Be concrete about what is
trusted and what is not: the issue author is authenticated by GitHub but
not trusted; the issue content is untrusted, and Scarp validates it at
realization; the operator's machine is trusted; the Scarp binary is
trusted; `main` is authoritative. Note explicitly that no conversational
agent and no automation holds write access anywhere in this design — the
agent drafts prose, a human files it, and a human realizes it from a
machine that already had the authority. **Nothing in this design
acquires an authority the repository did not already grant**, which is
the sharpest thing the sprint has to say and should not be buried.

**Why ideas went first.** Ideas are never load-bearing: no typed edge
may target one, and a bad idea landing costs a `reject` transition
rather than an invariant. That is a property of the collection, not a
convenience, and any proposal to extend the channel has to re-argue it
for the new collection.

**Extension points, named and unbuilt.** Dragons, evidence, decisions,
typed edges, task closure, capability manifests, external proposal
APIs, and MCP. For each, one line on what it would have to settle
first — and note that the sprint is ideas-only, so each of these is a
scope question a future sprint answers rather than something the
channel quietly grows into. Where the ceiling of this approach sits — and where
[[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|idea 22]]'s transport-neutral envelope
would take over — should be stated, since this sprint is evidence for
or against that idea rather than an implementation of it.

## The recipe

*Reshaped 2026-08-01 for Option B.* The recipe got dramatically shorter
when [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] chose operator-driven
realization. It is now six steps:

1. Install Scarp.
2. Add or scaffold the proposal Issue Form.
3. Capture ideas remotely as structured GitHub issues.
4. From an authenticated development machine, list and realize a
   proposal.
5. Review the resulting canonical Scarp artifact.
6. Commit and push normally.

Say plainly what is *absent*, because each absence is a prerequisite a
copied recipe cannot silently depend on: **no workflow file, no
permissions block, no secret, no token, and no repository settings to
configure.** [[tsk_01KYX1WHY82P2WNW9RG5KWVGYA|Task 52]] closed having
changed no repository setting at all, and a consumer inherits that.

What the recipe still needs:

- the Issue Form, ideally scaffolded by Scarp rather than copied from a
  documentation block, since a file the tool can write is a file a
  reader cannot get subtly wrong;
- the realization commands, and what they require: an authenticated
  `gh`, and a GitHub remote;
- what happens when those are missing, so a reader knows the failure is
  clean and local rather than wondering whether they broke something;
- that `gh` is the operator's own authenticated CLI. Scarp stores no
  credential and holds no token — worth stating, because a reader
  evaluating whether to adopt this will ask what gains access to their
  repository, and the honest answer is nothing new.

The recipe is prose and copyable or scaffolded files, not a package.
Packaging remains [[ide_01KYX31AG163NY0EQPCTXAQ066|idea 36]], and the
richer GitHub surface remains
[[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|idea 37]]; both stay parked.

## Acceptance criteria

- The document exists at a location a reader finds without being told —
  the choice between `README.md`, `CONTRIBUTING.md`, and a `docs/` page
  is made deliberately and recorded,
  and whatever surfaces already describe contributing are updated to
  point at it.
- It reads correctly without this sprint's conversation and without the
  task files open in another tab. Since no decision artifact exists, it
  cannot lean on one: the reasoning has to be here.
- The "why not just write the file" question is answered explicitly and
  early, in terms someone tempted to do it would find persuasive.
- The trust boundary is stated as an enumeration of what is and is not
  trusted, not as prose that gestures at security.
- Extension points are listed with their unresolved question each, and
  none is designed. The ideas-only grant is stated as the reason each
  costs an amendment.
- The recipe is verified by following it into a **genuinely separate
  repository** — a real second repository, not a branch, a directory,
  or alternate configuration inside this one. Alternate configuration
  here cannot expose the couplings that matter, because everything the
  recipe might accidentally depend on is still present.
- That repository is deliberately minimal and makes no Rust-project
  assumptions, since the consumers most likely to want this channel are
  not writing Rust. The exercise is designed to expose accidental
  dependence on at least: this repository's directory layout; existing
  labels; scripts that exist only here, `scripts/check.sh` above all;
  the repository name or owner; branch conventions; a pre-existing
  Scarp corpus, including whether the channel works when the target has
  zero ideas; a Rust toolchain beyond what the documented recipe
  installs on purpose; and repository settings or permissions the
  recipe never mentions.
- The end state of that exercise is a working idea-proposal channel in
  the second repository, demonstrated by filing a proposal there and
  getting an idea. Every step that turned out to be missing, ambiguous,
  or wrong is fixed in the recipe, and the exercise is recorded. A
  recipe nobody has run is a draft.
- Anything discovered to be irreducibly specific to this repository is
  named in the recipe as a prerequisite the reader must supply, rather
  than quietly patched over in the test repository.
- The recipe names the install cost honestly and points at
  [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]] rather than glossing it.
- The document records that Option A — automated realization — was
  considered, designed in detail, and deliberately deferred, with
  [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s promotion criterion
  quoted. A reader who wants the hands-off version should find out from
  this document that it was a choice, not an oversight, and what
  evidence would reopen it.
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

Written 2026-08-01 as [`docs/remote-proposals.md`](../../../docs/remote-proposals.md).
`scripts/check.sh` passes.

### Where it lives, and why

A `docs/` page, not `README.md` and not `CONTRIBUTING.md`. The README is
a landing surface for someone deciding whether Scarp is worth installing,
and a trust-boundary enumeration is not that. `CONTRIBUTING.md` addresses
people opening issues, who need the form rather than the architecture.
This is the account a maintainer or an agent reads before changing the
channel — a different audience from either, and long enough that folding
it into an existing file would bury both.

Both surfaces now point at it: the README status table names
`scarp proposal list` / `realize` with a link, and CONTRIBUTING's Ideas
bullet was corrected — it still said "idea issue template" with the old
three-section shape, which the form replaced.

### What it argues

The "why not just write the file" question is answered early and
concretely, because it is the thing most likely to be "simplified" by
someone who does not yet know what breaks: numbering, stable identity,
slug and path selection, template shape, line endings, and the guarantee
that a remotely-proposed artifact is byte-indistinguishable from a local
one. Two authors of canonical form is named as the failure mode.

Proposal authority versus mutation authority is defined generally rather
than for this feature, with the test a future channel must answer: *does
this grant an authority the repository did not already have?* Since
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] chose Option B and created no
decision artifact, this page is where that vocabulary is durable — it is
not restating a decision, it is the record.

The trust boundary is a table of what is and is not trusted, ending on
the sharpest claim the sprint has: **nothing in this design acquires an
authority the repository did not already grant.**

Option A is documented as considered, priced, and deferred, with the
promotion criterion quoted. A reader who wants the hands-off version
learns it was a choice rather than an oversight, and what evidence would
reopen it.

### The consumer proof

Run 2026-08-01 in a deliberately alien repository: a Python project with
no Rust toolchain, no `Cargo.toml`, no `scripts/`, no pre-existing Scarp
corpus, and a different name and owner.

| Coupling tested | Result |
|---|---|
| directory layout | `scarp init` created its own; nothing assumed |
| `scripts/check.sh` | never referenced by any step |
| repository name or owner | not referenced |
| pre-existing corpus | worked from zero artifacts |
| Rust toolchain | not needed beyond installing Scarp itself |
| existing labels | the form's `idea` label is the one documented prerequisite |
| the issue form | copied verbatim, parsed, sections match the idea template |
| unavailability | `proposal list` refused cleanly and specifically; `new`, `doctor` unaffected |
| the documented fallback | `scarp new idea --body-file` produced the identical artifact |

The recipe needed one correction while being followed, now fixed in the
document: it did not say the form only takes effect once it is on the
**default branch**, which is exactly the trap
[[tsk_01KYX1WJ03MD2WRNQBS3KGMXXA|task 53]] hit here.

### What the proof does not cover, and why

**The GitHub half was not exercised in a second repository.** Doing so
means creating a real repository under an account, and the authenticated
token in this environment carries `repo` but not `delete_repo` — so I
could create one and could not remove it, leaving something in Henry's
namespace for him to clean up by hand. Creating what I cannot clean up
is not a call to make unilaterally, so it was not made.

What that leaves unproven is narrow but real: that `gh repo view` and
`gh issue list` behave the same against a repository that is not this
one, and that the `idea` label prerequisite is sufficient. Both are
expected — nothing in the implementation reads this repository's name,
and the label is passed as a literal — but expected is not verified, and
the distinction is the whole reason this criterion exists.

The cheap way to close it: run the six steps against any second GitHub
repository, or grant `delete_repo` so a scratch repository can be
created and removed in one pass.
