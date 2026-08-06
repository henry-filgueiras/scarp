# Remote proposals

How something captured away from your development machine — an idea, or
a bug report from a stranger — becomes a canonical Scarp artifact, and,
more importantly, what this deliberately does not do.

The short version: **a GitHub issue carries mutation intent; Scarp
realizes canonical state.** Those are different authorities, held by
different parties, and keeping them apart is the entire design.

## Two classes, one loop

A proposal issue carries exactly one recognized label, and that label
decides what realizing it produces.

| Label | Command | Canonical result |
|---|---|---|
| `idea` | `scarp proposal realize N` | a parked idea |
| `bug` | `scarp proposal realize N` | a pending maintenance item |
| `bug` | `scarp proposal realize N --sprint sprint:X` | a pending task in that active sprint |

Other labels are ignored — a proposal may also be `documentation` or
`good first issue` — but exactly one *recognized* label must remain. An
issue with neither is not a proposal; an issue with both is refused
rather than guessed at.

## The shape

```text
phone, or a stranger's browser
    ↓
structured GitHub proposal issue      ← durable immediately
    ↓
(time passes; nothing is at risk)
    ↓
operator, on a trusted machine        ← scarp proposal realize <n>
    ↓
canonical artifact: idea, maintenance item, or task
    ↓
ordinary review, commit, and push
    ↓
    ├─ idea:  the artifact is on the default branch
    └─ bug:   the work is `closed` on the default branch
    ↓                                   ← the loop's precondition
operator, again                        ← scarp proposal reconcile <n>
    ↓
issue cites the artifact and closes
```

Nothing runs unattended. No automation holds a credential. Nothing
commits or pushes on your behalf.

### Why the last step is separate

Reconciliation cannot be folded into `realize`, and the reason is not
tidiness. At realization the artifact exists on exactly one disk.
Closing the proposal then would advertise something no reader can find.

Landing is a later, independent event — a commit and a push that
`realize` cannot observe and has no business waiting for. So the loop
has four steps, not three, and the fourth one is gated:

> **A proposal is closed only because the branch a reader sees already
> contains the artifact claiming it — and, for a bug, because that
> artifact has reached its terminal state there.**

## Why not just write the file?

This is the question a future contributor will ask while looking at a
`gh` invocation and a Markdown body, thinking *I could write that file
in three lines.* You can. Doing so breaks these, all at once:

- **numbering** — display sequences are allocated against the whole
  collection, and a hand-written file guesses;
- **stable identity** — the `id` is a prefixed ULID Scarp mints;
- **slug and path selection** — derived deterministically from the
  title, and refused rather than overwritten on collision;
- **template shape** — which sections exist, in what order;
- **line endings, front-matter field order, and the trailing-newline
  contract**;
- **the guarantee that matters most**: a remotely-proposed artifact is
  byte-indistinguishable from one authored locally.

Two authors of canonical form is the failure mode. Everything else on
this page follows from refusing to have one.

## Three authorities, held by different parties

**Proposal authority** is the right to *express* an intended change.
Anyone with a GitHub account has it. It costs nothing to grant, because
a proposal changes nothing.

**Mutation authority** is the right to *realize* one as canonical state.
It stays with whoever already holds the repository — the operator at a
machine with a checkout and push rights. It is never delegated, never
issued as a token, and never held by a workflow.

**Projection authority** is the right to *report* canonical state
outward — to comment on the issue and close it. It is the weakest of the
three, because by the time it is used the repository has already
decided: it writes nothing canonical and could not, and everything it
says is a fact `main` already published. Reconciliation holds only this
one.

It is still not granted casually. Closing an issue is public, terminal,
and posted under the repository's name, which is why it is gated on the
artifact actually being on the default branch rather than merely
existing.

The distinction generalizes past this feature. Any future channel —
another collection, another forge, an agent protocol — is answerable to
the same question: *does this grant an authority the repository did not
already have?* If yes, it needs a decision, not an implementation.

## The trust boundary, enumerated

| | Trusted? |
|---|---|
| The issue author | **No.** Authenticated by GitHub, trusted by nobody. |
| The issue content | **No.** Untrusted input, validated by Scarp at realization. |
| The operator's machine | **Yes.** It already has a checkout and push rights. |
| The `scarp` binary | **Yes.** It is the only author of canonical form. |
| `gh` and its session | **Yes** — it is the operator's own, borrowed, never stored. |
| The default branch | **Authoritative** — in both directions. It is what realization must not fake, and what reconciliation must confirm before speaking. |
| The reconciliation comment | **Derived.** A report of the branch above, never a source. Deleting it changes nothing. |

**Nothing in this design acquires an authority the repository did not
already grant.** No conversational agent has write access. No workflow
has a token. The agent drafts prose, a human files it, and a human
realizes it from a machine that already had the power to do so by hand.

Untrusted content is handled by Scarp rather than by convention. A body
that forges front matter, injects a heading the template reserves,
carries control characters, or is not UTF-8 is refused before anything
is written — and a refusal leaves no partial file and burns no display
sequence.

## Realization is one-shot, not synchronization

The issue is **never canonical**. Nothing mirrors state back to it.
Editing, closing, or deleting a proposal after realization changes
nothing in the repository, and nothing here will ever reconcile the two.

A realized artifact records where it came from in front matter:

```yaml
proposal: https://github.com/owner/repo/issues/2
```

That is one-way provenance, stamped once, never updated. It is not a
typed edge — edges target managed artifacts, and this names something
outside the repository — and it is not a live link. `scarp doctor`
validates its shape and its uniqueness; it never checks whether the URL
resolves, because no canonical operation may depend on a network.

Because the field is managed rather than prose, realizing one proposal
twice is **corruption Scarp can see**. `scarp proposal realize` refuses
if the artifact already exists on your branch — in **any** collection,
because the uniqueness rule is global rather than per-collection, and a
per-collection check would let one report become both a maintenance item
and a task. `scarp doctor` reports `duplicate-proposal` for the case no
single run can catch: two branches that each realized once, merged.

## Reconciliation is terminal too

Once the precondition holds — the artifact is on the default branch, and
for a bug, closed there:

```console
$ scarp proposal reconcile 2
reconciled proposal #2: closed, citing idea:38
```

The issue gets one comment naming the artifact — display sequence,
stable id, path, and the cited commit, each linked and pinned so they
still resolve after the file moves — and then closes as completed.

**That comment is never revisited.** Adopting or rejecting the idea
later does not amend, reopen, or annotate it, and neither does anything
that happens to a maintenance item after it closes. Editing the issue
afterwards reaches nothing. It is a settled fact, written once, and the
artifact is where the lifecycle actually lives.

So the two directions are terminal in the same way, for the same reason:
neither side is a mirror of the other. Realization does not watch the
issue; reconciliation does not watch the artifact.

Linkage is never guessed. The `proposal:` field above is the only thing
that connects an issue to an artifact — no title matching, no body
similarity, no timestamp or sequence proximity, and no fallback when the
field is absent. A proposal with no realizing artifact is simply not
reconcilable.

Re-running is safe in every state. An already-closed issue is a no-op. A
run that died after commenting closes on the next attempt without saying
it twice, which is why the comment is posted first: no reachable state
has a proposal closed without its explanation.

## Why ideas went first, and what bugs had to earn

Ideas are never load-bearing. No typed edge may target one, and a bad
idea landing costs a `reject` transition rather than an invariant. That
is what made realizing one a cheap, reversible act an operator can
perform on a stranger's say-so.

**A promoted bug report is not that.** A maintenance item asserts that
work exists; a task inside an active sprint can be planned around and
depended on. The safety argument above does not transfer, so the second
class had to earn its own, and it did so by narrowing what promotion
claims:

> **Realizing a `bug` accepts an obligation to investigate. It does not
> assert that the reporter's diagnosis is correct.**

That is why the artifact is titled *Investigate reported behavior:
&lt;issue title&gt;* rather than repeating the reporter's words as though
the project agreed with them, and why the generated body says so in
prose that a reader six months later will see before they see the report.

The obvious worry is what happens when the report turns out to be
wrong — recording a non-bug as completed maintenance sounds like a false
statement in the archaeology. It is not, because the finding lives in the
`Result`, not in the status. Working as intended, unreproducible, a
duplicate, already handled, and a considered decision not to act are all
things a `Result` says plainly, and all of them are true statements about
work that is over. Scarp therefore added **no** `cancelled` or
`withdrawn` state: they would have added vocabulary without adding
honesty.

Dragons and decisions still have no path here, and each would have to
re-argue the question for itself.

## Reconciling a bug is gated harder

An idea *is* the deliverable, so its arrival on the default branch is
what the filer was waiting for. A bug reporter is waiting on an outcome.
Closing their issue to announce that a tracking item now exists would be
worse than saying nothing.

So for a maintenance item or a task, reconciliation fetches the default
branch's copy of the artifact, parses it, and requires four facts before
it will speak: the stable id, the `kind`, the `proposal:` URL, and
`status: closed`. Four cheaper checks are deliberately **not** used:

- the **local status** — it says only what your disk believes, and a
  closed item you have not pushed is exactly the case that must refuse;
- a **remote-tracking ref** — it answers a question about your last
  fetch, not about what a reader sees;
- a **substring search** — it cannot tell `status: closed` in front
  matter from the same words quoted inside a `Result`;
- **path existence** — sufficient for an idea, and not sufficient here.

The commit the comment cites is the newest one to touch the path, not the
one that introduced the file, and Scarp re-reads the artifact at that
exact revision before citing it. A bug artifact arrives `pending`; citing
its arrival would pin a permalink that contradicts the sentence beside
it.

The comment itself says the work **reached its terminal result**, and
never that it was fixed:

> Investigated as **maintenance:4**, which has reached its terminal
> result in the canonical record.
>
> **Read the `Result` for what was concluded.** Reaching a terminal
> result is not a claim that a defect existed […]

That wording is enforced by a test, not by care. The `Result` may
conclude that nothing was wrong, and nothing ever comes back to correct a
published comment.

## Setting this up in your own repository

Six steps. There is **no workflow file, no permissions block, no secret,
no token, and no repository setting to configure** — each absence is one
fewer thing to get wrong.

1. **Install Scarp**, and have [`gh`](https://cli.github.com)
   authenticated (`gh auth login`).
2. **Add the issue forms** — copy
   [`.github/ISSUE_TEMPLATE/idea.yml`](../.github/ISSUE_TEMPLATE/idea.yml)
   and
   [`.github/ISSUE_TEMPLATE/bug.yml`](../.github/ISSUE_TEMPLATE/bug.yml).
   The idea form's textarea labels must match your idea template's
   sections exactly — `Problem`, `Sketch`, `Boundaries`, `Evidence` —
   because they become those sections. The bug form's labels are free:
   the whole report lands inside one section Scarp owns, so its headings
   are nested rather than promoted.

   Both labels must exist. Check with `gh label list`; `bug` is one of
   GitHub's defaults, and `idea` usually is not:
   ```console
   $ gh label create idea --description "Uncommitted proposals to explore"
   ```
   *A form only takes effect once it is on your default branch.*
3. **Capture proposals remotely** as structured issues, from anything
   with a browser. Bug reports arrive this way from people who have
   never seen your archaeology.
4. **List and realize**, from an authenticated development machine. The
   listing states what each proposal would become:
   ```console
   $ scarp proposal list
   #9  open      maintenance  Doctor miscounts artifacts after a merge
   #4  open      idea         Prebuilt release binaries

   $ scarp proposal realize 4                       # a parked idea
   $ scarp proposal realize 9                       # a maintenance item
   $ scarp proposal realize 9 --sprint sprint:13    # or a task, if a
                                                    # sprint owns it
   ```
5. **Review** the created artifact like any other new file.
6. **Commit and push** normally. For a bug, also **do the work** and
   close the item with a `Result` — that is what reconciliation waits
   for.
7. **Reconcile**, once the precondition holds:
   ```console
   $ scarp proposal reconcile 4    # idea: once it is on the branch
   $ scarp proposal reconcile 9    # bug: once it is closed there
   ```

Steps 1–2 are one-time; 3–7 are the loop.

**What has actually been run, precisely.** The idea half of this recipe
has been exercised live against GitHub in one repository — this one. The
Scarp half was followed into an unrelated project with no Rust and no
existing corpus and worked unchanged.

The **bug half has not yet been performed live.** It is covered
end to end by hermetic tests that drive the compiled binary against a
fake `gh`, which prove that Scarp builds the invocations it intends to
and never mutates an issue on an unproven claim — and prove nothing at
all about how GitHub answers them. Nothing about it is
repository-specific, but "expected to work" is a weaker claim than "was
run", and for that half it is the accurate one.

### When it refuses

Every refusal is typed, and the exit code tells you which kind of
problem you have.

| | Code | Exit | What to do |
|---|---|---|---|
| `gh` missing, unauthenticated, offline, or no GitHub remote | `integration-unavailable` | 11 | Install or authenticate `gh` — or skip it entirely, since `scarp new … --body-file` produces the identical artifact by hand. |
| The issue carries neither `idea` nor `bug` | `invalid-invocation` | 2 | Check the number, or label it on GitHub if it really is a proposal. |
| The issue carries **both** `idea` and `bug` | `invalid-invocation` | 2 | Remove the wrong label on GitHub. Scarp will not guess which one you meant. |
| `--sprint` on an `idea` | `invalid-invocation` | 2 | Drop it. An idea is an uncommitted proposal, never a sprint's committed work. |
| `--sprint` names a closed, missing, or ambiguous sprint | `invalid-invocation` / `artifact-not-found` | 2 / 7 | The same rules `scarp new task --sprint` applies. |
| The proposal was already realized | `artifact-conflict` | 4 | Nothing to do. One proposal realizes at most one artifact, across every collection. |
| No artifact records this proposal | `precondition-unmet` | 12 | Realize it first — or switch to the branch that already did. |
| The artifact is realized but not on the default branch | `precondition-unmet` | 12 | Commit and push it, then retry. Nothing is wrong; you are early. |
| A bug's artifact is on the default branch but not `closed` there | `precondition-unmet` | 12 | Close the work, then commit and push. The reporter is waiting on an outcome. |
| The default branch's copy is not the artifact being claimed | `precondition-unmet` | 12 | Investigate the mismatch by hand. Something replaced the file or two branches disagree. |

The distinction between exit 11 and 12 is worth knowing: **11 means
something is broken or absent, 12 means retrying later can succeed with
nothing repaired.**

**Every other Scarp command is unaffected** by all of these. The
integration is a convenience, never a dependency: a repository whose
operator has never installed `gh` is fully usable.

## Extension points, named and unbuilt

Each is deliberately absent, with the question it would have to settle
first:

- **Other collections** (dragons, decisions) — is a bad realization
  cheap, and if not, what narrower claim can promotion make instead?
  Ideas answered the first question; bugs answered the second. A dragon
  is an unresolved risk the project asserts it holds, and neither answer
  obviously transfers.
- **Typed edges from a proposal** — an edge targets a managed artifact,
  and an issue is not one. What would the target be?
- **Task closure from a proposal** — closing is a lifecycle transition
  on existing state, not creation. A different authority question.
- **Other forges** — GitLab and Forgejo need a consumer before an
  abstraction. A provider interface with one implementation is
  speculative framework-building.
- **Automated realization**, and **automated reconciliation** — both
  considered in full and both declined, each against its own recorded
  criterion. See the last two sections.
- **A reminder that a proposal is waiting** — reconciliation is the one
  step still carried by the operator's memory. A `doctor` finding is the
  wrong home for it, since an unreconciled proposal is not corruption.
- **A capability manifest, an external proposal API, MCP** — how would
  an arbitrary agent discover what this repository permits without
  bespoke prompting? Open.

Where this approach runs out is worth naming: it is GitHub-shaped and
transport-specific by construction. A transport-neutral mutation-intent
envelope is a different design, parked as an idea in the archaeology,
and this channel is evidence for or against it rather than an
implementation of it.

## Automated realization was considered and deferred

A GitHub Actions workflow could realize proposals with no operator step
at all. It was designed in detail, and declined — not because it is
wrong, but because it was not yet worth its price:

- a token with `contents: write` on a public repository;
- amendments to the project's standing rules against CI commits and
  automated pushes, in four places;
- replay and idempotency machinery for distributed side effects;
- late authorization re-checks and partial-state recovery;
- branch and pull-request publication machinery.

All of that buys the removal of **one command on a laptop**, and pays
for it by delegating mutation authority to a token. An open issue is
already durable; nothing is lost while a proposal waits.

The criterion for reopening it is recorded rather than left to taste:

> Reconsider automated realization when actual use of operator-driven
> realization demonstrates that the remaining explicit realization step
> is recurring material friction — for example proposals repeatedly
> accumulate, are abandoned, are delayed meaningfully, or require
> burdensome batching.

If you are reading this because the manual step is annoying you: that is
the evidence. The research for the automated design is preserved in the
archaeology, so reopening it starts from findings rather than a blank
page.

## Automated reconciliation was considered separately, and also declined

Closing the loop is a much cheaper grant than realizing proposals. A
workflow doing it would hold `issues: write` and never `contents:
write`, and by the time it ran the canonical change would already have
passed through the operator's machine. **If this had come down to
authority, the answer would have been yes.**

It came down to value. Two findings, in order of weight:

- **The judgment is the point.** The first reconciliation comment ever
  published had a defect every passing test had missed — a bare
  forty-character sha, unlinked, in a table otherwise made of links. It
  was caught because a person read it before it went out. Unattended,
  it would have been published to every proposal before anyone looked.
- **Automation would buy almost nothing.** A post-merge workflow's one
  real advantage is knowing for free that the change landed. But
  reconciliation must reach GitHub anyway to comment and close, so
  proving it locally costs two API calls on a command already making
  them — against a command that takes about a second.

The criterion for reopening it, again recorded rather than left to
taste:

> Reconsider automated reconciliation when **both** are true: five or
> more reconciliations have been performed (or two landed proposals
> await at once), **and** the comment body has gone unchanged across
> three consecutive runs — that is, reading it before publishing has
> stopped finding anything.

The conjunction is deliberate. Volume alone is the wrong trigger: a
frequently-published comment that is still changing is exactly when a
human should still be reading it.

**What declining costs, plainly:** nothing reminds you. A landed
proposal stays open until you run the command, so `scarp proposal list`
is worth a glance when you sit down. That is the one piece of this loop
still carried by memory rather than by the tool.
