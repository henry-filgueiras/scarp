---
id: tsk_01KZ738BNX70HQWFCBYV8CF9F1
sequence: 65
kind: task
status: closed
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
closed: 2026-08-04
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

## Result

The compatibility claim held, and it was verified rather than inferred —
which matters here more than usual, because this task's whole subject is
the shape [[prn_01KZ76WRJ5QMEDGCPB6076HEAC|A verification is blind to preconditions the work established]] names.

**Versions, and a hazard found while establishing them.** The released
Scarp on crates.io and this working tree both report `scarp 0.2.0`. The
released one predates logs, principles, and maintenance; the working
tree has all three and has not bumped. `--version` therefore cannot
distinguish the tool that implemented this sprint from the tool that
knows nothing about it, and every claim below had to be made
behaviourally instead. That is a live trap for exactly this kind of
test, and it is recorded here because the next release will be the
moment to fix it.

**Method.** `cargo install scarp --version 0.2.0` into a `--root` and
`CARGO_HOME` both verified empty first, so the older binary owes nothing
to this machine's state. A baseline was taken *before* the new
collection existed — released Scarp, 33 artifacts, doctor green — so the
after-reading has something to be compared against rather than merely
looking healthy.

**Result.** After a maintenance artifact was added, the released binary
reported 33 artifacts and exit 0, unchanged: `archaeology/maintenance/`
is invisible to it, exactly as `archaeology/logs/` was invisible for
eleven sprints. Every collection it does know still lists, and `show`
still resolves. A second, independently-provenanced old binary — the one
already on this machine's PATH, built 2026-08-02 — agreed. Then
WitnessGlass's own `scripts/check.sh`, unmodified, which invokes bare
`scarp` from PATH: green, 33 artifacts. That last one is the real
consumer path, and it is the evidence that matters, because it is the
gate their CI runs.

Current Scarp, installed from this work into a separate verified-empty
root and run from outside the Scarp checkout, reported 34 and resolved
the new artifact by sequence and by stable id. The delta is exactly one,
in exactly one direction.

**The specimen was truthful, and it was found rather than arranged.**
WitnessGlass carries four local `Scarp:` ideas. Its idea 1 asked for a
task's result to ride the close transition — filed after two closures
that both needed the append-then-close workaround. [[tsk_01KZ738BG7HDGBJDM57TW40ED5|Carry the terminal narrative on the close transition]] shipped
it. A parked idea that has silently been satisfied is worse than no
idea, because the next reader believes the gap is still open, so
recording that upstream shipped it is real work. It is also unarguably
too small for a sprint: all five of their sprints are closed, and under
the old shape this note would have required commissioning a sixth.

The terminal-narrative dogfood is that item's own `## Result`, supplied
through `close --body-file`, with `[[idea:1]]` bound at write time
against their corpus. No other WitnessGlass closure was genuinely owed —
no pending tasks, no active sprints, and their three open dragons are
open because the questions are open — so none was manufactured.

### An independent finding: the tool wrote a house style

Before this item, WitnessGlass's thirty-three artifacts contained zero
`[[...]]` markers. References there are plain prose: `task:4`,
`sprint:3`. The sugar in the maintenance item's `Work` section was
rewritten to a bound marker at write time, so the first wikilink in that
repository was written by Scarp rather than chosen by the project.

It is correct, it resolves, and their older binary validates it happily.
It is also a default arriving uninvited in a corpus that had settled on
another convention. Decision 10 governs Scarp's own writing, and a
consumer using `--body-file` inherits it without ever opting in. Not a
defect and not scope here; recorded because a second instance would make
it one. The dated section appended to their idea 1 deliberately used
their prose convention instead — a choice available only because that
append happened outside Scarp entirely.

### The write-boundary gap, from the consumer side

Appending to their idea 1 was `cat >>`, because adding a dated section
to an existing non-terminal artifact is not a Scarp write. Their own
[[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|Managed amendments: dated in-place supersession]]-shaped local idea 4 asks for precisely this, independently
of Scarp's. Two repositories, two separately-filed ideas, one unbuilt
mechanism. Left for a later adjudication, as scoped.

### One acceptance criterion, met late and worth naming

The criterion required the binary under test to be installed rather than
resolved from ambient state. The creation and closure of the WitnessGlass
artifact used the working tree's `target/debug/scarp` by absolute path —
deterministic and unambiguous, but a dev binary rather than an installed
one. The verification half was then redone with a properly installed
build in a verified-empty root, and agrees. The identical code produced
both, so nothing is in doubt; the honest statement is that the install
path was exercised for verification and not for authorship, rather than
that the criterion was met as written.
