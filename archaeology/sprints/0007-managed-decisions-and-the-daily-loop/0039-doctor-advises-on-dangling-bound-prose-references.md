---
id: tsk_01KYE2F91HWB3FME62E3EE73DK
sequence: 39
kind: task
status: closed
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-25
closed: 2026-07-25
---

# Doctor advises on dangling bound prose references

## Objective

Close the unsupervised-run gap named in this sprint's tablestakes
review: a `[[stable-id|label]]` marker in body prose with a
fat-fingered id is silent today — no check at write time, none from
doctor — and only surfaces when a human follows the link. Implement
the minimal slice of [[idea-doctor-reference-graph|idea 2]]: doctor
scans managed artifacts' bodies for bound markers and reports a
target no artifact claims.

Severity is pinned by decision 10, which defers prose-marker checking
to idea 2 and holds that "dangling *untyped* markers remain
diagnostic at most": this is an `advice` finding with problem code
`dangling-reference`, distinct from the error-tier `dangling-edge` on
typed front-matter edges. It never fails validation; promotion to a
failing tier is [[idea-strict-doctor|idea 13]]'s question, not this
task's.

Scope boundaries, per the same decision: sugar references
(`[[kind:N]]`) stay unchecked — they are legal-but-weak by
construction and their verification surface is idea 1's
`links bind --check`. A marker targeting a multiply-claimed id gets
no per-marker finding: the duplicate id is already its own error.
Label/target consistency stays parked with idea 2's addendum.
Fenced code blocks never contain markers (decision 10), mirroring
title extraction — and this scan also skips inline code spans
(backtick-run pairing), a diagnostic-layer choice decision 10 leaves
open: the first corpus run surfaced five grammar examples like
`[[stable-id|label]]` written in backticks (dragon 3, tasks 25, 38,
39) that are self-evidently mentions, not references. The grammar
module is untouched; only the advice scan reads code spans as
non-prose. The verification universe is the identity claimant
catalog, so prose may cite unmanaged decisions, comments, and logs.

## Acceptance criteria

- A managed artifact whose body carries a bound marker targeting an
  id no artifact claims yields one `dangling-reference` advice
  finding naming the artifact's path and the missing id; the
  repository stays healthy and doctor's exit stays zero.
- A bound marker resolving to any harvested claimant — including an
  unmanaged decision — yields no finding.
- Sugar markers, markers inside fenced code blocks or inline code
  spans, and non-marker `[[...]]` text yield no finding.
- Several markers on one line are each extracted; repeated citations
  of one missing id in one artifact produce one finding, not one per
  occurrence.
- The current corpus is clean: `strata doctor` over this repository
  reports no new findings.
- Tests cover the dangling case, the unmanaged-target case, fence
  skipping, multiple markers per line, per-file dedup, and sugar
  exclusion.
- `scripts/check.sh` passes.

## Result

`edges::check_prose` scans cleanly parsed managed artifacts' bodies
in doctor's existing per-artifact pass: bound markers are extracted
line by line (non-greedy to the next `]]`, which is exact for
targets, since a target ends at the first `|` or the close), resolved
against the claimant catalog, and each missing id yields one
`dangling-reference` advice finding per artifact. Sugar, ambiguous
targets, fenced blocks, and non-marker text are excluded as specced;
`healthy()` is untouched, so the future check-gate (task 33) ignores
these by construction.

The first corpus run earned its keep immediately by finding the
spec's own blind spot: five advisories, every one a grammar example
like `[[stable-id|label]]` written inside inline backticks (dragon 3,
tasks 25, 38, 39) — mentions, not references. Resolution: the advice
scan skips inline code spans under CommonMark backtick-run pairing
(`outside_code_spans`; an unpaired backtick hides nothing), the spec
was amended before closure, and no historical artifact was edited to
appease the new check. Decision 10's grammar is untouched — this is
diagnostic-layer behavior only — but the mention-versus-reference
question will recur at idea 1's bind surface, and this scan is the
precedent to consult.

Corpus verdict after the fix: 79 artifacts, no problems, no
advisories. Doctor now structurally covers the last unchecked leg of
the tablestakes review: typed edges (errors), prose references
(advice), identities, sequences, and placement.
