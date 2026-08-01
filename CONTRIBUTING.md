# Contributing to Scarp

Thanks for your interest. Scarp is early — a single maintainer proving a
small vertical slice — so the most valuable contributions right now are bug
reports, ideas, and focused fixes rather than large features.

This repository is unusual in one way that affects every contribution: it is
its own first user. The project's decisions, known risks, and work items live
in [`archaeology/`](archaeology/) as ordinary files, and substantive changes
are expected to keep that record current. It is also, deliberately, a case
study in human–AI collaboration; the same workflow applies whether a change
is written by a person, an agent, or both.

## Building and testing

Stable Rust, 1.88 or newer — the crate declares that as its `rust-version`,
and CI checks it against exactly that toolchain.

```sh
cargo build
cargo run -- --help
scripts/check.sh   # rustfmt --check, tests, clippy -D warnings, scarp doctor
```

`scripts/check.sh` must pass before any pull request. Filesystem mutation is
the primary correctness surface; tests use temporary directories and should
never touch the working repository.

## Before you write code

1. Read [`CLAUDE.md`](CLAUDE.md) — it holds the project invariants (files are
   canonical, no hostage-taking, identity vs. display sequence, and so on).
   These are enforced in review.
2. Read the current sprint under `archaeology/sprints/` and its pending
   tasks. Work that belongs to no sprint and serves no recorded idea or
   dragon is likely to be declined regardless of quality — open an issue
   first.
3. Check open dragons and recorded decisions so you don't reopen a settled
   tradeoff without new evidence. `scarp list dragons` and
   `scarp list decisions` are the quickest route; the files themselves are in
   `archaeology/dragons/` and `archaeology/decisions/`. Lifecycle state lives
   in front matter, so there are no `open/` or `closed/` subdirectories to
   look in.

## Making changes

- Prefer small, reviewable vertical slices; don't combine filesystem CRUD
  with unrelated concerns.
- Match the existing code style; `scripts/check.sh` enforces formatting and
  lints.
- Update the archaeology alongside the change: close or amend the relevant
  task, record unresolved risks as dragons, record consequential
  architectural choices as decisions. Routine changes don't need ceremony —
  see "What deserves durable archaeology" in `CLAUDE.md`.
- Commit messages follow `area: what changed` — lowercase, imperative
  (`doctor: validate repository invariants`,
  `resolve: batch reference-to-id resolution`).

## Bugs and ideas

- **Bug reports**: use the bug-report issue template. Include the command
  run, the repository state, and `scarp doctor` output where relevant.
- **Ideas**: use the idea issue form, shaped like the project's idea
  artifacts (Problem / Sketch / Boundaries / Evidence). Ideas are never
  load-bearing: an accepted proposal becomes a parked idea artifact in the
  archaeology, not a roadmap promise. A maintainer realizes an accepted
  proposal with `scarp proposal realize`, which creates the canonical file —
  see [`docs/remote-proposals.md`](docs/remote-proposals.md) for what the
  issue is and is not.
- **Security issues**: do not open a public issue — see
  [`SECURITY.md`](SECURITY.md).

## Licensing

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed under MIT OR Apache-2.0, without any
additional terms or conditions.

## Conduct

Participation is governed by the
[Code of Conduct](CODE_OF_CONDUCT.md).
