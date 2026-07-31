---
id: tsk_01KYX1WHRPEXG8Z8EBPQJRHHFH
sequence: 49
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
---

# Research auto-merge mechanics and the cost of protecting main

## Objective

Confirm that the one-tap merge model needs no repository governance
change, and price the auto-merge upgrade well enough that a later
sprint can reconsider it from a record rather than from scratch.

This task was originally the sprint's governance research. Henry
declined auto-merge on 2026-07-31, which removed most of it: no branch
protection, no ruleset, no `allow_auto_merge` flip, and no second
credential are needed for the channel to work. What survives is a
confirmation and a priced option.

Research only, and deliberately bounded — this is not an invitation to
build the auto-merge path speculatively.

## Starting facts

Observed against the live repository on 2026-07-31, to be re-verified
rather than trusted:

- `allow_auto_merge` is `false`.
- `main` has no branch protection (`/branches/main/protection` returns
  404) and the repository has no rulesets (`/rulesets` returns `[]`).
- `delete_branch_on_merge` is `false`.
- `allow_squash_merge` is `true`.
- The only workflow is `ci.yml`, on `push` and `pull_request`, with two
  jobs: `check` and `msrv`.

## Questions to settle

**Does the one-tap model need anything changed?** Confirm that a
workflow can create a branch, open a pull request, and have a human
merge it with the repository exactly as it is today. This is the load-
bearing confirmation: if it is wrong, the sprint's shape is wrong.

**Branch hygiene.** `delete_branch_on_merge` is `false` and this
channel creates a branch per proposal. Recommend whether to flip the
repository setting, delete the branch from the workflow, or leave
branches to accumulate, and note which choice a consumer inherits by
copying the workflow versus by changing their own settings. Whatever is
recommended is applied by
[[tsk_01KYX1WHY82P2WNW9RG5KWVGYA|task 52]].

**Merge method.** Recommend squash or merge commit for proposal pull
requests, given that the branch carries one commit and the archaeology
values legible history.

**The priced auto-merge option.** Record, without building it: whether
auto-merge requires a blocking branch-protection or ruleset requirement
to wait on; what the minimum honest requirement would be; the exact
required-check names and how they are discovered; whether rulesets or
classic protection is the right vehicle; and what protecting `main`
would cost the direct-to-`main` commit workflow CLAUDE.md describes,
including whether an owner bypass preserves it and whether such a
bypass voids the guarantee it bypasses. One page is enough — this is a
priced option, not a design.

**What one tap actually costs.** The honest counter-case for the
record: what the tap costs in practice, and whether anything makes it
worse than it sounds — notably whether two proposals in flight can
collide, since each allocates a display sequence against whatever
`main` it saw. That is [[drg_01KY169X7W0YXJ5QFV4D1MK4FB|dragon 1]]'s
branch-sequence collision arriving in production, and its mitigation
belongs to [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]], but the
frequency and severity estimate belongs here.

## Acceptance criteria

- Every claim is dated and attributed to a primary source or a live
  observation, with verified fact, inference, and judgment
  distinguished.
- The confirmation that the one-tap model needs no governance change is
  definitive. If it turns out to need one, that is escalated
  immediately rather than absorbed, because it contradicts the choice
  the sprint is built on.
- Branch hygiene and merge method are each recommended, written out as
  the exact setting or workflow step for
  [[tsk_01KYX1WHY82P2WNW9RG5KWVGYA|task 52]] to apply, and each notes
  what a consumer inherits.
- The auto-merge option is priced in about a page: what it requires,
  what it costs the human workflow, and what would count as evidence
  for revisiting it. It is not designed, and no settings are changed to
  explore it.
- The concurrent-proposal sequence-collision risk is estimated, and
  handed to [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] as a
  requirement rather than a note.
- No repository setting is changed by this task. If a temporary pull
  request or branch is created to observe behaviour, it is removed and
  the Result says so.
- The Result is useful standalone, without this task's originating
  conversation.

## Result
