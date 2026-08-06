---
id: tsk_01KZCCE4MCVWXVWXPCQ3NNHVJ7
sequence: 67
kind: task
status: closed
sprint: spr_01KZCCCPGW3V959HBDTZC56JAE
created: 2026-08-06
closed: 2026-08-06
---

# Classify and realize bug proposals

## Objective

Teach `scarp proposal` two source classes instead of one hardcoded label,
and make realization pick its target collection from what the issue is
labeled rather than from a constant.

Generalize only as far as two explicit classes require. No provider
registry, no many-to-many framework, no label configuration surface.

The adjudicated semantics:

| Recognized label | Command | Canonical result |
|---|---|---|
| `idea` | `proposal realize N` | parked idea |
| `bug` | `proposal realize N` | pending maintenance item |
| `bug` | `proposal realize N --sprint sprint:X` | pending task in that active sprint |
| neither or both | either form | typed refusal before any write |

Unrelated labels are ignored; exactly one *recognized* label must remain.

Realizing a `bug` means accepting an obligation to investigate, not
asserting the report is true, so the generated title is
`Investigate reported behavior: <issue title>`.

## Acceptance criteria

- `realize` fetches the issue's labels and classifies from them. This
  closes the current defect where any issue number at all can be realized
  as an idea.
- `idea` without `--sprint` preserves existing behavior and output
  byte for byte.
- `bug` without `--sprint` creates a pending maintenance item.
- `bug --sprint <selector>` creates a pending task owned by the selected
  sprint, using the same selection and validation semantics as
  `scarp new task --sprint`: `sprint:N` or a stable id, refusing a
  non-sprint reference, a missing sprint, an ambiguous one, and a sprint
  that is not active.
- `idea --sprint` is refused as an invalid invocation.
- Neither recognized label and both recognized labels are refused without
  allocating a sequence or touching the filesystem.
- Duplicate detection scans ideas, maintenance, and tasks: one proposal
  URL realizes at most one canonical artifact across all three.
- Canonical creation is reached through focused `create_maintenance_from`
  and `create_task_from` seams beside the existing `create_idea_from`,
  never by bypassing it. The `proposal:` field keeps its current
  one-shot semantics, validation, and doctor coverage.
- `proposal list` discovers the union of open `idea` and `bug` issues and
  exposes each one's default target — `idea` or `maintenance` — in both
  the human and `--json` projections, deterministically ordered.
- The union is built from two explicit queries plus deterministic
  deduplication, because repeated `gh issue list --label` flags are an
  AND, not an OR.
- A dual-labeled issue is surfaced as ambiguous rather than silently
  assigned: `list` refuses, naming the issue and its conflicting labels,
  instead of returning a misleading partial classification.
- Bug bodies land readably inside canonical sections: maintenance gets
  one `## Work` holding the normalized report; a task gets `## Objective`
  holding it plus a mechanically generated `## Acceptance criteria`
  requiring an evidence-backed terminal result and regression coverage
  when a confirmed defect changes behavior.
- Reporter-authored Markdown cannot forge a canonical `##` section.
  Fenced code is preserved untouched; headings outside fences are nested
  beneath the canonical section. GitHub's `_No response_` placeholder is
  still removed.
- Tests cover every row of the table above, unrelated labels,
  `idea --sprint`, duplicate detection across all three collections,
  deterministic list and JSON output, body normalization, active-sprint
  selection, and proof that every refusal happens before allocation or
  write.
- The public API changes are accounted for, including the added `sprint`
  field on the exhaustive `ProposalCommand::Realize` variant and the
  added `ProposalSummary` field.
- `scripts/check.sh` passes and the slice is committed with its
  archaeology.

## Result

Done. `scarp proposal` now has two source classes, and `realize` picks
its target collection from the issue's labels rather than from a
constant.

**The defect this closed on the way.** `realize` never fetched labels, so
any issue number at all could be realized as an idea — a support
question, a pull request, someone else's bug report. Classification had
to exist for the sprint's own sake, and repairing that was free once it
did.

**Shape.** A private `Class` enum with two variants and a two-row
`RECOGNIZED` table, not a registry. What the two classes actually need
from each other is different *semantics* — `Class::creation_aware` is the
asymmetry the whole sprint turns on — and a label-to-collection lookup
table would not have expressed that anyway.

`classify` is pure and is the first thing every command does. Unrelated
labels are ignored; exactly one recognized label must remain. Neither and
both are both `invalid-invocation`, exit 2, with distinct messages. One
code for both rather than splitting `precondition-unmet` off for the
ambiguous case: both are "Scarp cannot act on the issue you named", both
are repaired by relabelling on GitHub, and the existing non-proposal
refusal already used exit 2.

**Creation seams.** `create_maintenance_from` and `create_task_from` sit
beside `create_idea_from`, and the plain `create_maintenance` /
`create_task` now delegate to them, so there is one creation path per
collection rather than a stamped and an unstamped one. The `proposal:`
validation moved into a shared `validate_proposal`, so a value one path
would refuse cannot enter through another. On a task the stamp renders
after `sprint:` — the owning sprint is the more load-bearing field.

**Duplicate detection is global, not per-collection.** `realized_from`
scans ideas, maintenance, and tasks in a fixed order. A per-collection
check would have let one report become both a maintenance item and a
task, which is exactly the duplicate the field exists to prevent. Proven
in both directions: maintenance-then-task, and idea-then-maintenance
after a relabel.

**`list` runs two queries and deduplicates by number.** One query with
repeated `--label` flags would have been wrong in a way that looks right:
GitHub *intersects* repeated label filters, so a single call returns only
the issues carrying every recognized label — the ambiguous set, and an
empty result for the ordinary case. A `BTreeMap` keyed by issue number
makes the union deterministic whatever order `gh` returned either page
in. A dual-labeled issue refuses the whole listing; a listing is what an
operator reads before deciding what to realize, so one confidently wrong
row is worse than no listing.

**Bodies.** A bug report lands whole inside one section Scarp owns —
`## Work` for maintenance, `## Objective` for a task — beneath
mechanically generated framing that says promotion is an obligation to
look, not a finding, and names the honest terminal findings. Task
acceptance criteria are generated too: an outside reporter has no way to
know what this project considers done.

`to_report` demotes every unindented level-1 and level-2 ATX heading
outside fenced code to level 3. Unindented is the whole surface that
needs covering, because the body parser recognizes a section only as a
line beginning exactly `## `. Fenced code survives byte for byte: a
report quoting Markdown is quoting, and rewriting the quoted lines would
corrupt the evidence to protect a structure they were never going to
reach. The forgery test files a report containing `## Work`, `# Title`,
and a fence holding both, and checks that exactly one `## ` and one `# `
survive outside the fence while the fenced copies are untouched.

The idea path is byte-identical to before: same title, same sections in
template order, same `_No response_` handling, same output line.

**Public API.** `ProposalCommand::Realize` gained a `sprint` field and
`ProposalSummary` gained `target`; both are breaking, because nothing
here is `#[non_exhaustive]`. `proposal::realize` gained a third
parameter. Recorded for [[mnt_01KZA6MH5SCW0MDEJTKKW26Y9G|Publish 0.3.0 to crates.io]] in task 69.

## Verification

`scripts/check.sh` — passes. 264 lib tests, doctor 153 artifacts clean.

New `tests/proposal_cycle.rs`: 18 tests through the compiled binary
against a fake `gh` — a shell script on `PATH` reading canned JSON and
logging every invocation. Every row of the adjudicated table, unrelated
labels, `idea --sprint`, duplicate detection across all three
collections, closed-sprint and nonexistent-sprint and non-sprint-reference
refusals, forged sections, `_No response_`, the two-query union, stable
`--json` with `target`, the realized-annotation, and the dual-label
refusal. Every refusal test asserts the target collection is still empty.

The harness proves the half Scarp owns and nothing about GitHub: a
passing run says the arguments Scarp builds are the ones it intends to
build, not that GitHub answers them as assumed. Its module doc says so,
so it cannot later be cited as a live performance.

New unit tests in `src/proposal.rs` cover `classify` over all four label
shapes, the creation-aware asymmetry, heading demotion including
`#hashtag`, fence preservation, CRLF and placeholder handling, and that
the generated framing never says "the bug".

## Not done here

Reconciliation still treats every class the same — it enforces the
one-label rule and looks up the realizing artifact across all three
collections, but the terminal-result gate is [[tsk_01KZCCE4PDVWPMM6VY1XC0X3FJ|Reconcile bug proposals at their terminal result]]'s.
