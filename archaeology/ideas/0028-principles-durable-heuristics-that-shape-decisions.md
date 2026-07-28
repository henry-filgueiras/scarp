---
id: ide_01KYDZVN858BK52A35KJ3ZY5BP
sequence: 28
kind: idea
status: parked
created: 2026-07-25
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
recurred **four times in one sprint**, drafted below in the schema the
Sketch proposes — partly as evidence, partly to test whether that
schema survives contact with real material.

> **Statement.** A verification is blind to any defect whose
> precondition was established by the work being verified. Check in an
> environment the work did not touch, or the check proves only that
> the environment is already in the shape the work put it in.
>
> **Rationale.** Doing the work leaves residue — installed toolchains,
> warm caches, files present in the tree, a directory created by the
> first run. That residue is indistinguishable, to a local check, from
> a property of the artifact under test.
>
> **Application ordering.** Prefer a genuinely fresh environment
> (fresh `CARGO_HOME`, unpacked tarball, clean container). Failing
> that, snapshot the contaminating state and assert it did not change.
> Failing that, record explicitly which precondition the check assumed
> rather than claiming the check was clean.
>
> **Counterpressure.** Fresh environments cost wall-clock time and can
> themselves be wrong (a container that omits a dependency the real
> user has). A local check that *names its assumption* beats a slow
> one that gets skipped. The principle argues for stating the
> contamination, not for maximal isolation.
>
> **Failure signals.** A check that passes on the development machine
> and has never run anywhere else; a verification step positioned
> after the step that creates its precondition; the phrase "it works
> here" standing in for evidence; a green result whose mechanism was
> never distinguished from a plausible alternative.

The four instances, all in [[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|task 43]]
and [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]]:

1. The working tree contained `archaeology/`, so the test suite passed
   while the *packaged* crate's suite would have failed. Caught only
   by testing the unpacked tarball.
2. A warm cargo cache or a `target/debug/scarp` would have satisfied
   the install test; task 43 pre-empted this with a fresh
   `CARGO_HOME`, `CARGO_TARGET_DIR`, and install root.
3. The machine that determined MSRV = 1.88 had thereby installed the
   toolchain named `1.88`, so the local MSRV-gate check could not
   observe cargo-hack fetching it. The defect surfaced only on a clean
   GitHub runner — and surfaced there in a job that **passed**.
4. Running the quickstart once created `/tmp/scarp-demo`, so the
   documented `mkdir` could only fail on a *reader's* machine, never
   on the author's.

Instance 3 also yields a narrower companion candidate: **a passing
check is not evidence that its documented mechanism ran.** Job
90151540325 was green while its own log contradicted the workflow
comment above it. The repair was to make the mechanism falsifiable —
snapshot the toolchain list before and after and fail on any change —
rather than to trust the exit status.

That repair is the reason this belongs in a principle rather than a
retrospective: the heuristic is only useful if the *next* decision can
cite it while choosing between "verify locally and move on" and "build
a check that can fail". Sprint 8 answered that question four times
and, absent a principle artifact, left the reasoning recoverable only
by reading two task Results end to end.
