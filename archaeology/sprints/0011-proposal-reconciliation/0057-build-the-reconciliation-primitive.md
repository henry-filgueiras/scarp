---
id: tsk_01KYZXTN1EV8KKTK3Q75B8HSYR
sequence: 57
kind: task
status: pending
sprint: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
created: 2026-08-01
---

# Build the reconciliation primitive

## Objective

Build the one genuinely missing primitive: a bounded Scarp operation that
takes a proposal whose realized idea is already on `main` and closes the
loop on GitHub — one comment naming the canonical artifact and its
landing commit, then close as completed.

Naming notwithstanding, the target is a third verb on the existing
surface:

```console
$ scarp proposal reconcile 2
```

A third verb has to argue for itself, per task 54's target of two. The
argument is that `list` already reports realization but can do nothing
about it, and folding reconciliation into `realize` is barred by the
landing invariant below: at `realize` time the artifact is on no remote
at all.

### Proving the artifact landed

This is the substance of the task, not a detail of it.

> A proposal issue is closed only because canonical `main` contains an
> artifact explicitly claiming realization of that exact proposal.

`main`, not the working tree. Realization writes a file locally; landing
is a later, separate event that `realize` cannot observe. Closing an
issue advertises a durable public fact, so a reconciler that reads the
working tree is not merely imprecise — it publishes a claim about an
artifact that may exist on exactly one disk and never reach a remote.

Choose and record how landing is proved, including the failure modes of
the choice. The candidates differ in what they trust:

- the local `origin/main` ref, which is cheap and can be arbitrarily
  stale;
- a fetch or `git ls-remote` first, which costs a network call and
  answers about the ref rather than the file;
- asking GitHub for the file's presence on the default branch, which is
  authoritative for what a reader would see but reaches for `gh` where
  `git` might do;
- something else that is simpler and equally reliable.

Also settle **which commit is cited as the landing commit**, and what
happens when the answer is ambiguous — the artifact may have been
touched after it landed, and the useful citation is the commit that
introduced it, not the latest one that changed it.

Record the decision and its cost in the Result. Task 58 consumes it: if
proving landing locally turns out to be expensive or unreliable, that is
the strongest argument for moving reconciliation into a post-merge
workflow, and it should be stated in those terms rather than buried.

### Linkage

Only the managed `proposal:` front-matter field establishes that an
artifact realized a proposal. It already exists — written by `realize`,
parsed into the artifact summary, surfaced by `list`, and checked by
`doctor`'s `duplicate-proposal` finding — so this task consumes it and
adds no new provenance representation.

No title matching, body similarity, timestamp proximity, or sequence
adjacency, in any code path, including as a fallback when the field is
absent. A proposal with no realizing artifact is simply not
reconcilable.

### The comment

Written for someone who arrives at the issue months later with no
context. It should identify the artifact by display sequence, stable id,
and repository path; link the commit that landed it; and say plainly
that the Scarp artifact is now the canonical record and this issue is
not. It is a terminal projection, not a status update — nothing will
ever come back to amend it.

### Constraints

- Reuse the existing `gh` shell-out. No HTTP client, no SDK, no
  credential storage, no forge abstraction.
- Unavailability stays typed and clean: `gh` absent, unauthenticated,
  offline, or no GitHub remote yields `IntegrationUnavailable` naming
  what is missing and what to do, and every ordinary Scarp command is
  unaffected.
- Reconciliation reads canonical state and never writes it. It creates,
  edits, transitions, and deletes nothing, and it must not be able to.
- Ideas only.

## Acceptance criteria

- `scarp proposal reconcile` comments on and closes a proposal issue
  whose realized idea is on `main`, and the comment contains the
  artifact's sequence, stable id, path, and landing commit.
- It **refuses** when the realizing artifact is not on `main`, naming
  that as the reason. The repository provides both cases live: issue #2's
  idea 38 is on `origin/main`, and issue #3's idea 40 is committed
  locally and pushed nowhere.
- It refuses when the issue has no realizing artifact, when the issue is
  not a proposal, and when the number does not exist — each with a typed
  error a script can distinguish.
- Idempotent in every reachable state: an already-closed issue is a
  no-op; an existing reconciliation comment is never duplicated; a re-run
  after a partial failure completes rather than double-posting.
- Reconciling several proposals in one invocation, if the surface allows
  it, treats each independently: one failure does not strand the rest,
  and the exit status reflects that something failed.
- `--json` emits a deterministic object describing what was reconciled,
  consistent with the other `proposal` subcommands.
- Every failure mode is covered by tests that do not require network
  access, following the existing module's test strategy.
- The landing-proof decision, its failure modes, and its cost are
  recorded in the Result in terms task 58 can adjudicate from.
- `scripts/check.sh` passes.
