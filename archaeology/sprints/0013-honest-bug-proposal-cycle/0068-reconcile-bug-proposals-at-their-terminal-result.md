---
id: tsk_01KZCCE4PDVWPMM6VY1XC0X3FJ
sequence: 68
kind: task
status: pending
sprint: spr_01KZCCCPGW3V959HBDTZC56JAE
created: 2026-08-06
---

# Reconcile bug proposals at their terminal result

## Objective

Generalize proposal lookup and reconciliation across ideas, maintenance,
and tasks, and gate the bug-derived half on a stronger invariant than the
one ideas need.

For an idea, default-branch existence remains sufficient: the idea is the
deliverable. For a maintenance item or a task, the filer is waiting on an
outcome, so reconciliation must prove the work reached its terminal
result before saying anything public.

The invariant is deliberately stronger than "the path exists remotely".

## Acceptance criteria

- Idea reconciliation is unchanged: creation-aware, citing the commit
  that introduced the artifact.
- For a maintenance item or task, reconciliation fetches the remote
  default-branch artifact, parses it, and proves the matching stable id,
  `kind`, `proposal:` URL, and `status: closed` are all present.
- A locally closed artifact whose remote copy is still pending refuses
  with `precondition-unmet` / exit 12, and neither comments nor closes.
- Remote contents are authoritative. The local status, a remote-tracking
  ref, a substring search, and mere path existence are each insufficient
  and none of them is relied on.
- The permalink and cited commit for a bug point to a revision proven to
  contain the terminal state, not merely the commit that introduced the
  pending file.
- Invalid, mismatched, absent, or nonterminal remote contents never
  produce a GitHub mutation.
- Reconciliation enforces the same exactly-one-recognized-label rule as
  realization.
- The recovery ordering is preserved: prove canonical remote state, then
  comment unless the reconciliation marker is already present, then
  close. A crash after commenting stays recoverable without posting
  twice, an already-closed recognized issue remains a no-op, and nothing
  is ever reopened or synchronized.
- The bug comment reads well cold: it cites the reference, stable id,
  pinned artifact path, and the proven terminal commit, and says the work
  "reached its terminal result". It never says "fixed", because the
  `Result` may conclude that no defect existed.
- Tests cover unchanged idea reconciliation; pending remote maintenance
  and pending remote task; local closed with remote pending; remote
  closed with matching identity and provenance; remote content mismatch
  and malformed front matter; comment-before-close ordering;
  commented-but-open recovery; already-closed idempotence; both and
  neither recognized label; wording that claims a terminal result but not
  a fix; and a pinned commit that actually contains the terminal state.
- Tests use the existing pure-planning seam and a fake `gh` where a
  process boundary is needed. No unit test depends on the network.
- `scripts/check.sh` passes and the slice is committed with its
  archaeology.
