---
id: tsk_01KYX1WHPS3R7FDCKG23YTGHHY
sequence: 48
kind: task
status: closed
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
closed: 2026-08-01
---

# Research the GitHub Actions trust model for issue-triggered mutation

## Objective

Establish, against primary sources and against this repository's live
settings, what it actually takes to let a GitHub issue safely cause a
repository mutation, and specify the exact mechanism the workflow will
use.

Two answers are already fixed by owner direction (2026-07-31) and are
not reopened here: **authorization is a live repository-permission
check**, requiring write access or better, rather than a login
allowlist; and **a human merges the pull request**, so no auto-merge
path needs a credential. This task specifies how to implement the
first correctly and proves it fails closed. The research that remains
is everything that choice does not settle.

Research and recommendation only. No workflow file, no issue form, no
settings change, no decision artifact. The recommendation feeds
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s adjudication and, if that
passes, [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]]'s implementation.

The threat model is not hypothetical. `henry-filgueiras/scarp` is
public and has issues enabled, so **anyone with a GitHub account can
fire the trigger event**. Evaluate every question below on the
assumption that the issue author is hostile, the issue body is
attacker-chosen, and the attacker can read this repository's workflow
files.

## Questions to settle

**Trigger semantics.** Which event should carry the proposal —
`issues: opened`, `issues: labeled`, `issue_comment`, or another — and
for each, who can fire it and how often? Note whether an `edited` or
re-`labeled` event can replay a mutation that already happened, and
what idempotency that implies. Confirm which ref each candidate trigger
reads the workflow definition from, and what that means for a proposal
filed from a fork.

**Token identity and its consequences.** Compare the identities
available to the job: the default `GITHUB_TOKEN`, a fine-grained
personal access token, a deploy key, and a GitHub App installation
token. For each: what it can do, how it is stored, what happens when it
leaks, and its blast radius.

One interaction remains load-bearing even after auto-merge was
declined. Events created using `GITHUB_TOKEN` are reported not to start
new workflow runs — which would mean a pull request opened by the
default token shows **no checks at all**, so a human tapping merge
merges an unreviewed-by-CI diff. **Verify this against primary
documentation and state plainly whether it holds.**

If it holds, the intended answer is that the proposal workflow runs
`scripts/check.sh` and `scarp doctor` itself, before opening the pull
request. Three questions then have to be answered separately rather
than collapsed, because the sprint's language depends on the
distinction:

1. **Did validation run against the exact resulting state?** This is
   the invariant that matters and it is satisfied by ordering alone.
2. **Does GitHub expose a check run on the pull request's head SHA?**
   Determine this as observed fact — what a human actually sees on the
   pull request page — not as an inference from having run checks. If
   the answer is no, the sprint must stop saying the pull request
   "carries a green check" and say instead that the realization run is
   durably linked from it.
3. **Does `ci.yml` run on the `push` to `main` after a human merges?**
   The merge is performed by a human, not by `GITHUB_TOKEN`, so the
   suppression may not apply — but that is a hypothesis to verify, not
   a deduction to rely on. Record the answer as fact, since it is the
   difference between a deferred check and a permanent gap in coverage.

Also establish, without designing anything: whether the Check Runs API
would let the workflow publish a real check against the head SHA, what
permission that needs, and roughly what it would cost. This is
reconnaissance so
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] can take it only if it falls
naturally out of work already there. It is explicitly not a
recommendation to build it, and the sprint must not widen to make a
sentence true.

**Delivery guarantees.** Establish what GitHub actually promises about
workflow invocation: whether an event can be delivered more than once,
what a re-run does to `github.event`, whether re-running a failed job
replays the original payload or a fresh one, and what happens to an
in-flight run when the triggering issue is edited. The replay model in
task 54 is built on these answers, so guesses are not acceptable
substitutes. Note especially anything that makes exactly-once
invocation *not* guaranteed, since that is the assumption the
idempotency work exists to avoid.

**Authorization mechanics.** The model is fixed; the mechanics are not.
Determine the exact API call that establishes whether an actor has
write access, what it returns for an outside contributor, an org
member with indirect access, and a user who does not exist. Establish
whether the check must be re-evaluated at mutation time rather than
only at trigger time, and what happens if permission is revoked between
the two. Note how the same call behaves in a consumer's repository with
teams and an organization, since the workflow is meant to be copied
unchanged.

**Untrusted input.** Enumerate the injection surfaces this design
creates. At minimum: issue title and body interpolated into a `run:`
block through `${{ }}`; a title chosen to produce a hostile slug, a
path traversal, or a leading `-` that Scarp's argument parser reads as
a flag; control characters, extreme length, and non-UTF-8; body content
chosen to forge front matter or extra Markdown sections. Give each a
named mitigation and an owner — GitHub's `env:`-and-quote discipline,
an intermediate file, or Scarp's own input validation. State explicitly
which mitigations must live in Scarp because a workflow cannot be
trusted to hold them.

**Least privilege.** Recommend the exact `permissions:` block, per job
rather than per workflow, with what each grant is for, and reject
anything not required.

**Reachability of the failure path.** Establish how a refusal reaches
the person who filed the proposal, who may not be able to read Actions
logs. Compare commenting on the issue, labelling it, and closing it,
with the permission each needs.

## Acceptance criteria

- Every behavioural claim about GitHub is dated and attributed to a
  primary source — GitHub documentation, the REST API, or an
  observation against this repository — with verified fact, inference,
  and judgment distinguished as in
  [[tsk_01KYFRWF1X37N5TBJ139X7ZKA1|task 40]]'s Result.
- The `GITHUB_TOKEN`-does-not-trigger-workflows question is answered
  definitively, with its citation.
- The three check-visibility questions are answered separately and each
  labelled as observed fact or inference: validation ran against the
  resulting state; GitHub does or does not expose a check run on the
  head SHA; `ci.yml` does or does not run on the post-merge push.
  Where practical the second and third are confirmed by observation
  against a real pull request rather than from documentation.
- The Check Runs API option is described with its permission and rough
  cost, explicitly as reconnaissance rather than a recommendation.
- GitHub's delivery guarantees are established: whether an event may be
  delivered more than once, what a job re-run replays, and what happens
  to an in-flight run when the issue is edited. Anything that makes
  exactly-once invocation unguaranteed is stated plainly, since
  [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]]'s replay model depends on
  it.
- One authentication model is recommended, with at least two rejected
  alternatives and the reason for each rejection. The default
  `GITHUB_TOKEN` is expected to suffice now that auto-merge is out of
  scope; if it does not, say so, because that reopens a fork Henry
  already closed.
- The exact permission-check call is specified, with its behaviour for
  an outside contributor, an indirect org member, and a nonexistent
  user recorded from real responses rather than from documentation
  alone.
- The model fails closed: the Result states what happens when the token
  is missing, the permission API errors or rate-limits, and the actor
  cannot be resolved, and confirms that none of those paths mutate
  anything.
- The injection inventory is concrete rather than a gesture at
  "sanitize input": each surface has a named mitigation and an owner.
  Mitigations that must live in Scarp are handed to
  [[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task 51]] as requirements.
- The exact `permissions:` block is written out.
- Every recommendation is checked for whether it survives being copied
  into a repository that is not this one. Anything that depends on this
  repository having a single maintainer, or on the workflow living
  beside the Scarp source, is flagged as a portability defect rather
  than left implicit.
- Live repository facts are re-verified at research time rather than
  taken from this task's text.
- No workflow, form, settings change, or decision artifact is produced.
- The Result is useful standalone, without this task's originating
  conversation.

## Result

All research performed 2026-08-01. Epistemic labels: **[V]** verified
against a primary source that day — GitHub documentation quoted, or a
live API call against `henry-filgueiras/scarp`; **[I]** reasonable
inference from verified facts; **[A]** judgment.

Two findings change the sprint's design. One reverses an assumption the
sprint was built on; the other is a security trap that a plausible
implementation walks straight into.

### Headline 1: the GITHUB_TOKEN assumption was wrong

The sprint assumed a pull request opened with `GITHUB_TOKEN` would
trigger nothing, leaving it with no checks. **That is not what GitHub
does.** The general rule holds, but `pull_request` is an exception.

**[V]** The general rule, quoted: "When you use the repository's
`GITHUB_TOKEN` to perform tasks, events triggered by the `GITHUB_TOKEN`
will not create a new workflow run, with the following exceptions". Its
rationale is also quoted: "this behavior prevents you from accidentally
creating recursive workflow runs."

**[V]** The exceptions are `workflow_dispatch` and `repository_dispatch`,
**and `pull_request` events with the `opened`, `synchronize`, or
`reopened` activity types**. On that last one: "when a workflow using
`GITHUB_TOKEN` creates or updates a pull request, the resulting
`pull_request` event creates workflow runs in an **approval-required**
state." The stated purpose is to "prevent recursive workflow runs while
still allowing CI workflows to run on pull requests created by
automation." Other activity types — `labeled`, `edited`, `closed` — do
not create runs.

Consequences, in order of importance:

1. **`ci.yml` is triggered by the proposal pull request** **[V]**, so
   the pull request does get check runs associated with its head SHA.
   The sprint's original wording was closer to right than the amendment
   assumed, and the amendment's caution was still correct to demand
   verification rather than either guess.
2. **The run does not execute until approved** **[V]**. So what a
   reader sees on the pull request is a workflow run *awaiting
   approval*, not a green check. "Carries a green check" remains the
   wrong claim, for a different reason than the one recorded on
   2026-08-01.
3. **This does not make the channel two mandatory taps** **[I]**. `main`
   has no branch protection and no required checks, so nothing blocks
   the merge. The inline validation has already run against the exact
   realized state before the branch was pushed, which is the invariant
   that matters. Approving the `ci.yml` run is available and
   informative, not required.
4. **Post-merge coverage is intact** **[I]**, and by a simpler route
   than the exception: the merge is performed by Henry, not by
   `GITHUB_TOKEN`, so the resulting `push` to `main` is an ordinary
   human-triggered event and `ci.yml` runs on it normally. No exception
   is needed for this case.

Honest limit: 2 and 3 are documentation-derived, not observed. Confirming
the approval-required state as *seen on a real pull request* requires a
workflow that opens one, and this task is forbidden from producing a
workflow. That observation is handed to
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]]'s first run rather than
faked here, and whether a repository setting can waive the approval
requirement is an open question recorded below.

### Headline 2: the permission API authorizes the entire internet if misread

**[V]** `GET /repos/{owner}/{repo}/collaborators/{username}/permission`
returns `permission` from `admin`, `write`, `read`, `none`, with the
legacy mapping documented as maintain→`write` and triage→`read`.

Live results against this public repository, 2026-08-01 **[V]**:

| actor | status | `permission` | `role_name` |
|---|---|---|---|
| `henry-filgueiras` (owner) | 200 | `admin` | `admin` |
| `octocat` (no relationship) | 200 | `read` | `read` |
| `zzz-no-such-user-9x8q7` | 404 | — | — |

**`octocat` has no relationship to this repository whatsoever and still
returns 200.** Because the repository is public, every real GitHub user
has read access, so the endpoint answers 200 for all of them. The
documentation's table — "user is not a collaborator → 404" — does not
describe public repositories, and only a *nonexistent* user 404s here.

**[I]** Therefore two plausible implementations are catastrophically
wrong: treating HTTP 200 as authorization, and treating "not 404" as
authorization. Either grants the proposal channel to anyone with a
GitHub account.

**Recommended check [A]**: require `permission` to be exactly `admin`
or `write`. This is correct rather than merely strict — the documented
legacy mapping folds `maintain` into `write` (wanted) and `triage` into
`read` (correctly excluded). Matching on `role_name` instead would have
to enumerate `maintain` explicitly and would break on custom
organization roles, which is precisely the portability the channel
needs. Do **not** compare against `none`; on a public repository it
will essentially never appear.

Fail-closed behaviour **[A]**: 404, any non-200, a missing or
unparseable `permission` field, a network error, or a rate-limit
response all refuse. Only an explicit `admin`/`write` proceeds.

### Trigger semantics

**[V]** `issues` supports `opened, edited, deleted, transferred,
pinned, unpinned, closed, reopened, assigned, unassigned, labeled,
unlabeled, locked, unlocked, milestoned, demilestoned, typed, untyped,
field_added, field_removed`.

**[V]** "This event will only trigger a workflow run if the workflow
file exists on the default branch." `GITHUB_SHA` is the last commit on
the default branch and `GITHUB_REF` is the default branch.

**[I]** This is a significant safety property and should be stated in
[[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]]'s trust boundary: the
workflow definition always comes from `main`. A proposal cannot supply
or influence the workflow that processes it, and there is no fork
attack surface here because an issue has no branch. The `pull_request`
fork hazards and the `pull_request_target` trap are simply not reachable
by this design.

**Recommended trigger [A]**: a deliberate, authorized act rather than
`issues: opened`. `labeled` is the strongest fit — anyone may file a
proposal, but realization begins only when a write-capable person
applies the realization label. This separates *proposing* from
*realizing*, which is the sprint's own vocabulary, and it gives the
snapshot semantics task 53 owes a natural anchor: the label event is
the moment prose stops being input. `opened` would realize on filing
and make every edit a question. Note **[V]** that `labeled` events
raised by `GITHUB_TOKEN` do *not* create runs, so the channel cannot
label its own way into a loop.

### Delivery guarantees and replay

**[V]** "The workflow will also use the same `GITHUB_SHA` (commit SHA)
and `GITHUB_REF` (git ref) of the original event that triggered the
workflow run."

**[V]** "Re-runs use the privileges of the actor who initially
triggered the workflow, not the privileges of the actor who initiated
the re-run."

**[I]** The second is load-bearing for authorization and vindicates the
late re-check requirement: a re-run carries the *original* actor's
privileges, so token privileges cannot be trusted to reflect current
permission state. Only a live API call at mutation time answers "is
this person still authorized", which is exactly what
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] now requires.

**[I]** The first suggests a re-run replays the original event context
rather than re-reading live state, which would give task 54's snapshot
semantics partly for free. The documentation states this for
`GITHUB_SHA`/`GITHUB_REF` only and does not say it for the whole
`github.event` payload, so **this must not be relied on**: task 54
should snapshot explicitly and treat any payload preservation as a
bonus. Recorded as an inference precisely so nobody later mistakes it
for a guarantee.

Not established: whether GitHub may deliver the same `issues` event
twice absent a re-run. No primary source found either way, which is
itself the answer — an undocumented guarantee is not a guarantee, and
task 54's replay guard should assume at-least-once delivery.

### Check Runs API — reconnaissance only

**[V]** "Write permission for the REST API to interact with checks is
only available to GitHub Apps. OAuth apps and authenticated users can
view check runs and check suites, but they are not able to create
them."

**[I]** `GITHUB_TOKEN` acts as a GitHub App installation token, so a
job granted `checks: write` can create check runs; the documentation
does not state this explicitly for Actions, so it is inference.

**Recommendation [A]: do not build it.** Headline 1 removed the reason
it was being considered — `ci.yml` is triggered, so the pull request is
not check-less. Publishing a synthetic check run to describe validation
that already happened would add a permission, add machinery, and make
the pull request assert something the workflow run already records.
That is the "inventing machinery to satisfy wording" the task was told
to avoid. The Commit Statuses API is noted in the docs as the
non-App alternative and is not needed either.

### Untrusted input inventory

Public repository, issues enabled: the title and body are chosen by an
attacker **[V]**, and the workflow file is readable by them **[V]**.

**[V]** Mitigation guidance, quoted: "For inline scripts, the preferred
approach to handling untrusted input is to set the value of the
expression to an intermediate environment variable." And on privilege:
"You should therefore make sure that the `GITHUB_TOKEN` is granted the
minimum required permissions."

| # | Surface | Mitigation | Owner |
|---|---|---|---|
| 1 | Title/body via `${{ }}` into `run:` | never interpolate; bind to `env:` and quote every expansion | workflow |
| 2 | Body written to a file for `--body-file` | write via `env:` and a heredoc with a quoted delimiter, never `echo` of an expression | workflow |
| 3 | Title beginning `-` parsed as a flag | pass `--` before positionals; Scarp must also reject or safely handle a leading-dash title | **both** |
| 4 | Title producing traversal or absolute path in the slug | Scarp owns slugging; it must never emit a path outside the collection directory | **Scarp** |
| 5 | Body forging a `---` front-matter block | Scarp must refuse or escape; a workflow cannot be trusted to detect this | **Scarp** |
| 6 | Body injecting `#` headings that forge template sections | Scarp validates section structure against the template | **Scarp** |
| 7 | Control characters, NUL, non-UTF-8 | Scarp refuses with a typed error | **Scarp** |
| 8 | Extreme length (title or body) | bound in the form and again in Scarp | **both** |
| 9 | Title colliding with an existing slug | Scarp's existing refusal-to-overwrite path | **Scarp** |
| 10 | Body containing text that looks like a realization receipt | receipt must be written where issue-body text cannot forge it | workflow |

Rows 4–7 and 9 are handed to
[[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task 51]] as requirements: a workflow
cannot hold them, because they are properties of canonical form that
only the tool that owns canonical form can enforce. Row 10 is new and
belongs to task 54's receipt design — a receipt parsed out of the issue
body is forgeable by the person who wrote the body.

### Least privilege

**[V]** Live: `default_workflow_permissions` is `read` and
`can_approve_pull_request_reviews` is `false`. Actions are enabled with
`allowed_actions: all` and `sha_pinning_required: false`.

The default is already read-only, so every write must be declared. Per
job, not per workflow:

```yaml
# Job 1 — authorize, parse, validate. Reads only; comments on refusal.
permissions:
  contents: read
  issues: write        # post the diagnostic back to the filer

# Job 2 — realize and publish.
permissions:
  contents: write      # push the proposal branch
  pull-requests: write # open the PR
  issues: write        # write the realization receipt / diagnostics
```

Rejected **[A]**: `checks: write` (see above — not building it);
`actions: write` (nothing re-runs or cancels workflows); `statuses:
write` (same reasoning as checks); any org-level or `packages`
permission. **[A]** `sha_pinning_required: false` is worth noting for
[[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]]: this workflow will consume
third-party actions, and a consumer copying it inherits whatever pinning
discipline it demonstrates.

### Failure surfacing

**[A]** Comment on the issue. It needs only `issues: write`, it is
visible to a filer who cannot read Actions logs, it survives label
removal, and it arrives as a notification. Labelling alone is
insufficient — it carries no diagnostic text and any triager can remove
it, which is the same reasoning that disqualified a label as a
realization receipt. Closing the issue is too strong for a recoverable
refusal and destroys the proposal identity task 54 depends on.

### Portability

Nothing recommended here is specific to this repository **[A]**, with
two caveats for task 55's recipe:

- The `admin`/`write` check behaves differently on a **private**
  repository, where a stranger returns 404 rather than 200/`read`. The
  recommended check is correct in both cases, but a consumer testing on
  a private repository will not reproduce the `octocat` result and must
  not conclude the check is redundant.
- Organization repositories can define custom roles **[V]**, which is
  the concrete reason to match on `permission` rather than `role_name`.

### Open questions handed on

1. Whether a repository setting waives the approval-required state for
   `GITHUB_TOKEN`-created pull requests. Not found via the API surfaces
   queried; `actions/permissions/access` returned 422 with "Access
   policy only applies to internal and private repositories" **[V]**.
   → [[tsk_01KYX1WHRPEXG8Z8EBPQJRHHFH|task 49]], which owns settings
   reconnaissance.
2. Live confirmation of the approval-required state and of post-merge
   `ci.yml` execution. → task 54's first run.
3. Whether `issues` events can be delivered more than once absent a
   re-run. Undocumented; assume at-least-once. → task 54.

### Not done here, by design

No workflow, no issue form, no repository setting changed, no decision
artifact. No temporary branch or pull request was created; the two
observations that would have required one are deferred above rather
than performed.
