---
id: tsk_01KYE0DAA4EJZMW31621TQX1XY
sequence: 36
kind: task
status: closed
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-25
closed: 2026-07-25
---

# Add Boundaries section to the idea stub template

## Objective

Close a template desire path: every substantial idea since the
sketch/boundary house style settled (ideas 21, 22, 28, 29) has grown a
`## Boundaries` section by hand, but `strata new idea` still emits only
Problem/Sketch/Evidence. Add Boundaries to the stub so the template
matches how ideas are actually written.

## Acceptance criteria

- `strata new idea` emits the section headings Problem, Sketch,
  Boundaries, Evidence, in that order.
- The template test asserts the new heading and the ordering.
- Existing idea artifacts are not modified; the change is
  template-only.
- `scripts/check.sh` passes.

## Result

`create_idea`'s section list gained `Boundaries` between `Sketch` and
`Evidence` (`src/artifact.rs`), and the template test
`create_idea_writes_a_parked_artifact_with_idea_template` now walks
the headings with a forward cursor, asserting presence and order in
one pass. Verified end to end: a fresh `strata init` plus
`strata new idea` in a scratch directory emits the four headings in
order. No existing artifacts touched; other collections' templates
unchanged. Ideas 21, 22, 28, and 29 supplied the desire-path
evidence; parking idea 29 was the fourth hand-added Boundaries
section and prompted the fix.
