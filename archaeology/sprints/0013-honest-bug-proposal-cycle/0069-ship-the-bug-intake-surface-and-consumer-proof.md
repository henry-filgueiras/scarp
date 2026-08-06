---
id: tsk_01KZCCE4S9HKW1D6FK9WV6EY2V
sequence: 69
kind: task
status: pending
sprint: spr_01KZCCCPGW3V959HBDTZC56JAE
created: 2026-08-06
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
