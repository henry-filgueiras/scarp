---
id: spr_01KZCCCPGW3V959HBDTZC56JAE
sequence: 13
kind: sprint
status: active
created: 2026-08-06
---

# Honest bug proposal cycle

## Goal

A GitHub issue labeled `bug` can become bounded investigative work — a
pending maintenance item, or a pending task in an active sprint — while
public reconciliation happens only after the canonical default branch
proves that work reached its terminal result.

## Rationale

[[ide_01KZC769HTAF6F7GDDZW4HQGH7|Promote bug reports from GitHub issues into maintenance items or sprint tasks]] parked two load-bearing questions and refused to guess at
either. Both are now adjudicated, and the adjudication is what makes this
sprint buildable rather than a second round of design.

**Promotion means "worth investigating", not "confirmed".** Realizing a
`bug` is the operator accepting an obligation to look, not an assertion
that the reporter's diagnosis is true. That is why the generated title is
`Investigate reported behavior: <issue title>` rather than the report's
own words repeated as though the project agreed with them.

**A report that turns out not to be a bug already has an honest terminal
state.** The idea worried that closing a non-bug as `closed` would be a
false statement in the archaeology. It is not, because maintenance's
`Result` carries the finding: working as intended, unreproducible, a
duplicate, already handled, or consciously declined are all things a
`Result` can say plainly, and all of them are true statements about work
that is over. `cancelled` and `withdrawn` would add vocabulary without
adding honesty, so this sprint adds neither.

The asymmetry the idea identified survives both answers, and shapes the
second task. An idea *is* the deliverable, so a realized idea's mere
existence on the default branch is what the filer was waiting for. A bug
reporter is waiting for an outcome. Reconciling a bug at creation would
close their issue to announce that a tracking item exists, which is worse
than saying nothing. So bug reconciliation is gated on the remote
artifact's terminal state, proven by reading the default branch's copy
rather than trusting a local status, a remote-tracking ref, or the mere
existence of a path.

There is also a plain defect to repair on the way. `proposal realize`
never fetches labels, so today any issue number at all can be realized as
an idea — including one that is not a proposal. Classification fixes it
as a side effect of needing to exist.

## Success criteria

- `scarp proposal realize N` picks its target collection from the issue's
  labels: `idea` realizes a parked idea exactly as it does today, `bug`
  realizes a pending maintenance item, and `bug --sprint sprint:X`
  realizes a pending task owned by that active sprint.
- Neither recognized label, both recognized labels, and `idea --sprint`
  are refused with typed errors before a sequence is allocated or any
  path is touched.
- One proposal URL realizes at most one canonical artifact across ideas,
  maintenance, and tasks.
- `scarp proposal list` discovers the union of open `idea` and `bug`
  issues, states the default target for each, and refuses rather than
  guessing when an issue carries both labels.
- `scarp proposal reconcile N` closes a bug-derived proposal only after
  fetching the default branch's copy of the realizing artifact and
  proving its stable id, kind, `proposal:` URL, and `status: closed`; the
  commit it cites is proven to contain that terminal state.
- The reconciliation comment says the work reached its terminal result
  and never says it was fixed.
- `.github/ISSUE_TEMPLATE/bug.yml` exists, applies the `bug` label, and
  tells a reporter that filing is a report rather than canonical state.
- `docs/remote-proposals.md`, the README command surface, and the release
  runbook's shipped-surface verification describe both lifecycles.

## Non-goals

- No provider registry, forge abstraction, or generalized many-to-many
  label framework. Two explicit source classes is the whole
  generalization.
- No triage automation, severity, priority, assignment, or auto-labelling.
- No new lifecycle state. Maintenance stays `pending -> closed`.
- No synchronization. The issue is never canonical, nothing mirrors state
  back, and closing or deleting one invalidates nothing.
- No HTTP client, token storage, Actions workflow, or unattended run;
  `gh` stays shelled out from a machine that already holds the authority
  to commit.
- No specimen issue filed or closed during this sprint. The live
  end-to-end path is dogfooded afterwards, deliberately.
- No release. Publishing 0.3.0 remains [[mnt_01KZA6MH5SCW0MDEJTKKW26Y9G|Publish 0.3.0 to crates.io]]'s work, and this
  sprint only keeps its public-API inventory and runbook honest.
