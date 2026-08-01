---
id: tsk_01KYX1WHPS3R7FDCKG23YTGHHY
sequence: 48
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
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
