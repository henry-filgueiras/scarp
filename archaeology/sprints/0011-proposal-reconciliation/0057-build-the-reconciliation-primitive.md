---
id: tsk_01KYZXTN1EV8KKTK3Q75B8HSYR
sequence: 57
kind: task
status: closed
sprint: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
created: 2026-08-01
closed: 2026-08-01
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

## Result

Shipped as `scarp proposal reconcile <number>`.

### Landing is proved by asking GitHub, not the local checkout

**The choice, and the argument that decided it.** The invariant is about
what a *reader* would find on the branch the repository serves. The local
`origin/main` remote-tracking ref answers a different question — what this
machine saw at its last fetch — and only coincides with the real one when
the fetch is current. It fails safe when stale (behind means refuse), so
it was defensible.

What ruled it out was noticing that **reconciliation is inherently
online**. It has to reach GitHub to comment and close, so there is no
offline path to protect and no network cost being introduced; the API
call was already being paid for. Given that, trading an authoritative
answer for a cheaper approximate one buys nothing. Asking GitHub also
avoided introducing a `git` shell-out into a module that had none, and
sidestepped "which remote is `origin`" entirely, since `gh` has already
resolved the repository.

Two calls, deliberately separate because they answer different questions:

- `GET repos/{owner}/{repo}/contents/{path}?ref={default_branch}` —
  presence, which is what the invariant is actually about. A file added
  and later deleted has commits but is not there, so the commit list
  alone would be wrong.
- `GET repos/{owner}/{repo}/commits?path={path}&sha={default_branch}`,
  paginated, one sha per line via `--jq` — the *introducing* commit is
  the last line, since GitHub lists newest first. A later edit is not the
  landing.

The default branch is asked for rather than assumed to be `main`
(`gh repo view --json nameWithOwner,defaultBranchRef`), because a
consumer's repository need not agree with this one.

**Failure modes, stated rather than discovered later.**

- *404 is detected by matching `gh`'s stderr for `HTTP 404`.* `gh`
  exposes no exit code separating "not found" from any other API
  failure, so this is string matching on an external tool's output. It is
  the same technique `github_repo` already uses, and the match is
  deliberately narrow: mistaking a real failure for a clean "not there"
  is the dangerous direction, while the reverse merely refuses during an
  outage. Verified against the live API — `gh: Not Found (HTTP 404)`.
- *`--paginate` costs one request per 100 commits touching the path.* An
  archaeology file has one or two. A heavily-rewritten file would cost
  more, for a number nobody reads closely.
- *Both calls trust GitHub's view.* If the API is wrong or lagging, so is
  the answer. There is no second opinion, and adding one would be a
  cathedral.

**For [[tsk_01KYZXTN3AMPNJ482J4Q13ACTW|task 58]], the datum that matters:
proving landing locally turned out to be cheap and authoritative, not
awkward.** The sprint charter anticipated the opposite — that a
post-merge workflow would know for free what the operator's machine must
go and find out, and that this might be the workflow *buying
correctness*. It does not. The workflow's advantage here is roughly two
API calls, on a command that was making API calls anyway. That leaves
automation with the convenience argument alone, which is a materially
weaker case than the charter allowed for, and task 58 should not credit
it with a correctness benefit it does not have.

### A new error category earned its place

`Error::PreconditionUnmet` (code `precondition-unmet`, exit 12): the
command is well-formed, the repository is fine, and the integration
works — the world simply has not reached the required state, and
retrying later can succeed with nothing repaired. That is true of no
other category here, and it is the exact distinction `IntegrationUnavailable`
already draws one axis over, so it was built as its sibling rather than
overloaded onto `InvalidInvocation`.

It carries the refusals a script most needs to tell apart:

| Situation | Code | Exit |
|---|---|---|
| issue is not a proposal | `invalid-invocation` | 2 |
| no realizing artifact on this branch | `precondition-unmet` | 12 |
| artifact realized but not landed | `precondition-unmet` | 12 |

Honest limit: the two `precondition-unmet` cases are distinguished by
their `requirement` text, not by code. Splitting them further would mean
one error variant per refusal, which is not a trade worth making for a
distinction a caller can also draw by checking whether the artifact
exists.

### Decisions worth not rediscovering

- **The comment goes first, then the close.** That ordering is the whole
  recovery story: a run that dies between them leaves a commented, open
  issue, and the next run sees its own marker, skips the comment, and
  closes. No reachable state has a proposal closed without its
  explanation.
- **Idempotency keys off an HTML marker in the comment**
  (`<!-- scarp:reconciled -->`), not prose matching. It lives on GitHub
  rather than in canonical state on purpose: the artifact learns nothing
  from being reconciled, so there is nothing to record on the Scarp side
  — which also honours idea 40's own objection to opaque front matter
  added merely to make automation convenient.
- **An already-closed proposal is a no-op, not an error**, whatever
  closed it. A human who closed a proposal as unwanted has settled it;
  reopening to reconcile would be the synchronization this design
  refuses.
- **The number is required and singular.** A batch form was declined: one
  invocation, one public irreversible act. It also keeps the automation
  question honestly in task 58's hands rather than half-answered here.
- **The third verb was argued, not assumed.** Task 54 set two as the
  target. `list` already reports realization but can do nothing about it,
  and folding this into `realize` is barred by the landing invariant —
  at realization time the artifact is on exactly one disk.

### Verified

Live, against this repository:

- **The refusal.** `scarp proposal reconcile 3` refuses with
  `precondition-unmet`, exit 12, naming idea 40's path and saying to push
  — the real state, since idea 40 is committed locally and pushed
  nowhere.
- **The landing proof.** The exact calls `landed` makes were run by hand
  against idea 38: present on `main`, one commit, `ee8c611`.
- **The parse shapes.** `gh issue view --json
  number,url,state,labels,comments` returns issue 2 as `OPEN`, labelled
  `idea`, zero comments — so `plan` would reach `CommentAndClose`.

**Not verified: the comment-and-close path itself.** Exercising it means
posting publicly and closing a real issue, which is
[[tsk_01KYZXTN71EDDR370MD3F00CK9|task 60]] and needs Henry's go-ahead.
Everything up to the side effect is confirmed against live state; the
side effect is confirmed only by tests. Stated plainly rather than
rounded up, as sprint 10's retrospective did for its own unexercised
half.

Offline: 12 new tests, all of `plan`'s refusal ordering, the introducing-
commit rule, 404 classification, and the comment's content and marker.
The decision logic was extracted into a pure `plan` function specifically
so the ordering that keeps a public claim honest is testable without a
network or a repository.

### Friction

[[ide_01KYZY233Z7GAKFPFSKEAF89ZD|Idea 41]] confirmed itself immediately:
this `## Result` section had to be hand-added, because `new task` owns
the template and `doctor` validates no sections at all. Three artifacts
into the sprint, the finding has already recurred three times.
