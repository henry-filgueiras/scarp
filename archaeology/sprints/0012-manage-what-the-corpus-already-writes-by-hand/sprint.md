---
id: spr_01KZ7352BYX19E0DNDG05744AM
sequence: 12
kind: sprint
status: closed
created: 2026-08-04
closed: 2026-08-04
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

## Retrospective (2026-08-04)

### The sprint's own headline

Three collections shipped, but the result worth keeping is that **the
corpus specified the model twice, and the model gave way both times.**

`archaeology/logs/` held three canonical artifacts written in sprint 1
and untouched by any command for eleven sprints. Adopting them found
that they carry no `status:` line and share no section vocabulary. The
available moves were to stamp a synthetic state into three historical
files so the parser would accept them, or to admit that a managed
collection can have no lifecycle and no template. The second was chosen,
recorded as [[dec_01KZ74BQQJ1W5Q32GQS7RD4JCK|Stateless and template-free collections]], and **not one byte of any existing log
changed** — the load-bearing fact, because it is the evidence that the
model was wrong rather than the files.

`principle` and `maintenance` then exercised the resulting collection-data
vocabulary rather than extending it. Principle was the first collection
that was pure declaration; maintenance reused `TerminalSection` and a
`pending -> closed` pair verbatim and needed one new concept, `plural()`,
for a mass noun. Collections stopped requiring machinery and started
requiring only data.

### A write-authority boundary became explicit

Nobody had stated this rule before the sprint, and stating it settled an
argument that had been conducted twice in different terms:

> Wherever Scarp accepts authored prose that becomes canonical through a
> Scarp write, resolvable sugar in that prose is bound at that boundary.

Terminal closure satisfies it ([[tsk_01KZ738BG7HDGBJDM57TW40ED5|Carry the terminal narrative on the close transition]]); creation satisfies it
([[tsk_01KZ738BJ5MXNBDWECX8REA391|Add the principle collection and distil the first principle from log 3]]). There was never a semantic reason for a closure to bind
`[[log:3]]` while a creation preserved it — that distinction would only
have recorded which command received the binder first.

The third case is the one that matters now: **evolution of an existing
non-terminal artifact has no Scarp authority boundary at all.** Adding a
dated section to a parked idea is not a Scarp write, so it falls through
to `cat >>`, and every bound marker in it is transcribed by hand. This
sprint did that at least seven times, including in this very
retrospective's preparation.

### The sprint refused to model unobserved variation, six times

Each refusal was cheap to make and would have been expensive to undo:

- **No principle retirement.** `active` only, no transitions. A
  principle superseded by a sharper statement, one overtaken by a
  changed world, and one found to have been wrong are three different
  events; naming `retired` would have settled that on zero specimens.
  Single-state rather than stateless was chosen precisely so admitting a
  terminal state later is a transitions-table change with no corpus
  migration.
- **No temporal sharding of maintenance.** Flat, recorded as
  [[ide_01KZ73A671YV99APMXAWEQ20X9|Creation-time temporal sharding for high-volume collections]]. Monthly buckets would have made derived physical
  placement a canonical invariant and added a third stable-containment
  topology, a scanner, and a bucket-versus-`created:` check, before any
  maintenance corpus existed. Collection-global sequences keep the later
  migration a `git mv`.
- **Maintenance absorbed no chore semantics.** No staleness tolerance,
  no ledger, no recurrence, no `fortune` weighting; tests assert the
  absences so the boundary cannot erode quietly. [[idea-chore-artifacts|Chore artifacts: recurring maintenance with staleness and a ledger]]'s gate is
  unchanged, not satisfied.
- **No sixth principle section.** The first principle wanted a
  provenance section immediately, in a template drafted from a worked
  example. Recorded on [[ide_01KYZY233Z7GAKFPFSKEAF89ZD|Author-owned sections in managed templates]]; the template was not changed on one
  specimen.
- **No `applies` / `overrides` edges.** Declaring an edge kind is two
  entries; authoring one has no surface, because `resolve_edge` is
  reached only from a lifecycle verb's flag and decisions have no verb.
  No decision this sprint cites a principle, so there was no consumer,
  and inventing a decision mutation surface would have been the sketch
  driving the design.
- **No collection framework.** [[tsk_01KZ738BQTR4H7Z7YBKPPCXGHT|Measure the collection-definition duplication three collections cost]] measured it and declined.

### Task 66 found a different risk than the one it went looking for

The duplication everyone worries about turned out to be
**compiler-enforced**. Adding a throwaway ninth collection variant and
reading what `cargo check` demanded produced exactly eight exhaustive
match arms — three mechanical, five semantic — every one a build failure
until supplied. The standard argument for extraction is that a site will
be forgotten; nothing there can be forgotten.

The real hole was elsewhere and was **silent**: `doctor`'s validated
collection set is a plain array, not an exhaustive match. A collection
missing from it is never read, and the repository reports healthy with an
entire collection unchecked. `tests/collection_coverage.rs` closes that
specific hole — taking the CLI's own advertised vocabulary as authority
and proving `doctor` accounts for every collection — with **no
production-code abstraction at all**.

The general lesson is worth more than the specific test: a measurement
commissioned to evaluate an abstraction found a correctness gap the
abstraction would not have addressed, and the correct response was a
test rather than a refactor.

### Backward compatibility is now measured, and it is a property to protect

[[tsk_01KZ738BNX70HQWFCBYV8CF9F1|Validate the consumer affordances in WitnessGlass]] established by experiment, not inference, that explicit
enumeration gives older binaries a useful authority boundary. Released
Scarp 0.2.0, installed from crates.io into a verified-empty root, read a
WitnessGlass corpus containing `archaeology/maintenance/` and reported
the same **33 artifacts it reported before that directory existed** —
ignoring what it does not understand rather than calling it malformed.
WitnessGlass's own gate, unmodified, stayed green.

Any future collection or discovery refactor must preserve this. A
discovery-driven engine that treated unknown directories as errors would
make every repository unreadable by every older binary, which is worth
far more than the duplication such an engine would remove.

A deviation worth preserving honestly: the WitnessGlass artifact was
authored with the working tree's `target/debug` binary and verified with
a properly installed one. Identical code produced both, so no redo was
warranted, and the criterion asked for the install path at authorship
rather than at verification.

### Maintenance proved its niche twice, and immediately

[[mnt_01KZ78MZDN5VKPT6ETR4JPYDGD|Record the principle-template provenance finding on idea 41]] recorded a provenance finding on a parked idea:
genuinely worth tracking, genuinely too small to commission a sprint
task for. [[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]] carries a one-off obligation due at an
external trigger — bump the version before the next publish — so that
obligation survives independently of this prose. Both are exactly the
niche the collection was justified by, and neither is a chore.

### The version collision

Released Scarp and this working tree both report `scarp 0.2.0`. During
compatibility testing, `--version` could not distinguish the binary that
implemented this sprint from the one that knows nothing about it, and
every claim in [[tsk_01KZ738BNX70HQWFCBYV8CF9F1|Validate the consumer affordances in WitnessGlass]] had to be made behaviourally as a result. The
next publish **must bump the version before release**;
[[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]] carries it. The bump was deliberately not performed to
close this sprint — a release is its own decision, not closeout
paperwork.

### The principal unresolved desire path

Scarp and WitnessGlass independently filed the same missing shape.
[[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|Managed amendments: dated in-place supersession]] asks for authority-preserving amendment of an existing
artifact; WitnessGlass, with no knowledge of it, filed its own local
idea 4 asking for the same thing in different words. Two corpora, two
separately-authored ideas, one gap.

The sprint sharpened the boundary and deliberately built nothing, because
the semantics are an adjudication problem rather than an implementation
one: this repository has now produced `Update`, `Amendment` and
`Erratum`, and WitnessGlass independently produced `Findings`,
`Follow-up`, `Correction`, `Observation` and a bare `Pass 3:`. Whether
those are one operation or several is the question, and a command built
before answering it would freeze the wrong answer.

### What this sprint got wrong

One acceptance criterion on [[tsk_01KZ738BM2YPD3R55M0JG8QH5Z|Add a flat maintenance collection for unsprinted bounded work]] was not performed — [[idea-chore-artifacts|Chore artifacts: recurring maintenance with staleness and a ledger]]
was to gain a dated note recording that maintenance exists and that the
chore gate is unchanged — and the task was closed reporting delivery "as
specified". It was found during this closeout, written then, and the
task carries a dated erratum with the original claim preserved.

The shape is [[prn_01KZ76WRJ5QMEDGCPB6076HEAC|A verification is blind to preconditions the work established]] operating on the sprint's own process.
Every criterion whose subject was code was verified by a test; the one
criterion whose subject was the corpus was verified by nobody, and
`doctor` cannot catch it, because an idea missing a paragraph is a
perfectly valid artifact. Six tasks, six Results, one unverified class of
claim — and the class was invisible precisely because everything else
was so well covered.

### Carried forward

- [[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]], pending: bump the version before the next publish.
- [[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|Managed amendments: dated in-place supersession]], parked with materially stronger cross-repository
  evidence, and named here as the principal unresolved desire path.
- [[ide_01KYZY233Z7GAKFPFSKEAF89ZD|Author-owned sections in managed templates]], parked, now carrying a specimen from a template designed
  against a worked example.
- [[idea-declarative-collection-specs|Declarative collection specs instead of a policy-template framework]], parked with its question changed: the data half is built,
  and what remains is user-defined collections, which needs a user who
  wants one rather than a developer counting match arms.
- [[idea-doctor-reference-graph|Doctor checks over the derived reference graph]], parked with a second, committed specimen of a bound marker
  whose frozen label names the wrong artifact.
- [[idea-chore-artifacts|Chore artifacts: recurring maintenance with staleness and a ledger]], parked, gate unchanged.

Sprint 13 is deliberately not commissioned here.
