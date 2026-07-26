---
id: ide_01KYFWTA6D95AVDDXAED3XX98F
sequence: 32
kind: idea
status: parked
created: 2026-07-26
---

# Retroactive task import for externally completed work

## Problem

The commit-vertical-slices convention assumes the task artifact
exists before the work: mint a task, do the work, close it, commit
both together. Real work sometimes completes before its owning
repository exists. Concrete case (2026-07-26): a naming cantrip was
prototyped during [[tsk_01KYFRWF1X37N5TBJ139X7ZKA1|task 40]]'s
release-identity research and extracted to `~/cantrips/namegen`,
destined for a dev-env repository that has not been created yet.
The task brief was written at build time as a bare markdown file
beside the deliverable, deliberately without front matter, because
identities are never minted by hand outside a strata repository.

## Sketch

Recognize "import of a task artifact for already-completed work" as
a legitimate workflow, not a policy violation. The manual form:
`strata new task` in the receiving repository, paste the prepared
brief into the stub, close it immediately, and record the adopting
commit hash in the Result as provenance. If performances recur,
candidate tooling could be a create-closed path (a task born in its
terminal state with a provenance field) — but the manual ceremony
is the instrument that proves or disproves the need.

## Boundaries

- No automation now; each import is a hand performance and a chore
  ledger row of recurrence evidence.
- Commit hashes in Results are provenance text, not validated
  references; Git stays optional at the core.
- This complements, not weakens, commit-vertical-slices: it covers
  the arrival of work from outside a repository's lifecycle, and
  the default for in-repo work remains task-first.

## Evidence

- 2026-07-26: `~/cantrips/namegen/TASK.md` written as the first
  import candidate, awaiting the dev-env repository's first sprint.
