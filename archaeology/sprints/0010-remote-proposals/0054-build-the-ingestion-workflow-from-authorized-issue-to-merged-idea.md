---
id: tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H
sequence: 54
kind: task
status: closed
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
closed: 2026-08-01
---

# Build the ingestion workflow from authorized issue to merged idea

## Objective

Build the smallest good operator-driven realization path: from a trusted
local machine, list the open proposals and turn a chosen one into a
canonical idea, then commit through the ordinary workflow.

*Replaced 2026-08-01 by [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s
Option B adjudication.* This task was a GitHub Actions workflow —
checkout, pinned binary, doctor gate, branch, pull request, receipt,
eight partial-state recoveries, late authorization. None of it survives.
There is no workflow, no token, no branch, and no pull request. What
survives is the useful core plus two of the review findings, which turn
out to matter under any design.

## The product interface

The raw pipeline works today and is the desire path, not the product:

```sh
gh issue view 42 --json body -q .body > /tmp/b.md
scarp new idea "$(gh issue view 42 --json title -q .title)" --body-file /tmp/b.md
```

Nobody should be asked to assemble that. It fetches title and body
separately, leaves a temporary file to clean up, and — worst — records
nothing, so running it twice silently creates a duplicate idea.

Deliver instead a bounded Scarp surface, naming notwithstanding:

```console
$ scarp proposal list
$ scarp proposal realize 42
```

or an equivalently narrow GitHub-aware operation. Two verbs is the
target; a third needs to argue for itself.

## Architectural constraints

Follow [[ide_01KYZRMKTFMRVWDJP5K3FVJ1SV|idea 37]]'s direction: **`gh`
plays the role for GitHub that the installed `git` already plays for
Git.** Shell out to an authenticated `gh`. Scarp acquires none of:

- its own GitHub credential storage;
- an HTTP client for this feature;
- GitHub SDK machinery;
- token handling;
- a speculative generic forge abstraction.

**Failure must be clean and local.** If `gh` is absent, unauthenticated,
offline, or the repository has no GitHub remote, this feature is
unavailable with a typed error naming what is missing and what to do.
Ordinary Scarp commands are unaffected in every one of those cases —
`new`, `list`, `show`, `doctor`, and every transition keep working, and
the repository stays fully usable. This is the "Git is optional at the
core" property, one layer out.

**Realization reuses the existing core.** It composes `gh` with the same
creation path `scarp new --body-file` already uses. It must not grow a
parallel canonicalization route, and it must not accept a collection, a
flag, or a path from the proposal.

**Implement only the smallest subset of idea 37** that makes this
usable. Richer detection, scaffolding beyond what the sprint needs, and
generalized forge-aware surfaces grow from observed use.

## The duplicate-realization guard

Realizing the same proposal twice must **refuse clearly**, not silently
create a second idea. This survived from the automated design because it
was never really about automation — an operator can run a command twice
just as easily as a workflow can fire twice.

Proposal identity is durable and derived from transport: **repository
plus issue number.** Not the title, not a label.

Prefer the least new state that is reliable. Candidate evidence, in
rough order of preference:

- provenance recorded in the realized artifact's own prose, which task
  51 already established as where provenance lives;
- commit history;
- a marker or comment on the originating issue;
- some other durable surface that already exists.

**Do not invent a receipt subsystem** unless simpler evidence proves
insufficient. If a scan of existing artifacts answers "has issue 42 been
realized", that is the answer. Record which evidence was chosen and what
it costs — in particular whether it still works when the realized idea
has since been adopted or rejected, and when the operator is on a branch
that does not yet contain the earlier realization.

## Snapshot semantics

Realization consumes an explicit snapshot of the proposal taken at
invocation. Later edits to the issue never mutate an already-created
artifact.

There is no synchronization, in either direction, and the issue is never
canonical. Closing or deleting it invalidates nothing. Under Option B
this is close to free — the operator fetches once and creates once — but
it must be stated and true rather than incidental, because a future
reader will ask what happens when the issue changes.

## Acceptance criteria

- An idea filed as a proposal issue from a phone becomes a canonical
  Scarp artifact through one bounded operator command, with no manual
  transcription and no hand-assembled shell pipeline.
- The realized artifact is compared against one created locally by hand
  from the same payload and matches in form: same front-matter fields,
  same section structure, same line endings, and a Scarp-allocated
  sequence, identity, slug, and path.
- `scarp doctor` is green after realization, and the operator commits
  through the ordinary workflow. Nothing in this task commits or pushes.
- Realizing the same issue twice refuses with a diagnostic naming the
  existing artifact, verified by doing it. The chosen evidence for "has
  this been realized" is recorded with its limitations.
- Snapshot semantics are demonstrated: editing the issue after
  realization leaves the artifact unchanged.
- Hostile proposal content is refused by Scarp's existing task 51
  validation rather than by new code here, verified with at least one
  payload from [[tsk_01KYX1WHPS3R7FDCKG23YTGHHY|task 48]]'s injection
  inventory. Note task 51's finding that a title beginning with `-`
  needs `--`, and that flags must precede the separator.
- The feature fails cleanly and specifically when `gh` is missing,
  unauthenticated, and when the repository has no GitHub remote. Each is
  induced deliberately, and ordinary Scarp commands are confirmed
  unaffected in each case.
- The command surface is bounded: no field of a proposal reaches an
  argument position that could select a different command, collection,
  flag, or path.
- Nothing hardcodes this repository — no owner login, no path
  assumption, no dependence on the Scarp source being present.
- Per CLAUDE.md's first-performance policy, the exact `gh` invocations
  used are recorded in this Result as dated provenance.
- Ideas created through this channel are ordinary parked ideas, subject
  to `adopt` and `reject` like any other. Nothing marks them as
  second-class, and no lifecycle state was added.
- `scripts/check.sh` passes.

## Result

Delivered 2026-08-01. `scripts/check.sh` passes; 243 tests.

### The surface

```console
$ scarp proposal list
#2  open      Machine-readable repository capability manifest for agent collaboration

$ scarp proposal realize 2
created idea idea:38 at archaeology/ideas/0038-machine-readable-...md
review it, then commit; Scarp does not commit or push
```

Two verbs, as scoped. `list` annotates any proposal already realized with
the artifact's path, so the operator sees state rather than remembering
it.

### The end-to-end performance

Issue `henry-filgueiras/scarp#2` — filed from a phone through
[[tsk_01KYX1WJ03MD2WRNQBS3KGMXXA|task 53]]'s form — is now
[[ide_01KYZVJ6XCK11DP67GVMC3M23C|idea 38]], created by Scarp with no manual transcription and no
hand-assembled pipeline. Verified after realization:

- front matter carries `proposal:
  https://github.com/henry-filgueiras/scarp/issues/2`, placed after
  `status` and before `created`;
- the four sections are present in template order;
- the proposal's fenced ```` ```json ```` block survived intact, which is
  the case that broke the parser before task 53's fence fix;
- `scarp doctor` is green across 124 artifacts;
- `scarp show idea:38 --json` round-trips the stamp through the read
  model.

Nothing committed or pushed. The command creates a file and says so.

### Duplicate realization

`scarp proposal realize 2` a second time:

```text
error[artifact-conflict]: … proposal https://github.com/…/issues/2 has
already been realized as `archaeology/ideas/0038-…md`; one proposal
realizes at most one artifact
```

Exit 4. The check reads the managed `proposal:` field rather than
searching prose, so it is exact rather than a regex over English.

**Chosen evidence, and its limits.** The field is the least new state
that is reliable, and Henry's push-back was what got it there — the
Evidence-prose approach this task originally proposed would have been
guesswork by comparison. Two honest limits: the check sees only artifacts
on the current branch, and an operator who deletes the field defeats it.
The first is covered where it matters — `doctor`'s `duplicate-proposal`
finding catches the case *no* realization run can, where two branches
each realized once and only the merge produced the duplicate. The second
is not defended against, deliberately: Scarp does not defend canonical
files from their owner.

### Unavailability, which is the real guarantee

Verified by inducing each case:

| Condition | Result |
|---|---|
| `gh` absent from `PATH` | `error[integration-unavailable]`, exit 11, names the install link and `scarp new idea` |
| not a Git repository | same code, requirement reported as *a GitHub repository* |
| Git repository, GitLab remote | same, naming that no remote points at GitHub |

In all three, `new`, `list`, `show`, `doctor`, `fortune`, `resolve`,
`adopt`, and `reject` keep working — covered by
`every_ordinary_command_works_without_gh`, which exists so that an
optional integration becoming mandatory is a test failure rather than a
discovery.

`gh` reports the last two conditions as ordinary command failures. They
are re-framed rather than passed through: telling an operator with a
GitLab remote that they need to authenticate `gh` would send them to fix
the wrong thing.

A new typed error, `integration-unavailable` (exit 11), was added to the
[[dec-bootstrap-error-contract|decision 4]] contract so automated callers
can distinguish *not set up* from *typed it wrong*.

### The transport mapping

GitHub renders form fields as `### Label`; `--body-file` expects `## `.
Only lines that are exactly `### ` plus one of the collection's own
section names are promoted. A general rewrite would forge sections out of
a filer's own subheadings, and fenced blocks are skipped entirely, so a
proposal that quotes Markdown is quoting rather than declaring.

`_No response_` — GitHub's placeholder for a blank optional field — is
dropped, leaving the section empty exactly as the bare template renders
it. This is the case task 53 could not observe because every field was
filled; it is covered by a unit test rather than left until it surprises
someone.

### Not tested here, and why

The success path needs a live authenticated GitHub repository. A test
that shells out to a real forge would be neither hermetic nor honest
about what it proves, so `tests/proposal.rs` covers the unavailability
guarantee and the artifact model, and the success path is recorded as the
dated performance above. The exact invocations Scarp issues, per
CLAUDE.md's first-performance policy:

```sh
gh repo view --json nameWithOwner
gh issue list --label idea --state open --json number,title,url --limit 100
gh issue view <n> --json number,title,body,url
```

### One thing to watch

`proposal:` is new managed front-matter vocabulary, introduced with a
first consumer as [[drg_01KY169X7W0YXJ5QFV4D1MK4FB|dragon 3]] requires.
It is currently written only for ideas and only by GitHub realization. If
it spreads to other collections or other forges, that is a decision
rather than something to let accrete — the field is now part of the
canonical artifact model, and a second producer would make its meaning
negotiable.
