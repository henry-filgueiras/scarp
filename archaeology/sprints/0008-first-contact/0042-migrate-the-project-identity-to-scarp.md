---
id: tsk_01KYJE2K3PK4F5XC81N8S6PBNA
sequence: 42
kind: task
status: closed
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-27
closed: 2026-07-27
---

# Migrate the project identity to Scarp

## Objective

Migrate all repository-controlled current product, package,
executable, library, configuration, documentation, automation, test,
and contributor surfaces from Strata to Scarp; record the naming
decision; and produce an exact operator runbook for the external
GitHub cutover. Performing the push and GitHub mutations is
explicitly outside this task's authority.

Henry adjudicated the release identity on 2026-07-27 following
[[tsk_01KYFRWF1X37N5TBJ139X7ZKA1|task 40]]'s naming tournament and
its subsequent five-letter deep screen. The adjudication itself is
recorded as a standalone decision; this task is the implementation
and the runbook, not another naming round.

The accepted identity is:

```text
Scarp
henry-filgueiras/scarp
scarp
use scarp::
.scarp.toml
_scarp
target/debug/scarp
```

Because the project is unreleased, the marker change is a clean
pre-release cut: `.strata.toml` becomes `.scarp.toml` with no dual
discovery path and no `strata` compatibility executable. This
supersedes only the product-name-dependent marker spelling in
[[dec-bootstrap-repo-marker|decision 5]]; every other guarantee that
decision makes remains controlling.

The boundary between historical and current names is load-bearing.
Existing artifact ids, filenames, titles, and historical statements
are not renamed retroactively: earlier `strata` commands and
`.strata.toml` spellings were accurate at the commits where they
were written, and rewriting them would falsify the record this
project exists to keep.

## Acceptance criteria

- A standalone accepted decision records Henry's adjudication, the
  exact identity above, the positioning line, the capitalization and
  geological-metaphor rules, the absent compatibility executable, the
  hard marker cut, the unchanged config schema version, precisely
  what it supersedes in decision 5, the retroactive-rename
  prohibition, the acknowledged residual objections, and the
  crates.io recheck obligation before publication.
- crates.io `scarp` and `henry-filgueiras/scarp` are re-verified as
  genuinely absent — not-found results, not authentication or network
  failures — before any mutation.
- Every live repository-controlled identity surface is migrated:
  Cargo package name and repository URL, `Cargo.lock`, library
  imports, clap command name, generated completion command name, help
  text, diagnostics, doc comments, the repository marker via a
  tracked rename, discovery and initialization constants,
  product-owned temporary-file prefixes, all tests, `scripts/check.sh`,
  `scripts/session-start.sh`, `.claude/settings.json`,
  `.claude/commands/next.md`, `README.md`, `CLAUDE.md`,
  `CONTRIBUTING.md`, `SECURITY.md`, the GitHub issue templates, the
  light and dark wordmarks with their accessibility labels, and every
  current repository URL and command example.
- No binary or Cargo target named `strata` remains, verified through
  Cargo metadata rather than the presence or absence of a stale build
  artifact.
- `archaeology/`, existing stable ids, existing artifact filenames and
  titles, historical archaeology prose, and
  `scripts/bootstrap-inception.sh` are not renamed;
  `scripts/README.md` makes the frozen-provenance status of the
  bootstrap script's old product name, marker, and commands explicit.
- Regression coverage makes the rename behavioral, not textual:
  version output, help output, completion scripts for all supported
  shells, marker creation, discovery, mutation, and doctor all assert
  the Scarp identity.
- The full gate passes and representative commands — `doctor`,
  `list`, `show`, `fortune`, `resolve`, `completions` — are exercised
  through the renamed binary against this repository's own corpus.
- A classified audit of every remaining `Strata`/`strata` occurrence
  outside `archaeology/` is recorded, each hit assigned to intentional
  historical evidence, the lowercase geological metaphor, or the
  pending operator runbook. No stale current command, package, binary,
  marker, repository URL, or product name hides behind the metaphor
  exception.
- The Result carries a durable, exact runbook for the human-owned
  GitHub cutover — preflight checks, push, `gh repo rename`, remote
  update by transport, public metadata, CI verification, and
  non-repository references — stated as pending human execution and
  never implying it happened.
- No push, repository rename, GitHub settings change, crate
  publication, tag, or release is performed by this task.

## Result

All work performed 2026-07-27 from a clean checkout equal to
`origin/main` at `feb6b42`.

### Accepted identity

Henry adjudicated **Scarp** on 2026-07-27. The adjudication is
recorded in full as
[[dec_01KYJE2K3VRASS8A1X1E847S1B|decision 16]], which supersedes only
[[dec-bootstrap-repo-marker|decision 5]]'s product-name-dependent
marker spelling and leaves every other guarantee that decision makes
controlling.

```text
Scarp                      project / display name
henry-filgueiras/scarp     GitHub repository
scarp                      crates.io package, executable, library
use scarp::                Rust library import
.scarp.toml                repository marker
_scarp                     zsh completion function
target/debug/scarp         development binary
```

> Scarp exposes the strata of a repository: what changed, why, and
> what remains unsettled.

Availability re-verified before any mutation, both genuine
not-found results rather than authentication or network failures:
crates.io `scarp` returned HTTP 404 from both the API
(`{"errors":[{"detail":"crate 'scarp' does not exist"}]}`) and the
sparse index at `index.crates.io/sc/ar/scarp`, while
`gh repo view henry-filgueiras/scarp` failed with
`GraphQL: Could not resolve to a Repository` in the same session
where `gh auth status` reported an active login and
`gh repo view henry-filgueiras/strata` returned live JSON. A
repository rename reserves nothing on crates.io; the release task
must recheck immediately before publishing.

### Implementation summary

- **Package**: `Cargo.toml` name `strata` → `scarp`, repository URL
  → `https://github.com/henry-filgueiras/scarp`; `Cargo.lock`
  regenerated by Cargo (the package entry moved position because the
  file is name-sorted). The crate description and `version = "0.1.0"`
  are unchanged.
- **Marker**: `.strata.toml` → `.scarp.toml` as a tracked rename
  (`git mv`, recorded as `R`). `repo::CONFIG_FILE` and the
  surrounding discovery, initialization, and validation
  documentation follow it. Contents are byte-identical:
  `version = 1`, because only the filename is product-owned.
- **Binary and library**: clap `#[command(name = "scarp")]`, the
  completion generator's command name, `use scarp::…` in
  `src/main.rs`, and the crate-level docs in `src/lib.rs`.
- **Diagnostics**: every user-facing remedy string now names the
  current executable — `scarp init`, `scarp doctor`, `scarp new`,
  `scarp close`/`reopen`/`adopt`/`reject`, `scarp list`,
  `scarp new sprint`, and the `--sprint` misuse message.
- **Temporary-file prefixes**: `.strata.init.tmp` →
  `.scarp.init.tmp` in `repo.rs`; `.strata.artifact.tmp` →
  `.scarp.artifact.tmp` in `artifact.rs` and `transition.rs`, with
  the matching skip rule and its unit test in `read.rs`.
- **Tests**: all 15 existing integration files plus the new
  `tests/identity.rs` — `CARGO_BIN_EXE_scarp`, the `scarp_in`
  helpers, expected diagnostics, marker literals, temporary-file
  names, and completion assertions. `cargo fmt` reflowed several
  call sites because the shorter helper name changed line widths.
- **Automation and docs**: `scripts/check.sh`,
  `scripts/session-start.sh`, `.claude/settings.json` (the
  `./target/debug/scarp` allowlist entries), `.claude/commands/next.md`,
  `README.md`, `CLAUDE.md`, `CONTRIBUTING.md`, `SECURITY.md`, and both
  GitHub issue templates.
- **Wordmarks**: `assets/logo.svg` and `assets/logo-dark.svg` — the
  `aria-label` accessibility label and the wordmark text only. The
  strata-tile artwork, geometry, palette, and viewBox are untouched;
  the logo was not redesigned.
- **README**: the positioning line was added under the wordmark,
  where it also serves as the name's gloss for a first-time visitor
  who has no reason to know what a scarp is.
- **CLAUDE.md**: gained a short `### Name` section recording the
  rename, the capitalization rule, the surviving lowercase metaphor,
  and the do-not-rewrite-history boundary — future agents will meet a
  corpus that says both names and need the rule, not a search result.
- **scripts/README.md**: rewritten to state explicitly that
  `bootstrap-inception.sh` is frozen provenance whose `strata`
  commands and `.strata.toml` writes are a dated record of what the
  inception performance did, never instructions to run today.

### Historical versus current names

The boundary held everywhere. Not renamed: `archaeology/` and every
artifact in it, all stable ids (including the artifact id
`idea-strata-fortune`, which appears verbatim in `src/artifact.rs`'s
documentation of legacy id validity), all artifact filenames and
titles, all historical prose, and `scripts/bootstrap-inception.sh`
(50 occurrences, deliberately frozen). Earlier `strata` commands and
`.strata.toml` spellings remain accurate at the commits where they
were written.

### Verification

Exact commands and results, 2026-07-27:

- `scripts/check.sh` — **passed**: `cargo fmt --check` clean, full
  test suite green, `cargo clippy --all-targets --all-features -D
  warnings` clean, `scarp doctor` reporting
  `102 artifact(s) checked, no problems found`.
- `cargo metadata --format-version 1 --no-deps` — exactly one
  package, `scarp 0.1.0`, with targets `scarp [lib]`, `scarp [bin]`,
  and sixteen test targets. **No target named `strata` exists.**
  A stale `target/debug/strata` binary from a pre-rename build is
  still present on disk; it is untracked build output, not a Cargo
  target, and `cargo clean` was deliberately not used to make it
  disappear, since Cargo metadata is the load-bearing evidence.
- `cargo run --quiet -- --version` → `scarp 0.1.0`.
- `./target/debug/scarp --help` → `Usage: scarp <COMMAND>`.
- `./target/debug/scarp doctor` →
  `doctor: 102 artifact(s) checked, no problems found` against this
  repository's own corpus.
- `./target/debug/scarp list tasks --active` → tasks 40, 41, and 42
  under sprint 8.
- `./target/debug/scarp show decision:16` → the new decision.
- `./target/debug/scarp resolve task:42 decision:16 --json` → both
  stable ids and paths, deterministic single-line JSON.
- `./target/debug/scarp fortune` → surfaced idea 25.
- `./target/debug/scarp completions zsh|bash|fish` →
  `#compdef scarp`, `_scarp()`, and
  `__fish_scarp_global_optspecs` respectively.

New behavioral coverage in `tests/identity.rs` (6 tests), so the
rename cannot silently regress to a textual one:

- `--version` reports `scarp <version>`, not merely the number;
- `--help` shows `Usage: scarp` and contains no `strata` anywhere;
- completions for all five supported shells (bash, zsh, fish,
  elvish, powershell) contain `scarp` and no `strata`, with the
  `#compdef scarp`, `_scarp`, and bash `_scarp` names pinned;
- `init` writes `.scarp.toml` with `version = 1`, never the retired
  marker, and reports the file it wrote;
- a directory carrying only `.strata.toml` is **not** a repository:
  `list dragons` exits 3 with `error[missing-repository]` and a
  remedy naming `scarp init` — the hard cut is enforced, not assumed;
- discovery from a nested directory, `new`, `close`, and `doctor`
  all operate through `.scarp.toml`, and renaming the marker away
  removes the repository.

The last two matter because every pre-existing marker test resolved
through the `CONFIG_FILE` constant, so the literal filename and the
absence of a fallback probe were previously unasserted.

### Remaining `Strata`/`strata` occurrences, classified

Audit command:

```sh
rg -n --hidden -i '\bstrata\b|\.strata|henry-filgueiras/strata|_strata|CARGO_BIN_EXE_strata' \
  --glob '!.git/**' --glob '!target/**' --glob '!archaeology/**'
```

Every hit outside `archaeology/` falls into exactly one intentional
class:

| Location | Class |
|---|---|
| `scripts/bootstrap-inception.sh` (50) | frozen historical provenance |
| `scripts/README.md:11` | prose declaring that script frozen |
| `src/artifact.rs:13` | the stable id `idea-strata-fortune` |
| `CLAUDE.md:17,22–23` | the rename record and metaphor rule |
| `README.md:9` | positioning line, lowercase metaphor |
| `tests/identity.rs:3,18,59,82,118` | negative assertions and the retired-marker constant the rename regression needs |

No current command, package name, binary name, config marker,
repository URL, or product name remains. `henry-filgueiras/strata`
appears nowhere outside `archaeology/` and the runbook below.
Inside `archaeology/`, historical occurrences are left untouched by
design.

### Human-owned GitHub cutover — not yet performed

**Nothing in this section has been executed.** No push, no repository
rename, no GitHub settings change, no crate publication, no tag, and
no release was performed by this task. The steps below are a runbook
for Henry, written to be run later and adapted where inspection
proves necessary.

Primary interface references:

- GitHub CLI `gh repo rename`:
  <https://cli.github.com/manual/gh_repo_rename>
- GitHub repository rename behavior:
  <https://docs.github.com/en/repositories/creating-and-managing-repositories/renaming-a-repository>

Several conditional branches were pre-resolved by read-only
inspection on 2026-07-27; each is flagged inline. Re-verify anything
that may have changed since.

#### 1. Confirm the local migration commit and GitHub access

```sh
git status --short --branch
git log -1 --oneline
scripts/check.sh

gh auth status
gh repo view henry-filgueiras/strata \
  --json nameWithOwner,url,defaultBranchRef,description,homepageUrl
```

The worktree must be clean and the migration commit must be `HEAD`.

Check the target explicitly:

```sh
gh repo view henry-filgueiras/scarp
```

This must fail specifically because the repository does not exist. If
it succeeds, or fails for authentication or network reasons, stop
rather than renaming blindly.

Snapshot external state before cutover:

```sh
gh pr list --repo henry-filgueiras/strata --state open
gh run list --repo henry-filgueiras/strata --limit 10
gh api repos/henry-filgueiras/strata/pages
```

For the Pages call: HTTP 404 means Pages is disabled and no Pages
action is needed; a JSON response means Pages is enabled, and its URL
must be accounted for before renaming because GitHub does not
redirect repository project-site URLs.

*Observed 2026-07-27*: no open pull requests; the five most recent CI
runs on `main` all succeeded; `gh api …/pages` returned HTTP 404, so
Pages is disabled and every Pages step below is currently a no-op.

#### 2. Push the migration commit under the old repository name

```sh
git push origin main
```

Do not force-push.

#### 3. Rename the GitHub repository

```sh
gh repo rename scarp \
  --repo henry-filgueiras/strata \
  --yes
```

If the CLI command is unavailable, the exact web fallback is:

1. Open <https://github.com/henry-filgueiras/strata/settings>.
2. Stay on **Settings → General**.
3. In **Repository name**, replace `strata` with `scarp`.
4. Click **Rename**.

Do not create a second repository and mirror into it. Rename the
existing repository so issues, history, stars, and redirects stay
attached.

#### 4. Update this clone's remote explicitly

First inspect its current transport:

```sh
git remote get-url origin
```

Run exactly one matching command.

HTTPS:

```sh
git remote set-url origin https://github.com/henry-filgueiras/scarp.git
```

SSH:

```sh
git remote set-url origin git@github.com:henry-filgueiras/scarp.git
```

If the remote uses another GitHub URL form, preserve its transport
and host and change only `henry-filgueiras/strata` to
`henry-filgueiras/scarp`.

*Observed 2026-07-27*: this clone's `origin` is
`git@github.com:henry-filgueiras/strata.git`, so the SSH command is
the matching one — but inspect rather than assume, in case the
transport changed.

Then verify:

```sh
git remote -v
git fetch --prune origin
git status --short --branch
test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)"
```

Repeat the remote update in every other local clone. Optionally
rename the local checkout directory from its parent directory, but
only after confirming no sibling `scarp` path exists.

#### 5. Update GitHub's public metadata and verify CI

```sh
gh repo edit henry-filgueiras/scarp \
  --description "Scarp exposes the strata of a repository: what changed, why, and what remains unsettled."

gh repo view henry-filgueiras/scarp \
  --json nameWithOwner,url,defaultBranchRef,description,homepageUrl

gh run list --repo henry-filgueiras/scarp --limit 10
```

In the GitHub web UI:

- open the renamed repository's **Actions** tab and confirm the
  migration commit's CI run passes;
- inspect the **About** panel and correct its description, homepage,
  and topics if any still carry the old identity;
- if branch rules or rulesets exist, inspect **Settings → Rules** and
  confirm they still target `main`;
- if Pages was enabled, inspect **Settings → Pages** and update any
  links or configuration affected by the new project-site URL
  (currently a no-op — Pages is disabled);
- inspect configured webhooks or external integrations only if any
  exist, and ensure none keys itself permanently to the old full
  name.

GitHub redirects ordinary web and Git traffic from the old repository
name, but calls to a GitHub Action hosted in a renamed repository are
**not** redirected. Confirm this repository is not consumed as
`uses: henry-filgueiras/strata@…`; if such consumers exist, treat
them as a separate migration obligation.

*Observed 2026-07-27*: the repository publishes no `action.yml` or
`action.yaml`, so it cannot be consumed as a GitHub Action; the only
workflow, `.github/workflows/ci.yml`, hard-codes no repository name
and needs no change.

Never create a new `henry-filgueiras/strata` repository later:
reusing the old name destroys GitHub's redirect.

#### 6. Update non-repository references

Manually update the ChatGPT Project's instructions from:

```text
Repo is at https://github.com/henry-filgueiras/strata
```

to:

```text
Repo is at https://github.com/henry-filgueiras/scarp
```

Also update bookmarks, saved prompts, local automation, and any other
project configuration outside the Git repository that hard-codes the
old URL.

If the old local pre-release package was installed, inspect before
removing anything:

```sh
cargo install --list | rg '^strata v0\.1\.0'
```

Only if that output proves the local pre-release package is
installed:

```sh
cargo uninstall strata
cargo install --path .
```

Do not remove an unrelated package such as `strata-rs`.

If the old zsh completion exists, preserve it temporarily and
generate the new one:

```sh
mv ~/.zfunc/_strata ~/.zfunc/_strata.pre-scarp
scarp completions zsh > ~/.zfunc/_scarp
```

This local cleanup is optional and conditional; it is not part of the
GitHub rename.

*Observed 2026-07-27*: `cargo install --list` shows no `strata` or
`scarp` package installed, and `~/.zfunc` does not exist — so both
cleanups are currently no-ops. The non-repository reference updates
(ChatGPT Project instructions, bookmarks, saved prompts) remain real
work.

#### What this task did not do

No push, repository rename, GitHub settings change, crate
publication, tag, or release was performed. Every external operation
above is **pending human execution**. If Henry runs the cutover and
wants the first performance preserved as dated provenance per
`CLAUDE.md`, return the actual command output and it will be appended
to this Result as an addendum.
