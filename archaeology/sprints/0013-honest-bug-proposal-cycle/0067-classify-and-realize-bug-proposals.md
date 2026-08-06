---
id: tsk_01KZCCE4MCVWXVWXPCQ3NNHVJ7
sequence: 67
kind: task
status: pending
sprint: spr_01KZCCCPGW3V959HBDTZC56JAE
created: 2026-08-06
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
