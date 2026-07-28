---
id: ide_01KY7S6GMN26BFTEVGGKZHN4ZC
sequence: 26
kind: idea
status: parked
created: 2026-07-23
---

# Managed amendments: dated in-place supersession

## Problem

The preserve-history invariant means canonical artifacts change by
appending dated sections, never by rewriting: the decisions corpus
already carries seven such sections, sprint 6 amended its own rationale
the same way, and the pattern is load-bearing — CLAUDE.md cites
"decision 11 as amended" as authority. Yet the mechanism is entirely
free-form. Two heading grammars have already diverged
(`## Update (date): title` and `## Amendment: title (date)`), nothing
marks which earlier text an amendment supersedes, and a reader must
diff headings by eye to discover that a decision has drifted from its
original statement. The most authority-bearing operation in the
repository is the least structured one.

## Sketch

Make the amendment a first-class operation: `strata amend <ref>
"<title>"` appends a scaffolded dated amendment section, and possibly
records an `amended:` date list in front matter so `list` and `show`
can flag amended artifacts without reading the body. Convention before
command is acceptable and may be the whole first slice: a recorded
grammar for the heading, adopted by decision, captures most of the
value; the command mechanizes it later if recurrence justifies.

## Boundaries

- No history rewrite, no versioning system, no diff storage — Git
  already owns textual history.
- Amendment does not change lifecycle status: an amended decision is
  still `accepted`; superseding a decision outright is a different,
  future operation.
- Old free-form sections are not migrated; the grammar binds new
  writing only, matching how [[dec-reference-syntax|decision 10]]
  handled prose references.

## Evidence

Seven dated update/amendment sections across four decision files, in
two divergent grammars; sprint 6's rationale amendment; decision 14's
post-close narrowing (task 31), which is the pattern operating at its
highest stakes — correcting an incident's own output. Each was
hand-typed with no scaffold and no structural trace.

Proposed by Claude during the sprint 7 pitch, 2026-07-23.

### 2026-07-27, task 46: the grammar diverged again, mid-session

The Problem section counts two divergent heading grammars.
[[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|Task 46]] added a third and a fourth
in a single sitting, without anyone deciding to:

| Heading written | File |
|---|---|
| `## Amendment (2026-07-27): two planning assumptions were wrong` | task 43 |
| `## Amendment (2026-07-27): release topology` | sprint 8 |
| `### Amendment (2026-07-27): task 46 sits between 43 and 44` | sprint 8 |
| `## Erratum (2026-07-27, post-close)` | task 43 |

The third differs only in **heading level** — `###` because it nests
under an existing `##` amendment, which is a defensible local choice
and also exactly how free-form grammars drift. The fourth introduces a
new **noun**.

That noun is the finding, not the accident. "Erratum" was reached for
because the operation genuinely differs from the amendments above it:
an amendment revises a *live* artifact's guidance going forward, while
this correction targets the Result of a **closed** artifact whose
conclusions were already relied on downstream. The distinction is real
— one changes what to do next, the other changes what you should
believe about work already finished — and free-form prose is where it
had to be expressed, since nothing structural carries it.

So the sketch may be under-specified. A single `scarp amend` assumes
one operation; this session found two that want different words,
different scaffolds, and possibly different front-matter treatment.
An amendment to an `accepted` decision leaves it accepted; an erratum
against a `closed` task's Result arguably wants to be discoverable
from `list` without reading the body, because its whole purpose is to
stop a reader from trusting a conclusion at face value. Deciding
whether "amend" and "correct" are one operation or two belongs to the
first slice, not after the command exists.

Also relevant to boundaries: the erratum was written under an explicit
instruction *not* to rewrite the original Result. Preserving the
falsified text alongside the correction is what made the record
useful — the original local evidence was sound and its insufficiency
is the lesson. Any managed form must make append-without-rewrite the
easy path, since the tempting edit is always to fix the sentence in
place.
