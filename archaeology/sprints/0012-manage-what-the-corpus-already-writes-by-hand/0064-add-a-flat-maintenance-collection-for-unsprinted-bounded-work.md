---
id: tsk_01KZ738BM2YPD3R55M0JG8QH5Z
sequence: 64
kind: task
status: pending
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
---

# Add a flat maintenance collection for unsprinted bounded work

## Objective

Give bounded repository work somewhere to live that is not a sprint.

WitnessGlass had to commission a whole sprint — goal, rationale, success
criteria, non-goals — to file one piece of housekeeping, because a task
cannot exist outside an active sprint and both of its sprints were
closed. It observed that the pressure runs the wrong way: the cheapest
path is to skip tracking entirely, which is what the workflow exists to
prevent.

`maintenance` is bounded repository work worth recording but not
commissioned in service of a sprint goal. It is sharply not a `task`,
which is commissioned by a sprint and owned by it, and sharply not the
`chore` of [[idea-chore-artifacts|idea 7]], which is a recurring
obligation with a staleness tolerance and a performance ledger. Neither
"unsprinted" nor a sentinel junk-drawer sprint is an acceptable
substitute: scheduling state is not domain meaning.

Placement is flat, adjudicated 2026-08-04. Creation-time temporal
sharding would make derived physical placement a new canonical invariant
and would add a third stable-containment topology, a dedicated scanner,
and a bucket-versus-`created:` agreement check, all before any
maintenance corpus exists to demonstrate the need. The browsing concern
is a scaling hypothesis, not a requirement. Collection-global sequences
and stable ids keep the later migration a pure `git mv`.

Lifecycle is `pending -> closed`, with no `cancelled`. A maintenance
item that turns out not to be worth doing closes with a Result saying
so, which is lossless and needs no new status vocabulary, no second
terminal stamp, and no new verb.

## Acceptance criteria

- A `maintenance` collection at `archaeology/maintenance/`, flat, files
  named `NNNN-slug.md`, created `pending`, transitioning
  `pending -> closed` and no further.
- Sequences are collection-scoped and globally allocated within the
  collection, and ids are `mnt_`-prefixed ULIDs, so a later move to
  temporally sharded directories can be performed by `git mv` alone with
  no identity or sequence change. A test pins allocation.
- The creation template is a single `Work` section: what needs doing and
  why it is worth recording. The terminal narrative is the `Result`
  section supplied through `close --body-file`, not a creation stub.
- `new`, `list`, `show`, `close`, and `doctor` all work, and
  `scarp close maintenance:N --body-file` writes `## Result` through the
  machinery task 62 built, with no maintenance-specific special case.
- A maintenance artifact carries no `sprint:` field, and `doctor` does
  not look for one; nothing about it references a sprint.
- No staleness tolerance, no `stale_after`, no performance ledger, no
  JSONL sidecar, no fortune weighting.
- `scarp init` behaviour is unchanged: directories are still created by
  `init` or first `new`, and no empty `maintenance/` directory is
  pre-created for Git to fail to track.
- [[idea-chore-artifacts|Idea 7]] stays `parked` and gains a dated note
  recording that `maintenance` exists, what it deliberately omits, and
  that the chore gate is therefore unchanged rather than satisfied.
- `scripts/check.sh` passes.
