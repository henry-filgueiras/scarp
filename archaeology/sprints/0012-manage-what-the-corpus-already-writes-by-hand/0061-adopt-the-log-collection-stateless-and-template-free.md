---
id: tsk_01KZ738BECT3VAFX99CKPM9VDB
sequence: 61
kind: task
status: pending
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
---

# Adopt the log collection, stateless and template-free

## Objective

Make `archaeology/logs/` a managed collection, admitting the two things
its existing three artifacts prove a collection can lack: a lifecycle
and a section template.

The corpus is the specification. All three logs already satisfy every
rule the reader enforces — `NNNN-slug.md` filenames agreeing with their
front-matter sequences, non-empty `created:`, a single `# Title`
heading, and ids in both the hand-seeded (`log-bootstrap-inception`) and
generated (`log_01KY…`) forms the model already tolerates. The only
thing they lack is `status:`, which `FrontMatter` currently requires of
everything. Stamping `status: recorded` into three files to satisfy the
parser would invent a lifecycle the corpus does not have, and would be a
migration of canonical history performed for the tool's convenience. The
parser gives way instead.

Their sections diverge just as sharply: log 1 is bare prose under its
title, log 2 and log 3 each invent their own headings. There is no log
template to derive, so logs get none, and `--body-file` for a
template-free collection accepts its body verbatim rather than refusing
every `## ` heading in it.

Logs are already visible to `edges::Catalog`, which is why idea 28's
bound reference to log 3 resolves today. Adoption moves them from
identity-only visibility into the artifact scan.

## Acceptance criteria

- `scarp new log "<title>"` writes `archaeology/logs/0004-<slug>.md`
  with a `log_`-prefixed ULID id, no `status:` line, and no section
  scaffolding beyond the title heading.
- `scarp new log --body-file` accepts a body containing arbitrary `## `
  headings verbatim, and still refuses oversize bodies, control
  characters, and CRLF on the same terms as every other collection.
- `scarp list logs` and `scarp show log:3` resolve all four logs,
  including the two hand-seeded ids, by sequence and by stable id.
- `scarp list logs --json` omits the `status` key entirely rather than
  emitting `null`, and the `--json` output of every pre-existing
  collection is byte-identical to its output before this task. A test
  pins both halves.
- `scarp doctor` counts the logs among its checked artifacts (132 rises
  to 135) and reports no findings.
- A log file carrying a `status:` line is a `malformed-artifact` finding
  whose reason says logs have no lifecycle, rather than being silently
  ignored. Silently tolerating it would let a fake lifecycle accumulate
  in a collection that has none.
- Every lifecycle verb refuses a log target with a message naming the
  reason, in the shape `verb_guidance` already uses for decisions.
- `git diff --stat archaeology/logs` is empty over the whole task: no
  existing log gains, loses, or reorders a byte.
- A decision records that a managed collection may have no lifecycle,
  that `status:` is required exactly of collections that declare states
  and refused for those that do not, and that a collection with no
  managed sections takes its body verbatim.
- `scripts/check.sh` passes.
