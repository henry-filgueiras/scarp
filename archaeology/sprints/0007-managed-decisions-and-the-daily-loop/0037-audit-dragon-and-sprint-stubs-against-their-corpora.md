---
id: tsk_01KYE0RY4P541W5BH6KNRWYH5B
sequence: 37
kind: task
status: closed
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-25
closed: 2026-07-25
---

# Audit dragon and sprint stubs against their corpora

## Objective

Task 36 fixed the idea stub's drift from its corpus but audited only
ideas. Apply the same audit to the dragon and sprint stubs: compare
each template's section list against the sections real artifacts
carry, classify deviations as creation-time drift (template gap) or
lifecycle additions (not the stub's job), and update a template only
where the corpus shows a hand-added creation-time section recurring
at the working rule of three.

## Acceptance criteria

- Every dragon and every sprint artifact's section headings are
  compared against their stub templates, and the audit's findings and
  classification are recorded in this task's result.
- Any template change the audit justifies is implemented with an
  ordering-asserting test, mirroring task 36; any deviation judged
  not stub-worthy is recorded with its reasoning.
- Existing artifacts are not modified.
- `scripts/check.sh` passes.

## Result

Audit of all 4 dragons and all 7 sprints against their stubs.

Dragons: one creation-time drift found. `## Candidate direction`
(between Constraints and Resolution criteria) appears in dragons 1-3
and is absent only from dragon 4 — three hand-additions meets the
working rule of three, and dragon 3 is the sharpest specimen: it is
tool-created, so its author added the section against the stub's
gravity. Dragon 4 shows a dragon can honestly lack a candidate, but
the cost asymmetry favors the template: deleting an inapplicable
heading is cheaper than recalling its house name and position.
Implemented: `create_dragon` gains the section and the template test
now asserts heading order with the task 36 cursor walk. The
`## Resolution (date)` sections on dragons 2-3 are close-time
additions, not stub material.

Sprints: no template change. Every deviation is a post-creation
lifecycle section — `## Retrospective (date)` on all six closed
sprints, `## Amendments`/`## Amendment: … (date)` on four — which
cannot exist meaningfully at creation and would be noise as empty
stubs on active sprints. Sprint 1's missing Rationale is historical
(predates the section settling), not drift.

Cross-cutting observation, recorded here rather than acted on: the
close-time section family — dragon `Resolution`, task `Result`,
sprint `Retrospective` — is hand-appended across three kinds on
every closure. That is a transition-time desire path, not a
creation-stub gap; it is promotion evidence for editor-mediated
prose at transition time ([[idea-strata-edit|idea 3]] adjacency,
the seam idea 9 explicitly left open) and exactly the row shape
[[ide_01KY7S6GG3NAA35KBJTC6CA1TM|idea 23]]'s ledger would capture.
