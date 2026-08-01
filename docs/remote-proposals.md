# Remote proposals

How an idea captured away from your development machine becomes a
canonical Scarp artifact — and, more importantly, what this deliberately
does not do.

The short version: **a GitHub issue carries mutation intent; Scarp
realizes canonical state.** Those are different authorities, held by
different parties, and keeping them apart is the entire design.

## The shape

```text
phone or remote conversation
    ↓
structured GitHub proposal issue      ← durable immediately
    ↓
(time passes; nothing is at risk)
    ↓
operator, on a trusted machine        ← scarp proposal realize <n>
    ↓
canonical idea artifact
    ↓
ordinary review, commit, and push
```

Nothing runs unattended. No automation holds a credential. Nothing
commits or pushes on your behalf.

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

## Proposal authority and mutation authority

**Proposal authority** is the right to *express* an intended change.
Anyone with a GitHub account has it. It costs nothing to grant, because
a proposal changes nothing.

**Mutation authority** is the right to *realize* one as canonical state.
It stays with whoever already holds the repository — the operator at a
machine with a checkout and push rights. It is never delegated, never
issued as a token, and never held by a workflow.

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
| `main` | **Authoritative.** |

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
if the artifact already exists on your branch, and `scarp doctor`
reports `duplicate-proposal` for the case no single run can catch: two
branches that each realized once, merged.

## Why ideas went first

Ideas are never load-bearing. No typed edge may target one, and a bad
idea landing costs a `reject` transition rather than an invariant.

That is a property of the collection, not a convenience. Any proposal to
extend this channel to dragons, decisions, or tasks has to re-argue it
for that collection, where the answer is different and worse.

## Setting this up in your own repository

Six steps. There is **no workflow file, no permissions block, no secret,
no token, and no repository setting to configure** — each absence is one
fewer thing to get wrong.

1. **Install Scarp**, and have [`gh`](https://cli.github.com)
   authenticated (`gh auth login`).
2. **Add the proposal issue form** — copy
   [`.github/ISSUE_TEMPLATE/idea.yml`](../.github/ISSUE_TEMPLATE/idea.yml).
   Its textarea labels must match your idea template's sections exactly:
   `Problem`, `Sketch`, `Boundaries`, `Evidence`. It needs an `idea`
   label to exist — `gh label create idea --description "Uncommitted
   proposals to explore"` if yours does not have one.
   *The form only takes effect once it is on your default branch.*
3. **Capture ideas remotely** as structured issues, from anything with a
   browser.
4. **List and realize**, from an authenticated development machine:
   ```console
   $ scarp proposal list
   $ scarp proposal realize 2
   ```
5. **Review** the created artifact like any other new file.
6. **Commit and push** normally.

### When it is unavailable

If `gh` is missing, unauthenticated, or your repository has no GitHub
remote, `scarp proposal` refuses with
`error[integration-unavailable]` (exit 11) naming what is missing and
what to do. **Every other Scarp command is unaffected**, and
`scarp new idea --body-file` produces the identical artifact by hand.
The integration is a convenience, never a dependency.

## Extension points, named and unbuilt

Each is deliberately absent, with the question it would have to settle
first:

- **Other collections** (dragons, decisions, tasks) — are they
  non-load-bearing enough that a bad realization is cheap? For ideas the
  answer is yes; elsewhere it is not.
- **Typed edges from a proposal** — an edge targets a managed artifact,
  and an issue is not one. What would the target be?
- **Task closure from a proposal** — closing is a lifecycle transition
  on existing state, not creation. A different authority question.
- **Other forges** — GitLab and Forgejo need a consumer before an
  abstraction. A provider interface with one implementation is
  speculative framework-building.
- **Automated realization** — see below.
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
