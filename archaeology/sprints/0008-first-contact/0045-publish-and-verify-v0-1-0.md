---
id: tsk_01KYK0PTQV9PGZTHRDAPG6YGYM
sequence: 45
kind: task
status: closed
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-27
closed: 2026-07-30
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

Performed 2026-07-30 local time (US/Pacific). Registry, tag, and release
timestamps quoted below are UTC, where the same evening falls on
2026-07-31; the dated front matter follows the repository's local-date
convention. Scarp `0.1.0` is published,
tagged, released, and verified from an environment that had never seen
this repository. One live-surface defect was found after publication and
is recorded honestly below rather than smoothed over.

### The release source

```text
e69e722ef873d39a8c30328146d3edbc1945237c
```

That one commit is the publication source, the annotated tag's target,
and the GitHub release's target. It was proved to be `HEAD`,
`origin/main`, and the expected SHA simultaneously, with
`git status --porcelain -uall` empty — re-checked after every packaging
and verification step, because `cargo publish` refuses a dirty tree by
default and this task never used `--allow-dirty` or `--no-verify`.

No commit followed it on `origin/main`; no `v0.1.0` tag existed locally
or remotely; no GitHub release existed. Tasks 43, 46, and 44 were closed
and task 45 pending, checked through `scarp list tasks --active` rather
than by reading front matter.

Task 44's human-owned GitHub changes were re-verified read-only and all
held: private vulnerability reporting `true`, the `idea` label at
`7057FF` / "Uncommitted proposals to explore", homepage empty,
`agent-memory` absent from the eleven topics, and the description still
decision 16's positioning line.

### The inherited CI check, discharged at log level

Task 46 could only prove its repaired MSRV gate locally, and task 44's
addendum verified it on `f66a94e` — not the release source. Run
`30324691092` was therefore read in full on the release commit itself.
Both jobs' checkout steps name `e69e722…`; a green badge was not
accepted as evidence.

**MSRV job `90167489583`:**

```text
declared rust-version: 1.88 -> rustup toolchain: 1.88
TOOLCHAIN: 1.88
1.88-x86_64-unknown-linux-gnu installed - rustc 1.88.0 (6b00bc388 2025-06-23)
stable-x86_64-unknown-linux-gnu (active, default)
1.88-x86_64-unknown-linux-gnu
/home/runner/.rustup/toolchains/1.88-x86_64-unknown-linux-gnu/bin/cargo
cargo 1.88.0 (873a06493 2025-05-10)
rustc 1.88.0 (6b00bc388 2025-06-23)
cargo-hack 0.6.45
running `rustup run 1.88 cargo check --all-targets --locked` on scarp (1/1)
toolchain list unchanged across the gate
```

The toolchain list contains the **name** `1.88` and no `1.88.0` — the
exact inversion of run `30319275441`, the passing job whose log
contained its own refutation. `grep 'toolchain add'` over the entire
log returns **zero** occurrences, so cargo-hack fetched nothing. The
before/after guard ran and stayed silent for the right reason.

**check job `90167489628`:** nineteen `test result: ok.` lines summing
to **385 passed, 0 failed**, and `doctor: 108 artifact(s) checked, no
problems found`.

### Package, rebuilt from the clean commit

`scripts/check.sh` passed (doctor: 108 artifacts, no problems).

- `cargo package --locked --list` — **36 files**: `src/` (11),
  `tests/` (16), `README.md`, `LICENSE-APACHE`, `LICENSE-MIT`,
  `assets/logo.svg`, `assets/logo-dark.svg`, plus Cargo's automatic
  `Cargo.toml`, `Cargo.toml.orig`, minimized `Cargo.lock` (93 packages),
  and `.cargo_vcs_info.json`. Absent as designed: `archaeology/`,
  `CLAUDE.md`, `.claude/`, `.github/`, `scripts/`, `rustfmt.toml`,
  `.scarp.toml`, and the three community files.
- `cargo package --locked` — **36 files, 561.8 KiB (123.9 KiB
  compressed)**, matching task 44's measurement exactly. 1.21% of the
  10 MiB limit.
- `cargo publish --dry-run --locked --registry crates-io` — packaged,
  `Verifying scarp v0.1.0`, `warning: aborting upload due to dry run`.

The `.crate` was unpacked outside the checkout and treated as the
product under test: file list `diff`-identical to the manifest listing;
`Cargo.toml.orig`, `README.md`, both licenses and both SVGs
byte-identical to source; and in a fresh `CARGO_HOME` and
`CARGO_TARGET_DIR` verified empty first, it built, passed **385 tests
across 19 binaries**, documented with only the three known
private-intra-doc-link warnings, and installed to a temporary root
(`command -v scarp` resolved there; no `scarp` in `~/.cargo/bin`).

Its `.cargo_vcs_info.json` read `{"git":{"sha1":"e69e722ef873…237c"}}`
with **no `dirty` flag** — the provenance chain that later ties the
registry artifact to this commit.

### crates.io name recheck, immediately before publication

Run at **2026-07-30T23:58:03Z**, with an identifying User-Agent, against
both authoritative surfaces:

```text
https://crates.io/api/v1/crates/scarp
  -> HTTP 404  {"errors":[{"detail":"crate `scarp` does not exist"}]}

https://index.crates.io/sc/ar/scarp
  -> HTTP 404  <Code>NoSuchKey</Code><Key>sc/ar/scarp</Key>
```

Both a genuine not-found, satisfying decision 16's obligation that
absence be proved rather than inferred from a 403, a rate limit, or a
network error.

### Publication — Henry's, verbatim

Run by Henry from `/Users/henry/strata` at the clean release commit. No
agent ran it; no registry token was requested, inspected, printed,
echoed, or relocated at any point in this task, and none appears in this
archaeology.

```text
cargo publish --locked --registry crates-io
    Updating crates.io index
   Packaging scarp v0.1.0 (/Users/henry/strata)
    Updating crates.io index
    Packaged 36 files, 561.8KiB (123.9KiB compressed)
   Verifying scarp v0.1.0 (/Users/henry/strata)
   Compiling scarp v0.1.0 (/Users/henry/strata/target/package/scarp-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.74s
   Uploading scarp v0.1.0 (/Users/henry/strata)
    Uploaded scarp v0.1.0 to registry `crates-io`
note: waiting for scarp v0.1.0 to be available at registry `crates-io`
help: you may press ctrl-c to skip waiting; the crate should be available shortly
   Published scarp v0.1.0 at registry `crates-io`
```

His `Packaged 36 files, 561.8KiB (123.9KiB compressed)` is identical to
the dry run's, which is the first sign that what he uploaded is what was
reviewed.

**Token scopes, as dated provenance.** The token was created for this
release with endpoint scope `publish-new` only — not `publish-update`,
not `yank`, not `change-owners`, not `legacy` — and crate scope `scarp`.
Two things are worth keeping. crates.io accepts a crate-scope pattern
naming a crate that does not exist yet and only warns that it matches
nothing the user owns, which is the correct and expected warning when
claiming a new name (RFC 2947). And omitting `yank` is deliberate rather
than minimal-by-habit: it means a yank cannot happen by reflex, because
performing one would require minting a new token, which is itself the
severity checkpoint the contingency demands.

### What crates.io actually received

Registry metadata: `scarp` `0.1.0`, published `2026-07-31T00:04:51Z`,
description and repository as declared, `homepage` and `documentation`
both `null` as designed, keywords and categories as published, license
`MIT OR Apache-2.0`, `rust_version` `1.88`, `yanked: false`,
`crate_size` 126 900 bytes. The sparse index holds exactly one record,
`0.1.0`, with the ten declared dependencies.

Ownership was confirmed twice. `cargo owner --list scarp --registry
crates-io` returned `henry-filgueiras (Henry Filgueiras)`. Because a
`publish-new` token might not have satisfied an authenticated command,
a token-free proof was taken independently from the public owners
endpoint, which returns HTTP 200 unauthenticated:
`login=henry-filgueiras`, `kind=user`, `id=436227`. Sole owner. The
scoped token turned out to satisfy `cargo owner --list` anyway; the
independent path is recorded because it is the one that does not depend
on a credential that should not outlive the release.

**The artifact was tied to the commit rather than assumed to be.** The
`.crate` was downloaded from `https://crates.io/api/v1/crates/scarp/0.1.0/download`
— the registry, not a path or Git source — into scratch:

```text
downloaded archive SHA-256 : 70c581ce09ab554e9448adccec954647d7fca0c64848a1e1af690e069bec6cad
sparse-index cksum         : 70c581ce09ab554e9448adccec954647d7fca0c64848a1e1af690e069bec6cad
pre-publication package    : 70c581ce09ab554e9448adccec954647d7fca0c64848a1e1af690e069bec6cad
```

All three agree, so the comparison is stronger than the criterion asked
for: the published artifact is **byte-identical** to the package built
and verified before publication, not merely equivalent in payload.
`diff -r` over the two unpacked trees reports no difference at all, the
file set is the same 36, and the registry copy's
`.cargo_vcs_info.json` names `e69e722ef873d39a8c30328146d3edbc1945237c`
with no dirty flag.

That byte-identity is worth one sentence of caution: it is a property
observed here, not a guarantee Cargo makes. It held because nothing
between the two packagings touched a packaged file — which is exactly
what the `include` allowlist is for, since this task's own Result lives
in `archaeology/` and cannot move the payload.

### Tag and release

```text
git tag -a v0.1.0 e69e722ef873d39a8c30328146d3edbc1945237c -m "Scarp v0.1.0"
git push origin refs/tags/v0.1.0:refs/tags/v0.1.0
```

Both run by Henry. The tag object is `8e6acf9fefaf4665f20271b88607a9386f7dc12e`,
type `tag` — annotated, not lightweight — tagged `2026-07-30 17:06:51
-0700`. Verified read-only on the remote, before and again after the
release was created:

```text
8e6acf9fefaf4665f20271b88607a9386f7dc12e  refs/tags/v0.1.0
e69e722ef873d39a8c30328146d3edbc1945237c  refs/tags/v0.1.0^{}
```

The peeled commit is the release source. `--verify-tag` proves `gh` did
not invent a tag from the default branch; the peeled-commit check is the
separate question of *which* commit that tag names, and both were asked.

GitHub release: <https://github.com/henry-filgueiras/scarp/releases/tag/v0.1.0>
— `Scarp v0.1.0`, published `2026-07-31T00:21:59Z`, not a draft, not a
prerelease, **zero assets**. Notes were written and reviewed, not
autogenerated, and no prebuilt binaries were attached: sprint 8 prefers
one honest install path to an unverified matrix.

### Clean-room install and quickstart

The environment was secured **before** publication, so that a missing
container runtime could not become a reason to skip the check after the
irreversible step. Docker Desktop was not running and was started; the
official `rust:1.88` image was pulled ahead of time.

```text
image      rust:1.88
digest     sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0
platform   linux/arm64, Debian GNU/Linux 12 (bookworm)
toolchain  rustc 1.88.0 (6b00bc388 2025-06-23) / cargo 1.88.0 (873a06493 2025-05-10)
```

The container received no checkout, no `target/`, and no host
`CARGO_HOME`; freshness was asserted inside it rather than assumed —
registry cache absent, no `scarp` on `PATH`, zero host mounts. Rust
1.88 was chosen so the install also exercises the declared MSRV rather
than a convenient newer toolchain.

```text
cargo install scarp --version 0.1.0 --locked
  ... 67 crates downloaded from the registry ...
  Installing /usr/local/cargo/bin/scarp
   Installed package `scarp v0.1.0` (executable `scarp`)
scarp --version -> scarp 0.1.0
```

`Installed package \`scarp v0.1.0\`` carries **no path or Git source in
the parentheses**, which is how a registry install is distinguished from
the local-path form seen earlier in this task
(`scarp v0.1.0 (/tmp/.../prepub/scarp-0.1.0)`).

The quickstart was not retyped. It was extracted mechanically from the
packaged README and proved a **byte-exact substring of the
release-source README** (485 bytes at offset 4776, immediately after the
` ```sh ` fence), carried into the container as base64 with matching
SHA-256 on both sides, and run verbatim with no adaptation. Exit 0; the
caller's working directory and contents unchanged; no leftover temporary
directory. Its output compares **identical** to the README's `console`
block after normalising only the three fields the prose already declares
as varying — the temporary path, the ULID, and the date.

**Timings, separated as the sprint amendment requires:**

| Measurement | Result |
|---|---|
| Registry fetch + install/compile, cold | **8.004 s** (cargo's own release-profile timer: 7.96 s) |
| Post-install quickstart | **5 ms** |

Only the second is what sprint 8's ~60-second criterion measures, and it
is met by four orders of magnitude. The first is machine-dependent and
outside this project's control; on an 18-core host with a container and
a cold cache it is fast, and the README says so without promising a
number.

### Live surfaces, and the one defect

**docs.rs reached a successful terminal state**, not merely a queued
one. `https://docs.rs/crate/scarp/0.1.0/status.json` returns
`{"doc_status": true}`; build `3996525` is check-marked, took 11 s under
`rustc 1.99.0-nightly (d0babd8b6 2026-07-15)` and `docsrs 0.0.0
(f1b150f0cd 2026-07-19)` in the network-blocked sandbox, and its log
ends `Generated /opt/rustwide/target/x86_64-unknown-linux-gnu/doc/scarp/index.html`.
The only warnings are the three known private-intra-doc-link warnings.
`https://docs.rs/scarp/0.1.0/scarp/` loads and its front page carries
the intended crate description.

**crates.io was inspected in a browser as a reader sees it**, in both
light and dark theme, rather than only in HTML source.

Supported: the wordmark renders in **both** themes with the correct
variant selected and no broken-image placeholder, so the `<source
srcset>` repair task 43 made from first principles is now confirmed on
the live page; **both Mermaid diagrams render** as flowchart SVGs with
every `<br/>`-separated label legible, confirming task 43's jsdom-bound
prediction against the real renderer; the metadata sidebar is correct
(v1.88.0, `MIT OR Apache-2.0`, 124 KiB, docs.rs link, repository, owner,
categories); the install command reads `cargo install scarp@0.1.0`; all
seven absolute README links return HTTP 200, as do both media URLs; and
there is no stale product identity — zero `Strata`, zero `SCARP`, zero
`.strata.toml`, and exactly one lowercase `strata`, which is the
positioning line's metaphor and correct.

**The defect: the `#quickstart` anchor is dead on crates.io.**

The *See it work* paragraph links `[quickstart](#quickstart)`.
crates.io prefixes every heading id with `user-content-`, so the heading
is `id="user-content-quickstart"`, but it does **not** rewrite
author-written same-page hrefs and ships no scroll shim. No element with
`id="quickstart"` exists in the rendered README. Reproduced in the
browser rather than inferred: clicking the link changed the URL to
`…/0.1.0#quickstart` and the viewport did not move. The ten anchors
crates.io generates for itself all resolve, because it emits both sides
of those.

It is **not** a defect on GitHub. GitHub emits the same
`user-content-quickstart` id, but its own permalink also uses
`href="#quickstart"` and a client-side handler performs the mapping —
verified against the live repository page. The README source is correct
where it was checked and wrong only here.

This is [[log_01KYK8RC0YEY51YP37RGV7M7N4|log 3]]'s shape once more, in
its purest form. Task 44 verified this anchor and its verdict —
"the `#quickstart` anchor matches a real heading" — was **true**. It was
checked against the README source and against GitHub, and it was
structurally incapable of seeing crates.io's id rewriting, because that
surface did not exist yet. The precondition for the defect was
established by the renderer, not by the work; but the principle holds in
its general form all the same: a check run only where the artifact
already behaves correctly proves nothing about where it does not.

**Contingency applied as written, not improvised.** The classification
was fixed in scratch *before* the publish command was handed over. A
cosmetic README, logo, or metadata defect does not justify a yank; this
is one convenience link, to a section three screens below on the same
page, which a reader reaches by scrolling. **`0.1.0` is not yanked.**
The remediation is [[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|task 47]] in
[[spr_01KYTS35VYCFQ6BJN7HD2F0TK7|sprint 9]], shipping in `0.1.1`.
`0.1.0` stays in the record with the defect stated.

### Credential cleanup

After ownership verification was complete, Henry was instructed to
remove Cargo's local copy of the credential:

```sh
cargo logout --registry crates-io
```

He was also reminded that, because the token was created for this
release, `cargo logout` removes **only Cargo's local copy** and the
token must additionally be revoked in crates.io account settings.
Neither action was performed or inspected by any agent, and no token
material appears anywhere in this archaeology or in any command line.

Henry confirmed on 2026-07-30 that both were done: `cargo logout`
succeeded, and the token was separately revoked in the crates.io web
interface. The distinction matters and is the reason the reminder was
given — `cargo logout` only removes the local credential, so a token
that outlived it would still have been live at the registry.

### Why this commit necessarily comes after the tagged commit

The tag points at `e69e722…` because that is what was published, and it
must keep pointing there. Everything in this Result — the publish
output, the registry checksum, the tag object's own hash, the release
URL, the container timings, the crates.io rendering verdict — describes
events that **could not truthfully exist** until after that commit was
published and tagged. A record of a publication cannot be inside the
commit it publishes.

So the closure commit is deliberately later and deliberately **not**
tagged. This is an ordering consequence, not an oversight, and it is
stated here so that a future reader does not "correct" the tag to
include the archaeology. Doing so would make the tag name a tree that
was never published, which is the precise failure this project exists to
prevent.
