---
id: tsk_01KYJG0S7GY51W8M1WYFMEV7MQ
sequence: 43
kind: task
status: pending
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-27
---

# Make the crate publishable and write the quickstart

## Objective

Bring the `scarp` crate to a state worth freezing forever, and give a
stranger a documented path from "never heard of this" to a meaningful
first result in about sixty seconds. Publication itself is a separate
task; this one earns the right to perform it.

The forcing constraint is irreversibility. `cargo publish` cannot be
undone — a yank hides a version but never replaces it — so every
packaging mistake costs a version number and every rendering mistake
is permanent on the crate page. Verification must therefore run
against the *packaged artifact*, not the working tree, because the
working tree is the one environment guaranteed to hide packaging
bugs.

Scope covers the crate manifest, what the tarball contains, and the
install and quickstart prose. It does not cover publishing, tagging,
releasing, or the wider claim audit — the latter is
[[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]], which runs after this one
so that the quickstart prose this task writes is itself audited.

Measured starting state, 2026-07-27: `cargo package --list` yields
167 files, of which 114 are `archaeology/`, alongside `CLAUDE.md`,
`.claude/`, `.github/`, `scripts/`, `rustfmt.toml`, and `.scarp.toml`
— which makes an unpacked crate a Scarp repository in its own right.
`Cargo.toml` declares none of `keywords`, `categories`, `readme`,
`homepage`, `documentation`, `rust-version`, `exclude`, or `include`.
`README.md` contains no install instructions at all.

## Acceptance criteria

- The archaeology is excluded from the published tarball, per Henry's
  adjudication on 2026-07-27. Development-only surfaces that serve no
  consumer of the crate — agent configuration, CI, contributor
  scripts, the repository marker — are excluded on the same
  reasoning. `src/`, `tests/`, both licenses, `README.md`, and the
  manifest remain. The exclusion mechanism and the final file list
  are recorded, and the reasoning for each excluded class is stated
  once rather than per file.
- The gap this exclusion opens is recorded rather than silently
  accepted: nothing in the published crate will demonstrate a
  populated repository. Parked as
  [[ide_01KYJG0S6X9NQGHANGTRVDQ1JA|idea 33]]; this task does not
  implement a showcase corpus.
- `Cargo.toml` carries the metadata a stranger needs to find and
  build the crate: `keywords` and `categories` chosen from crates.io's
  accepted values, `readme`, `homepage`, `documentation`, and
  `rust-version`. The description is reviewed against decision 16's
  positioning line.
- `rust-version` is determined rather than guessed. The floor implied
  by `edition = "2024"` is a lower bound, not an answer; dependency
  requirements may raise it. If the declared version cannot be
  verified by building with that exact toolchain on this machine, the
  Result says so plainly and states what was verified instead — an
  unverified MSRV is a First Contact failure mode that surfaces as a
  confusing compile error on a stranger's older toolchain.
- The README gains an install section and a quickstart that takes a
  newcomer from installation to a meaningful first result in about
  sixty seconds, deterministically. Every command is executed as
  written and its real output recorded; no output is paraphrased,
  invented, or reflowed to look tidier than it is.
- The crates.io rendering of the README is corrected: the relative
  `assets/logo.svg` reference resolves on the crate page, and the two
  Mermaid blocks are handled deliberately — crates.io does not render
  Mermaid, so they will appear as raw source unless changed. Whatever
  is decided is recorded with its reasoning; "it looks fine on
  GitHub" is not a verification.
- Verification runs against the packaged artifact: `cargo package`
  succeeds, the tarball is unpacked in a location isolated from this
  working tree, the crate builds and its tests pass there, and the
  quickstart is executed end to end against a binary installed from
  that unpacked tarball. Testing only the development tree does not
  satisfy this.
- `cargo publish --dry-run` succeeds, and its output is recorded.
  Nothing is published.
- `scripts/check.sh` passes, and the work is committed per the commit
  policy in `CLAUDE.md`.

## Result
