---
id: tsk_01KYX1WJ03MD2WRNQBS3KGMXXA
sequence: 53
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
---

# Define the idea proposal issue form and its validation

## Objective

Deliver the durable remote proposal-entry surface: a structured GitHub
Issue Form that captures an idea proposal well enough for
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] to realize it without anyone
retyping anything.

*Narrowed 2026-08-01 by [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s
Option B adjudication.* This task previously owned half of a GitHub
Actions workflow — authorization, payload validation, refusal
diagnostics posted back to the filer. None of that survives. There is no
workflow, no token, and no untrusted execution context: the operator
realizes proposals from a trusted machine, so authorization is
"whoever holds the laptop" and validation belongs to Scarp at
realization time, where it already lives after task 51.

What remains is small on purpose: **a good form, and nothing else.**

## The form

An Issue Form rather than a Markdown template, because the fields must
be machine-readable — task 54 parses them — rather than recovered from
prose a filer can restructure.

The fields map onto what Scarp needs to create an idea: a title, and the
idea template's sections. Nothing more. A field whose value would become
a command, a flag, a path, a collection name, or a target artifact is
out of scope by the sprint's non-goals; the form expresses one
operation, and which operation it is, is fixed by the form itself.

**The issue is mutation intent, not canonical state.** Nothing about the
form should suggest the issue is the artifact, that editing it changes
anything downstream, or that its state mirrors a Scarp lifecycle. No
synchronization semantics, in the copy or in the design.

**The existing `.github/ISSUE_TEMPLATE/idea.md`** is a Markdown template
aimed at any visitor proposing a direction for discussion. That one is a
conversation starter; this one is a realization payload. They must not
be confused by someone browsing the new-issue chooser. Decide and record
whether both survive, how each is named and described so the distinction
is obvious at the moment of choosing, and whether the existing template
should point at the other.

**It must be useful in a repository that is not this one.** No owner
login, no path assumption, no reference to this project's own
archaeology. A consumer should be able to take the file unchanged.

## Acceptance criteria

- The Issue Form exists, renders in the new-issue chooser, and produces
  a body that task 54 can parse into a title and idea sections without
  guessing.
- A real proposal is filed through it — from a phone, since that is the
  motivating context — and the resulting issue is confirmed to carry
  everything realization needs. Filing it on a laptop would not test the
  thing that matters.
- Its relationship to the existing `idea.md` template is decided and
  recorded, including whichever is renamed or re-described.
- The form is legible: a filer who has never read this repository's
  archaeology can complete it correctly, and the fields say what they
  are for.
- Nothing in the form hardcodes this repository, and nothing implies
  synchronization or that the issue is canonical.
- Where practical the form is a file Scarp can scaffold rather than one
  a consumer reconstructs from documentation — the scaffolding itself is
  [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]]'s to implement if it earns
  its place, but the form should be shaped so scaffolding is trivial.
- `scripts/check.sh` passes.
- No workflow file, no authorization logic, no validation logic. Those
  either do not exist under Option B or belong to Scarp.

## Result
