---
id: spr_01KYTS35VYCFQ6BJN7HD2F0TK7
sequence: 9
kind: sprint
status: active
created: 2026-07-30
---

# Post-release remediation

## Goal

Hold the defects that `0.1.0` shipped with, and the follow-up work that
publication itself created, until they are fixed or deliberately
declined.

## Rationale

This sprint exists because [[spr_01KYFRWF0B8QKN89NHVKQG2TQT|sprint 8]]
closed with a published, immutable `0.1.0` and one known cosmetic defect
in it. A version that cannot be amended needs somewhere for its
remediation to live: `cargo publish` cannot be undone, so every defect
found after the fact costs a version number rather than a correction.

It was opened during [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]'s
closure, not planned in advance, and its scope is deliberately narrow
at creation. Sprint 8's own success criteria are all met; nothing here
reopens them. This is the container for what publication revealed, and
its shape beyond the first task is Henry's to set.

Opening it was a choice with a cost, recorded so it is not mistaken for
housekeeping: a task needs an owning sprint, and `scarp close sprint`
refuses while any task is pending, so the alternative was leaving the
defect tracked only inside a closed task's Result. That would have made
the record depend on someone reading prose rather than on a listable
pending artifact.

## Success criteria

- Every defect `0.1.0` shipped with is either repaired in a released
  `0.1.1` or explicitly declined with a recorded reason.
- No remediation silently rewrites the `0.1.0` record. `0.1.0` stays in
  the archaeology with its defects stated, per
  [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]'s contingency.

## Non-goals

- Yanking `0.1.0`. Its one known defect is cosmetic, and the contingency
  fixed in advance reserves yanking for security defects, destructive
  behaviour, an unusable package, or a materially harmful contract
  defect.
- Replacing or republishing `0.1.0`, or moving the `v0.1.0` tag. Neither
  is possible honestly, and both are barred by the same contingency.
- A release-automation cathedral. Sprint 8's non-goal survives; the
  recurring form of a hand-performed release remains a chore ledger
  ([[idea-chore-artifacts|idea 7]]), not a script.
- The standing bootstrap non-goals: daemon, watcher, index, embeddings,
  MCP, GraphQL.
