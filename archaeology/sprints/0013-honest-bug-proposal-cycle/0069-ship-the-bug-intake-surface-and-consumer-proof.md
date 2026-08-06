---
id: tsk_01KZCCE4S9HKW1D6FK9WV6EY2V
sequence: 69
kind: task
status: closed
sprint: spr_01KZCCCPGW3V959HBDTZC56JAE
created: 2026-08-06
closed: 2026-08-06
---

# Ship the bug intake surface and consumer proof

## Objective

Give the bug half of the cycle its intake surface, its documentation, and
consumer-visible proof, and bring the release accounting back in line
with the public API this sprint changed.

## Acceptance criteria

- `.github/ISSUE_TEMPLATE/bug.yml` exists and applies the `bug` label.
- The form explains that filing is a report — not canonical project
  state, and not confirmation that a defect exists — and that a
  maintainer may promote it into an investigative maintenance item or
  sprint task.
- Its title field suits later prefixing, with no redundant `[Bug]`
  prefix, because Scarp generates
  `Investigate reported behavior: <issue title>`.
- It collects observed behavior, expected behavior, reproduction and
  evidence, environment, and optional context, and does not ask an
  outside contributor to write Scarp-specific acceptance criteria.
- The repository's labels are checked explicitly with
  `gh label list --repo henry-filgueiras/scarp`, with the exact command
  and its result recorded here. The `bug` label is created through `gh`
  only if it is absent; that would be the one live GitHub mutation
  authorized in this round. No specimen issue is created or closed —
  the live end-to-end path is dogfooded separately.
- `docs/remote-proposals.md` documents both lifecycles: the commands, the
  classification and refusal table, the authority boundary for promoting
  a stranger's report into work the project owes, and terminal-result
  reconciliation.
- The README command surface reflects `proposal realize --sprint` and the
  two source classes.
- [[mnt_01KZA6MH5SCW0MDEJTKKW26Y9G|Publish 0.3.0 to crates.io]] carries the complete public API-break inventory,
  including everything this sprint added.
- The release runbook's shipped-surface verification proves the new
  `proposal realize --sprint` surface from an installed 0.3.0, without
  depending on this checkout.
- Consumer-visible, hermetic CLI proof exists for the whole cycle —
  list, realize, land-state check, reconcile — covering both the
  maintenance target and the sprint-task target. The evidence standard is
  preserved: if only the fake `gh` harness was exercised, the result says
  so and does not claim the GitHub half was live-tested.
- [[ide_01KZC769HTAF6F7GDDZW4HQGH7|Promote bug reports from GitHub issues into maintenance items or sprint tasks]] is adopted, and only after the whole described path exists.
  Existing provenance and transition conventions are used; no new
  lifecycle decision is minted merely to restate the already-documented
  maintenance contract.
- WitnessGlass [[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]] is untouched: no retrospective issue is
  fabricated for it and no `proposal:` stamp is added to it.
- `scripts/check.sh` passes and the slice is committed with its
  archaeology.

## Result

Done. The bug half has an intake surface, documentation, consumer proof,
and the release accounting it perturbed.

## Intake

`.github/ISSUE_TEMPLATE/bug.yml` applies the `bug` label and collects
observed behavior, expected behavior, reproduction and evidence,
environment, and optional context. The three that matter are required.

Its markdown preamble does the work the form itself cannot. It says
filing is a report rather than project state or a confirmation that a
defect exists; that a maintainer may promote it into an investigative
maintenance item or a sprint task titled *Investigate reported behavior:
&lt;your title&gt;*; that the investigation may end in a confirmed defect,
in intended behavior, in something unreproducible, in a duplicate, or in
a decision not to act; and that a promoted issue stays open until that
work reaches its terminal result on the default branch. It asks for a
title naming what went wrong and says explicitly not to add `[Bug]`,
because Scarp adds its own prefix. It asks nothing about acceptance
criteria — those are generated, because an outside reporter has no way to
know what this project considers done. "If you could not reproduce it a
second time, say that; it is useful information, not a
disqualification" is deliberate: the form should not select for
overconfident reports.

**`.github/ISSUE_TEMPLATE/bug-report.md` was removed in the same
change.** It was a legacy Markdown template that also applied `bug`, so
leaving it would have offered a contributor two bug forms with different
shapes — and its `## What happened` headings would now be demoted to
`###` on promotion, which reads oddly beside a form whose headings were
designed to be nested. One intake per class.

## The label, checked rather than assumed

```console
$ gh label list --repo henry-filgueiras/scarp
bug	Something isn't working	#d73a4a
documentation	Improvements or additions to documentation	#0075ca
duplicate	This issue or pull request already exists	#cfd3d7
enhancement	New feature or request	#a2eeef
good first issue	Good for newcomers	#7057ff
help wanted	Extra attention is needed	#008672
invalid	This doesn't seem right	#e4e669
question	Further information is requested	#d876e3
wontfix	This will not be worked on	#ffffff
idea	Uncommitted proposals to explore	#7057FF
```

`bug` already exists as one of GitHub's repository defaults, so **no
label was created and no live GitHub mutation was performed in this
round.** The one that was authorized turned out to be unnecessary.

No specimen issue was filed or closed. The live end-to-end path is
Henry's to dogfood.

## Documentation

`docs/remote-proposals.md` gained the classification table up front, a
two-branch shape diagram, a rewritten *Why ideas went first, and what
bugs had to earn* section carrying the adjudication — promotion accepts
an obligation to investigate, and the `Result` rather than the status
carries the finding, which is why no `cancelled` state was added — and a
*Reconciling a bug is gated harder* section naming the four cheaper
checks that are deliberately not used. The refusal table went from four
rows to ten. The setup recipe now installs both forms and shows the
`list` output stating each proposal's target.

Its evidence paragraph was narrowed rather than extended: the idea half
has been exercised live against GitHub, the bug half has not, and the
page now says so in those words.

The README command row covers `realize N [--sprint sprint:X]` and both
source classes.

## Release accounting

[[mnt_01KZA6MH5SCW0MDEJTKKW26Y9G|Publish 0.3.0 to crates.io]]'s inventory gained the `sprint` field on
`cli::ProposalCommand::Realize`, the `target` field on
`proposal::ProposalSummary`, the third parameter on
`proposal::realize`, and a note that `create_maintenance_from` and
`create_task_from` are additive. Its command-surface table gained the
`proposal realize --sprint` row, and its shipped-surface obligation now
names that flag. The version conclusion is unchanged and the item stays
`pending`; nothing here published, bumped, tagged, or released anything.

`docs/release-runbook.md`'s `verify-shipped-surfaces` cell now proves,
against a binary installed from crates.io, that `proposal realize --help`
mentions `--sprint`, and that `proposal list` refuses with no `gh` on
`PATH` — which doubles as proof that every ordinary command above it ran
without one. `--help` is the whole check available there: exercising the
surface for real needs an authenticated `gh` and a live issue, which is
dogfooding rather than a release gate, and the cell says so. Verified by
`runme run --dry-run`, which renders it at the repository root under
`#!/bin/bash`; `bash -n` accepts the rendered script.

## Consumer proof

`tests/proposal_cycle.rs` — 28 tests through the compiled binary against
a fake `gh` — covers the whole cycle in both directions and for both bug
targets: list, realize, the land-state check, and reconcile, for a
maintenance item and for a sprint task.

Its module doc states what it does and does not prove, and the same
sentence is in the docs and in this Result: **the GitHub half of the bug
cycle has not been live-tested.** The harness proves Scarp builds the
invocations it intends to build and never mutates an issue on an
unproven claim. It proves nothing about how GitHub answers them, and it
must not be cited as though it were a dated performance.

## Idea 43

Adopted, after the described path existed rather than before, with
`adopted-by` naming [[tsk_01KZCCE4MCVWXVWXPCQ3NNHVJ7|Classify and realize bug proposals]] — the task that realized its sketch. No
new decision was minted: the lifecycle question it raised was answered by
the maintenance contract already recorded in `read::MAINTENANCE`'s own
documentation, and restating that in a decision would be ceremony.

WitnessGlass [[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]] is untouched — no fabricated issue, no
`proposal:` stamp.

## Verification

`scripts/check.sh` — passes. 284 lib tests, 28 proposal-cycle tests,
doctor 153 artifacts clean. `git diff --check` clean.
