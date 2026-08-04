---
id: idea-doctor-reference-graph
sequence: 2
kind: idea
status: parked
created: 2026-07-20
---

# Doctor checks over the derived reference graph

## Problem

Cross-references and the promises they carry are invisible to tooling: a
dragon can pledge "record the outcome in decision 0005" and nothing will
ever notice the pledge dangling. Duplicate-collision detection exists for
sequences (dragon 1) but nothing analogous exists for references.

## Sketch

Doctor derives the full reference graph (typed front-matter edges plus
untyped inline markers, tagged by provenance) as a disposable projection,
then reports per decision 0006 severity: a dangling typed edge is a
corruption-level finding; unbound sugar and dangling untyped markers are
diagnostics; frozen-label drift is information only. Cycle checks apply
only to typed edge kinds with DAG semantics.

## Addendum: label/target consistency (2026-07-25)

A bound marker can carry a label that names a different artifact than
the one its stable id resolves to — `[[<id of idea 22>|idea 26]]` —
and remain structurally valid, since the target exists. This is
distinct from frozen-label drift (target's title changed under a
once-correct label): here the label was wrong at write time. A cheap
check: when a label matches the `kind N` or `kind:N` shape, verify N
against the resolved target's sequence and report mismatch as a
diagnostic. Specimen: while authoring idea 28, an agent pasted
idea 22's ULID into a link labeled "idea 26"; self-caught before
commit, but doctor could not have flagged it.

### Second specimen, 2026-08-04: the same fault, committed and green

The addendum's specimen was self-caught before commit. This one was not.

Closing [[tsk_01KZ738BECT3VAFX99CKPM9VDB|task 61]], its Result carried
two hand-written bound markers. One named a fabricated id and `doctor`
flagged it immediately as `dangling-reference`, which is the check
working. The other was written as `[[tsk_...391|task 62]]`, and
`...391` is **task 63's** id. `doctor` passed it. The corpus was green
with a marker whose frozen label named one artifact and whose target
resolved to another, and it was caught only because a human read the
prose.

Two things this adds beyond the addendum.

**The proposed check would have caught it.** The label was literally
`task 62` — the `kind N` shape the addendum names — against a target
whose sequence is 63. That is the cheap comparison working on a real
specimen rather than a constructed one, which is the evidence this
idea was missing.

**The failure mode is structural, not careless.** Both markers were
written by an agent transcribing ULIDs by hand, in a terminal narrative,
at the moment of closing — where there is no scaffold and no
verification. [[tsk_01KZ738BG7HDGBJDM57TW40ED5|Task 62]] removes the
transcription for the authoring path by binding legal sugar at write
time, which shrinks the population that can exhibit this fault but does
not eliminate it: fully bound markers stay legal, hand-editing stays
legal, and neither is validated. The check proposed here remains the
only thing that would catch a wrong-but-resolving label, and it stays
parked with one more reason to exist.

## Evidence

Decision 0006 (`dec-bootstrap-reference-model`), which requires each typed
edge kind to define doctor semantics up front; the stringly-typed
cross-reference gap observed while recording dragon 2. Blocked on dragon 3
(`drg_01KY169X7W0YXJ5QFV4D1MK4FB`).
