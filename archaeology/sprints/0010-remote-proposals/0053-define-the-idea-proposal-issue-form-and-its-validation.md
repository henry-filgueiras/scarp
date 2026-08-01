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

Define the payload an idea proposal carries, as a GitHub Issue Form,
and build the half of the workflow that reads it: authorize the
requester, parse and validate the payload, and report a useful
diagnostic — stopping short of any mutation.

This is a complete vertical slice on its own. At the end of it, filing
a proposal issue produces either "this would be accepted" or a specific
refusal, and nothing is ever written. The mutation half is
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]]. Splitting there means the
authorization and parsing logic can be exercised against real hostile
input before anything can commit.

## The form

An Issue Form, not a Markdown template, because the fields must be
machine-readable and individually required rather than parsed out of
free prose a filer can restructure.

**This repository already has `.github/ISSUE_TEMPLATE/idea.md`**, a
Markdown template shaped like an idea artifact and aimed at any
visitor proposing a direction for discussion. That one is a
conversation starter; this one is an ingestion payload. They must not
be confused by someone browsing the new-issue chooser. Decide and
record whether both survive, how each is named and described so the
distinction is obvious at the moment of choosing, and whether the
existing template needs a line pointing at the other.

The form's fields should map onto what Scarp actually needs to create
an idea — a title and the template's sections — and nothing more. A
field whose value would become a command, a flag, a path, a collection
name, or a target artifact is out of scope by the sprint's non-goals;
the payload expresses one operation, and which operation it is, is
fixed by the form itself.

## Validation

Structural validation belongs here; canonical validation belongs to
Scarp and is checked by `scarp doctor` in task 54. The split matters:
this step should reject what Scarp should never be asked to process,
not re-implement Scarp's judgment about artifacts.

At minimum, refusals for: an unauthorized requester, a missing required
field, a title that is empty or unusable, and a payload that trips one
of [[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|task 48]]'s injection surfaces.

## What the realization event is

This half decides the thing the mutation half depends on: **which
event means "realize this now"**. Filing an issue and asking for it to
become canonical may or may not be the same act, and the choice has
consequences [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] inherits
directly.

The choice determines when proposal prose stops being editable input.
The intended semantics, unless the design produces something stronger:
creating or editing an issue does not by itself mutate canon; an
explicit authorized realization event snapshots the title, body, and
parsed fields; later edits do not regenerate an already-realized
artifact. A trigger that fires on every edit makes that harder to
honour than one that fires on a deliberate act, and that is a reason to
prefer the latter, not merely a note.

The event must also carry a stable proposal identity — repository plus
issue number — because task 54's replay guard is built on it, and an
identity derived from anything mutable is not an identity.

## Acceptance criteria

- The Issue Form exists, renders in the new-issue chooser, and its
  relationship to the existing `idea.md` template is decided and
  recorded — including whichever is renamed or re-described.
- The workflow half built here authorizes the requester by a live
  repository-permission check requiring write access or better, per
  task 48's specified mechanics, and refuses an unauthorized requester
  before parsing anything. No login is hardcoded: the same workflow
  must authorize correctly in a repository whose maintainers are not
  Henry.
- The payload is parsed into structured values without any
  `${{ }}` interpolation of issue-authored text into a shell context,
  per task 48's mitigations.
- Each refusal class produces a distinct, specific diagnostic that
  reaches the filer through the channel task 48 recommended — naming
  what was wrong and what to do about it, not "validation failed".
- Refusals are verified against real issues filed for the purpose, at
  minimum: an unauthorized user, a missing field, and one deliberately
  hostile payload drawn from task 48's inventory. The test issues and
  their outcomes are recorded, and the issues are closed afterwards.
- A valid proposal reaches the end of this half and reports what it
  would create, writing nothing: no branch, no commit, no pull request,
  no file.
- The realization event is chosen and recorded, with what it means for
  when prose becomes immutable input. If the chosen trigger gives
  stronger snapshot guarantees than task 54 assumes, that is documented
  so task 54 implements the stronger thing rather than the weaker.
- The payload is captured as a snapshot the mutation half can consume
  later, keyed by a proposal identity derived from repository and issue
  number. Nothing downstream is required to re-read the live issue to
  learn what was proposed.
- Authorization is established here and re-established late in task 54.
  This half's check is not treated as sufficient on its own, and the
  Result says so, so nobody later mistakes it for the gate.
- The workflow declares the exact `permissions:` block task 48
  recommended, per job.
- Nothing in the form or the validation is idea-specific in a way that
  would have to be torn out to add a second collection later, and
  nothing generalizes speculatively to collections this sprint does not
  support. Note that the grant
  [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] records is ideas-only, so
  adding a collection costs an amendment; the code should not make it
  *harder* than that, and must not make it look easier.
- Nothing in the form or the workflow half hardcodes this repository:
  no owner login, no absolute path, no assumption that the archaeology
  lives where it lives here rather than wherever `.scarp.toml` says.
- `scripts/check.sh` passes.

## Result
