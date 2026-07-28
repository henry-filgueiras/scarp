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
install and quickstart prose. It does not cover the claim audit,
which is [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]] and runs after
this one so that the quickstart prose written here is itself audited;
nor publishing, tagging, releasing, or any live-surface verification,
which are [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]].

Measured starting state, 2026-07-27: `cargo package --list` yields
167 files, of which 114 are `archaeology/`, alongside `CLAUDE.md`,
`.claude/`, `.github/`, `scripts/`, `rustfmt.toml`, and `.scarp.toml`
— which makes an unpacked crate a Scarp repository in its own right.
`Cargo.toml` declares none of `keywords`, `categories`, `readme`,
`homepage`, `documentation`, `rust-version`, `exclude`, or `include`.
`README.md` contains no install instructions at all.

## Toolchain prerequisite

Henry decided on 2026-07-27 to test the MSRV against a real pinned
toolchain rather than declare one and hope. This machine cannot do
that today: **`rustup` is not installed**, and Rust comes from
Homebrew (`brew rust 1.96.1`, providing `/opt/homebrew/bin/rustc` and
`/opt/homebrew/bin/cargo`). Homebrew's `rustup` formula exists
(1.29.0, keg-only, formerly `rustup-init`) but installs nothing onto
`PATH` by default.

The hazard to design around is `PATH` shadowing. rustup installs
shims into `~/.cargo/bin`; Homebrew's Rust lives in
`/opt/homebrew/bin`. Whichever appears first wins, so an MSRV test
can silently run under 1.96 while appearing to test the floor. Any
toolchain claim in the Result must therefore be accompanied by
evidence of which binary actually ran.

Henry chose on 2026-07-27 to make rustup the single source of truth,
rather than run it alongside the Homebrew toolchain. The alternative
— installing Homebrew's keg-only `rustup` formula and leaving
`brew rust` in place — was considered and rejected: it leaves two
Rust installations racing for `PATH`, which is the failure mode this
prerequisite exists to eliminate, and it trades a one-time
uninstall for a permanent ambiguity.

Removing Homebrew's Rust leaves the machine with **no** Rust
toolchain until the rustup installation completes, so the two
commands belong together:

```sh
brew uninstall rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then:

```sh
rustup toolchain install <MSRV>
rustup toolchain list
cargo +<MSRV> --version   # must report <MSRV>, not 1.96.1
command -v cargo          # records which cargo is winning PATH
```

Anything already installed through the old toolchain — `cargo
install`ed binaries in particular — lives outside the Homebrew keg
and is not removed by the uninstall, but should be confirmed rather
than assumed.

The Result records the verbatim commands run, their output, and the
resulting `rustup toolchain list` — as dated provenance per
`CLAUDE.md`, not as an installer this project maintains.

## Acceptance criteria

- The archaeology is excluded from the published tarball, per Henry's
  adjudication on 2026-07-27. Development-only surfaces that serve no
  consumer of the crate — agent configuration, CI, contributor
  scripts, the repository marker — are excluded on the same
  reasoning.
- Packaging uses a positive `include` allowlist rather than a growing
  negative `exclude` list, unless inspection finds a concrete reason
  not to; the reason is recorded if so. Cargo's current manifest
  reference states the two are mutually exclusive and that `include`
  overrides `exclude`, and supports `!` negation within an `include`
  list. The allowlist must account for what Cargo includes
  automatically regardless of configuration — the manifest itself, a
  minimized `Cargo.lock`, and `license-file` when that field is used
  — and for what it always excludes, namely sub-packages and
  `target/`. This project sets `license` rather than `license-file`,
  so `LICENSE-MIT` and `LICENSE-APACHE` are **not** auto-included and
  must be listed explicitly.
- `src/`, `tests/`, both license files, `README.md`, and any asset the
  packaged README genuinely needs are present in the final artifact,
  verified by listing the packaged files rather than by reading the
  manifest.
- Every relative link in the packaged README is audited against the
  packaged artifact. `CLAUDE.md` and `archaeology/` are excluded, so
  links to them must resolve to something useful from the crates.io
  page and from an unpacked crate — an absolute repository URL, or
  removal — rather than silently 404ing. A link that works only in a
  GitHub checkout is a defect, not an acceptable degradation.
- The gap the exclusion opens is recorded rather than silently
  accepted: nothing in the published crate will demonstrate a
  populated repository. Parked as
  [[ide_01KYJG0S6X9NQGHANGTRVDQ1JA|idea 33]]; this task does not
  implement a showcase corpus.
- `Cargo.toml` carries `keywords`, `categories`, `readme`, and
  `rust-version`, and the description is reviewed against decision
  16's positioning line. Keywords and categories are different
  things and are chosen as such: keywords are free-form search terms
  subject to Cargo's constraints (at most five, ASCII, at most twenty
  characters each, beginning with an alphanumeric and otherwise
  limited to letters, numbers, `_`, `-`, `+`), while categories must
  match crates.io's published slugs **exactly** and are also capped
  at five. Candidate slugs verified to exist on 2026-07-27 include
  `command-line-utilities`, `command-line-interface`,
  `development-tools`, `filesystem`, and `text-processing`; the set
  is re-verified against the live list at implementation time.
- `homepage` is **not** set. Cargo's guidance is that it should carry
  a dedicated site distinct from the source repository and the API
  documentation, and that it must not be made redundant with
  `repository` or `documentation`. Scarp has no such site; setting it
  to the GitHub URL would be exactly the redundancy the guidance
  warns against. If a dedicated site ever exists, that is when the
  field earns its place.
- `documentation` is **not** set merely to fill the field. crates.io
  automatically links a crate to its docs.rs page once documentation
  has been built, so a hand-written URL adds nothing and can only go
  stale. Instead, the rustdoc build is verified locally with
  `cargo doc --no-deps` (or the current equivalent) so that the
  documentation crates.io will link is known to build. An explicit
  URL is retained only if inspection produces a concrete reason,
  which is then recorded.
- `rust-version` is determined rather than guessed, and tested with
  that exact toolchain. The floor implied by `edition = "2024"` is a
  lower bound, not an answer; dependencies may raise it. Cargo errors
  rather than warns when a package is built on an older toolchain, and
  its documentation is explicit that verification is the maintainer's
  responsibility and not automatic. Testing covers building, the test
  suite, and documentation as appropriate, with evidence of which
  toolchain actually executed.
- A narrow persistent CI gate prevents the MSRV from drifting
  immediately after release, implemented with `cargo hack` per
  Henry's decision on 2026-07-27 and matching Cargo's current
  continuous-integration guidance. The recommended shape is a single
  job on one platform running `cargo hack check --rust-version`,
  which is check-only rather than a build-and-test matrix. Cargo's
  rationale is worth preserving: one platform because most projects
  are platform-agnostic, `check` only because most MSRV breakage is
  API availability rather than behavioral difference.
- The gate's one non-obvious requirement is recorded so it is not
  rediscovered as a CI failure: `cargo hack --rust-version` **does
  not install toolchains**. It shells out to
  `rustup run <toolchain> cargo`, so the job must have rustup present
  *and* the MSRV toolchain already installed — the same PATH-and-
  toolchain discipline the local prerequisite above imposes,
  transplanted into CI. Whether the workflow installs it via a
  toolchain action or an explicit `rustup toolchain install` is an
  implementation choice; having neither is a silent failure.
- Flag selection is deliberate rather than copied. Cargo's example
  invocation carries `--workspace --all-targets --ignore-private`;
  this repository is a single published package, not a workspace with
  private members, so `--workspace` and `--ignore-private` are inert
  here. Keep or drop them on stated reasoning — future-proofing is a
  legitimate reason, cargo-culting is not.
- The gate stays narrow either way. This is a guard against silent
  drift, not a compatibility matrix, and it must not grow into one
  without a recorded reason.
- The README gains an install section and a quickstart taking a
  newcomer from a working binary to a meaningful first result in
  about sixty seconds, deterministically. The quickstart operates in
  an explicitly disposable directory, states its filesystem effects
  plainly, and depends on no undeclared helper — `jq` in particular
  is not assumed to be installed. Every command is executed exactly
  as documented and its real output recorded; no output is
  paraphrased, invented, or reflowed to look tidier than it is.
- The sixty seconds measures the post-install quickstart, not Rust
  dependency compilation, per the sprint amendment. Install time is
  measured and reported separately.
- The logo and the two Mermaid blocks are handled deliberately.
  crates.io renders the README as Markdown and does not render
  Mermaid, and a relative `assets/logo.svg` reference does not
  resolve on the crate page. Before publication the closest available
  rendering check is used and **its limitation is documented** — no
  local tool reproduces crates.io exactly. Inspection of the actual
  rendered page belongs to
  [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]], because no live page
  exists yet; "it looks fine on GitHub" is not a verification of
  either surface.
- Verification runs against the packaged artifact, not the working
  tree. `cargo package` succeeds; the resulting `.crate` is unpacked
  somewhere isolated from this checkout; the crate builds and its
  tests pass there; and the quickstart is executed end to end against
  a binary installed from that unpacked tarball. The installation
  uses a temporary install root, target directory, and Cargo home, so
  that no development-tree binary, no `target/debug/scarp`, and no
  warm cargo cache can accidentally satisfy the test. The packaged
  size is checked against crates.io's 10 MB limit.
- `cargo publish --dry-run` succeeds and its output is recorded.
  Nothing is published, tagged, or released. `--allow-dirty` and
  `--no-verify` are not used: the first defeats the clean-tree
  guarantee publication depends on, the second defeats the point of
  the dry run.
- `scripts/check.sh` passes, and the work is committed per the commit
  policy in `CLAUDE.md`.

## Result
