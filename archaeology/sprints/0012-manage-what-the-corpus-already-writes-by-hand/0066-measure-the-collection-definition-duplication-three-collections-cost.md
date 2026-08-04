---
id: tsk_01KZ738BQTR4H7Z7YBKPPCXGHT
sequence: 66
kind: task
status: closed
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
closed: 2026-08-04
---

# Measure the collection-definition duplication three collections cost

## Objective

Measure what adding three collections actually cost, and write the
number back into the idea that has been waiting for it since sprint 2.

[[idea-declarative-collection-specs|Idea 10]] asked for exactly this and
named the discipline: derive the spec shape from the second and third
concrete collections, not ahead of them; extract from working duplicated
code or the abstraction encodes guesses. This sprint supplies three at
once, and two of them break the mould in opposite directions — `log` has
neither lifecycle nor template, `principle` and `maintenance` have both.
That asymmetry is the most useful thing the measurement can report,
because a spec derived only from collections that resemble each other
would not have survived the log.

This is a checkpoint, not a mandate. The default outcome is evidence
recorded and the idea left parked.

## Acceptance criteria

- A per-collection inventory of what adding `log`, `principle`, and
  `maintenance` touched: which files, and for each site whether it was a
  pure constant or enum arm mechanically implied by the collection's
  existence, or a genuine per-collection semantic choice. The
  `Collection` static, the directory constant, the CLI enum arm and its
  `FromStr`, the `verb_guidance` arm, the `doctor` scan array, the
  `create_*` function, and the `probe_reachability` arm are each
  classified, not merely counted.
- The count distinguishes duplication that a data-driven spec would
  erase from duplication that encodes meaning a spec would have to carry
  anyway.
- The measurement is recorded as a dated section in idea 10's Evidence,
  in concrete terms — files, sites, and which of them a spec would
  actually remove — not as a verdict.
- An extraction is performed only if it is small, falls out of the work
  rather than being designed for it, and removes repeated arms or
  constants without collapsing a per-collection semantic difference. The
  log's absent lifecycle and absent template are the test any candidate
  extraction has to pass.
- If no extraction is performed, the reason is recorded and idea 10
  stays `parked`. That is a legitimate and expected result.
- Whatever this sprint learned about the log's statelessness and empty
  template is also recorded on
  [[ide_01KYZY233Z7GAKFPFSKEAF89ZD|idea 41]], since a collection with no
  managed sections has no ordering or ownership conflict to resolve and
  therefore bounds what that idea still has to answer.
- `scripts/check.sh` passes.

## Result

Measured, adjudicated, and **no framework built** — which the task
commissioned as a legitimate outcome and which the evidence supports
unambiguously.

### The measurements

**Semantic collection data.** `Collection` carries six fields across
eight collections, holding 7, 5, 2 and 4 distinct values for `states`,
`transitions`, `stamp_closed` and `terminal`. That is real variation,
not boilerplate that happens to be spelled per collection. Half of
[[idea-declarative-collection-specs|Declarative collection specs instead of a policy-template framework]]'s proposal — "generalize with data, not types" — is already
built, arrived at by extraction from working code exactly as that idea
prescribed, without anyone writing a spec interpreter.

**Mechanical dispatch, measured directly rather than counted by eye.** A
throwaway ninth variant was added to `cli::Collection` and `cargo check`
was asked what it demanded: **eight exhaustive match arms**, two in
`cli.rs` and six in `main.rs`. Three are mechanical relay — `scan`, the
transition descriptor map, the reachability probe. Five carry genuine
per-collection meaning — `name`, `plural`, `verb_guidance`, `close`
dispatch, `create`.

The decisive property is that all eight are **compiler-enforced**. The
standard argument for extraction is that someone will forget a site;
here nothing can be forgotten, because a missing arm is a build failure.
A `descriptor(Collection) -> &'static read::Collection` helper would
collapse the three mechanical arms: it removes about twenty lines and
adds twelve, net eight lines, no correctness gain, one more indirection.
Not worth disturbing an explicit design for.

**Necessary irregularity.** `maintenance` is a mass noun, so `plural()`
exists: seven mechanical arms and one real exception. A spec that moved
this to `plural: "maintenance"` would have relocated the exception, not
removed it — the irregularity is in English, not in the architecture.

**Marginal cost is falling.** Source lines per collection, excluding
tests and one-time bundled work: log +471 (dominated by the optional-
`status` model change it forced), principle +103, maintenance +118. Log
was expensive because it falsified two model assumptions, not because a
sixth collection is expensive. Principle was the first collection that
was pure declaration. The vocabulary is converging, which is the
opposite of the trend that would justify a framework.

### What the measurement actually found

The risk was never the duplication. Three sites are **not** behind
exhaustive matches: `doctor`'s validated set, `show`'s bare-id union,
and `close`'s bare-id union. The first fails **silently** — a collection
missing from it is never read, and the repository reports healthy with
an entire collection unchecked. That is a worse failure than anything
the duplication could cause, and no extraction was needed to fix it.

`tests/collection_coverage.rs` pins it: it takes the CLI's own
advertised vocabulary as the authority, asserts this test's list matches
it, then creates one artifact in every collection and proves `doctor`
accounts for all eight and that every one resolves by bare stable id. A
ninth collection forces an update to the advertised list, which fails
the first assertion, which pulls the author into the second.

That is the whole code change: one test file, no production code
touched.

### Two properties any future extraction must preserve

- **Backward compatibility comes from explicit enumeration.**
  [[tsk_01KZ738BNX70HQWFCBYV8CF9F1|Validate the consumer affordances in WitnessGlass]] measured it: released 0.2.0 read a corpus containing
  `archaeology/maintenance/` and reported the same 33 artifacts as
  before the directory existed, because `doctor` iterates a known list
  rather than asserting authority over all of `archaeology/`. A
  discovery-driven engine that treated unknown directories as errors
  would destroy that, and it is worth more than the duplication it would
  remove.
- **The set of collections a binary understands must stay readable from
  source.** Line-count reduction is not a win if it obscures the
  authority boundary.

### Disposition

[[idea-declarative-collection-specs|Declarative collection specs instead of a policy-template framework]] stays parked, with its question changed rather than
answered. Its data half is built; what remains is the end-state it
mentions in passing — user-defined collections in `.scarp.toml` — which
is an explicit CLAUDE.md non-goal and now needs a different
justification, because duplication is no longer the argument. Reopening
it should require a user who wants a collection Scarp does not ship, not
a developer counting match arms.

[[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|Managed amendments: dated in-place supersession]] gained the stronger evidence this task was asked to record:
WitnessGlass independently filed the same missing mechanism as its own
local idea 4, in a different repository with entirely different subject
matter. Two corpora, two separately-authored ideas, one gap. The
mechanism was deliberately not built here.
