---
id: tsk_01KZ738BECT3VAFX99CKPM9VDB
sequence: 61
kind: task
status: closed
sprint: spr_01KZ7352BYX19E0DNDG05744AM
created: 2026-08-04
closed: 2026-08-04
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

## Result

Delivered as specified. Logs are a managed collection, and not one byte
of the three pre-existing logs changed — `git diff archaeology/logs` is
empty across the whole task, which is the claim the objective rested on.

**Shape.** Two absences, both expressed as ordinary `Collection` data
rather than as special cases: `states: &[]` means stateless,
`sections: &[]` means template-free. `Summary::status` became
`Option<Status>` with `skip_serializing_if`, so a stateless artifact
omits the key and every stateful collection's `--json` output is
byte-identical to before. `Body` became an enum whose `Verbatim` arm
carries the whole body for a template-free collection. Recorded as
[[dec_01KZ74BQQJ1W5Q32GQS7RD4JCK|decision 17]].

The two design choices worth stating, because both could have gone the
lazy way:

- **A `status:` line on a log is malformed, not ignored.** Serde would
  have dropped it silently. Tolerating it lets a lifecycle vocabulary no
  code admits accumulate in the corpus, and the point of decision 11 is
  that front matter is the *one* authority — which it cannot be if it
  may carry claims the collection denies.
- **`status:` is now explicitly required of stateful collections.** It
  had been enforced only by the deserializer, so a dragon missing its
  status produced an opaque serde error. Making statelessness legal
  meant making the requirement a checked invariant with a message that
  names the collection's vocabulary.

**The corpus specified the collection, twice.** Both absences were read
off the three existing logs rather than chosen: none carries `status:`,
and they share no section vocabulary (log 1 is unbroken prose, logs 2
and 3 each invent their own headings). The alternative — stamping
`status: recorded` into three canonical files so the parser would accept
them — would have invented a lifecycle and rewritten history to make the
tool's job easier. That the adoption needed no migration at all is the
evidence that the model was wrong rather than the files.

**Fences matter in the verbatim path.** A template-free body still
refuses a level-1 heading, because a second `# ` would make the written
artifact fail title extraction on read-back. That refusal had to become
fence-aware: title extraction ignores fenced text, so refusing a shell
snippet's `# install it` would have rejected a body that reads back
perfectly. The fence state machine is now shared by both body paths
rather than duplicated.

### Two things found by doing the work

**Adopting a collection breaks the tests that used it as an example of
an unmanaged one.** Two doctor tests seeded `archaeology/logs/` to
produce a catalog claimant outside the managed set; the moment logs
became managed, one fixture turned into a malformed managed artifact and
both tests failed. They now name genuinely unmanaged kinds. This is a
predictable cost of every future adoption and is recorded in decision
17's consequences, not just here — `principle` and `maintenance` should
each expect it.

**The `132 rises to 135` figure in the acceptance criteria was stale
before execution began.** Commissioning the sprint added eight artifacts
after the criterion was written. The delta is what the criterion meant
and the delta is right: 140 before this task, 144 after, being three
logs and this task's decision. Behavioural criteria that quote an
absolute count of a growing corpus date badly; the next ones should
quote deltas.

### Desire-path friction

This `## Result` arrived by `cat >>`, hand-matching a heading level
against a template nothing validates — the exact friction
[[tsk_01KZ738BG7HDGBJDM57TW40ED5|task 62]] exists to remove, recorded
one task before the fix lands. `scarp close` was then run separately, so
the closure was two operations and two chances to forget the second.
