---
id: tsk_01KZ738BNX70HQWFCBYV8CF9F1
sequence: 65
kind: task
status: pending
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
---

# Validate the consumer affordances in WitnessGlass

## Objective

Exercise the two affordances this sprint builds for a consumer against
the consumer that asked for them, from a repository that did not build
them.

WitnessGlass is the source of both requirements: its task 12 recorded
the `## Result` that arrived by `cat >>`, and the sprint it had to
commission to hold that task. Validating either one only inside Scarp
would be the shape principle 1 names — a verification blind to any
defect whose precondition was established by the work being verified.
The residue here is concrete: Scarp's own corpus, fixtures, and
`archaeology/` layout are exactly what the features were written
against.

WitnessGlass has unrelated active work in its sprint 3. This task does
not touch it, does not close its tasks, does not modify its dragons, and
does not alter its adapter or docs. Only genuinely wanted housekeeping
is filed, and only a closure WitnessGlass actually owes is performed.
Anything committed there is committed in that repository and pushed from
neither.

## Acceptance criteria

- The binary under test is built and installed from this work into a
  location WitnessGlass reaches on its own — not invoked as
  `cargo run` from the Scarp checkout, and not resolved from a
  development binary already on `PATH`. The installation root is
  verified empty first, as task 43's install check established.
- At least one real maintenance item is created in WitnessGlass for
  housekeeping it genuinely wants, and closed with a Result through
  `close maintenance:N --body-file`, with no sprint commissioned to hold
  it.
- At least one terminal narrative is supplied through `close --body-file`
  on a WitnessGlass artifact whose closure is genuinely owed, and the
  resulting file is compared against the shape its predecessors reached
  by hand: same heading, same level, same position.
- What a *released* Scarp binary sees in that repository afterwards is
  established and recorded — specifically whether an unmanaged
  `archaeology/maintenance/` directory is invisible to an older
  `doctor`, the way `archaeology/logs/` was invisible for eleven
  sprints, or whether it is a finding. If it is a finding, that is a
  compatibility fact this sprint has to state plainly.
- WitnessGlass's sprint 3 work is untouched: `git diff` there names only
  the files this task deliberately added or closed.
- Nothing is pushed from either repository.
- Friction found from the consumer side is recorded where it belongs —
  in this task's Result, and as evidence on the relevant parked idea if
  it names a shape Scarp does not have.
- `scripts/check.sh` passes in Scarp.
