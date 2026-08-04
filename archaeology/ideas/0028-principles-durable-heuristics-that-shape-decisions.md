---
id: ide_01KYDZVN858BK52A35KJ3ZY5BP
sequence: 28
kind: idea
status: adopted
created: 2026-07-25
adopted-by: "[[tsk_01KZ738BJ5MXNBDWECX8REA391|Add the principle collection and distil the first principle from log 3]]"
---

# Principles: durable heuristics that shape decisions

## Problem

The archaeology records concrete choices (decisions) and unresolved
risks (dragons), but has no home for durable engineering heuristics
that influence many future decisions without themselves being
decisions. Examples that already operate in this repository:

- minimize hostage-taking of host repositories;
- prefer Strata-owned storage over mutation of shared repository
  files;
- when mutation of shared files is unavoidable, prefer reversible,
  conflict-aware managed mechanisms.

Today such guidance lives in two unsatisfying places. CLAUDE.md's
"Core invariants" section is a hand-maintained principle registry with
no identity, lifecycle, or citability; and some principles get
recorded as decision-shaped artifacts
([[dec-bootstrap-files-canonical|decision 1]] is really a principle
wearing a decision's clothes). Neither form lets a future decision
cite the heuristic it applied — or explicitly overrode — so the
reasoning pressure that shaped a choice stays invisible.

The distinction from decisions:

- a decision records a concrete choice made in context;
- a principle provides reusable guidance shaping many future choices;
- decisions may cite principles they apply;
- decisions may intentionally override principles with explicit
  rationale — an override is a signal worth surfacing, not a
  violation.

## Sketch

A `principle` collection whose artifacts carry:

- the principle statement;
- rationale;
- preferred application ordering (which alternatives to reach for
  first);
- counterpressure: legitimate exceptions and when they win;
- failure signals that suggest the principle is being violated or has
  gone stale;
- related decisions, as they accumulate.

Relations stay in the existing reference model: decisions cite
principles with typed edges (e.g. `applies` / `overrides`) whose
values are bound markers, per decision 10. Tasks and reviews would
reference principles as reading, not as validation: surfacing
applicable principles during implementation or audit is a retrieval
question already parked as
[[idea-proposal-relevance-surfacing|proposal-time relevance
surfacing]], and principles would simply be a high-yield corpus for
it.

Lifecycle is an open question: principles likely need `active` and
some terminal state (`retired`, `superseded`), and their amendment
story overlaps [[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|idea 26]]'s dated
in-place supersession. A sixth managed collection would also feed the
copy-versus-extract evidence for
[[idea-declarative-collection-specs|declarative collection specs]].

## Boundaries

- Not a bootstrap item; no collection work is justified until the
  corpus demonstrates need (principles being cited from real
  decisions, or an override worth recording).
- Principles advise; they do not validate. A decision that overrides a
  principle is not a doctor finding, and no tooling may enforce
  principle conformance structurally.
- Not a general knowledge base: a principle must be actionable
  guidance with counterpressure, not a mission statement.
- CLAUDE.md is not migrated wholesale; which invariants become
  principle artifacts is a per-item editorial call at adoption time.
- "RFCs" are not an artifact type; how external proposals relate to
  principles waits on [[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|reviewable
  mutation intents]] evidence.

## Evidence

Origin: Henry, 2026-07-25, distilled from the review that produced
[[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|idea 22]], where the example
principles above were applied in the negative — rejecting a staging
collection because it mutated the host repository's shape. Prior art:
architectural principles registries (e.g. government/enterprise
"architecture principles" catalogs), Amazon-style tenets ("unless you
know better ones"), and ADR practice, which distinguishes decisions
from the forces acting on them; the forces are the principles.

### 2026-07-27, sprint 8: a candidate principle with four instances

The Boundaries section holds that no collection work is justified
until the corpus demonstrates need. Sprint 8 supplied a candidate that
recurred **four times in one sprint**:

> A verification is blind to any defect whose precondition was
> established by the work being verified.

with a narrower companion — *a passing check is not evidence that its
documented mechanism ran*. The statement, its rationale, application
ordering, counterpressure, failure signals, and all four instances are
recorded in
[[log_01KYK8RC0YEY51YP37RGV7M7N4|log 3, verification blind spots]].
That log is the canonical copy, deliberately: this idea may be
rejected, and the reasoning has to survive that.

Two things it demonstrates for this idea specifically.

**The proposed schema survived contact with real material.** The
candidate was drafted in the Sketch's shape — statement, rationale,
application ordering, counterpressure, failure signals — and every
field had something non-obvious to hold. Counterpressure in
particular was not filler: isolation costs wall-clock time and a
wrong clean environment proves the opposite of what was intended, so
the principle argues for *naming* contamination rather than for
maximal isolation. A principle without that field would have read as
an absolute and been overapplied.

**The citation need is concrete, not hypothetical.** The heuristic
earns its keep when a future task chooses between "verify locally and
move on" and "build a check that can fail" — the choice sprint 8 faced
four times. Absent a citable artifact, that reasoning is recoverable
only by reading two task Results end to end, which is how it was
nearly lost. Writing it as a log is the available approximation;
what a log cannot do is be cited by a decision as `applies` or
`overrides`, which is precisely the gap this idea proposes to close.

### Adopted 2026-08-04, sprint 12 task 63: what was built and what was not

The collection exists. `archaeology/principles/` is managed, its
template is this idea's five fields in this idea's order, and
[[prn_01KZ76WRJ5QMEDGCPB6076HEAC|principle 1]] carries the
verification blind-spot heuristic distilled from
[[log_01KYK8RC0YEY51YP37RGV7M7N4|log 3]], which is unchanged and
remains the canonical account of how the shape was found. `doctor`
validates a principle's structure exactly as it validates any
artifact's, and judges conformance to one never — the Boundaries
section's rule survived intact.

Three parts of the sketch were deliberately not built.

**Lifecycle.** Principles ship with one admitted state, `active`, and
no transitions, as decisions do. The sketch left lifecycle open and
guessed at `retired`/`superseded`; adjudicated 2026-08-04, that guess
stays unmade, because there is evidence for durable principles — four
instances in one sprint — and none for principle retirement. A
principle superseded by a sharper statement of the same force, one
overtaken by a changed world, and one found to have been wrong are
three different events, and the last two look more like this idea's
neighbour [[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|idea 26]] than like a
lifecycle transition. Single-state rather than stateless is what keeps
the deferral cheap: the `status:` line already exists, so admitting a
terminal state later is a transitions-table change with no corpus
migration.

**`applies` / `overrides` edges, with the cost now measured rather
than estimated.** Declaring an edge kind is two `EDGE_KINDS` entries
and nothing else; `settled_status` is trivially `accepted`, since a
decision has no other state. *Authoring* one is the expensive half,
and task 62 established why: the only authoring surface is
`resolve_edge`, reached exclusively from a lifecycle verb's flag
(`close --resolved-by`, `adopt --adopted-by`). Decisions have no
lifecycle verb, so an `applies` edge would need either a
`new decision --applies` flag or a hand-edited front-matter line —
and a hand-edited bound marker is exactly the transcription fault
[[idea-doctor-reference-graph|idea 2]] now records twice. Deferred
until a decision genuinely cites a principle; no decision written in
this sprint does. Inventing a decision mutation surface to complete
the sketch would have been the sketch driving the design.

**"Related decisions, as they accumulate"** — the sketch's sixth
field — is not a template section. It is the edges above under
another name, and it waits on them.

One finding from writing the first principle. Its provenance —
which log it came from, and that the log is not superseded — wanted
to be its own section, and the template does not have one, so it was
folded into `Rationale`, where it reads acceptably. That is the
[[ide_01KYZY233Z7GAKFPFSKEAF89ZD|idea 41]] tension appearing on its
first day of use, in a template designed from a worked example. It is
recorded rather than fixed: a sixth section added on one specimen's
evidence would be the same premature guess the lifecycle deferral
avoided.
