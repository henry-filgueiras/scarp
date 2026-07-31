---
id: tsk_01KYX1WHWDG6DBCXBQH2J7YJWN
sequence: 51
kind: task
status: pending
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
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
- No `$EDITOR` integration, no editing of existing artifacts, no new
  collection, and no new edge kind is added.
- The Result records the chosen spelling and the rejected alternatives,
  and states where provenance lives and why.

## Result
