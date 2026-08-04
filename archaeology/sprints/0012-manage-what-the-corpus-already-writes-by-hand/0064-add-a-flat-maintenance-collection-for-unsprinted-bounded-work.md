---
id: tsk_01KZ738BM2YPD3R55M0JG8QH5Z
sequence: 64
kind: task
status: closed
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
closed: 2026-08-04
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

## Result

Delivered as specified, and the cheapest collection yet: no new `Status`
variant, no new verb, no new scanner, no new doctor check, no placement
topology of its own.

**Shape.** `pending -> closed`, flat under `archaeology/maintenance/`,
`mnt_` ids, one creation section `Work`, and a `Result` that arrives
through [[tsk_01KZ738BG7HDGBJDM57TW40ED5|Carry the terminal narrative on the close transition]]'s machinery with no maintenance-specific special
case — `TerminalSection { name: "Result", dated: false }` in the
collection data, matching tasks, because tasks are what the corpus
already writes that way.

Dropping `cancelled` is the choice worth stating. An item that turns out
not to be worth doing closes with a Result saying so, which is lossless
and costs nothing; a `cancelled` state would have needed a new `Status`
variant, a second terminal stamp (`closed:` is a lie on a cancelled
item), and a new verb, to record a distinction a sentence already
carries.

No sprint membership anywhere: no `sprint:` field, no `--sprint`, and
`doctor` looks for neither. That is the whole point — WitnessGlass had
to commission an entire sprint to file one piece of housekeeping, and
the pressure ran the wrong way, since the cheapest path was to skip
tracking altogether.

**What it deliberately is not.** Not a chore ([[idea-chore-artifacts|Chore artifacts: recurring maintenance with staleness and a ledger]]): no staleness
tolerance, no recurring-performance ledger, no JSONL sidecar, no fortune
weighting. Tests assert each absence rather than leaving it to reading,
including that `fortune` does not surface maintenance.

**Placement.** Flat, per the 2026-08-04 adjudication recorded as
[[ide_01KZ73A671YV99APMXAWEQ20X9|Creation-time temporal sharding for high-volume collections]]. Sequences are collection-global and ids owe nothing to
placement, which is exactly the property that keeps a later move to
temporal buckets a pure `git mv`; a test pins it by closing an item
mid-sequence and asserting nothing moved.

Nothing was extracted. The collection went in through the same pattern
[[tsk_01KZ738BECT3VAFX99CKPM9VDB|Adopt the log collection, stateless and template-free]] and [[tsk_01KZ738BJ5MXNBDWECX8REA391|Add the principle collection and distil the first principle from log 3]] used, untidied, so [[tsk_01KZ738BQTR4H7Z7YBKPPCXGHT|Measure the collection-definition duplication three collections cost]] measures what
is really there.

### One thing the mass noun cost

`maintenance` does not pluralize, and `list`'s empty-collection message
was built from `name()` plus an `s`. That needed a `plural()` method —
seven mechanical arms and one real exception. It is a small thing, and
it is also the first place where a collection's *linguistic* shape, not
its lifecycle or template, forced a per-collection branch. Worth handing
to task 66: a declarative spec would have to carry the plural as data,
and would not have made this any smaller.

### Erratum (2026-08-04, post-close)

This task was closed reporting delivery "as specified", and one
acceptance criterion had not been performed: idea 7 was to gain a dated
note recording that `maintenance` exists, what it omits, and that the
chore gate is therefore unchanged. It did not, and the omission was
found during sprint closeout rather than at closure.

The note has since been written to idea 7. The original Result stands
unedited above, because the falsified claim is the lesson: every code
criterion was verified by a test, and the one criterion whose subject
was the corpus rather than the code was verified by nobody. `doctor`
cannot catch it — an idea missing a paragraph is a perfectly valid
artifact — so nothing failed, and the closure passed every gate it had.

That is [[prn_01KZ76WRJ5QMEDGCPB6076HEAC|principle 1]] operating on this
task's own closure: the verification was blind to a defect whose
precondition — "I know what the criteria said" — was established by the
same session that wrote them.
