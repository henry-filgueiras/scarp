---
id: tsk_01KYJG0S7SYMYY1FEG7H4QQX8G
sequence: 44
kind: task
status: closed
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-27
closed: 2026-07-27
---

# Audit the landing surfaces a first-time visitor sees

## Objective

Take a deliberate once-over of every surface a stranger encounters
before they have decided whether to care, and make them agree with
each other and with what the tool actually does. This is sprint 8's
"the repository presents coherently to a first-time external visitor"
criterion, run as its own pass rather than assumed as a side effect
of writing the README.

It runs after [[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|task 43]] so that the
install and quickstart prose that task writes is inside the audit's
scope rather than outside it.

It also runs after [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]], which
replaced that quickstart wholesale after task 43 closed: the original
was unsafe under ordinary pasted-shell semantics and could leave
Scarp output in a reader's own directory. The surface this audit
covers is therefore task 46's quickstart, not task 43's. Task 46 is
sequenced later than this task only because it was allocated later;
the sprint's amended execution order — 43, 46, 44, 45 — governs.

The audit has a specific standard to apply, not merely taste.
[[tsk_01KYFYKAZRGEJPJYKAWV8W9BB4|Task 41]] compared Scarp against
Edda and retired a list of claims as unsupportable: not "the only" or
first repo-local, Git-friendly, or agent-aware memory tool; not
"other tools lock your history in an opaque database"; not
determinism as a differentiator; not "safe for concurrent agents";
not "memory for coding agents" as the headline job, since that
promises capture and injection Scarp does not ship; no
tamper-evidence or audit-trail claims. It also fixed the positive
framing: Git-native, reviewable project archaeology, whose honest
distinguishing property is *continuability* — records that can be
directly edited, reviewed, branched, and merged — rather than mere
readability.

This is the **pre-publication** audit. It covers the final package
and README source and every surface that can be checked before a live
page exists. Inspection of the rendered crates.io page and the
docs.rs build belongs to [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]],
because neither exists until publication has happened.

### Suspects to resolve, verdicts not assumed

Each was observed on 2026-07-27 and is recorded here so the audit
starts from evidence rather than from scratch. None is a finding yet:
the audit may conclude a given wording is defensible. What it may not
do is leave one unexamined.

- **The README's "real output" is stale.** It shows `list dragons`
  with dragons 1, 2, and 3 all `open`; dragons 2 and 3 are now
  `closed`, and dragon 4 (power-loss durability) is absent entirely.
  The `jq '.[-1]'` example claims to show dragon 3, but the last
  element is now dragon 4. Prefer replacing corpus-dependent
  showcase output with output from the deterministic quickstart
  fixture: another snapshot of the live corpus would be accurate on
  the day it was pasted and wrong again within a sprint. Note also
  that the example depends on `jq`, which task 43 bars from the
  quickstart for the same reason it is questionable here.
- **The README presents Markdown, JSON, and JSONL as current payload
  formats**, though only Markdown CRUD ships.
  [[dec-bootstrap-payload-separation|Decision 3]] makes the multi-format
  architecture real as a design boundary, but the README's framing
  should not read as shipped capability.
- **"Safe writes", the hero's coding-agent framing, and the Cargo
  description** must be reconciled with task 41's retired claims and
  with open [[drg-bootstrap-branch-collisions|dragon 1]] (branch
  sequence collisions) and
  [[drg_01KY3C0S3JQKEMEB9BH6NVJ35F|dragon 4]] (power-loss durability
  of mutations). The hero currently reads "structured repository
  memory for humans and coding agents", close to the headline claim
  task 41 retired.
- **`CONTRIBUTING.md` names `archaeology/dragons/open/`**, a
  directory that does not exist — placement has been flat since
  decision 11 as amended. Confirmed 2026-07-27: `archaeology/dragons/`
  contains four files and no subdirectories.
- **`SECURITY.md` promises GitHub private vulnerability reporting,
  which is disabled.** `gh api
  repos/henry-filgueiras/scarp/private-vulnerability-reporting`
  returned `{"enabled":false}` on 2026-07-27. A security policy that
  directs people to a reporting channel that does not exist is a
  **release blocker**: it fails exactly when it matters, and
  publication is what brings strangers who might use it. Resolve it
  either by preparing the exact human-owned enablement command — the
  REST surface is `PUT
  /repos/{owner}/{repo}/private-vulnerability-reporting`, requiring
  repository admin and returning 204 on success — or by rewriting
  `SECURITY.md` to a reporting path that is actually open. **Do not
  silently execute the repository-setting mutation**; it is an
  external GitHub change and therefore Henry's.
- **`.github/ISSUE_TEMPLATE/idea.md` requests a nonexistent `idea`
  label.** `gh label list` on 2026-07-27 returned only the nine
  GitHub defaults: `bug`, `documentation`, `duplicate`,
  `enhancement`, `good first issue`, `help wanted`, `invalid`,
  `question`, `wontfix`. Either plan a human-owned label creation or
  deliberately use an existing label; a template referencing a label
  that does not exist applies no label at all.
- **GitHub detects a single license.** `gh api repos/…/license`
  reports Apache-2.0 from `LICENSE-APACHE`, and the repository's
  license badge says "Apache License 2.0", while
  [[dec-dual-mit-apache-licensing|decision 9]] establishes dual
  MIT/Apache-2.0. Confirm the README, the manifest's `license`
  field, the packaged files, and the repository's first screen make
  the actual dual contract unmistakable to a human reader. Do not
  contort source files merely to satisfy GitHub's detector — the
  detector's opinion is not the license.
- **GitHub About metadata** is inspected read-only: description,
  homepage, and topics. As of 2026-07-27 the description is decision
  16's positioning line, homepage is empty, and twelve topics carry
  no stale product name. Any change required is an external mutation
  and gets an exact operator runbook plus subsequent verification,
  not a silent edit.

## Acceptance criteria

- The audited surface list is explicit and complete, covering at
  least: `README.md` including the hero, positioning line, feature
  claims, status scoreboard, and the new quickstart; the packaged
  README source as it will be shipped; the GitHub About panel —
  description, homepage, topics — read only; `CONTRIBUTING.md`,
  `SECURITY.md`, `CODE_OF_CONDUCT.md`, and the issue and
  pull-request templates; the light and dark wordmarks; and the
  repository's own first screen. Any surface deliberately excluded is
  named with its reason.
- Every claim made on those surfaces is checked against what ships
  today, and each is classified as supported, overstated, retired by
  task 41, or aspirational-stated-as-present. Corrections are applied
  to the wording, not merely listed.
- Open dragons are reconciled against safety and reliability claims.
  Where an open risk contradicts a claim, either the claim changes or
  the surface acknowledges the risk; a claim is not left standing
  because the dragon is "probably fine in practice".
- The scoreboard and any feature list match the real command surface,
  verified by running the binary rather than by reading the source.
- The surfaces agree with each other: the name, the positioning line,
  the description, the license statement, and the quickstart tell one
  story. Contradictions between GitHub metadata, the crate page, and
  the README are resolved rather than tolerated because they live in
  different places.
- Every suspect listed above reaches a recorded verdict, including
  the ones the audit decides are fine. "Examined and defensible" is a
  finding; silence is not.
- The two external blockers — private vulnerability reporting and the
  missing `idea` label — leave this task either resolved in prose or
  carrying an exact human-owned runbook for Henry. Neither is
  executed here, and publication does not proceed with a security
  policy pointing at a closed channel.
- Live-surface verification is explicitly **out of scope** and
  deferred to [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]: the
  rendered crates.io page, the docs.rs build, and the installed
  package do not exist yet, and asserting anything about them here
  would be prediction rather than verification.
- Anything found that is real but out of scope becomes an idea or a
  dragon rather than an unrecorded observation or an opportunistic
  fix.
- `scripts/check.sh` passes, and the work is committed per the commit
  policy in `CLAUDE.md`.

## Result

All work performed 2026-07-27 from a clean checkout equal to `origin/main`
at `5c5b2010b24278953440ec4c1917cd70165f69ca`. That is two commits past
the `f66a94e` this task was commissioned against; both are additive
archaeology (ideas 23, 26, 28 amended, idea 34 added, log 3 added) and
touch no landing surface. The audit ran over the later tree.

### Surface inventory

Every surface below was opened and read in full, not sampled. The command
surface was taken from the **running binary** (`scarp --help` plus
`--help` on all eleven subcommands), never from `src/`.

| Surface | Verdict |
|---|---|
| README hero, wordmark, positioning | **corrected** — memory-for-agents headline retired |
| README `See it work` | **corrected** — stale corpus paste and `jq` both removed |
| README install, timing claims | **corrected** — measurement restated precisely |
| README quickstart block and transcript | **corrected** — two boundary defects fixed |
| README `How it fits` diagram and invariants | **corrected** — payload formats, safety wording |
| README lifecycles, containment, `archaeology/` tree | **supported** — checked against decision 11 as amended |
| README status scoreboard | **corrected** — two shipped commands were missing |
| README development, completions, case-study note | **supported** |
| README license section | **corrected** — dual grant made unmistakable |
| All 10 README link targets | **supported** — verified from GitHub *and* an unpacked crate |
| `Cargo.toml` description, keywords, categories, `include` | **supported** — see below |
| Packaged README as shipped | **supported** — byte-identical to source, 36-file set |
| `assets/logo.svg`, `assets/logo-dark.svg` | **supported** — identical but for text fill; `scarp`, never `SCARP` |
| `src/cli.rs` `--help` line, `src/lib.rs` crate doc | **corrected** — disagreed with every other surface |
| `CONTRIBUTING.md` | **corrected** — nonexistent path, MSRV, check-script contents |
| `SECURITY.md` | **externally resolved** + wording fixes |
| `CODE_OF_CONDUCT.md` | **supported** — Contributor Covenant, live contact address |
| `.github/ISSUE_TEMPLATE/bug-report.md` | **supported** — `bug` label exists |
| `.github/ISSUE_TEMPLATE/idea.md` | **externally resolved** — `idea` label now exists |
| `.github/PULL_REQUEST_TEMPLATE.md` | **supported** |
| GitHub About: description, homepage, topics, license | **externally resolved** (topics) / **supported** (rest) |

Deliberately excluded, with reason: the rendered crates.io page, the
docs.rs build, and a registry install do not exist before publication.
Asserting anything about them here would be prediction, not verification;
they belong to [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]].

### Verdict on every recorded suspect

**1. Stale `See it work` output — corrected, and structurally so.**
The section showed dragons 1–3 all `open` (2 and 3 are closed, 4 absent)
and a `jq '.[-1]'` example claiming to show dragon 3. Rather than paste a
fresher snapshot — accurate the day it is written and wrong within a
sprint — the showcase is now a three-command excerpt of the **quickstart
fixture**, whose corpus is created and destroyed by the block itself and
therefore cannot drift. The self-dogfooding story is kept, but as prose
and a link rather than as pasted output that rots: the claim now made is
that `scripts/check.sh` gates every commit on `scarp doctor` over the
whole corpus, which stays true as the corpus grows.

**2. `jq` dependency — retired.** No `jq` appears anywhere in the README.
Machine-readable output is still shown, as `scarp list dragons --json`
inside the quickstart, printing one deterministic line that a reader can
simply look at. The accompanying prose makes the point `jq` was making —
key on the stable `id`, not the display sequence — without the tool.

**3. Markdown/JSON/JSONL as current payload formats — corrected.** The
diagram node now reads `plain files / Markdown`. A new invariant bullet
states that payload format is not the artifact model (decision 3's real
boundary) and that **only Markdown artifacts ship today**. The Status
section draws the distinction the old text blurred: `--json` is an
*output* format that ships; JSON and JSONL *payloads* are architecture,
not a feature a reader can use.

**4. Hero, "safe writes", "assigned safely", Cargo description —
corrected against task 41 and dragons 1 and 4.**
The hero read "structured repository memory for humans and coding agents";
it now reads **"Git-native, reviewable project archaeology"**, matching
the manifest `description` and decision 16's positioning line word for
word. Agents survive as an *audience* ("a person or a coding agent"), never
as a promised capability. A new Status paragraph states plainly that there
is no automatic capture, no context injection, no search, no index, and no
coordination between concurrent writers.

Bare "safely" is gone from the scoreboard. In its place a new section,
*What "safe" means here, and where the dragons are*, states decision 8's
three failure classes in a reader's language: returned errors leave one
valid artifact; abrupt termination is covered by single atomic renames;
**power loss and kernel crashes are explicitly out of scope because
nothing is fsynced** ([[drg_01KY3C0S3JQKEMEB9BH6NVJ35F|dragon 4]]). A
following paragraph states that Scarp does **not** claim concurrent-writer
safety and that duplicate display sequences after a merge are reconciled
manually ([[drg-bootstrap-branch-collisions|dragon 1]]). The old bullet
claiming branches "can collide on sequence numbers without corrupting
anything" is gone. Both dragons are linked, which is the tool's own
argument made on itself.

**5. `CONTRIBUTING.md` names `archaeology/dragons/open/` — corrected.**
Confirmed again: `archaeology/dragons/` holds four files and no
subdirectories. The instruction now points at `scarp list dragons` and
`scarp list decisions`, names the real directories, and says explicitly
that lifecycle state lives in front matter so there are no `open/` or
`closed/` subdirectories to look in. Two further inaccuracies found in the
same file while reading it: "Stable Rust toolchain" is now "Stable Rust,
1.88 or newer", and `scripts/check.sh` was described without the `scarp
doctor` step it has run since sprint 7.

**6. `SECURITY.md` pointed at a disabled channel — externally resolved.**
Henry enabled private vulnerability reporting during this task (provenance
below). The policy keeps its truthful text. Two wording fixes were applied
while the file was open: "has no releases yet" would have become false the
moment task 45 publishes, so it now reads in a form true before and after;
and the decision 8 reference became a real link that names which failure
classes are out of scope.

**7. `idea` issue template requested a nonexistent label — externally
resolved.** Henry created the label. The template is unchanged, and now
actually applies a label.

**8. GitHub reports a single license — corrected on the surfaces, and an
erratum filed.** The detector's opinion was not fought. Instead: the README
hero now carries `MIT OR Apache-2.0` on the first screen, and the License
section states the dual grant and explains the sidebar as a detector
artifact. The manifest already declared `license = "MIT OR Apache-2.0"`,
and both license texts are in the 36-file package.

[[dec-dual-mit-apache-licensing|Decision 9]]'s claim that "the dual-file
layout means GitHub reports both licenses" is **demonstrably false** and
received a dated erratum. The evidence is sharper than expected: the same
API query against the three exemplars decision 9 itself cited returns a
single `Apache-2.0` for all of them.

```text
serde-rs/serde  -> Apache-2.0
rust-lang/rust  -> Apache-2.0
clap-rs/clap    -> Apache-2.0
```

The decision's *second* clause therefore survives and explains the first:
the layout is ecosystem-standard and Scarp does appear exactly as serde,
clap, and rustc appear — the error was believing that appearance ever
showed both.

**9. GitHub About metadata — adjudicated, one change, externally
resolved.** Description is decision 16's positioning line: **supported**,
unchanged. Homepage is empty: **supported** — Scarp has no dedicated site,
and a repository URL duplicating the sidebar link would be noise.

Topics were **not** treated as exempt from task 41. Discovery metadata
makes claims to exactly the audience task 41 was about, and the two topics
split:

- `agent-memory` — **retired.** It is task 41's retired headline job
  compressed into one word. Someone searching it wants capture and
  injection; Scarp ships neither, and the topic would recruit precisely
  the disappointed visitor task 41 warned about. Henry removed it.
- `ai-agents` — **supported, kept.** It names an audience, not a
  capability. Agents genuinely are intended callers and the repository is
  a real human–AI collaboration case study; nothing is promised.

Manifest keywords were adjudicated by the same standard, since they are
crates.io's equivalent surface. `project-memory` is **supported**: it
describes what the files are, and unlike `agent-memory` it promises no
capture mechanism. `archaeology`, `decision-records`, `adr`, and
`documentation` are supported. Categories
(`command-line-utilities`, `development-tools`) are supported.

**10. Scoreboard versus the real command surface — corrected; this suspect
was not on the list and mattered most.** Running the binary rather than
reading the source found the scoreboard was **missing two shipped
commands**: `scarp resolve` (absent entirely) and `scarp completions`
(documented only in a later prose section). Also unlisted: `--json` on
`new`, `show`, and `resolve`; `--resolved-by` on `close`; `--adopted-by`
on `adopt`; `--active` on `list`. The table now carries all eleven
subcommands with their real flags, and the prose corrects a genuine
overstatement — lifecycle verbs are **collection-specific**, not
universal: `close` takes dragons, sprints, and tasks; `reopen` only
dragons; `adopt`/`reject` only ideas.

**11. Surface disagreement found during the audit — corrected.** Four
surfaces told three different stories. `scarp --help` and the crate-level
doc (which is docs.rs's front page) both read "Git-friendly project
archaeology and repository-local memory", disagreeing with the manifest
and carrying the memory framing task 41 retired. Both now carry the same
sentence as `Cargo.toml`. The name, positioning line, description, license
statement, and quickstart now tell one story across README, manifest, CLI
help, crate docs, and GitHub About.

### The two carried-forward quickstart boundary findings

Both were reproduced before being fixed, and neither reopened task 46's
closed defect.

**Ctrl-C portability — claim removed, and the reported finding was
understated.** Tested in a genuine PTY (`pty.fork`, real `INTR` character
through the line discipline, delivered to the foreground process group),
three trials per shell, all deterministic:

| Shell | `EXIT` trap after Ctrl-C | Directory removed |
|---|---|---|
| `bash` 3.2.57 | runs | yes |
| `/bin/sh` (bash in POSIX mode) | runs | yes |
| `zsh` 5.9 | **does not run** | **no** |
| `dash` | **does not run** | **no** |

The prompt's evidence named Bash and dash; **zsh also leaks**, which
matters because zsh is Henry's shell and the first one a macOS reader
will paste into. The README's "on success, on failure, and on `Ctrl-C`
alike" was therefore false in half the shells tested, including the most
likely one. The portable guarantee is **removed** rather than defended
with signal handling. The README now states the proven contract — normal
completion and ordinary command failure — and names the interrupt
behaviour honestly, per-shell, with the cost bounded: one throwaway
directory whose path `scarp init` printed on the first line. No trap
cathedral was built.

**Relative `TMPDIR` — reproduced, fixed, verified.** With `TMPDIR=tmp`,
`mktemp -d` returns a *relative* path; after the `cd`, the trap resolves
that same string against the new working directory, `rm -rf` silently
succeeds against a nonexistent path because of `-f`, and the real
directory survives. Reproduced exactly: the block exited 0 and left
`tmp/scarp-demo.VGfrWV/` containing `.scarp.toml`, `archaeology/`, and the
created dragon.

The fix is one POSIX line, no subshell trick and no `CDPATH` hazard:

```sh
case $scarp_demo_dir in /*) ;; *) scarp_demo_dir="$PWD/$scarp_demo_dir" ;; esac
```

The full matrix was then executed — 4 shells × 4 cases, 16 runs, against
the binary installed from the unpacked package, each in a caller directory
seeded with its own `IMPORTANT.md`:

| Case | Result in `zsh`, `bash`, `dash`, `sh` |
|---|---|
| Ordinary success | rc 0; no leak; caller `$PWD` and `$-` unchanged |
| Mid-run failure (`scarp show dragon:99`) | rc 7, typed `artifact-not-found`; trap fired; no leak |
| Setup failure (`TMPDIR=/nonexistent-xyz/`) | rc 1; **no `scarp` command ran at all**; no `.scarp.toml`, no `archaeology/`, nothing new in the caller's directory |
| Relative `TMPDIR=reltmp` | rc 0; **no leak** — `reltmp/` verified empty afterwards |

All sixteen passed. The caller's working directory and shell options were
unchanged in every run.

### Package and gate verification

`README.md` is crate payload, so task 46's product checks were repeated
rather than inherited. `--allow-dirty` and `--no-verify` were not used;
no publishing credential was inspected or requested.

- `scripts/check.sh`: passes. `doctor: 108 artifact(s) checked, no
  problems found` — 107 in CI at `f66a94e` plus idea 34, added since.
  Cross-checked against per-collection counts: 4 + 34 + 16 + 8 + 46 = 108.
- Clean-tree paradox resolved as tasks 43 and 46 did: 172 paths copied
  from the working tree into a disposable directory outside the checkout,
  every one verified `git hash-object` byte-identical on both sides,
  committed locally, and packaged from there.
- **36 files, 561.8 KiB (123.9 KiB compressed)** — 1.21% of the 10 MiB
  limit. The file *set* is `diff`-identical to task 46's, as predicted:
  this task edited already-packaged files and added none. Size rose from
  556.2 KiB because the README grew.
- Unpacked outside the checkout; built, **385 tests across 19 binaries all
  pass**, `cargo doc --no-deps` succeeds with the same three pre-existing
  private-intra-doc-link warnings.
- Installed with a fresh `CARGO_HOME`, `CARGO_TARGET_DIR`, and `--root`,
  all three verified empty beforehand; `command -v scarp` resolved to the
  temporary root, and no `scarp` exists in `~/.cargo/bin`.
- Every README link inspected from **both** surfaces: six absolute URLs
  returned HTTP 200; the three relative targets (`assets/logo.svg`,
  `LICENSE-APACHE`, `LICENSE-MIT`) are tracked in Git *and* present in the
  unpacked crate; the `#quickstart` anchor matches a real heading. The
  split is deliberate and correct — `archaeology/`, `CLAUDE.md`, and
  `scripts/check.sh` are absent from the crate and are linked absolutely.
- The documented quickstart was executed **verbatim** against the binary
  installed from the unpacked package. Post-install time: **0.043 s**,
  against the ~60-second criterion. The README's `console` block was then
  compared mechanically to the captured stream: identical line for line
  once the `$ ` prompts and their blank separators are stripped, and after
  normalising the temporary path and the ULID, **identical** — the exact
  fields the README tells the reader will differ. No output was reflowed,
  trimmed, or reconstructed.
- `cargo publish --dry-run --locked` from the disposable clean snapshot:
  `Packaged 36 files, 561.8KiB (123.9KiB compressed)`, `Verifying scarp
  v0.1.0`, `warning: aborting upload due to dry run`.

### Task 46's CI evidence

Run `30321601486` was inspected at log level rather than trusted from its
badge, and every claim confirmed: commit `f66a94e`, manifest `1.88`,
toolchain **name** `1.88` installed by rustup, that toolchain reporting
cargo and rustc `1.88.0`, cargo-hack pinned at exactly `0.6.45`,
`rustup run 1.88 cargo check --all-targets --locked`, `toolchain list
unchanged across the gate`, and 385 tests plus `doctor: 107 artifact(s)
checked, no problems found`. The full evidence is recorded as a dated
post-close addendum on
[[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]] rather than duplicated here;
its original Result and Limitations were left standing as written.

### Human-owned GitHub changes, as dated provenance

Performed by Henry on 2026-07-27; not executed by this task. Verified
read-only afterwards.

```sh
# 1. private vulnerability reporting — was {"enabled":false}
gh api --method PUT \
  repos/henry-filgueiras/scarp/private-vulnerability-reporting
# 204 No Content; gh pipes output through $PAGER, so an empty vim buffer
# is the success case, not a prompt. Verify rather than read the screen:
gh api repos/henry-filgueiras/scarp/private-vulnerability-reporting \
  --jq .enabled            # -> true

# 2. the idea label
gh label create idea --repo henry-filgueiras/scarp \
  --color 7057FF --description "Uncommitted proposals to explore"
gh api repos/henry-filgueiras/scarp/labels/idea \
  --jq '{name, color, description}'
# -> {"color":"7057FF","description":"Uncommitted proposals to explore",
#     "name":"idea"}

# 3. removing the agent-memory topic — performed in the GitHub web UI
gh api repos/henry-filgueiras/scarp --jq '.topics'
# -> ["adr","ai-agents","architecture-decision-records","cli",
#     "decision-records","developer-tools","documentation","git",
#     "knowledge-management","markdown","rust"]
```

One interface note worth keeping, since it cost a round trip: the topics
REST endpoint replaces the whole set, and the obvious `gh api --method PUT
… -f names[]=adr` form **fails in zsh** with `zsh: no matches found:
names[]=adr`, because `[]` is a glob. The arguments need quoting
(`-f 'names[]=adr'`). This is dated provenance for the interface as it
behaved on 2026-07-27, not a script.

### Anything real but out of scope

Nothing was found that warranted a new dragon or idea. The two boundary
findings carried into this task were public-claim defects, fixed here; the
zsh Ctrl-C behaviour is a documented shell property, not a Scarp risk; and
`gh`'s pager and zsh's globbing are recorded above as provenance rather
than as artifacts. Task 45's inherited obligation — repeat the CI check on
the eventual release-source commit — is recorded on task 46's addendum,
where its precondition already lives.

## Addendum (2026-07-30, post-close): two counts stated too strongly

Appended during [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]. The Result
above is **not** rewritten. Neither correction changes a public surface,
and neither blocked publication; both are recorded because the Result
will outlive the memory of what was meant.

### The subcommand count was wrong; the table was not

The Result says the command surface was taken from `scarp --help` "plus
`--help` on all eleven subcommands", and later that the scoreboard "now
carries all eleven subcommands with their real flags".

**Scarp ships twelve subcommands.** Verified against the running binary
on 2026-07-30, and again from the registry install inside a clean
container: `init`, `new`, `list`, `show`, `doctor`, `close`, `reopen`,
`adopt`, `reject`, `fortune`, `resolve`, `completions` — twelve, not
counting clap's built-in `help`.

The **table is correct** and always was. It has eleven rows because
`adopt` and `reject` share one row, which is the right editorial call:
they are the two terminal transitions of a single lifecycle and reading
them on one line is how the lifecycle is understood. Eleven rows
covering twelve subcommands is not an omission.

What went wrong is only that the row count was read back as a
subcommand count and then repeated as one. No surface changes.

### "Word for word" and "the same sentence" overstate the agreement

The Result says the README hero "now reads **Git-native, reviewable
project archaeology**, matching the manifest `description` and decision
16's positioning line word for word", and that the surfaces "tell one
story". The second claim holds. The first does not.

Three distinct wordings ship, and they are deliberately different
lengths for different slots. The positioning line below is
[[dec_01KYJE2K3VRASS8A1X1E847S1B|decision 16]]'s, quoted exactly:

| Wording | Where |
|---|---|
| Scarp exposes the strata of a repository: what changed, why, and what remains unsettled. | decision 16, the README italic line, GitHub About |
| Git-native, reviewable project archaeology. | the README hero |
| Git-native, reviewable project archaeology: what changed, why, and what remains unsettled | `Cargo.toml`, `scarp --help`, the crate-level doc |

The hero is the short form; the manifest and CLI carry the expanded
form; the positioning line is its own sentence. They are consistent —
same framing, same retired claims stayed retired, no surface promising
what another denies — but no single sentence appears identically on all
of them, and the audit should have said "consistent" rather than "word
for word".

**No public wording change is required**, and none was made. A registry
description, a CLI one-liner, and a README hero have different length
budgets; forcing one string through all three would make at least one of
them read badly. The claim was too strong; the surfaces were fine.
