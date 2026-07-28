---
id: tsk_01KYK0PTQV9PGZTHRDAPG6YGYM
sequence: 45
kind: task
status: pending
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-27
---

# Publish and verify v0.1.0

## Objective

Publish `scarp 0.1.0` to crates.io, tag and release the exact source
commit that was published, verify every surface that only exists once
publication has happened, prove the install and quickstart work in an
environment that has never seen this repository, and close sprint 8.

This is the sprint's irreversible task, and its shape follows from
that. `cargo publish` cannot be undone: a yank hides a version from
future dependency resolution but never deletes it and never permits
re-publishing the same version. Everything in
[[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|task 43]] and
[[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]] can be redone; nothing here
can. Verification therefore front-loads everything provable before the
upload, and treats everything after it as confirmation of a committed
decision rather than a gate that could still save us.

The publication itself is human-owned. An agent prepares the exact
command and stops; Henry runs it. This is the same boundary
[[tsk_01KYJE2K3PK4F5XC81N8S6PBNA|task 42]] drew around the GitHub
rename, applied to an action with no undo at all.

## Acceptance criteria

### Preconditions

- Tasks 43, [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|46]], and 44 are closed
  and every blocker they identified is resolved. In particular,
  publication does not proceed while `SECURITY.md` directs reporters
  at a channel that is disabled.
- Task 46's repairs are present in the release source, and its one
  externally-unverifiable claim has been verified: the MSRV job must
  be **green on the release commit as pushed**, with its
  before-and-after toolchain-list comparison passing. Task 46 could
  only prove its workflow locally, so that check is inherited here
  rather than assumed. A gate that silently installs its own
  toolchain is not a gate, and this is the last point at which that
  can be caught.
- The release worktree is clean and exactly equal to `origin/main`;
  `scripts/check.sh` passes; CI is green on the release commit.
- crates.io `scarp` is re-checked as unclaimed **immediately** before
  publication, per
  [[dec_01KYJE2K3VRASS8A1X1E847S1B|decision 16]]. Absence must be a
  genuine not-found: an authentication failure, HTTP 403, a rate
  limit, or a network error is not evidence of availability, and stops
  the release rather than being read as a clear path.
- One **release-source commit SHA** is recorded. The published crate,
  the annotated tag, and the GitHub release all refer to that same
  commit. Any later commit — including this task's own closure — is
  not the release source.

### Publication

- Package inspection is re-run against the release commit rather than
  trusted from task 43: the packaged file list, the packaged size
  against crates.io's 10 MB limit, and
  `cargo publish --dry-run --locked`.
- `--locked` is used deliberately. Cargo's documentation is explicit
  that it asserts the exact dependencies and versions from the
  existing `Cargo.lock`, and errors if the lock file is missing or
  would change. That is what makes the published artifact correspond
  to what was actually tested.
- The actual `cargo publish --locked` is **human-owned and
  irreversible**. This task produces the exact command and stops for
  Henry to run it. Never request, inspect, print, echo, copy, or
  relocate his registry token: the credential is his, and it never
  enters the archaeology, a command line, or a log.
- `--allow-dirty` and `--no-verify` are not used. Cargo requires a
  clean VCS state by default and verifies the package by building it;
  both flags exist precisely to defeat the guarantees this release
  depends on.
- Henry's real output is preserved verbatim as dated provenance per
  `CLAUDE.md`. Nothing is described as done before its output exists.

### Post-publication verification

- After registry propagation, `scarp 0.1.0` is confirmed through
  registry metadata and `cargo owner --list`.
- The install is performed **from the published registry artifact** —
  not a path dependency, not a Git checkout — in an environment that
  has genuinely never seen this repository. A temporary directory on
  the development host does not satisfy sprint 8's criterion: that
  host carries a warm cargo cache, a built `target/`, an installed
  toolchain, and this checkout. Prefer a minimal no-checkout GitHub
  Actions smoke workflow, or a fresh container.
- The install pins the registry version and uses its lockfile — the
  current equivalent of
  `cargo install scarp --version 0.1.0 --locked`. `scarp --version`,
  `scarp --help`, and the documented quickstart are then run
  **exactly as documented**, with no adaptation. Needing to adapt a
  command is a defect in the documentation, not a detail of the test.
- Timings are reported separately: install (dependency compilation,
  machine-dependent, outside this project's control) and the
  post-install quickstart, which is what the sprint's sixty-second
  criterion measures.
- The live crates.io page is inspected as a reader sees it: README
  rendering, the logo, every link, metadata, license display, and
  formatting. This is the first point at which that rendering can be
  observed rather than approximated, so task 43's documented
  approximation is checked against it and the gap recorded.
- The docs.rs build is verified to have reached a **successful
  terminal state**, not merely to have been queued. docs.rs builds on
  nightly in a sandbox with no network access and a fifteen-minute
  limit, so a build can fail there for reasons a local `cargo doc`
  never surfaces. crates.io links documentation only once it has
  built; a failure is a finding, not a delay.

### Contingency

- A contingency is defined **before** publication for a defect
  discovered after it, and applied honestly if one occurs: classify
  the defect's real severity, yank only when the version is actively
  harmful rather than merely imperfect, and open a `0.1.1`
  remediation task. Do not rewrite history, do not retro-edit the
  record to claim the release was clean, and do not describe a flawed
  release as successful. `0.1.0` stays in the record either way.

### Tag and release

- After successful publication, an **annotated** tag `v0.1.0` is
  created pointing at the release-source SHA and pushed through a
  human-owned runbook. Pushing remains Henry's action; `CLAUDE.md`
  admits no exception for tags.
- The GitHub release is created with
  `gh release create --verify-tag`, which aborts if the tag does not
  already exist on the remote. This is chosen deliberately: without
  it, `gh` creates a tag itself from the default branch, and a tag
  invented by the release tool is not guaranteed to be the commit
  that was published. Notes are concise and curated. No prebuilt
  binaries are required — sprint 8 explicitly prefers one honest
  install path over an unverified matrix.

### Closure

- The Result records why the task-closure and sprint-closure commit is
  **intentionally after** the tagged release-source commit: the tag
  must point at what was published, and the archaeology recording the
  publication cannot exist until the publication has happened. This is
  an ordering consequence, not an oversight, and stating it prevents a
  future reader from "correcting" the tag.
- The sprint 8 retrospective is appended, covering what First Contact
  cost, what the identity detour taught, and friction to fix next.
- Task 45 and sprint 8 are closed **only after** every external
  verification has actually succeeded.
- The full gate is run, and the archaeology closure is committed
  separately from the release-source commit. Nothing is pushed by an
  agent.

## Result
