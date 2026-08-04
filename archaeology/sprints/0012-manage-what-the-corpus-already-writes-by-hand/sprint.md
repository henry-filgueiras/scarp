---
id: spr_01KZ7352BYX19E0DNDG05744AM
sequence: 12
kind: sprint
status: active
created: 2026-08-04
---

# Manage what the corpus already writes by hand

## Goal

Scarp manages three structures its repositories already maintain by hand
— logs, principles, and unsprinted maintenance work — and the terminal
narrative every closure already carries starts arriving through the
transition that produces it.

## Rationale

The theme is not new capability. It is the gap between what the corpus
already contains and what the tool admits exists.

**Logs are the sharpest case.** `archaeology/logs/` holds three
canonical artifacts with `kind: log`, correct four-digit sequences,
stable ids, and real inbound references — and no Scarp command has ever
touched them. They are already conformant to every rule the reader
enforces except one: none carries `status:`, which the front-matter
parser requires. The corpus is therefore evidence that a collection can
have no lifecycle at all, and the model simply never admitted one. The
same three files also share no section vocabulary — log 1 is bare prose,
log 2 and log 3 invent their own headings — so logs are equally evidence
for a collection with no managed template. Adoption should change no
byte of any existing log.

**The principle gate is satisfied.** [[ide_01KYDZVN858BK52A35KJ3ZY5BP|Idea 28]]
parked itself behind "no collection work is justified until the corpus
demonstrates need". [[log_01KYK8RC0YEY51YP37RGV7M7N4|Log 3]] then
recorded a heuristic that recurred four times in one sprint, drafted in
the exact schema idea 28 proposed, with every field carrying something
non-obvious. Log 3 states the remaining gap itself: a log cannot be
cited by a decision as `applies` or `overrides`. The heuristic is
distilled into the first principle and the log stays where it is —
history is not relocated to make room for guidance.

**The terminal-narrative friction is now second-repository evidence.**
[[ide_01KYE386E7T9AZW4Z6MW39JB0R|Idea 30]] parked with a 42-specimen
census and zero violations, all hand-appended. WitnessGlass then hit the
same wall from the consumer side: its workflow contractually requires a
`## Result` on every closed task, `new --body-file` correctly refuses
that section, `close` does not supply it, no append command exists, and
the section arrived by `cat >>`. Two repositories, one desire path, and
the mechanism half of idea 30 is small — `transition::perform_with_edge`
already rewrites status, stamps `closed:`, and inserts an edge line in a
single safe write.

**Unsprinted work has nowhere to live.** WitnessGlass had to commission
an entire sprint to file one piece of housekeeping, because a task
cannot exist outside an active sprint. "Unsprinted" is scheduling state
and not enough domain meaning to name a collection after; bounded
repository work not commissioned in service of a sprint goal is.
`maintenance` is that collection, and it is deliberately not the
recurring-obligation ledger [[idea-chore-artifacts|idea 7]] describes.

Three collections in one sprint is also the measurement
[[idea-declarative-collection-specs|idea 10]] has been waiting for since
sprint 2. That is a checkpoint, not a goal: the sprint records what the
duplication actually costs and leaves the idea parked unless a very
small extraction falls out of the work.

## Success criteria

- `archaeology/logs/` is a managed collection: creation, listing,
  inspection, and `doctor` coverage, with `git diff` over the directory
  empty across the whole sprint.
- A terminal narrative can be supplied to the transition that produces
  it, in the same write, for every collection whose corpus already
  carries one.
- The heuristic in log 3 exists as a citable principle, and log 3 is
  still the canonical account of how it was found.
- Bounded housekeeping can be recorded without commissioning a sprint.
- Both new affordances are exercised against a real consumer repository
  by a binary built from this work, in a repository that did not build
  it.
- The cost of adding three collections is measured and written back into
  the parked idea that asked for the measurement.

## Execution order and dependencies (2026-08-04)

Sequential, in task order. The dependencies are real rather than
tidiness:

```text
61 log ──────────────┐
                     ├──► 63 principle ──┐
62 close narrative ──┤                   ├──► 66 duplication checkpoint
                     └──► 64 maintenance ┘
                              │
                              └──► 65 WitnessGlass validation
                                        ▲
                     62 ─────────────────┘
```

- **[[task:61]] first.** It is the cheapest collection to add — no
  lifecycle, no template, no corpus migration — so it establishes the
  add-a-collection baseline that task 66 measures the other two against.
  It also puts log 3 into the artifact scan before task 63 writes a
  principle citing it.
- **[[task:62]] is independent of 61** and could run first; it is
  sequenced second because 64 needs it. A maintenance item's `Result`
  must arrive through the same machinery as a task's, with no
  collection-specific special case, which is only checkable if the
  machinery exists first.
- **[[task:63]] depends on 61** for the citation target and for the
  baseline comparison; it does not depend on 62, since principles have
  no terminal narrative — `retired` is a state, not a story.
- **[[task:64]] depends on 62.** Its creation template is deliberately
  only `Work`; the `Result` half is task 62's, and building maintenance
  first would tempt a creation-time `Result` stub that task 37's audit
  already ruled against.
- **[[task:65]] depends on 62 and 64** — it validates exactly those two
  affordances — and must run after both are complete rather than after
  each, because the WitnessGlass repository should be entered once.
- **[[task:66]] depends on 61, 63, and 64.** It cannot measure
  duplication until all three collections exist, and the log's
  statelessness is the test any candidate extraction has to survive.

Nothing here blocks on adjudication. The two forks settled while
commissioning — flat maintenance placement, and terminal narrative
without a general append — are recorded in the tasks that carry them and
in ideas 42 and 26 respectively.

## Non-goals

- **No general append primitive.** The other half of the WitnessGlass
  desire path — six dated, free-form, author-titled sections appended to
  three *open* dragons — is not a bounded append over managed sections;
  a command restricted to a collection's own section vocabulary would
  have refused every one of them. That shape belongs to
  [[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|idea 26]] and is recorded there, not
  solved here.
- **No resolution of author-owned creation sections.**
  [[ide_01KYZY233Z7GAKFPFSKEAF89ZD|Idea 41]]'s ordering and ownership
  semantics stay open. Logs sidestep it by having no managed template at
  all, which is a data point for that idea rather than an answer to it.
- **No collection DSL, framework, or declarative spec.** Idea 10 gets
  evidence and stays parked.
- **No chore semantics on `maintenance`**: no staleness tolerance, no
  recurring-performance ledger, no fortune weighting, no sprint
  membership.
- **No temporal sharding of `maintenance` placement.** Flat now; the
  deferred direction is recorded as its own idea.
- **No doctor conformance judgement over principles.** Principles
  advise; `doctor` validates their structure and never their
  application.
- **No typed `applies`/`overrides` edges without a real instance.** The
  cost is measured; the edges are added only if a decision in this
  sprint genuinely cites a principle.
- Out of scope entirely: chores, protocols, `scarp gh init`, trusted
  publishing, prebuilt binaries, a reusable proposal action, arbitrary
  Markdown structural editing, user-defined collections, migrating
  CLAUDE.md wholesale, and sprint dashboards.
