---
id: spr_01KZCCCPGW3V959HBDTZC56JAE
sequence: 13
kind: sprint
status: closed
created: 2026-08-06
closed: 2026-08-06
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

## Retrospective (2026-08-06)

All three tasks closed. A `bug`-labeled GitHub issue can now become
bounded investigative work, and the issue stays open until the canonical
default branch proves that work reached its terminal result.

## What the adjudication bought

[[ide_01KZC769HTAF6F7GDDZW4HQGH7|Promote bug reports from GitHub issues into maintenance items or sprint tasks]] parked two questions and refused to guess at either, which
was the right call: both answers turned out to shape the code rather than
just the prose.

**"Promotion means worth investigating, not confirmed"** is not a
disclaimer. It decides the generated title (`Investigate reported
behavior: …` rather than the reporter's claim repeated as the project's),
it decides the generated body, it decides the wording of the
reconciliation comment, and it is the reason no new lifecycle state was
needed. A design that had promoted reports as confirmed defects would
have needed `cancelled` — and would then have had to explain what
`cancelled` means when the investigation was real and only the defect was
not.

**"The `Result` carries the finding, not the status"** is what made the
existing `pending -> closed` lifecycle sufficient. It is worth stating
plainly because it generalizes: a terminal state says the work is over,
and the narrative says what happened. Adding a status per outcome would
have been vocabulary growth standing in for prose.

## Three findings worth keeping

**A defect fell out of the feature.** `proposal realize` never fetched
labels, so before this sprint any issue number at all could be realized
as an idea — a support question, someone else's bug report, a pull
request. Nothing caught it because the only proposal form applied the
only recognized label, so every issue the operator would plausibly type
was in fact an idea. Adding a second class made the assumption
observable. Generalizing a hardcoded constant is a good way to find out
what was silently depending on it.

**GitHub intersects repeated `--label` flags.** A single
`gh issue list --label idea --label bug` returns issues carrying *both* —
the ambiguous set, and empty for every ordinary case. That is wrong in
the shape that survives review: plausible, compact, and it would have
produced an empty listing that reads as "no open proposals" rather than
as a bug. Two queries and a `BTreeMap` union instead. Recorded because
the next forge integration will be tempted by the same one-liner.

**The obvious gate would have been the wrong one.** Checking the local
`status: closed` before reconciling is one field access and it is
exactly backwards: it closes a reporter's issue on the strength of what
the operator's disk believes, which is the one thing a reporter cannot
see. The invariant had to be phrased against remote *contents* — parsed,
not searched, since `status: closed` quoted inside a `Result` is not the
front matter saying so — and the commit cited had to move from the one
that introduced the file to the one proven to hold its terminal state. A
bug artifact arrives `pending`, so the introducing commit is a permalink
that contradicts the sentence beside it.

## What the sprint deliberately did not build

No provider registry and no label configuration. Two classes are
enumerated in a two-row table, and what they actually need from each
other is different semantics — `Class::creation_aware` — which a lookup
table would not have expressed. No triage, severity, priority, or
assignment. No synchronization, no new terminal state, no HTTP client, no
workflow.

## The honest gap

**The bug half has never been run against GitHub.** It is covered end to
end by 28 hermetic tests driving the compiled binary against a fake `gh`,
which prove Scarp builds the invocations it intends to and never mutates
an issue on an unproven claim — and prove nothing about how GitHub
answers them. The docs, the task result, and the test module all say so
in those words, so no later reader can mistake the harness for a
performance. Dogfooding the live path is the next thing to do, and it is
deliberately not something this sprint claimed.

Nothing here published, bumped, tagged, or released. [[mnt_01KZA6MH5SCW0MDEJTKKW26Y9G|Publish 0.3.0 to crates.io]]
remains pending, with its public API-break inventory now complete.
