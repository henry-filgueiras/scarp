---
id: idea-declarative-collection-specs
sequence: 10
kind: idea
status: parked
created: 2026-07-21
---

# Declarative collection specs instead of a policy-template framework

## Problem

The dragon collection's mechanics — directory layout per lifecycle
state, front-matter schema, valid transitions, projection shapes — are
hardcoded. A second CLI-managed collection (ideas is the obvious next)
either copy-pastes that or generalizes it. The tempting generalization
is a C++-style compile-time policy framework: type-level knobs for
serialize/validate/deserialize, state machines, projection mappings,
invoked from thin per-collection product layers.

## Sketch

Generalize with data, not types. A `CollectionSpec` is a plain value —
name and plural, id prefix, lifecycle states with their directories and
allowed transitions, required front-matter fields, payload format,
projection field set — interpreted by one generic engine that already
knows how to scan, parse, validate, transition, and project. Behavior
escapes to a trait only where data cannot express it (payload codecs:
Markdown, JSON, JSONL — the seam decision 3 already reserved).

Why data over type-level machinery, opinionated:

- a spec that is a value can be printed, diffed, tested as a table, and
  read by `doctor` to validate *itself*; a template instantiation
  cannot;
- it keeps the door open to user-defined collections declared in
  `.strata.toml` — the natural end-state for a tool whose repositories
  must remain understandable without the executable. A compile-time
  tower forecloses that future; a spec interpreter gets it nearly free;
- Rust punishes the C++ pattern: trait-solver errors and monomorphization
  sprawl buy nothing here, because none of these knobs are
  performance-critical — the workload is parsing a handful of small
  files;
- rule of three: derive the spec shape from the second and third
  concrete collections, not ahead of them. The framework should be
  extracted from working duplicated code, or it will encode guesses.

The instinct itself is sound — collections *should* become declarations
consumed by shared machinery. The disagreement is only about when the
abstraction is earned and which axis (values, not types) carries it.

## Evidence

CLAUDE.md: bootstrap may hardcode one collection, "core abstractions
must not assume every artifact is Markdown", and speculative frameworks
are explicitly warned against — leave seams, extract later. Decision 3
(`dec-bootstrap-payload-separation`) is the reserved codec seam.
Sprint-2-era friction (ideas managed by hand while dragons are managed
by the tool) is the concrete forcing function. Prior art: serde's
derive-plus-attributes model (declarative surface, generic engine),
Kubernetes CRDs (collections as data), and the general "rule of three"
extraction discipline.

### 2026-08-04, task 66: measured across eight collections, and the answer is no

This idea asked for the spec shape to be derived from the second and
third concrete collections. Sprint 12 added the sixth, seventh, and
eighth — `log`, `principle`, `maintenance` — and task 66 measured the
result rather than assuming it. The measurement says: **do not build the
framework**, and the reason is more interesting than the verdict.

**Half of this idea already happened, incrementally.** `Collection` is a
plain value with six fields — kind, dir, states, transitions,
stamp_closed, terminal — interpreted by shared machinery (`scan_collection`,
`perform`, `doctor`, the terminal-narrative write). That is precisely
"generalize with data, not types", arrived at by extracting from working
duplicated code exactly as this idea prescribed, without anyone building
a spec interpreter. Across eight collections those fields hold 7, 5, 2
and 4 distinct values respectively: real semantic variation, not
boilerplate that happens to be spelled per collection.

**The remaining per-collection sites are compiler-enforced.** Adding a
ninth collection was measured directly, by adding a throwaway variant
and reading what `cargo check` demanded: **eight exhaustive `match`
arms**, two in `cli.rs` (`name`, `plural`) and six in `main.rs` (`scan`,
`verb_guidance`, the transition descriptor map, `close` dispatch,
`create`, the reachability probe). Every one is a build failure until
supplied. The classic argument for extraction — someone will forget a
site — does not apply, because nothing here can be forgotten.

Of those eight, three are mechanical relay (`scan`, the descriptor map,
the probe) and five carry genuine per-collection meaning. A
`descriptor(Collection) -> &'static read::Collection` helper would
collapse the three, and was costed: it removes roughly twenty lines of
relay and adds twelve, for a net of about eight lines, no correctness
gain, and one more indirection between "which collection" and "which
descriptor". That is not worth disturbing an explicit design for.

**The real risk was elsewhere, and it was not duplication.** Three sites
are *not* behind exhaustive matches: `doctor`'s validated set, `show`'s
bare-id union, and `close`'s bare-id union. The first fails **silently**
— a collection missing from it is never read, and the repository reports
healthy with an entire collection unchecked. No amount of spec
extraction was needed to fix that; one test that takes the CLI's own
advertised vocabulary as authority and proves `doctor` accounts for
every collection does it, and leaves the explicitness intact.

**Marginal cost is falling, not rising.** Source lines added per
collection, excluding tests and excluding one-time work bundled into the
same commits (the optional-`status` model change with `log`, the binder
move to `edges` with `principle`):

| collection | src lines | new machinery it required |
|---|---|---|
| log | +471 total, mostly the one-time optional-`status` change | stateless collections; template-free bodies |
| principle | +103 | none — first collection that was pure declaration |
| maintenance | +118 | `plural()`, for a mass noun |

Log was expensive because it falsified two model assumptions, not
because a sixth collection is expensive. Principle and maintenance cost
about a hundred lines each, and maintenance's hundred includes its own
`Work` template and a doctor entry. The vocabulary is converging, which
is the opposite of the trend that would justify a framework.

**Two constraints any future extraction must not break**, both now
measured rather than assumed:

- **Backward compatibility comes from explicit enumeration.** Task 65
  established that a repository containing a collection introduced by a
  newer Scarp stays usable by an older one, because `doctor` iterates a
  known list rather than asserting authority over all of
  `archaeology/`. Released 0.2.0 read a corpus containing
  `archaeology/maintenance/` and reported the same 33 artifacts it did
  before the directory existed. A discovery-driven engine that treated
  unknown directories as errors would destroy that property, which is
  worth more than the duplication it would remove.
- **The set of collections a binary understands must stay readable from
  source.** Line-count reduction is not a win if it makes the authority
  boundary harder to determine.

**Disposition: still parked, but the question has changed.** The data
half of this idea is built. What remains is only the end-state it
mentions in passing — user-defined collections declared in
`.scarp.toml` — which is an explicit CLAUDE.md non-goal and needs a
different justification than duplication, because duplication is no
longer the argument. Reopening this should require a user who wants a
collection Scarp does not ship, not a developer counting match arms.
