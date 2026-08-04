---
id: ide_01KYZY233Z7GAKFPFSKEAF89ZD
sequence: 41
kind: idea
status: parked
created: 2026-08-01
---

# Author-owned sections in managed templates

## Problem

`--body-file` refuses any `## ` heading that is not one of the
collection's managed sections. That invariant is right — Scarp owns the
template, its order, and its layout — but it is enforced only on the
creation path, and the corpus it is supposed to protect does not obey it.

**Creation refuses what validation accepts.** `doctor` checks no section
structure whatsoever: it never looks at headings, so an artifact with
author-added `## ` sections is fully valid and always has been. The
constraint exists in exactly one place, and it is the place that writes
files rather than the place that judges them.

The corpus is the evidence. Tasks own `Objective` and `Acceptance
criteria`; [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] carries six more —
`The product interface`, `Architectural constraints`, `The
duplicate-realization guard`, `Snapshot semantics` — between them.
Sprints own four sections; [[spr_01KYX1WAD7CC0RHVZY0V7VE4X1|sprint 10]]'s
charter carries `Adjudicated shape`, two `Superseded:` sections, and a
`Retrospective`. Sprint 1's retrospective is the same shape. None of
those could have been created by the tool that manages them. Every one
was closed by hand-editing the file after `new` wrote it — the
hand-performed operation [[ide_01KY7S6GG3NAA35KBJTC6CA1TM|idea 23]]
treats as the strongest promotion evidence Scarp produces.

So an author with a structured body has three moves, and all three are
bad:

- demote to `###`, which preserves the outline but makes the heading
  level assert a subordination that is not true — `Acceptance criteria`
  is not a peer of `Objective` while `Architectural constraints` is a
  child of it;
- create through the tool, then hand-edit the file it just wrote,
  performing the operation twice;
- give up on `--body-file` and write the artifact by hand, which is the
  transcription the flag exists to abolish.

The refusal is also silent about the escape. Its message enumerates the
managed sections and explains that Scarp owns the template, which is
accurate and leaves the author to invent a workaround unaided. Each one
invents a different one, which is how the corpus ends up carrying both
shapes.

Observed 2026-08-01, creating sprint 11's tasks 57 through 60: four
bodies were demoted to `###` to stay inside the tool, immediately after
reading task 54's `##` sections as the house style.

## Sketch

Separate two claims that are currently one:

- Scarp owns **which managed sections exist, that they are all present,
  and their relative order.** This is load-bearing and should not
  weaken.
- Scarp owns **every heading in the file.** This is what the creation
  path currently enforces, and what the corpus disproves.

Candidate directions, cheapest first:

- **Say `###` is the convention and make the error teach it.** Costs
  nothing structural, and the error message stops being a dead end. It
  leaves the corpus inconsistent and keeps asserting a false hierarchy,
  so it is a floor rather than an answer.
- **Accept unmanaged `## ` sections and preserve their position
  relative to the managed ones.** Matches what authors already do by
  hand. The real work is ordering: managed sections must still appear in
  template order, and an unmanaged section has to land somewhere
  definite — task 54 wants its extras between `Objective` and
  `Acceptance criteria`, so appending them all at the end is not
  equivalent.
- **An explicit opt-in** — a flag or an in-body marker — so the strict
  default survives and the escape is visible in the invocation rather
  than inferred from the file.

Whichever is chosen, the asymmetry is worth resolving deliberately
rather than by accident: either `doctor` grows an opinion about section
structure, or the creation path stops holding one the validator does not
share. Silently keeping both is the current state and is the thing that
produced two house styles.

## Boundaries

- Not user-definable collections, not a general templating system, not
  front-matter extension. The managed sections stay Scarp's.
- Not a relaxation of the presence or order guarantees for managed
  sections.
- Not a migration. Existing artifacts are correct as written and are not
  rewritten to whichever shape wins; the convention applies to new
  writing, as decision 10's wikilink migration rule already establishes.
- Not a reason to make `doctor` reject anything that is valid today.
  Artifacts with author-added sections have always passed and must keep
  passing.

## Evidence

The friction is recurring rather than incidental: it appears once per
task whose specification is long enough to want structure, which in this
repository is most of them.

The asymmetry was found by looking, not by failing — `doctor` was
checked for section validation while writing this idea, and has none. A
constraint enforced on write and unenforced on read is a constraint the
corpus can drift away from without anything noticing, and it has.

### 2026-08-04, two problems that look like one

Sprint 12's commissioning round separated this idea from a neighbour it
was being conflated with. WitnessGlass reported its `## Result` friction
and its dragon follow-up friction together, and both read as "Scarp will
not write a `##` section I need". They are different problems:

- a **terminal narrative** is a section the collection already owns,
  written at a transition Scarp already performs —
  [[ide_01KYE386E7T9AZW4Z6MW39JB0R|idea 30]], being built now;
- a **dated follow-up** is an author-titled section appended to a live
  artifact over its lifetime —
  [[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|idea 26]], where the specimens are
  now recorded;
- **this** idea is about author-authored sections *interleaved with
  managed ones at creation time*, where ordering and ownership are the
  hard part. Neither of the other two touches that.

Sprint 12 resolves none of it. What it does supply is a bounding case:
the `log` collection is being adopted with no managed sections at all,
because its three existing artifacts share no section vocabulary. A
collection with an empty template has no ordering question and no
ownership conflict — its body is authored verbatim. That is worth
stating because it shows the tension here is not "authors want `##`
headings" but specifically "authors want them *among* Scarp's", which is
a narrower and harder claim than the Problem section currently makes.
