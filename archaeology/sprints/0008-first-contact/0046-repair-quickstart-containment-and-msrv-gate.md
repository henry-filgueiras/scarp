---
id: tsk_01KYK608A5Q5CAEPYYKW4YFQSH
sequence: 46
kind: task
status: closed
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-27
closed: 2026-07-27
---

# Repair quickstart containment and MSRV gate

## Objective

Repair two defects in [[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|task 43]]'s
output that were found after it closed, and correct the record that
described one of them as verified.

Both defects are pre-publication, and that is the whole reason this
task exists as a task rather than as a follow-up. The README is part
of the crate payload, so the quickstart defect would be frozen into
`0.1.0` by [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] and could then
be fixed only by spending a version number. The MSRV defect is
cheaper, but it makes the gate assert something it does not check,
which is worse than having no gate.

Sequence 46 is later than 44 and 45 only because it was allocated
later. The sprint's amended execution order is authoritative:
43, **46**, 44, 45.

### Defect one: the quickstart is unsafe under pasted-shell semantics

The shipped quickstart opens:

```sh
mkdir /tmp/scarp-demo && cd /tmp/scarp-demo
scarp init
```

An ordinary interactive shell does not stop when a pasted line fails.
If `/tmp/scarp-demo` already exists — a stale run, another user on a
shared host, a hostile symlink — `mkdir` fails, `&&` correctly skips
the `cd`, and then **every following line runs in whatever directory
the reader was already standing in**. The reader's own repository
acquires a `.scarp.toml` and an `archaeology/` tree. Nothing warns
them; `scarp init` succeeds, because succeeding is its job.

The documented cleanup compounds it:

```sh
cd .. && rm -rf /tmp/scarp-demo
```

That is a fixed-path recursive deletion of a directory the quickstart
may not have created. In the failure case above it deletes someone
else's directory, and it is the one command in the README whose
mistake is unrecoverable.

Neither hazard was hypothetical to reason about and neither was
tested: task 43 executed the quickstart on a machine where the happy
path held.

### Defect two: the MSRV gate installs a toolchain it does not use

Task 43 recorded that the `1.88` toolchain is "explicitly installed
via `dtolnay/rust-toolchain@1.88`", so that cargo-hack would not
silently materialise one. The GitHub Actions run on
`d0a3775cb440cceaf9e0ad3ccc12d17c6d1d78cd` (run 30319275441, job
90151540325) shows that claim is false, and shows it in the log of a
job that **passed**:

- `dtolnay/rust-toolchain@1.88` normalises its input to a full
  version — the log shows `toolchain: 1.88.0` — and runs
  `rustup toolchain install 1.88.0 --profile minimal --no-self-update`,
  installing `1.88.0-x86_64-unknown-linux-gnu`;
- the evidence step then prints a toolchain list containing exactly
  `stable-x86_64-unknown-linux-gnu` and
  `1.88.0-x86_64-unknown-linux-gnu (active, default)` — **no `1.88`**;
- `cargo hack check --rust-version` then emits
  `running 'rustup toolchain add 1.88 --no-self-update'` and installs
  a second toolchain, `1.88-x86_64-unknown-linux-gnu`, before running
  `rustup run 1.88 cargo check --all-targets --locked`.

The job passed. It passed on a toolchain cargo-hack fetched for
itself, from the network, at gate time — which is the exact condition
the explicit install was added to prevent. Task 43's own amendment
had already established the mechanism (cargo-hack 0.6.45's
`format!("1.{cargo_version}")` strips the patch component, and its
`rustup::install_toolchain` runs `rustup toolchain add` when
`rustup run <toolchain> cargo --version` fails); what it missed is
that `dtolnay/rust-toolchain@1.88` does the *opposite* normalisation,
expanding `1.88` to `1.88.0`. The two normalisations disagree, and
nothing in the job noticed.

Both facts are true at once and must stay distinguishable in the
record: **`1.88` is a rustup toolchain name; `1.88.0` is a compiler
version.** The toolchain named `1.88` reports `rustc 1.88.0`. Fixing
this means installing the *name*, not the version.

## Acceptance criteria

### Quickstart containment

- The quickstart runs inside a single subshell so that the reader's
  own shell is never modified: no `set` option, no working-directory
  change, and no variable leaks out of it. Verified by checking the
  caller's `$PWD` after the block, not by inspection.
- Fail-fast behaviour is enabled **inside** the subshell, so that a
  failed setup step aborts the block instead of letting later
  commands run somewhere unintended.
- The working directory is obtained from `mktemp -d`, not from a
  fixed path. No `scarp` command runs before the `cd` into that exact
  directory succeeds.
- The cleanup trap is installed **after** the directory exists, and
  removes only the path `mktemp` returned. No fixed-path recursive
  deletion appears anywhere in the README.
- A setup-failure case is executed, not merely argued — an unusable
  `TMPDIR` is one — and it is proved that no `.scarp.toml`, no
  `archaeology/`, and no other Scarp output appears in the caller's
  directory, and that no `scarp` command ran at all.
- The behaviour is confirmed in more than one shell rather than
  assumed from the sketch. `zsh` is Henry's shell; `bash` and a
  POSIX `sh` are what a reader is likely to have.
- The prose declares its real dependencies: a Unix-like shell and the
  standard utilities the block invokes, naming `mktemp` and `rm`. It
  continues to require no optional helper — `jq` in particular stays
  out, for the reason [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]]
  already records against the `See it work` section.
- The prose states which parts of the recorded output vary between
  machines: the temporary path, the ULID, and the date. The path now
  varies in full, not merely in its `/private` prefix, so the
  previous sentence about macOS resolving `/tmp` is replaced rather
  than kept.
- The instruction to open the generated file in an editor is removed
  or reconciled. It currently follows a block that has already
  deleted the file, so as written it cannot be followed. The point it
  makes — that these are ordinary Markdown files needing no Scarp to
  read — is worth keeping and must be made in a way the reader can
  actually verify from the transcript in front of them.
- The separate cleanup command is removed, since the trap now owns
  cleanup on both the success and failure paths.
- The sixty-second post-install target is retained; the sprint's
  distinction between install time and quickstart time is unchanged.
- The final documented block is executed **verbatim** against a
  binary installed from the unpacked package, and its real output is
  what the README shows. No output is reflowed, trimmed, or
  reconstructed from an earlier run.

### MSRV gate

- The contract is unchanged and is not renegotiated here: one Ubuntu
  job, check-only rather than a test matrix, MSRV declared in the
  manifest, `cargo hack check --rust-version --all-targets --locked`,
  and no `rust-toolchain.toml`. Only the mechanism changes.
- The MSRV is read from the manifest at job time rather than written
  twice. `cargo metadata --no-deps --format-version 1` is the source;
  the one package's `rust_version` is required to be non-null, and a
  null or missing value fails the job rather than silently degrading
  to a default.
- The toolchain **name cargo-hack will invoke** is installed before
  cargo-hack runs, using the equivalent of
  `rustup toolchain install "$msrv" --profile minimal
  --no-self-update`. For the current manifest that name is `1.88`.
  The workflow must not hand the name to an action that expands it to
  `1.88.0`.
- cargo-hack is pinned to the audited release rather than floating on
  latest, via `taiki-e/install-action@v2` with `tool:
  cargo-hack@0.6.45`. A gate whose tool changes under it is not a
  fixed gate, and 0.6.45 is the version whose patch-stripping
  behaviour this task and task 43 actually read.
- Evidence is recorded against the **selected** toolchain, not
  against whatever is default: `rustup toolchain list`, `rustup which
  --toolchain "$msrv" cargo`, `rustup run "$msrv" cargo --version`,
  `rustup run "$msrv" rustc --version`, and `cargo hack --version`.
- `rustup toolchain list` is snapshotted immediately before and
  immediately after the cargo-hack invocation, and the job **fails**
  if the two differ. Hidden toolchain materialisation is the defect
  this task exists to close, so it is a gate failure and not a line
  in the log that a reader has to notice.
- Every workflow comment and every task 43 claim asserting that the
  existing setup already installed the name cargo-hack executes is
  removed or corrected. A comment that documents a false mechanism is
  worse than no comment, because it is what a future reader will
  trust instead of the log.
- Henry's local `1.88` and `1.88.0` toolchains are left alone. They
  are system state outside this repair, and the gate's correctness
  does not depend on the development machine's toolchain set.

### Package reverification

- Because `README.md` is part of the crate payload, task 43's product
  checks are repeated rather than inherited: `scripts/check.sh`;
  package construction with a complete file-list inspection; unpacking
  outside the checkout; a fresh `CARGO_HOME`, `CARGO_TARGET_DIR`, and
  install root; build, test, document, and install from the unpacked
  package; the exact quickstart plus the setup-failure containment
  test against that installed binary; package size and file count;
  `cargo publish --dry-run --locked` from a disposable clean Git
  snapshot; and a final `cargo publish --dry-run --locked` from the
  real clean commit.
- `--allow-dirty` and `--no-verify` are not used, for the reasons
  [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] records.
- The corrected package size is recorded. The **file set** is expected
  to be unchanged at 36, since this task edits existing packaged files
  and adds none; that expectation is stated and then checked against
  the listing rather than assumed.

### Record repair

- Task 43's Result is **not** rewritten to read as though it had
  always been correct. A dated post-close erratum is appended to it,
  linked to this task, preserving the original local evidence, the
  GitHub CI evidence that falsified part of the conclusion, the
  quickstart failure mode found in review, and the corrected outcome.
  The original text stands; the erratum is additive.
- Sprint 8's amendment records the execution order 43, 46, 44, 45,
  and says why 46 sits between them despite its later sequence.
- Task 44's introductory dependency and task 45's preconditions both
  recognise this task, so neither can be started on the belief that
  43 was the last packaging change.
- All new prose references are bound to stable ids.

### Gate

- `scripts/check.sh` passes, the complete diff is inspected, and the
  work is committed as one focused vertical slice per the commit
  policy in `CLAUDE.md`. Nothing is pushed.
- No claim is made about the repaired GitHub Actions run. The
  workflow cannot be observed until Henry pushes, so the Result
  records the exact commands he can run to verify it and stops there.
  This task's evidence is local by construction, and saying so is
  part of the deliverable.

## Result

All work performed 2026-07-27 from a clean checkout equal to
`origin/main` at `d0a3775cb440cceaf9e0ad3ccc12d17c6d1d78cd`.

### The CI contradiction, confirmed at source

Run `30319275441`, job `90151540325`, is a **passing** job whose log
contains its own refutation, quoted verbatim:

```text
Install Rust 1.88…   toolchain: 1.88.0
Install Rust 1.88…   rustup toolchain install 1.88.0 --profile minimal --no-self-update
Install Rust 1.88…   1.88.0-x86_64-unknown-linux-gnu installed - rustc 1.88.0 (6b00bc388 2025-06-23)
Record which…        stable-x86_64-unknown-linux-gnu
Record which…        1.88.0-x86_64-unknown-linux-gnu (active, default)
cargo hack check…    ##[group]running `rustup toolchain add 1.88 --no-self-update`
cargo hack check…    1.88-x86_64-unknown-linux-gnu installed - rustc 1.88.0 (6b00bc388 2025-06-23)
cargo hack check…    ##[group]running `rustup run 1.88 cargo check --all-targets --locked`
```

The evidence step's toolchain list contains no `1.88`. cargo-hack
installed it, from the network, at gate time.

Both mechanisms were re-read at primary source rather than taken from
the prior record. cargo-hack `v0.6.45`: `src/main.rs` derives the
toolchain as `let toolchain = format!("1.{cargo_version}")` from the
minor version alone, and `src/rustup.rs` probes with `cmd!("rustup",
"run", toolchain, "cargo", "--version")` and, on failure, runs
`cmd!("rustup", "toolchain", "add", toolchain, "--no-self-update")`.
`taiki-e/install-action` pins a tool with `tool: <name>@<version>`
under `@v2`; omitting the version means latest, which is what
`@cargo-hack` was doing — the log shows `installing cargo-hack@latest`
resolving to 0.6.45 by luck of timing, not by contract.

### Repaired workflow

`.github/workflows/ci.yml`, `msrv` job. The contract is untouched: one
Ubuntu job, check-only, manifest-declared MSRV, `cargo hack check
--rust-version --all-targets --locked`, no `rust-toolchain.toml`. Only
the mechanism changed.

1. `dtolnay/rust-toolchain@stable` — demoted to what it actually is
   here: a source of rustup and a cargo able to run `cargo metadata`,
   and a host for cargo-hack. It is **not** what the gate checks
   against, and the comment says so.
2. `cargo metadata --no-deps --format-version 1` piped through `jq -er`
   that errors unless there is exactly one package with a non-null
   `rust_version`. The value is then normalised with `cut -d. -f1,2`,
   mirroring cargo-hack's own patch-stripping, so a future
   `rust-version = "1.90.1"` still installs the name `1.90` that
   cargo-hack will invoke. Verified locally: `declared=1.88
   toolchain=1.88`, and the null case exits non-zero with `the package
   declares no rust-version`.
3. `rustup toolchain install "$TOOLCHAIN" --profile minimal
   --no-self-update` — the toolchain installed **by name**. No action
   normalises it upward.
4. `taiki-e/install-action@v2` with `tool: cargo-hack@0.6.45`, pinned.
5. Evidence taken against the selected toolchain rather than the
   default: `rustup toolchain list`, `rustup which --toolchain
   "$TOOLCHAIN" cargo`, `rustup run "$TOOLCHAIN" cargo --version`,
   `rustup run "$TOOLCHAIN" rustc --version`, `cargo hack --version`.
6. `rustup toolchain list | sort` snapshotted immediately before and
   after the gate, compared with `diff -u` in a step marked `if:
   always()`, failing the job on any difference.

Step 6 is the part that matters. The previous workflow's defect was
undetectable from its own exit status; this one converts it into a
failure.

### Local evidence for the gate

The steady state was executed here, with `TOOLCHAIN=1.88`:

```text
rustup which --toolchain 1.88 cargo   /Users/henry/.rustup/toolchains/1.88-aarch64-apple-darwin/bin/cargo
rustup run 1.88 cargo --version       cargo 1.88.0 (873a06493 2025-05-10)
rustup run 1.88 rustc --version       rustc 1.88.0 (6b00bc388 2025-06-23)
cargo hack --version                  cargo-hack 0.6.45
cargo hack check --rust-version…      Finished `dev` profile … in 4.84s
toolchain list before vs after        unchanged
```

That is the name-versus-version distinction shown rather than
asserted: the toolchain **named** `1.88` reports version `1.88.0`.

This machine already has `1.88` installed, so it cannot reproduce the
materialisation itself — the same blind spot that let task 43's local
run look clean. The detector was therefore exercised against the
historical CI condition directly: given the exact before-list that run
`30319275441` printed and the after-list it would have had, `diff -u`
reports `+1.88-x86_64-unknown-linux-gnu` and the step exits 1. The
guard fires on precisely the case that previously passed.

Henry's local `1.88` and `1.88.0` toolchains were left untouched.

### Quickstart containment

The README quickstart is now one subshell:

```sh
(
  set -eu
  scarp_demo_dir="$(mktemp -d "${TMPDIR:-/tmp}/scarp-demo.XXXXXX")"
  trap 'command rm -rf "$scarp_demo_dir"' EXIT
  cd "$scarp_demo_dir"
  …
)
```

Behaviour was tested, not inferred from the shape, in `zsh` (Henry's
shell), `bash`, and `sh` (dash):

| Path | Observed |
|---|---|
| Success | block runs, exits 0, trap removes the `mktemp` directory; caller's `$PWD` unchanged |
| Failure mid-run | `set -e` aborts, **trap still fires**, directory removed, caller's `$PWD` unchanged |
| Setup failure (`TMPDIR=/nonexistent-xyz/`) | `mktemp: mkdtemp failed…`, subshell exits 1 **before any `scarp` command**; caller's directory contains no `.scarp.toml`, no `archaeology/`, nothing new at all |

After every run, `${TMPDIR}scarp-demo.*` matched nothing: no directory
was left behind.

The failure the old text permitted was reproduced rather than argued.
With a stale `/tmp/scarp-demo` present, the shipped commands produced
`mkdir: /tmp/scarp-demo: File exists` followed by `initialized Scarp
repository at …/olddemo` — a directory holding an unrelated
`IMPORTANT.md` — and gained `.scarp.toml` and `archaeology/`.

Prose changes: the dependency declaration now names a Unix-like shell
plus `mktemp` and `rm`, and still requires no `jq`; the three varying
fields are the temporary path (which now varies in full, replacing the
old sentence about macOS resolving `/tmp`), the ULID, and the date;
the fixed-path cleanup command is gone, the trap owning both paths;
and the "open the file in an editor" instruction — which followed a
block that had already deleted the file — is replaced by pointing at
the `scarp show` transcript, which *is* the file's literal content,
with a note that deleting the `trap` line keeps the directory for
readers who want to poke at it. The about-a-minute framing is retained
and stated in the intro.

The documented block was executed **verbatim** against the binary
installed from the unpacked package, and the README's `console` block
was then checked mechanically against the captured output: identical
line for line, modulo the `$ ` prompt lines and their blank
separators. Post-install quickstart time: **0.027 s**, against the
~60-second criterion.

### Package evidence

The `.crate` was the product under test throughout; `--allow-dirty`
and `--no-verify` were not used.

- `scripts/check.sh`: passes (doctor: 107 artifacts, no problems).
- **36 files, 556.2 KiB (122.0 KiB compressed)** — 1.19% of the 10 MiB
  limit. The file list is unchanged from task 43's 36, as predicted:
  this task edited two already-packaged files and added none. Checked
  against the listing, not assumed. The size rose from 554.5 KiB
  because the README grew.
- Unpacked outside the checkout; built, **385 tests across 19 binaries
  all pass**, `cargo doc --no-deps` succeeds (the same three
  pre-existing private-intra-doc-link warnings).
- Installed with a fresh `CARGO_HOME`, `CARGO_TARGET_DIR`, and
  `--root`, all three verified empty beforehand. `command -v scarp`
  resolved to the temporary install root; no `scarp` exists in
  `~/.cargo/bin`. Cold install: 4.87 s real, 42.59 s user.
- Clean-tree paradox resolved as task 43 did: 170 paths (169 tracked
  plus this task's own file, then untracked) copied from the working
  tree into a disposable directory, `git hash-object` byte-identical
  on both sides, committed locally, and `cargo publish --dry-run
  --locked` run from there — `Packaged 36 files, 556.2KiB (122.0KiB
  compressed)`, `Verifying scarp v0.1.0`, `warning: aborting upload
  due to dry run`.

### Limitations

- **The repaired workflow has not run on GitHub.** It cannot, until
  the commit is pushed, and pushing is Henry's. Everything above is
  local evidence plus a detector exercised against reconstructed CI
  input. The claim "the MSRV gate now runs on the toolchain it
  installs" is *designed and locally supported*, not *observed in CI*.
  [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|Task 45]]'s preconditions were
  amended to inherit that check.
- The materialisation guard could not be exercised end-to-end on a
  machine lacking `1.88`, because this machine has it and removing it
  is out of scope. The guard's *logic* was proven against the
  historical CI lists; its *integration* is unverified until CI runs.
- Shell coverage is `zsh`, `bash`, and `sh` on macOS. `mktemp -d` with
  an explicit template, `trap … EXIT` in a subshell, and `command rm`
  are POSIX-portable, but no BSD or Linux host was exercised here.
- The `${TMPDIR:-/tmp}/` construction yields a harmless doubled slash
  when `TMPDIR` ends in `/`, as it does on macOS. `mktemp` accepts it
  and `scarp init` prints the canonicalised path, so it is visible
  only in a `mktemp` error message.
- Task 44's suspects are untouched. The stale `See it work` showcase
  and its `jq` dependency remain that task's, and the new quickstart
  is the deterministic fixture it can be rebuilt from.

### Addendum (2026-07-27): the repaired gate observed in CI

Appended after this task closed, during
[[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]]. The Result and the
Limitations above are **not rewritten**: every limitation was true when
this task closed, because the workflow genuinely had not run on GitHub
and could not until Henry pushed. This addendum records that he did, and
what the run showed.

Henry pushed the repair, and run `30321601486` (workflow `CI`) executed
against commit `f66a94ee7de917bca435fa5d65b78396d8cf74d8` — this task's
own commit. Both jobs succeeded. The logs were read in full rather than
trusted from the green badge, because a passing job containing its own
refutation is exactly the failure this task exists to close:

```sh
gh run view 30321601486 --repo henry-filgueiras/scarp \
  --job 90158451436 --log    # MSRV
gh run view 30321601486 --repo henry-filgueiras/scarp \
  --job 90158451472 --log    # check
```

Verified in the MSRV job log (`90158451436`), quoted verbatim:

```text
Read the declared MSRV…   declared rust-version: 1.88 -> rustup toolchain: 1.88
Install the MSRV…         rustup toolchain install "$TOOLCHAIN" --profile minimal --no-self-update
Install the MSRV…         1.88-x86_64-unknown-linux-gnu installed - rustc 1.88.0 (6b00bc388 2025-06-23)
Install cargo-hack…       tool: cargo-hack@0.6.45
Install cargo-hack…       info: installing cargo-hack@0.6.45
Record which toolchain…   stable-x86_64-unknown-linux-gnu (active, default)
Record which toolchain…   1.88-x86_64-unknown-linux-gnu
Record which toolchain…   /home/runner/.rustup/toolchains/1.88-x86_64-unknown-linux-gnu/bin/cargo
Record which toolchain…   cargo 1.88.0 (873a06493 2025-05-10)
Record which toolchain…   rustc 1.88.0 (6b00bc388 2025-06-23)
Record which toolchain…   cargo-hack 0.6.45
cargo hack check…         running `rustup run 1.88 cargo check --all-targets --locked` on scarp (1/1)
cargo hack check…         Finished `dev` profile … in 13.18s
Fail if cargo-hack…       toolchain list unchanged across the gate
```

Point by point against the defect this task repaired:

- The manifest declared `1.88`, and the job derived the rustup toolchain
  **name** `1.88` from it rather than hard-coding either spelling.
- `rustup toolchain install` installed that name. The list printed by the
  evidence step contains `1.88-x86_64-unknown-linux-gnu` and **no**
  `1.88.0` — the exact inversion of run `30319275441`, where the list
  contained `1.88.0` and no `1.88`.
- That toolchain reported cargo `1.88.0` and rustc `1.88.0`, showing the
  name-versus-version distinction rather than asserting it.
- cargo-hack was `0.6.45` exactly, pinned by `tool: cargo-hack@0.6.45`
  and confirmed by both `installing cargo-hack@0.6.45` and
  `cargo hack --version`. It no longer resolves `@latest` by luck of
  timing.
- cargo-hack invoked `rustup run 1.88 cargo check --all-targets
  --locked` — the toolchain the job installed, not one it fetched for
  itself. No `rustup toolchain add` line appears anywhere in the log.
- The before/after snapshot compared clean: `toolchain list unchanged
  across the gate`. The guard whose logic was proven here only against
  reconstructed input has now run for real and stayed silent for the
  right reason.

The `check` job (`90158451472`) passed **385 tests across 19 binaries**,
matching the local count exactly, and finished with `doctor: 107
artifact(s) checked, no problems found`.

This resolves the first limitation above: "the MSRV gate now runs on the
toolchain it installs" is no longer *designed and locally supported* but
**observed in CI**. The second limitation is narrowed rather than
resolved — the materialisation guard was exercised end-to-end and
reported no change, which proves it runs and parses, but a runner that
already lacked `1.88` was not available to make it fire in anger; its
firing behaviour still rests on the reconstructed-input proof above.

This does not discharge [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]'s
inherited check. Run `30321601486` verifies commit `f66a94e`, which is
not the commit `0.1.0` will be published from — task 44 changed
`README.md`, which is part of the crate payload, and further commits may
follow. Task 45 must repeat this verification against whatever commit
becomes the release source.
