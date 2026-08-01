---
id: tsk_01KYX1WHWDG6DBCXBQH2J7YJWN
sequence: 51
kind: task
status: closed
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
closed: 2026-08-01
---

# Give scarp new a non-interactive body input surface

## Objective

Add the smallest CLI surface that lets a caller supply an artifact's
narrative content at creation time, so the ingestion workflow can
realize a proposal's prose through Scarp instead of writing Markdown
itself.

This is the sprint's only Rust change, and it exists because of a
concrete gap: `scarp new idea "Title"` accepts a title and nothing
else, and writes a template whose `Problem`, `Sketch`, `Boundaries`,
and `Evidence` sections are empty headings. Without this, the workflow
would have to append prose to a file Scarp just wrote — which is
exactly the duplication of canonicalization the sprint exists to
prevent, and which would make GitHub Actions a second author of
canonical form.

## Design constraints

**Keep the surface minimal and general.** The flag is a property of
`new`, not of ideas: it should work for any narrative collection, since
nothing about the mechanism is idea-specific. But this sprint only
requires and only verifies the idea path, and no other collection gets
special handling.

Note the deliberate asymmetry with the sprint's governance. The CLI
mechanism is collection-general; the *permission* granted to automation
by [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]] is ideas-only. Those are
different layers and must not be conflated: Scarp does not enforce the
grant, and nothing in this task should try to. A local human running
`scarp new dragon --body-file` is doing something ordinary and
permitted.

**This flag is the whole consumer story.** Once it exists, a proposal
workflow in anyone's repository is a short YAML file, because Scarp
does the canonical work. That makes this the highest-leverage change in
the sprint, and it is the reason the surface must be pleasant to invoke
from a shell script that is not ours: predictable exit codes, no
interactive prompt, no dependence on a terminal, no reliance on
anything in this repository.

**Scarp owns the canonical form.** The caller supplies section content;
Scarp decides which sections exist, in what order, and how the file is
laid out. A body that names a section the collection's template does
not have is a typed error, not a section Scarp invents. Section order
comes from the template, never from the input.

**Raw Markdown stays first-class.** Per
[[dec-bootstrap-interaction-surfaces|decision 7]], the input format is
evaluated on how the *result* reads in a raw diff. The output must be
byte-indistinguishable from a hand-filled template — no marker
comments, no provenance smuggled into the body's structure, no
formatting the local workflow would not produce.

**This is not `scarp edit`.** [[idea-strata-edit|Idea 3]] remains
parked. This task adds non-interactive input at creation time only; no
`$EDITOR` round-trip, no editing of an existing artifact, no relaxed
projection grammar. Say so in the Result so the two are not later
confused.

**Input is hostile.** [[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|Task 48]]'s
injection inventory assigns some mitigations to Scarp specifically
because a workflow cannot be trusted to hold them. Those are
requirements on this task, and the ones that matter most are the ones a
caller cannot be trusted to pre-sanitize: content that would forge a
front-matter block, content that would inject a heading at a level the
template reserves, control characters, and non-UTF-8 input.

## Questions to settle in the Result

- The exact spelling: a `--body-file <path>` flag, a stdin form, or
  both, and why. Stdin is convenient for a workflow but interacts with
  `--json`; a file is easier to audit and to test.
- The input's own format — how a caller expresses "this text belongs to
  the Sketch section". Whether that is Markdown headings matching the
  template, or something else, and how a partially-filled body behaves
  when only some sections are supplied.
- Whether a body may be supplied for collections whose template Scarp
  manages differently (sprints, tasks), or whether those are refused
  for now.
- Where the proposal's provenance ends up. Note two findings that
  constrain this: `EDGE_KINDS` is a closed allowlist and edges must
  target managed artifacts, so a GitHub issue URL cannot be a typed
  edge; and doctor treats unknown front-matter keys as inert, so a
  provenance key would pass validation while being unmanaged and
  invisible to every Scarp surface. Recommend where provenance actually
  belongs — most likely authored prose in `Evidence` plus the pull
  request and commit trail — and record why a new front-matter field
  and a new edge kind were both declined, so a later collection does
  not relitigate it. Introducing edge vocabulary without a first
  consumer is barred by [[drg_01KY169X7W0YXJ5QFV4D1MK4FB|dragon 3]].

## Acceptance criteria

- `scarp new idea "Title" <body-input>` produces an artifact whose
  sections carry the supplied content and whose front matter, sequence,
  identity, slug, and path are allocated by Scarp exactly as before.
- The output is byte-identical to the same artifact created by hand
  from the template, verified by a test rather than by inspection.
- A body naming an unknown section is refused with a typed error before
  any write, naming the offending section and the sections the
  collection has, per
  [[dec-bootstrap-error-contract|decision 4]]'s contract.
- Malformed, oversized, non-UTF-8, and front-matter-forging input are
  each refused before any write, and each is covered by a test.
- Every refusal leaves no partial file and no allocated sequence.
  Failure without content loss is the primary correctness surface here.
- `--json` output for a body-supplied creation is unchanged in shape
  from a bare creation, and remains deterministic.
- Line endings follow [[dec-lf-line-ending-policy|decision 14]]
  regardless of what the input contained, with a test for CRLF input.
- Tests use temporary directories per the project's testing
  priorities, and cover refusal to overwrite an existing path.
- The human-readable failures explain the operation, the artifact or
  path, the invariant, and the next step.
- `scripts/check.sh` passes.
- The surface is invocable from a non-interactive shell with no TTY,
  and its exit codes are documented, since the proposal workflow and
  every consumer's workflow will branch on them.
- No `$EDITOR` integration, no editing of existing artifacts, no new
  collection, and no new edge kind is added.
- The Result notes that this ships in the release
  [[tsk_01KYX31ACH05NGA3GYH0TJA870|task 56]] cuts. The channel installs
  a published `scarp`, so an unreleased flag is an unusable one, and
  this task is not done in any practical sense until it is published.
- The Result records the chosen spelling and the rejected alternatives,
  and states where provenance lives and why.

## Result

Implemented 2026-08-01 as `--body-file <PATH>` on `scarp new`, available
for every narrative collection. `scripts/check.sh` passes; the suite
went from 219 to 227 tests.

### The chosen spelling

```console
$ scarp new idea "Remote proposal ingestion" --body-file body.md --json
{"kind":"idea","id":"ide_01KYZQEA91BH8ZC8JFENP9CR1R","sequence":1,...}
```

The body is Markdown whose `## ` headings name the collection's own
template sections:

```markdown
## Problem

Ideas arrive on a phone and die there.

## Evidence

Sprint 10.
```

Sections may be supplied in any order, omitted entirely, or left empty;
Scarp renders them in **template** order, and an unsupplied section is
byte-identical to the bare template's. Deeper headings (`### …`) are
ordinary prose, because the corpus already uses them inside sections.

### Rejected alternatives

- **Stdin (`--body-file -` or a bare `--body`).** Convenient for a
  workflow, but it interacts awkwardly with `--json` on the same stream,
  and a file is auditable after the fact — a workflow can keep the exact
  bytes it fed to Scarp. A caller that has prose in a variable writes one
  temporary file. Not implemented rather than implemented-and-unused.
- **Per-section flags (`--section Problem=…`).** Unreadable at more than
  two sections, forces shell quoting of prose, and invents a second
  syntax for something Markdown already expresses.
- **JSON body.** Would make the input format disagree with the output
  format for no gain, and puts a parser between the author's prose and
  the file.
- **Refusing sprints and tasks.** Considered per the task's open
  question and rejected: supporting all five uniformly is *less*
  special-casing than excluding two, and nothing about the mechanism is
  collection-specific.

### Where provenance lives — and what was declined

Confirmed both constraints the task flagged, and both hold:

- `EDGE_KINDS` is a closed allowlist whose targets must be managed
  artifacts, so a GitHub issue URL cannot be a typed edge. Minting one
  would also violate [[drg_01KY169X7W0YXJ5QFV4D1MK4FB|dragon 3]]'s rule
  against edge vocabulary without a first consumer.
- Doctor treats unknown front-matter keys as inert, so a `proposed-by:`
  key would validate while being invisible to every Scarp surface —
  worse than useless, because it would look managed.

**Provenance therefore lives in authored prose** — the proposal's own
`Evidence` section, which the workflow composes — plus the commit
message and pull request. No new front-matter field and no new edge
kind. This is recorded so a later collection does not relitigate it:
the reason is not that provenance does not matter, but that Scarp has
no managed representation for a reference to something outside the
repository, and inventing an unmanaged one is a worse lie than prose.

### Refusals

Every refusal happens before a sequence is allocated or a path is
touched. `a_rejected_body_writes_nothing_and_burns_no_sequence` proves
it the only way that really counts: after six distinct refusals the
collection is still empty *and* the next creation still gets sequence 1.
A burned sequence would have been a silent leak — no error, no file,
just a gap the next artifact inherits.

Refused, each with a test: an unknown section (naming the offender and
the template's real sections); a duplicate section; content before any
heading; a level-1 heading; a control character; and an oversized body
(64 KiB cap).

### Task 48's injection rows, verified against the built binary

| Row | Input | Result |
|---|---|---|
| 4 | title `../../etc/passwd` | slug neutralized to `0002-etc-passwd.md`, inside the collection; nothing escaped |
| 5 | body opening `---\nid: forged\n---` | refused — content before any heading |
| 6 | body with `## Consequences` on an idea | refused, naming the four real sections |
| 7 | NUL and other control characters | refused with the codepoint |
| 7 | non-UTF-8 file | typed filesystem error, exit 6, nothing created |
| 9 | title colliding with an existing slug | existing no-clobber path, unchanged |

**Row 3 has an operational consequence for
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]].** A title beginning with
`-` needs `--` to separate it, and clap treats *everything* after `--`
as positional — so `scarp new idea -- "-rf x" --body-file b.md` fails
with "unexpected argument". The flags must precede the separator:

```console
$ scarp new idea --body-file body.md --json -- "-rf dangerous"
```

This is correct clap behaviour, not a defect, but it is exactly the
kind of thing a workflow gets wrong once and then only for hostile
titles. Recorded here so task 54 writes it correctly the first time.

### Non-interactive contract

Verified with stdin closed (`< /dev/null`): no prompt, no TTY
dependency. Exit codes are the existing typed contract — `0` success,
`2` invalid invocation (every body parse refusal), `6` filesystem
failure (unreadable or non-UTF-8 body file). Automated callers branch
on these without parsing prose.

### Line endings

CRLF input is normalized at parse time, so the artifact never carries
the caller's line endings regardless of what platform authored the
proposal ([[dec-lf-line-ending-policy|decision 14]]).

### Not done, deliberately

No `$EDITOR` integration — [[idea-strata-edit|idea 3]] stays parked, and
this is creation-time input only, not an edit-through-projection flow.
No editing of existing artifacts, no new collection, no new edge kind,
no lifecycle change.

### Incidental fix

`validate_title` has always said "cannot create **a** idea"; `idea` is
the only vowel-initial managed kind. Since this task added four more
messages inheriting the same pattern, an `article()` helper now derives
it rather than hardcoding, so a future vowel-initial collection does not
silently reintroduce it.

### Release dependency

This surface is unusable by the proposal channel until it is published:
the workflow installs a pinned released `scarp`, not this checkout. That
is [[tsk_01KYX31ACH05NGA3GYH0TJA870|task 56]], and this task is not done
in any practical sense until it lands.
