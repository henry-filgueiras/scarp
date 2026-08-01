---
id: tsk_01KYX1WHTGXMBCBA7NE27RM9CF
sequence: 50
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
---

# Adjudicate whether CI may author canonical commits

## Objective

Choose which shape the proposal channel takes, and only then settle what
decision — if any — that shape requires. This task gates
[[tsk_01KYX1WJ03MD2WRNQBS3KGMXXA|task 53]] and
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]].

*Reframed 2026-08-01.* This task was created assuming the answer was
yes-with-constraints, and its title reflects that. The title is left
narrow rather than corrected, because the fact that the sprint began by
assuming an automated channel and then found two cheaper shapes is
itself worth preserving. **Two of the three options below require no
decision at all**, so "whether CI may author canonical commits" is now
one branch of the question rather than the whole of it.

What prompted the reframe: task 51 shipped `--body-file`, which was the
only genuinely missing capability. With it in hand, the channel's
remaining complexity is not serving the motivating use case — it is
serving the threat model of a workflow holding a write token on a public
repository. Every mechanism the sprint accumulated (replay guard,
receipt, partial-state recovery, late authorization, postcondition
proof, four reconciled policy sites) exists because of that token.
Remove the token and they evaporate. That is worth adjudicating
deliberately rather than discovering after building.

The project as written forbids **option A's** central mechanic in four
places, and all four must be reconciled or the repository is left giving
contradictory guidance. Options B and C touch none of them:

1. [[dec-bootstrap-interaction-surfaces|Decision 7]]: "Tooling never
   rewrites prose outside an explicit, user-initiated operation whose
   diff the user can review. No save hooks, no CI commits (automatic
   commits are already a recorded non-goal), no batch rewrites hidden
   inside unrelated commands."
2. CLAUDE.md's non-goal list: "automatic commits".
3. CLAUDE.md's non-goal list: "GitHub Issues synchronization".
4. CLAUDE.md's commit policy: "Never push. Pushing is always a human
   decision." The workflow pushes a proposal branch, so this is a
   literal violation even though nothing reaches `main` without a
   human.

CLAUDE.md's change discipline requires a new recorded decision plus
evidence before any non-goal is implemented. Building the workflow
first and recording the decision afterwards would be exactly the
ceremony this repository exists to prevent.

## The three options

All three deliver the motivating use case: an idea drafted in
conversation on a phone becomes a canonical artifact with **no manual
transcription**. They differ in who performs the mutation, and therefore
in what has to be trusted.

One observation that applies to all three, and lowers the stakes: **an
open GitHub issue is already durable.** Nothing is lost while a proposal
waits. What the channel buys is *canonicalization*, not persistence — so
the urgency for automation is lower than it first appears, and a delay
between proposing and realizing costs judgment time rather than data.

### Option A — the automated channel

An authorized issue triggers a workflow that realizes the idea, opens a
pull request, and Henry merges it.

- **Buys:** the idea lands without the operator doing anything.
  The consumer story is fully hands-off.
- **Costs:** a decision reconciling all four sites above; a token with
  `contents: write` on a public repository; the replay guard, receipt,
  and eight partial-state behaviours; late authorization re-checking;
  postcondition proof; and the release in
  [[tsk_01KYX31ACH05NGA3GYH0TJA870|task 56]] on the critical path.
- **Requires a decision:** yes — the grant below.
- **Task effect:** the sprint proceeds as currently written.

### Option B — realization from the operator's machine

The proposal issue is filed from the phone. Later, from the laptop, the
operator lists open proposals and realizes a chosen one through Scarp,
then commits normally.

- **Buys:** zero transcription; structured payload; Scarp still the sole
  author of canonical form; GitHub-native transport; and a *shorter*
  consumer story — install Scarp, add an issue form, run a command, with
  no workflow file, no permissions block, and no secrets.
- **Costs:** the operator must run a command. That is the whole cost.
- **Requires a decision:** **no.** No CI commits, so decision 7 is
  untouched; no push, so the commit policy is untouched; no automation,
  so the "automatic commits" non-goal is untouched. Authorization
  collapses to "whoever holds the laptop and can push", which is the
  authority model the repository already has.
- **Task effect:** tasks 48 and 49's findings become background;
  [[tsk_01KYX1WHY82P2WNW9RG5KWVGYA|task 52]] closes having changed
  nothing; task 53 shrinks to the issue form; task 54 becomes the
  realization command and its guard against realizing the same issue
  twice; task 56 is no longer on the critical path, though the release
  is still wanted.
- **Grows into:** [[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|idea 37]], where the
  `gh … | scarp new` pipeline becomes a detected first-class surface
  with `gh` playing the role `git` already plays — shelled out to,
  narrowly scoped, optional, and never required.

### Option C — the operator writes to an inbox

Rough text is dropped into an in-repository `inbox/` directory from
anywhere — a phone Git client, a sync folder — and a later operator
command promotes each file into a canonical artifact and removes it.

- **Buys:** works without GitHub entirely, and without a network at
  realization time. The lowest-trust design of the three: nothing but
  the operator ever writes anything.
- **Costs:** a new non-canonical directory in a repository whose first
  invariant is that files are canonical. Inbox files would be neither
  canonical artifacts nor derived projections but a third thing —
  pending unvalidated input — which doctor must be taught to ignore.
- **Requires a decision:** no policy amendment, but probably a decision
  about what the inbox *is*, because the third category is a real
  addition to the artifact model.
- **Prior art that must be answered:**
  [[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|idea 22]] already considered and
  rejected a repository-local `proposals/` staging area with a
  `proposed` status, on review of an external RFC. Its recorded reason
  applies directly: *a parked idea is already a proposal.* Ideas are
  non-load-bearing, already await review, and already have a terminal
  `reject`. Any inbox has to explain what it offers that a parked idea
  does not.
- **Important distinction:** if the thing writing to `inbox/` is CI,
  this option solves nothing — the token, the commit, the push, and the
  replay guard all remain, and a concept is added on top. Option C is
  only cheap if the writer is the operator.

### Complementarity

B and C are not competitors. B is the GitHub-shaped path; C is the
transport-free path for when there is no forge, no network, or no issue
worth filing. A repository could sensibly have both, and neither
forecloses A later — if the manual step grates after enough
repetitions, wrapping it in a workflow is small, and by then the
recurrence is the evidence CLAUDE.md's change discipline asks for.

### The recommendation this task carries

**Option B, with C as a later complement.** The reasoning, stated so it
can be argued with rather than deferred to: A's entire additional cost
buys the removal of one command on a laptop, and it pays for that by
delegating mutation authority to a token — which is precisely the
authority the sprint's own architecture says should stay with the
repository. B keeps the separation the sprint exists to demonstrate and
makes it *sharper*, not weaker: proposal authority is open to anyone,
mutation authority stays undelegated.

This is a recommendation, not the adjudication. A is a legitimate
choice, and its hands-off consumer story is a real product argument that
B does not answer.

## The grant, if option A is chosen

Everything from here applies only under option A. Henry fixed this shape
on 2026-07-31, before options B and C were articulated; it is preserved
unchanged so that choosing A does not require re-deriving it. The grant:

> Automation may cause Scarp to **create** a new artifact, in
> collections that are never load-bearing. It may never modify or
> delete an existing artifact.

with the conditions that creation is realized through the Scarp CLI,
that `scarp doctor` is green before a pull request opens, and that
canonical state reaches the default branch only through a pull request
a human merges.

Two things about that wording are deliberate and should survive
drafting. **Creation-only** is the clause that tracks decision 7's
actual rationale: decision 7 prohibits *rewriting prose*, and the ban's
three examples are all rewrites. Authoring a file that did not exist is
not the act it banned. It is also the only clause checkable from the
diff alone — a conforming proposal changes exactly one added file under
a managed collection — so the grant is enforceable rather than trusted.
**Ideas-only** is the deliberately tighter dial: the broader
creation-only rule was available and was not taken, so extending the
channel to a second collection costs an amendment on purpose.

Auto-merge is **outside** the grant. Declining it is what lets decision
7's "diff the user can review" clause be satisfied literally rather
than argued around, and the decision should say so, since that is the
clause a future reader will test any extension against.

## The other three sites

- **CLAUDE.md non-goal "automatic commits"** is qualified, not
  removed, and points at this decision. The list is already conditional
  on "a new recorded decision and evidence"; this is that decision.
- **CLAUDE.md non-goal "GitHub Issues synchronization"** stays, with
  the distinction stated: this channel is one-shot *realization*, not
  synchronization. Nothing mirrors state back, the issue is never
  canonical, and deleting the issue invalidates nothing. Without that
  sentence a future reader will reasonably conclude the project shipped
  something on its own non-goal list.
- **CLAUDE.md "Never push"** is scoped rather than deleted. Its real
  content — nothing reaches `main` without a human — survives intact;
  what changes is that the proposal channel may push its own proposal
  branch. Sessions still never push.

## Scope

Adjudication first. Then, under option A only, one decision artifact
created with `scarp new decision` and the reconciling edits to CLAUDE.md
and decision 7 in the same state.

Under **every** option, the vocabulary the rest of the sprint and
[[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]] use must be fixed somewhere
durable: **proposal authority** (who may express mutation intent) versus
**mutation authority** (what may realize canonical state), and why they
are separate. Under A that belongs in the decision. Under B or C it
still needs a home — most likely task 55's documentation — because it is
the idea the channel exists to demonstrate, and it would otherwise
survive only in this task's prose.

The shape above is owner direction, but the drafted decision still
returns to Henry before the sprint proceeds — the same boundary
[[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] drew around publication. An
agent drafting a decision is not the same as the decision being made.

Consistency check that must not be skipped:
[[idea-single-invocation-commits|Idea 9]] proposes an opt-in `--commit`
flag and names the automatic-commits non-goal as its own adoption gate.
Whatever is written here changes what idea 9 would still need, so the
two must not settle the same question differently.

## Acceptance criteria

### Under every option

- Henry chooses A, B, or C. The choice and its reasoning are recorded
  here, including what the rejected options would have bought, so a
  later reader can tell a decision from a drift.
- The proposal-authority versus mutation-authority distinction is
  written down somewhere durable, in terms a later reader can apply to a
  collection this sprint never touched and to a repository that is not
  this one.
- The consequences for tasks 49, 52, 53, 54, and 56 are stated, and any
  task the choice makes unnecessary is transitioned to a terminal state
  with its reason rather than left pending. A task list that still
  describes the unchosen option is the failure mode here.
- Tasks 48 and 49's findings are preserved whatever is chosen. They cost
  real research and they remain true; under B or C they become
  background rather than waste.

### Under option A additionally

- The drafted decision is ratified by Henry, not inferred from the
  sprint existing. A recorded refusal remains a legitimate outcome; if
  it is the outcome, the sprint's implementation tasks are transitioned
  rather than left pending, and the sprint closes with the research
  kept.
- The decision records the grant, its four conditions, and what remains
  forbidden — with modification, deletion, load-bearing collections,
  and auto-merge each named as outside it rather than merely unmentioned.
- The creation-only clause is stated in a form that can be checked
  against a diff, and the decision says so, so a later reader knows the
  grant is enforceable rather than trusted.
- The decision states its relationship to
  [[dec-bootstrap-interaction-surfaces|decision 7]] explicitly —
  narrowing it, not superseding it — and decision 7 gains a pointer so
  neither artifact can be read alone and come away wrong. In-place
  dated amendment is established practice here; decision 11 carries
  `## Amendment: narrowed to lifecycle authority (2026-07-22)` as the
  model.
- All three CLAUDE.md sites are reconciled in the same state: the
  "automatic commits" non-goal qualified and pointing at the decision,
  the "GitHub Issues synchronization" non-goal kept with the
  realization-versus-synchronization distinction stated, and the
  "Never push" policy scoped to preserve its real content.
- The decision is written to be true of any Scarp repository, not only
  this one. A consumer adopting the channel adopts this grant, so a
  clause that only makes sense here is a defect.
- The evidence CLAUDE.md's change discipline demands is cited: the
  concrete recurring need, not the attractiveness of the design.
- Consistency with [[idea-single-invocation-commits|idea 9]] is stated,
  and idea 9 is left parked unless the decision genuinely adopts it.

### Under option C additionally

- What an inbox file *is* is settled before one is written: not a
  canonical artifact, not a derived projection, and therefore a third
  category the artifact model does not currently have. Doctor's
  behaviour toward it is stated.
- [[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|Idea 22]]'s rejection of a
  repository-local staging area is answered rather than ignored,
  including what an inbox offers that a parked idea does not.
- The writer is the operator. If any automation writes to the inbox,
  option A's costs return in full and the choice is recorded as A-with-
  an-inbox rather than as C.

### In every case

- No workflow file, issue form, or repository setting changes here.

## Result
