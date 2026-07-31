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
repository mutation — and recommend one authentication model and one
authorization model, each with its rejected alternatives and reasons.

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

One interaction is load-bearing and breaks the whole design if missed.
Events created using `GITHUB_TOKEN` are reported not to start new
workflow runs — which would mean a pull request opened by the default
token never triggers `ci.yml`, no check ever reports, and an auto-merge
waiting on required checks waits forever. **Verify this against primary
documentation and state plainly whether it holds.** If it does, name
the identities that escape it and price each. This constraint, not
preference, then selects the token model.

**Authorization of the requester.** Compare at least: an explicit
allowlist of logins in the workflow; a live check against the
repository collaborators or permission API; `github.event.issue
.author_association`; and requiring a label only a write-capable user
can apply. Assess each on whether it can be forged, whether it survives
the requester's permissions changing, and whether it fails closed. Say
whether the check must be re-evaluated at mutation time rather than
only at trigger time.

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
  definitively, with its citation, and its consequence for auto-merge
  is stated. If it holds, the recommended escape is named and priced.
- One authentication model and one authorization model are recommended,
  each with at least two rejected alternatives and the reason for each
  rejection.
- The recommended model fails closed: the Result states what happens
  when the token is missing, the permission API errors, and the actor
  cannot be resolved, and confirms that none of those paths mutate
  anything.
- The injection inventory is concrete rather than a gesture at
  "sanitize input": each surface has a named mitigation and an owner.
  Mitigations that must live in Scarp are handed to
  [[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task 51]] as requirements.
- The exact `permissions:` block is written out.
- Live repository facts are re-verified at research time rather than
  taken from this task's text.
- No workflow, form, settings change, or decision artifact is produced.
- The Result is useful standalone, without this task's originating
  conversation.

## Result
