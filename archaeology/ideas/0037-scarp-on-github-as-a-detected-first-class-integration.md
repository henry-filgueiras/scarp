---
id: ide_01KYZRMKTFMRVWDJP5K3FVJ1SV
sequence: 37
kind: idea
status: parked
created: 2026-08-01
---

# Scarp-on-GitHub as a detected first-class integration

## Problem

The pattern that makes remote proposals work without any automation
holding a token is a pipeline the operator runs by hand:

```sh
gh issue view 42 --json body -q .body > /tmp/b.md
scarp new idea "$(gh issue view 42 --json title -q .title)" --body-file /tmp/b.md
```

It works today. It is also unpleasant enough that nobody will do it
twice without wanting the tool to know about it: the title and body are
fetched separately, the temporary file is bookkeeping, and nothing
records that issue 42 has already been realized — so the second run
silently creates a duplicate idea.

The broader gap is that Scarp has no notion of the forge its repository
lives on, even though the forge is where the collaboration actually
happens. Every GitHub-shaped affordance — proposal issue forms, a
listing of unrealized proposals, the realization step itself — has to be
hand-assembled per repository, and a consumer copying the pattern
reassembles it from prose rather than getting it.

## Sketch

Detect the forge and let it unlock a bounded command surface. Detection
is the same shape as repository discovery: a Git remote pointing at
`github.com` makes GitHub-aware commands available; their absence makes
them unavailable rather than broken.

Two halves, and only one of them touches a network:

- **Scaffolding** — writing the proposal issue form and any other
  GitHub-shaped files into `.github/`. This is ordinary file creation
  with no network at all, and it is most of what a consumer needs to be
  handed rather than told.
- **Realization** — listing open proposal issues and turning a chosen
  one into a canonical artifact through the same core `new` path the
  CLI already uses.

**The architectural key is that `gh` should be to GitHub what `git` is
to version control here.** The Rust guidance already says to avoid
`git2` and shell out to the installed `git` for narrowly scoped
Git-aware features. The same rule applied to the forge gives the
first-class ergonomics without Scarp becoming a network client: no HTTP
stack, no API SDK, no credential handling, no token storage, no
rate-limit logic. Scarp shells out to an authenticated `gh` exactly as
it shells out to `git`, and inherits the operator's existing
credentials rather than acquiring any of its own.

This also settles the invariant question cleanly. "Git is optional at
the core" already licenses optional integrations: Git-aware features may
add provenance and change detection, but Git is never required. A
GitHub-aware feature sits in the same slot one level out — never
required, never load-bearing for reading or modifying a repository, and
absent-by-default outside a GitHub remote. What must stay true is that
no canonical operation depends on it, and that a repository whose
`gh` is missing, unauthenticated, or offline remains fully usable with
a typed error rather than a broken command.

Realization needs to know what it has already realized, or it will
create duplicates. The cheapest honest answer is to look: an issue whose
realized artifact already exists is already recorded in that artifact's
prose and in the commit that added it. Whether that is sufficient, or
whether something durable is needed, is deliberately unsettled — see
the receipt discussion in
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]], which faced the same
question under much harder conditions.

## Boundaries

- Scarp acquires no network stack, no API client, and no credentials.
  If `gh` cannot be shelled out to, the feature is unavailable, not
  degraded.
- No synchronization. Realization is one-shot: the issue is never
  canonical, nothing mirrors state back, and closing or deleting the
  issue invalidates nothing. The
  "GitHub Issues synchronization" non-goal survives intact.
- No forge abstraction layer. GitHub is the only forge with a first
  consumer; a provider interface with one implementation is speculative
  framework-building, and GitLab or Forgejo support should wait for
  someone who wants it.
- Not a replacement for the plain CLI. Everything the integration does
  must be expressible as ordinary commands a human could run, because
  that is what keeps the repository un-hostage.
- Scaffolding writes files a human reviews and commits. It does not
  commit, and it does not push.
- This is not the automated channel. A workflow that realizes proposals
  without an operator is a different design with a different trust
  boundary, adjudicated in
  [[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]].

## Evidence

Henry's framing, 2026-08-01: in an ideal world the `gh … | scarp new`
pattern is standardized to the point of being a first-class concept when
scarp-on-GitHub is detected, together with the GitHub-specific
machinery — issue types and templates — that the pattern needs to exist
at all.

The first consumer is concrete rather than hypothetical: sprint 10's
proposal channel needs exactly this pipeline, and
[[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task 51]] already shipped the half that
does not depend on the forge — `--body-file` is what makes the
realization step one command rather than a script. The manual pipeline
above is the desire path
[[ide_01KY7S6GG3NAA35KBJTC6CA1TM|idea 23]] argues should be counted
before it is automated; this idea exists so the count has somewhere to
accumulate.

Prior art for the shell-out stance is the project's own: the Rust
guidance rejects `git2` in favour of the installed `git` binary for
narrowly scoped features, and `gh` is the same trade one layer out —
an authenticated, maintained, ubiquitous CLI whose absence is a clean
unavailability rather than a broken build.

Task 51's Result records one detail this idea inherits: `--body-file`
deliberately does not read stdin, because a file is auditable and a
workflow can trivially write one. If the dominant caller becomes a
`gh … | scarp new` pipeline rather than a workflow, that trade is worth
revisiting — adding stdin later is additive and cheap, but the reasoning
behind its absence should not be mistaken for a reasoning about
pipelines.
