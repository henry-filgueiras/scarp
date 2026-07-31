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

Determine what this repository would have to become for a proposal pull
request to merge itself after its checks pass, and what that costs the
way Henry works today. Recommend the minimum governance change that
makes the sprint's last success criterion reachable — or report that it
is not worth its price and recommend the fallback.

Research and recommendation only. No settings are changed here;
applying whatever is recommended is
[[tsk_01KYX1WHY82P2WNW9RG5KWVGYA|task 52]], after
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] adjudicates.

## Starting facts

Observed against the live repository on 2026-07-31, and to be
re-verified rather than trusted:

- `allow_auto_merge` is `false`.
- `main` has no branch protection (`/branches/main/protection` returns
  404) and the repository has no rulesets (`/rulesets` returns `[]`).
- `delete_branch_on_merge` is `false`.
- `allow_squash_merge` is `true`.
- The only workflow is `ci.yml`, on `push` and `pull_request`, with two
  jobs: `check` and `msrv`.

## Questions to settle

**Does auto-merge require something to wait on?** Auto-merge is
reported to be enablable only on a pull request that is currently
blocked by a branch protection or ruleset requirement — meaning that on
an unprotected `main`, there is nothing to auto-merge *after*, and the
API call fails. Verify this against primary documentation and, if
practical, against a real pull request in this repository. If it holds,
protecting `main` is a precondition, not a preference.

**What is the minimum blocking requirement?** If a requirement is
needed, find the smallest one that is honest: required status checks
naming the existing `ci.yml` jobs, a required review, or something
else. Record the exact check names GitHub matches on and how they are
discovered, since a required check whose name does not match anything
that reports blocks every pull request forever — including Henry's.

**Rulesets or classic branch protection?** Compare the two on
expressiveness, bypass actors, API ergonomics, and what a public
repository on a free plan actually supports. Note which of the two the
repository can use for a private-plan-gated feature, if that applies.

**What does protecting `main` cost the human workflow?** This is the
part that must not be waved through. CLAUDE.md's commit policy has
sessions commit completed slices directly, and every commit in this
repository's history landed on `main` without a pull request. State
concretely what changes: whether Henry (as owner) can still push
directly, whether a bypass actor entry preserves that, whether agent
sessions can, and whether preserving direct push undermines the
protection that makes auto-merge possible in the first place. If the
honest answer is that the repository must move to a pull-request
workflow, say so and price it rather than proposing a bypass that
quietly voids the guarantee.

**Merge method and branch hygiene.** Recommend the merge method for
proposal pull requests and whether `delete_branch_on_merge` should be
enabled, given that this channel creates a branch per proposal and
nothing currently cleans them up.

**Interaction with task 48's token finding.** If the default token
cannot trigger `ci.yml`, then required checks and auto-merge interact
directly: state the combined design that works end to end, rather than
two findings that each work alone.

**The fallback.** If auto-merge proves unreachable or its governance
price is judged too high, recommend the best alternative that still
honours the sprint's "no direct push to `main`" criterion — for example
a pull request left open for one human click. Say what is lost.

## Acceptance criteria

- Every claim is dated and attributed to a primary source or a live
  observation, with verified fact, inference, and judgment
  distinguished.
- The "does auto-merge need a blocking requirement" question is
  answered definitively, with its citation and, where practical,
  a live confirmation.
- A single minimum governance change is recommended, written out as the
  exact settings and, where applicable, the exact API payload that
  [[tsk_01KYX1WHY82P2WNW9RG5KWVGYA|task 52]] would apply.
- The cost to the existing direct-to-`main` workflow is stated
  explicitly, including whether it survives, and any proposed bypass is
  evaluated for whether it voids the guarantee it bypasses.
- The exact required-check names are recorded, along with how they were
  discovered and what happens if one is renamed later.
- The combined design with [[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|task 48]]'s
  token finding is stated as one working end-to-end path, not two
  independent recommendations.
- A fallback is recommended for the case where auto-merge is rejected,
  and what it loses is named.
- No repository setting is changed by this task. If a temporary pull
  request or branch is created to observe behaviour, it is removed and
  the Result says so.
- The Result is useful standalone, without this task's originating
  conversation.

## Result
