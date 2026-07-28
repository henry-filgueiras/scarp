---
id: tsk_01KYJG0S7GY51W8M1WYFMEV7MQ
sequence: 43
kind: task
status: closed
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-27
closed: 2026-07-27
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

## Amendment (2026-07-27): two planning assumptions were wrong

Both were checked against current primary sources at implementation
time rather than trusted, and both failed. The criteria above stand
except where corrected here.

### crates.io renders Mermaid, and it does rewrite relative media

The criterion above asserts that "crates.io renders the README as
Markdown and does not render Mermaid, and a relative
`assets/logo.svg` reference does not resolve on the crate page".
That was already false when written. Verified against
`rust-lang/crates.io` at `HEAD` on 2026-07-27:

- `crates/crates_io_markdown/src/lib.rs` lists `language-mermaid`
  among the sanitizer's allowed `code` classes, with a unit test
  asserting the class survives;
  `svelte/src/lib/attachments/mermaid.ts` runs
  `mermaid.run({ nodes })` over `.language-mermaid` with
  `securityLevel: 'strict'`; and
  `e2e/acceptance/readme-rendering.spec.ts` asserts
  `pre > code.language-mermaid svg.flowchart` becomes **visible**.
  Mermaid is rendered client-side, so the diagrams are **retained**.
- Relative URLs are rewritten to `<repository>/raw/HEAD/<dir>/<path>`
  for media and `blob/HEAD` otherwise, with `?sanitize=true` appended
  for `.svg`. This applies to raw-HTML `<img src>` as well as Markdown
  image syntax. **The rewritten URL follows repository `HEAD` and is
  therefore mutable**: the crate page for a frozen version will track
  whatever the default branch later contains.

The consequence is the opposite of the plan: relative paths are the
*correct* choice for `<img src>`, and are kept.

### `<source srcset>` is the real defect, and it was not in the plan

crates.io allows `srcset` on `<source>`, but ammonia rewrites only the
attributes in its `is_url_attr` list — `href`, `xlink:href`, `src`,
`form/action`, `object/data`, `formaction`, `a/ping`, `video/poster`.
`srcset` is absent, so a relative dark-mode `srcset` is emitted
verbatim and resolves against `https://crates.io/crates/scarp`.
crates.io's own `pictures_and_sources` test uses absolute URLs only
and does not cover this. Because `<picture>` does **not** fall back to
`<img>` when the selected `<source>` fails to load, every dark-mode
visitor would have seen a broken logo. Fixed by making that one
attribute an absolute `raw.githubusercontent.com/.../HEAD/...` URL —
mutable for the same reason, and recorded as such.

### `cargo hack --rust-version` does install toolchains

The criterion above records, as the gate's "one non-obvious
requirement", that `cargo hack --rust-version` does not install
toolchains. That is no longer true of cargo-hack 0.6.45. Its
`rustup::install_toolchain` runs
`rustup toolchain add <toolchain> --no-self-update` whenever
`rustup run <toolchain> cargo --version` fails, and this was observed
locally: the first gate run emitted
`info: running 'rustup toolchain add 1.88 --no-self-update'`.

What remains true is the part that matters: **rustup itself must be
present**, because cargo-hack shells out to `rustup run` (deliberately,
not `cargo +toolchain`, per rustup bug #3036).

A second, sharper detail replaces it: cargo-hack derives the toolchain
name as `format!("1.{minor}")`, **stripping the patch component**. A
`rust-version` of `1.88.0` gates on the toolchain named `1.88`, not
`1.88.0`. Installing `1.88.0` in CI would therefore leave the gate to
silently fetch a different toolchain than the one the job pinned —
defeating the purpose of installing it explicitly. `rust-version` is
declared as `1.88` so the manifest, the CI toolchain, and the toolchain
that actually executes are the same string.

## Result

All work performed 2026-07-27 from a clean checkout equal to
`origin/main` at `5c7900d151b95cc5f90597774599fbbbeb013117`.

### Toolchain migration

The prerequisite was performed by Henry, not by the agent: `brew
uninstall rust` (which also autoremoved `llvm`, `pkgconf`, and `z3`;
`brew uses --installed rust` was empty beforehand, and `~/.cargo/bin`
did not exist, so no `cargo install`ed binary was lost), followed by
the rustup installer. rustup 1.29.0 now owns the toolchain and
`/opt/homebrew/bin/{cargo,rustc}` are gone.

Every toolchain claim below is backed by `rustup which` rather than
PATH appearance, because PATH shadowing was the hazard the migration
existed to remove:

```text
rustup which cargo                    /Users/henry/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo
cargo --version                       cargo 1.97.1 (c980f4866 2026-06-30)
rustc -Vv                             rustc 1.97.1 (8bab26f4f 2026-07-14), host aarch64-apple-darwin
rustup which --toolchain 1.88.0 cargo /Users/henry/.rustup/toolchains/1.88.0-aarch64-apple-darwin/bin/cargo
cargo +1.88.0 --version               cargo 1.88.0 (873a06493 2025-05-10)
```

One wrinkle worth recording: rustup wires itself into `~/.zshenv` and
`~/.profile`, but an already-running shell keeps the old PATH. Agent
sessions must `. "$HOME/.cargo/env"` explicitly rather than assume a
fresh login shell.

### MSRV: 1.88

Determined by testing, not declared. Edition 2024 sets 1.85.0 as the
theoretical floor, so 1.85.0 was tested first and the search moved
upward only on concrete failures:

| Toolchain | Result |
|---|---|
| 1.85.0 | **fails** — `error[E0658]: 'let' expressions in this position are unstable` (issue #53667), exactly 5 sites, all in `src/main.rs` (lines 136, 151, 176, 191, 456) |
| 1.87.0 | **fails** — same error, same sites |
| 1.88.0 | **passes** — `let` chains stabilised in 1.88.0 |

Those failures were observed *before* `rust-version` was declared. With
the contract in place, Cargo now refuses earlier and more usefully —
`error: rustc 1.85.0 is not supported by the following packages: scarp@0.1.0
requires rustc 1.88` — which is Cargo erroring rather than warning, as its
documentation promises. Re-deriving the compile-level evidence afterwards
requires `--ignore-rust-version` to get past the manifest gate.

The floor is a language feature, not a dependency: no dependency was
downgraded, and nothing was changed to advertise a lower number. With
1.88.0, all three of `cargo +1.88.0 build --locked`,
`cargo +1.88.0 test --locked`, and `cargo +1.88.0 doc --no-deps
--locked` succeed, and `cargo +1.88 check --all-targets --locked` also
succeeds against the *packaged* artifact.

`rust-version = "1.88"` rather than `"1.88.0"` for the reason in the
amendment above: cargo-hack strips the patch, so `1.88` is the string
the gate actually resolves and installs.

### Manifest metadata

```toml
rust-version = "1.88"
description  = "Git-native, reviewable project archaeology: what changed, why, and what remains unsettled"
readme       = "README.md"
keywords     = ["archaeology", "decision-records", "adr", "project-memory", "documentation"]
categories   = ["command-line-utilities", "development-tools"]
```

The description was reconciled minimally with
[[dec_01KYJE2K3VRASS8A1X1E847S1B|decision 16]] and
[[tsk_01KYFYKAZRGEJPJYKAWV8W9BB4|task 41]]: the previous wording ended
"for humans and coding agents", which is close to the headline claim
task 41 retired because it promises capture and injection Scarp does
not ship. It now uses task 41's endorsed positive framing. The
comprehensive claim audit remains
[[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]]'s.

Both category slugs were re-verified against the live
`crates.io/api/v1/categories` list on 2026-07-27 (58 top-level
categories). Three of the plan's five candidates were **dropped on
inspection rather than kept to fill the cap**: `command-line-interface`
is for crates that help *build* CLIs, and `filesystem` and
`text-processing` describe libraries rather than this tool. Two
accurate categories beat five padded ones.

`homepage` and `documentation` are both unset, as planned; no new
evidence emerged to justify either. `cargo doc --no-deps` builds, so
the docs.rs page crates.io will link is known to compile. It emits
three pre-existing warnings — public docs in `artifact.rs`, `edges.rs`,
and `read.rs` link to private items — which are warnings, not failures,
and were left alone as out of scope.

### Package contents

A positive `include` allowlist, as planned; no reason to prefer
`exclude` was found. **36 files, 554.5 KiB uncompressed, 121.2 KiB
compressed — 1.18% of crates.io's 10 MiB limit.** Down from 171 files
on the starting commit.

Verified by listing the actual packaged files and by unpacking the
`.crate`, never by reading the manifest:

- `src/` (11), `tests/` (16), `README.md`, `LICENSE-APACHE`,
  `LICENSE-MIT`, `assets/logo.svg`, `assets/logo-dark.svg`;
- plus Cargo's automatic four: `Cargo.toml`, `Cargo.toml.orig`, a
  minimized `Cargo.lock` (93 packages), and `.cargo_vcs_info.json`.

Confirmed absent: `archaeology/` (118 files), `CLAUDE.md`, `.claude/`,
`.github/`, `scripts/`, `rustfmt.toml`, `.scarp.toml`, `.gitignore`,
`CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`. Since `license`
is set rather than `license-file`, both license files are listed
explicitly, and both are present in the artifact.

**One genuine packaging bug was caught by this, and only by this.**
`tests/init.rs::shipped_policy_exists_only_at_the_nested_archaeology_path`
read `$CARGO_MANIFEST_DIR/archaeology/.gitattributes` at runtime, so
excluding the archaeology broke the test suite *inside the packaged
crate* while the working tree stayed green. It asserts a property of
the host repository, not of the crate, so it is now gated on the
presence of `.scarp.toml` — which the package also excludes. Every
development checkout and CI job carries the marker, so the assertion is
never skipped where it means anything; an unpacked crate is simply not
a Scarp repository and has nothing to assert.

### README: links, logo, Mermaid

Every link was audited in both contexts — an unpacked crate, and
crates.io's current rewriting rules. All eight resolve:

| Link | On crates.io | In an unpacked crate |
|---|---|---|
| `assets/logo.svg` (`<img src>`) | rewritten to `raw/HEAD/assets/logo.svg?sanitize=true` | packaged |
| `…/HEAD/assets/logo-dark.svg` (`<source srcset>`) | absolute, passed through | absolute |
| `LICENSE-APACHE`, `LICENSE-MIT` | rewritten to `blob/HEAD/…` | packaged |
| `CLAUDE.md`, `archaeology/` | absolute repository URLs | absolute |
| two license autolinks | absolute | absolute |

`CLAUDE.md` and `archaeology/` were relative links to excluded
material. They would have worked on crates.io by accident of the
rewrite while 404ing in an unpacked crate, so both were converted to
absolute repository URLs with a sentence stating they are not part of
the crate. The two `http://` license autolinks were examined and left:
they resolve.

The `Development` section gained a lead-in stating it assumes a
repository checkout, since it invokes `scripts/check.sh`, which is
deliberately not packaged.

**Mermaid rendering check.** crates.io pins mermaid `11.16.0`
(`svelte/package.json`); both diagrams were extracted from the shipped
README and run through that exact version with
`initialize({ startOnLoad: false, securityLevel: 'strict' })` — the
same configuration `mermaid.ts` uses. Both `parse` and `render`
succeeded, producing flowchart-classed SVGs (13.9 KB and 360.6 KB), and
all `<br/>`-separated labels survived strict-mode sanitisation as six
`<br>` elements. **Documented limitation:** this ran under jsdom, which
has no layout engine, so `getBBox` and `getComputedTextLength` were
stubbed. It establishes syntax validity, strict-mode acceptance, and
that a flowchart SVG is produced — *not* pixel-accurate appearance,
font metrics, or final geometry. No local tool reproduces the crate
page. Inspection of the rendered page remains
[[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]'s.

### Install and quickstart

The README gained an `Install` section (`cargo install scarp --locked`,
Rust 1.88+) and a `Quickstart` operating in `/tmp/scarp-demo`, an
explicitly disposable directory, with its filesystem effects stated and
a cleanup command. It depends on no helper beyond a shell — no `jq`.

Every command was executed exactly as documented against a binary
installed from the unpacked package, and its real output is preserved
in the README unedited. Three fields are marked as varying: the ULID,
the date, and the absolute path macOS prints (`/private/tmp/…`).

**Timings, measured separately as the sprint amendment requires:**

- **Install: 5.72 s real, 43.00 s user** — genuinely cold (fresh
  `CARGO_HOME`; 65 crates downloaded, 179 dependencies built) on an
  18-core Apple M5 Pro. This is a fast machine and the README says so
  without promising the number.
- **Post-install quickstart: 0.04 s** of machine time, against the
  ~60-second criterion. The budget is comfortably met; the sixty
  seconds is human reading time, not compute.

### Verification against the packaged artifact

The `.crate` was treated as the product under test throughout. Unpacked
outside the development checkout, it builds, its **385 tests across 19
binaries all pass**, and `cargo doc --no-deps` succeeds. Installation
used a fresh `CARGO_HOME`, a fresh `CARGO_TARGET_DIR`, and a temporary
`--root`, all three verified empty beforehand, so no warm cache, no
`target/debug/scarp`, and no development binary could satisfy the test.
`command -v scarp` resolved to the temporary install root, and no
`scarp` exists in `~/.cargo/bin`.

### Resolving the clean-tree paradox

`cargo publish` requires clean VCS state; implementing the task dirties
the checkout. Neither `--allow-dirty` nor `--no-verify` was used, and
no WIP commit was made in the real repository. Instead:

1. every tracked path was copied from the **working tree** (carrying
   the uncommitted edits) into a temporary directory, excluding the
   original `.git` and all build products;
2. representation was proven rather than assumed —
   `git ls-files --others --exclude-standard` confirmed **zero**
   untracked non-ignored files to account for, file counts matched at
   169, and `git hash-object` over all 169 tracked paths was
   **byte-identical** on both sides;
3. a disposable Git repository was initialised there and the snapshot
   committed locally;
4. `cargo package --locked` and `cargo publish --dry-run --locked` ran
   from that clean snapshot.

`--locked` deliberately, so this task exercises the same lockfile
contract task 45 will.

**Two consequences to carry forward.** The snapshot's
`.cargo_vcs_info.json` records the *snapshot's* commit SHA, not this
repository's — an artifact of the method; the real publication will
embed the real release SHA, which is task 45's to record. And the
later Result, status, and closure edits in this file touch
`archaeology/` only, which the allowlist excludes, so **they cannot
alter the packaged payload**; the final dry run from the real clean
commit confirms this rather than assuming it.

### MSRV CI gate

One job, `msrv`, added to `.github/workflows/ci.yml`: Ubuntu, the
`1.88` toolchain explicitly installed via
`dtolnay/rust-toolchain@1.88`, prebuilt cargo-hack via
`taiki-e/install-action@cargo-hack`, a step printing `rustup toolchain
list`, `rustup which cargo`, `cargo --version`, `rustc --version`, and
`cargo hack --version` so the log shows what actually ran, then:

```sh
cargo hack check --rust-version --all-targets --locked
```

`--workspace` and `--ignore-private` from Cargo's example invocation
are **omitted**: Scarp is a single published package with no private
members, so both are inert here, and copying them would be
cargo-culting. The gate stays check-only on one platform — a drift
guard, not a compatibility matrix — with a comment saying it must not
grow into one without a recorded reason. No repository-wide
`rust-toolchain.toml` was introduced; `rust-version` plus this gate is
the whole contract.

The exact invocation was also run locally against cargo-hack 0.6.45:
`running 'rustup run 1.88 cargo check --all-targets --locked' on scarp
(1/1)` … `Finished`.

### Outcome

`cargo publish --dry-run --locked` from the clean snapshot: `Packaged
36 files, 554.4KiB (121.2KiB compressed)`, `Verifying scarp v0.1.0`,
then `warning: aborting upload due to dry run`.

It was then repeated from the **real repository at the clean commit**,
after `scripts/check.sh` passed (doctor: 106 artifacts, no problems)
and the work was committed: `Packaged 36 files, 554.5KiB (121.2KiB
compressed)` and the same clean abort. The small delta from the
snapshot figure is a `Cargo.toml` comment edited after the snapshot was
taken, not a change in what ships.

That run also confirms the two predictions above rather than leaving
them asserted: the packaged file list is unchanged at 36 despite this
Result being written in between — because `archaeology/` is excluded,
editing it cannot move the payload — and `.cargo_vcs_info.json` now
carries this repository's commit rather than the disposable snapshot's.
The precise release SHA belongs to
[[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] and is deliberately not
quoted here, since a task cannot name the commit that contains it.

Nothing was published, tagged, or released; no tag was created, no
GitHub release made, and no repository setting mutated.

### For task 44

- The README hero still reads "structured repository memory for humans
  and coding agents", and `src/cli.rs` still describes the binary as
  "Git-friendly project archaeology and repository-local memory". The
  Cargo description no longer matches either. Reconciling all three is
  task 44's, and `--help` output is now part of that surface.
- The `See it work` showcase remains stale as task 44 already records,
  and still depends on `jq`. The quickstart added here is the
  deterministic fixture that section can be rebuilt from.
- Both logo URLs and crates.io's own rewrite follow `HEAD`, so the
  crate page for a frozen `0.1.0` tracks whatever the default branch
  later contains. Moving assets or renaming the default branch breaks a
  published page retroactively.
- `SECURITY.md`, `CONTRIBUTING.md`, and `CODE_OF_CONDUCT.md` are not
  packaged, so they are GitHub-only surfaces; the
  private-vulnerability-reporting blocker is unaffected by packaging.
