---
id: tsk_01KZ738BG7HDGBJDM57TW40ED5
sequence: 62
kind: task
status: closed
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
closed: 2026-08-04
---

# Carry the terminal narrative on the close transition

## Objective

Let the transition that produces a terminal narrative also carry it, so
the section every closure in this repository already has stops arriving
by `cat >>`.

This is the mechanism half of
[[ide_01KYE386E7T9AZW4Z6MW39JB0R|idea 30]] and nothing more. That idea
parked with a 42-specimen census and zero violations; WitnessGlass
supplied the second-repository confirmation from the consumer side,
where a workflow contractually requires a `## Result` that
`new --body-file` correctly refuses and `close` declines to write.

The section name and its dated-or-not form are read off the corpus, not
chosen: 58 of 60 closed tasks carry a bare `## Result`, while both
closed dragons carry `## Resolution (YYYY-MM-DD)` and all ten closed
sprints carry `## Retrospective (YYYY-MM-DD)`. The date the dated forms
want is the transition date, which `perform_with_edge` already computes
for the `closed:` stamp.

The check half of idea 30 — a doctor finding for a terminal artifact
with an empty narrative — is deliberately not in this task. Idea 30
argues the mechanism must land first so the check arrives as a backstop
for a path the tool already paves, and its promotion question belongs to
the strict tier parked as idea 13.

## Acceptance criteria

- `scarp close <ref> --body-file <path>` appends the collection's
  terminal section, populated from the file, in the same safe write that
  rewrites `status` and stamps `closed:`. A failure at any point leaves
  the artifact byte-identical, as the existing atomic replace already
  guarantees.
- The section written matches the corpus per collection: `## Result` for
  tasks, `## Resolution (<transition date>)` for dragons,
  `## Retrospective (<transition date>)` for sprints.
- A body file that itself contains the terminal section's heading is
  refused, before any mutation, with a message saying Scarp writes that
  heading and the file should contain only what goes beneath it.
- `close` without `--body-file` behaves exactly as before and appends
  nothing: a creation stub must never grow an empty terminal section.
- On dragons, `--body-file` composes with `--resolved-by`; both land in
  the one write.
- `adopt` and `reject` gain nothing. Idea terminal states have zero
  specimens, and idea 30 defers them explicitly.
- Every already-closed artifact is untouched; this changes no existing
  file.
- [[ide_01KYE386E7T9AZW4Z6MW39JB0R|Idea 30]] transitions to `adopted`
  with an `adopted-by` edge naming this task.
- `scripts/check.sh` passes.

## Result

Delivered as specified, and this `## Result` is the proof: it was
supplied to `scarp close task:62 --body-file`, so the section and the
transition landed in one write. The markers below were written as
`[[idea:30]]` and `[[task:61]]` sugar and bound on the way in — no ULID
was transcribed by hand anywhere in this closure.

**Shape.** `TerminalSection` is collection data — a heading name and
whether it carries the transition date — beside `states` and
`transitions`, so the corpus's own convention is declared rather than
branched on: `Result` bare for tasks, `Resolution (date)` for dragons,
`Retrospective (date)` for sprints, `None` for the three collections
whose corpus has no such section. The write path grew one parameter;
`perform` now stamps `closed:`, inserts any provenance edge, and appends
the narrative in the same staged-and-renamed payload, so every refusal
still leaves the artifact byte-identical.

**The binder was reusable, and reusing it removed duplication.**
`resolve_edge` already resolved `kind:N` and bare ids through the
identity catalog and built a validated `[[id|label]]`. It split cleanly
into `resolve_claim` and `bound_marker`, both now shared with prose
binding; the provenance path kept its behaviour, which the existing
suite proves. The one genuinely new piece is `markers_in_prose`, and it
too came out of existing code: `check_prose`'s traversal already skipped
fenced blocks and inline code spans, and now yields byte ranges so a
caller can rewrite through them. Both consumers share one traversal
instead of two, and three tests pin the ranges, because an off-by-one
there corrupts an artifact rather than mis-reporting one.

**Two binding choices worth stating.**

An author's explicit label survives: `[[dragon:1|the risk we already
knew about]]` keeps those words and gains the stable id. The label was
written beside a reference the tool then verified, so the pairing is
correct by construction, and overwriting it with the target's title
would flatten prose into a heading.

Unresolvable sugar refuses the entire closure rather than binding what
it can. A marker naming nothing is a typo, and a half-bound section is
one nobody re-reads. The diagnostic names the marker rather than the
bare reference — a closure of `task:62` whose narrative cites a missing
`task:999` must not read as though the closure's own target went
missing.

### What this does not do

The check half of [[ide_01KYE386E7T9AZW4Z6MW39JB0R|Terminal narratives ride the close transition]] is not here, deliberately: that idea
argues the mechanism must land first so a doctor finding arrives as a
backstop for a path the tool already paves, and its promotion belongs to
the strict tier parked as idea 13. `adopt` and `reject` gained nothing,
since idea terminal states still have zero specimens. Nothing validates
or repairs an already-bound marker, which remains idea 2's question —
the mislabeled marker found while closing [[tsk_01KZ738BECT3VAFX99CKPM9VDB|Adopt the log collection, stateless and template-free]] is recorded there
and is not fixed here.

### Dogfood

Closing this task exercised the path against the real corpus rather than
a fixture: a task in a repository of 145 artifacts, with two sugar
markers resolving to a genuine adopted idea and a genuine closed task,
and a `closed:` stamp sharing the date the narrative was written. The
previous task's Result needed `cat >>` and a separate `scarp close`; this
one needed neither.

The friction that remains is honest and small: the narrative still has
to be written to a file first, because there is no stdin path yet. Idea
30 named a non-interactive argument or stdin as the first input surface
and `$EDITOR` through idea 3 as the later one; only `--body-file` exists.
For an agent, which is the demonstrated closer, writing a temporary file
is not friction worth a flag yet — recorded rather than fixed.
