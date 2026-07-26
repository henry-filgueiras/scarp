---
id: tsk_01KYFYKAZRGEJPJYKAWV8W9BB4
sequence: 41
kind: task
status: closed
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-26
closed: 2026-07-26
---

# Adversarially map the Edda boundary

## Objective

Adversarially compare Strata with Edda
(github.com/fagemx/edda), the closest discovered neighbor from
[[tsk_01KYFRWF1X37N5TBJ139X7ZKA1|task 40]]'s landscape note, closely
enough to determine: whether the two tools solve the same underlying
job, neighboring jobs, or composable jobs; which apparent Strata
differentiators are real architectural boundaries versus current
maturity gaps; what Edda demonstrably does better; what public claims
Strata can honestly make before `v0.1.0`; and whether any discovery
blocks First Contact, belongs after release, or should remain a
deliberate non-goal. The comparison must be willing to conclude that
Strata is less differentiated than expected; no distinction may be
manufactured because the task asks for one.

This is bounded research, comparison, and positioning only. It is not
permission to implement feature parity, redesign Strata, change its
storage architecture, rename the project, create a naming decision,
or modify public positioning immediately.

Storage authority is a first-class axis: Edda stores its ledger in
SQLite (WAL mode per a preliminary source pass — to be verified at a
pinned commit), and the analysis must compare actual trades —
atomicity, querying, concurrency, migrations, hash-chaining versus
Git-native canonical files, tool-independent legibility, and manual
repair — without reducing to "plaintext good, database bad."
[[dec-bootstrap-files-canonical|Decision 1]] makes ordinary files
canonical today; the "do not hold repositories hostage" language is a
current core invariant and a proposed durable principle in
[[ide_01KYDZVN858BK52A35KJ3ZY5BP|idea 28]], not an implemented
principles collection.

### Owner concurrency note

Preserved from the commissioning prompt as durable case-study
evidence: it is explicitly okay to create and execute this task while
naming research continues independently. Task 40 is closed, although
its evocative-moniker direction may continue through an addendum or
follow-up; this task must not reopen or edit task 40 and must not
wait for naming adjudication. Multiple open work items and non-linear
task selection are intentional — part of the experiment is whether
work can be randomized safely when ownership and the baton/hot-potato
handoff are explicit. Temporal overlap does not authorize two agents
to mutate the same dirty checkout, index, or artifact: work happens
only from an independently clean checkout or after an explicit
sequential handoff, and this slice stays disjoint from naming work.

## Acceptance criteria

- The subjects are pinned: exact Strata commit examined, Edda's
  current default-branch commit, Edda's latest published release and
  date, and any meaningful divergence between released and current
  Edda. Implementation claims use commit-pinned links; shipped
  behavior, current-main behavior, documentation claims, and roadmap
  claims are distinguished.
- Edda's documentation and representative implementation are
  inspected — not only its README — including manifests and install
  path, ledger/event model and schema, SQLite open/configuration and
  migrations, init behavior and `.edda/` layout, Git tracking policy,
  record-type surfaces (decision, ratification, draft, policy, task,
  branch, coordination), retrieval/context/hook integration, locking
  and peer identity, export/backup/recovery/inspection facilities,
  and the canonical-versus-derived boundary.
- Edda's quickstart is run in a disposable repository isolated from
  real user configuration (HOME, XDG, agent hooks, user-level Edda
  storage), never inside Strata, with the exact version and commands
  recorded; the repository is inspected before init, after init,
  after representative records, during any live process, and after
  shutdown, covering created/modified files, ignored-versus-tracked
  state, clone portability, and Edda-absent legibility and
  editability. If safe isolation or execution is impractical, the
  blocker is recorded and runtime behavior is not implied as tested.
- Storage authority is analyzed precisely: SQLite bundling versus
  runtime dependency, whether the `sqlite3` CLI is needed, canonical
  versus config/blob/index/cache/projection files in `.edda/`, what
  "append-only" actually covers, what the hash chain makes
  tamper-evident and whether it is a correctness aid or security
  boundary, WAL/`-wal`/`-shm` behavior across ordinary use,
  concurrency, crashes, and clean shutdown, WAL implications for
  network filesystems, clones, cross-machine use, backups, and Git
  snapshots (citing official SQLite documentation), the topology of
  Edda's peer/cross-agent claims, schema-forward/binary-backward
  behavior, and recovery/export paths without the Edda executable.
- An evidence-backed product-boundary matrix covers at least: primary
  user and core job; canonical ontology and lifecycle; human versus
  agent authority; formulation/ratification/mutation boundaries;
  manual versus hook/transcript capture; retrieval and context
  injection; Git awareness versus dependence; repo-local versus
  user-global state; branch/merge semantics; coordination topology
  and locking; integrations; deterministic core versus optional LLM;
  installation and release maturity; interoperability.
- Every apparent Strata advantage is classified as exactly one of:
  durable architectural differentiator, deliberate tradeoff, current
  implementation lead, current Edda deficit, current Strata deficit,
  or mostly vocabulary/marketing. Edda coverage of described future
  Strata territory (ratification, drafts, policies, tasks, context
  injection, agent bridges, peer coordination, planning,
  orchestration) is compared on semantics, not command names.
- The verdict includes both steelmen — the strongest case that Edda
  makes Strata unnecessary and the strongest case that Strata remains
  distinct and useful — an adjudication between them, a defensible
  one-sentence distinction, a skeptical 60-second explanation for a
  first-time visitor, public claims Strata should not make, a
  substitutes/complements/overlap answer, and a kill/narrow/continue
  judgment that does not use "more ambitious" as a substitute for
  shipped differentiation.
- Every actionable finding lands in exactly one bucket: required
  before `v0.1.0` (genuine First Contact blockers only), post-release
  opportunity, or deliberate non-goal. No finding is implemented in
  this task; a discovered release blocker is recorded durably and
  stopped on for Henry's adjudication. New ideas or dragons are
  created only when a finding genuinely fits and would otherwise be
  lost; no architectural decision is created.
- The Result stands alone without the commissioning prompt: commits,
  releases, and research date; methods and runtime experiments
  actually performed; executive verdict; boundary comparison;
  storage-authority and WAL analysis; capability/topology table; what
  Edda wins, what Strata wins or trades, where Strata is behind; both
  steelmen; one-sentence distinction and 60-second positioning;
  claims to avoid; the three implication buckets; dated, commit-pinned
  primary links; and explicit separation of verified fact, reasonable
  inference, and judgment.
- A short workflow observation records that this task was
  intentionally selected while naming work continued independently,
  what ownership boundary kept the work disjoint, whether any
  worktree, artifact, or commit collision occurred, and whether this
  is evidence for or against safe non-linear task selection — as
  case-study evidence, not an agent-scheduling design.

## Result

All research performed 2026-07-26. Epistemic labels: **[V]**
verified that day against a primary source (pinned source tree,
runtime experiment, registry API, official documentation), **[I]**
reasonable inference from verified facts, **[J]** judgment.

### Subjects pinned

- **Strata**: commit `01bdaeb6b97dcbcaf9b516c8a963c803539cab2d`
  (repository state examined before this task's artifact was added);
  binary `strata 0.1.0`, unreleased, twelve subcommands **[V]**.
- **Edda main**: `c581b853660541c7e4a3a076e726e7b99b5d22bb`
  (2026-07-16), github.com/fagemx/edda, Apache-2.0 license file with
  `MIT OR Apache-2.0` workspace manifests, 34 stars **[V]**.
- **Edda latest release**: v0.2.1, tag commit `8c812c6d`, GitHub
  release 2026-07-13 with prebuilt binaries for five targets plus
  sha256 files; crates.io `edda` 0.2.1 published 2026-07-13
  (crate created 2026-02-18, 47 total downloads, installs a binary
  named `edda`) **[V]**.
- **Release-versus-main divergence is meaningful**: 28 commits
  separate v0.2.1 from main, adding `edda ratify` and decision
  provenance (recorded ≠ ratified, GH-401), the task rail
  (`edda task` verbs and `task.*` events), fleet cross-project reads
  (`--fleet` on ask/log/search/task, GH-407/408), and search
  auto-indexing. `ratify` and `task` do not exist in the released
  v0.2.1 CLI — verified both by grep of the tagged tree and by
  running the built v0.2.1 binary **[V]**. Any claim below marked
  *(main)* is unreleased behavior.
- **edda.sh**, the homepage claimed by the crate and repository,
  returned HTTP 404 on 2026-07-26 **[V]**.

### Methods actually performed

- Full-tree source inspection of the clone at `c581b85` by two
  parallel readers (storage layer; product surface and docs), with
  load-bearing citations re-verified by hand against `git show
  c581b85:...` **[V]**. A transient checkout race during setup
  briefly placed the shared clone on the v0.2.1 tag; it was restored
  to `c581b85` before file reads that matter, and every line cited
  in this Result was independently re-checked at `c581b85`.
- Runtime experiment: built the CLI from the v0.2.1 tag in a
  separate git worktree (`cargo build --release -p edda`; the
  official release binaries were not executed) and ran it in a
  disposable git repository under the session scratchpad with
  `env -i HOME=<fake-home> PATH=/usr/bin:/bin XDG_*=<fake-home>/...`
  so no real user configuration, Claude/Codex/Cursor hooks, or
  user-level Edda storage could be touched; `edda init` was never
  run inside Strata **[V]**. Commands exercised: `init`, `note`,
  `decide`, `commit --title`, `status`, `log`, `ask`, `context`,
  `export --out <dir> md`, `rebuild`, `doctor`, `serve` (live then
  `kill -9`), plus sqlite3 inspection, a git commit of `.edda/`, a
  clone, and a two-sided divergence-and-merge test.
- Registry and platform facts from crates.io API and GitHub API;
  WAL portability claims from the official SQLite WAL documentation
  (sqlite.org/wal.html) **[V]**.
- Not tested: main-only features (`ratify`, task rail, fleet) were
  verified in source only, not executed; multi-process concurrent
  writers and the bridge hooks were not exercised (hook installation
  was avoided deliberately) **[V]**.

### Executive verdict

Edda is not a hypothetical competitor; it is a shipped, fast-moving
tool occupying the same category with a deliberately inverted
storage contract. The two tools overlap on the marketing sentence
("repo-local decision memory for agents") but differ at the root on
two load-bearing axes: **what the canonical record is** (Edda: an
append-only hash-chained SQLite event ledger, with Markdown as
explicitly do-not-edit projections; Strata: human-written Markdown
artifacts, with everything else disposable) and **where memory
lives** (Edda: per-machine — `.edda/` gitignored by its own
convention plus a user-global `~/.edda` store; Strata: inside the
repository, traveling with every clone and merge). Edda is far ahead
on capture, retrieval, integration, and release engineering; Strata's
defensible ground is the curated, reviewable, Git-native corpus and
its narrative ontology. Verdict: **continue, narrowed** — the
product thesis survives, but several claims Strata might have made
must be retired, and positioning must not promise Edda's job.
No First Contact release blocker was found **[J]**.

### What Edda is (verified)

A 23-crate Rust workspace **[V]**. Released v0.2.1 already has ~50
CLI subcommands including init, note, key=value decisions, its own
internal branch/switch/merge/commit model, draft proposals with
approvals, RBAC actors/policy/tool-tier governance, Tantivy
full-text search, an MCP server (8 tools, 2 resources), a ratatui
TUI (`edda watch`), a localhost HTTP API (`edda serve`, axum),
push notifications, transcript ingestion, deterministic context
injection into Claude Code via 12 hook events installed into the
project's `.claude/settings.local.json`, bridges for
Codex/Cursor/Hermes/OpenClaw at varying wiring depth, and opt-in
LLM assists gated behind `EDDA_LLM_API_KEY` with a daily budget cap
— the core record/retrieve/inject loop is deterministic and makes
no network calls **[V]**. Its own docs distinguish shipped behavior
from "v0 design spec" governance documents ("where details differ,
the code is authoritative") **[V]**. Maturity signals: ~1,985 test
functions, three-OS CI including Windows, prebuilt release binaries,
a Homebrew tap and curl installer documented in the README **[V]**.
Command names referencing sibling products (havamal doctrine packs,
bryti, Karvi controls) indicate one repo inside a larger private
ecosystem **[I]**.

### Storage authority: two coherent, mirrored contracts

Edda's contract, verified at `c581b85`:

- SQLite is **bundled** via rusqlite's `bundled` feature
  (`crates/edda-ledger/Cargo.toml` line 21); no system SQLite and no
  `sqlite3` CLI are required for any normal operation **[V]**.
- Connections apply `PRAGMA journal_mode = WAL; foreign_keys = ON;
  busy_timeout = 5000;` (`crates/edda-ledger/src/sqlite_store/`
  `mod.rs` lines 54–61), and `Drop` runs
  `PRAGMA wal_checkpoint(TRUNCATE)` "so users see a single file when
  idle" (lines 64–68). The task's breadcrumb is confirmed at the
  pinned commit **[V]**.
- "Append-only" means the **`events` table only**. Derived tables
  (`decisions`, `task_briefs`, `refs`/HEAD, snapshots, suggestions,
  device tokens) mutate freely and are rebuildable from the log;
  invariant INV-02 in `docs/architecture/consistency-contract.md`
  (line 155) states "Ledger events are append-only; no UPDATE or
  DELETE", and no production UPDATE/DELETE against `events` exists
  in the tree **[V]**.
- The hash chain is SHA-256 over a canonicalized JSON event that
  includes `parent_hash`, so each event commits to its predecessor.
  It is a **tamper-evidence and correctness aid, not a security
  boundary**: there is no signing or key material, `ratified_by` is
  self-asserted, and `verify_chain()` exists in the library but is
  wired to no CLI command at the pinned commit — `edda doctor`
  checks only bridge installations **[V]**.
- No forward schema guard: migrations only run upward; an older
  binary opening a newer database does not refuse, it proceeds and
  can fail later on missing columns **[V]** — a real operational
  sharp edge for a tool this young **[J]**.
- Recovery without the binary: the database is plain unencrypted
  SQLite with JSON text payloads; every event is readable with the
  stock `sqlite3` CLI, blobs are hash-named files, config is
  YAML/JSON, and `.edda/branches/main/log.md` is a continuously
  maintained readable event log **[V]**. `edda export --out <dir>
  md` writes a Markdown projection whose header reads "GENERATED
  FILE, DO NOT EDIT — SQLite ledger is authoritative" **[V]**.
- User-global state exists and is written **even by `init`**:
  `~/.edda/registry.json` (or platform data dir) mapping project-id
  hashes to absolute local paths, plus per-project user-scoped
  directories holding the Tantivy index, transcripts, and the
  ephemeral peer-coordination store **[V]**.

Runtime observations (v0.2.1 binary, isolated environment) **[V]**:

- `edda init` creates `.edda/` with `ledger.db`, `LOCK`, three
  editable YAML config files, and per-branch Markdown projections;
  it does not touch `.gitignore` — but Edda's own repository and its
  demo project gitignore `.edda/`, so the intended convention is
  that the ledger does **not** travel through Git.
- No `-wal`/`-shm` files were observable while a server process was
  live, after `kill -9`, or after clean exit — the
  truncate-on-drop behavior keeps the idle state a single file.
- Committing `.edda/` to Git anyway and cloning **does** carry the
  memory, and the clone works. But two clones that each append one
  event produce `CONFLICT (content): Merge conflict in
  .edda/ledger.db … Cannot merge binary files` plus conflicts in the
  generated projections. Divergent Edda histories cannot be merged
  by Git; Edda's answer is its internal single-writer branch model
  plus `edda sync` *(main)*, which is pull-only and decision-scoped
  (shared/global-scope decisions are imported as new events;
  conflicting values import inactive), not a general ledger merge.
- `edda decide` refuses narrative input: "decision must be in
  key=value format (e.g. `auth.method=JWT RS256`)". Decisions are
  structured settings with optional glob scopes, status lifecycle
  (`active`/`superseded`/…), and *(main)* a ratification tier.

Official SQLite WAL facts relevant to the trade **[V]**: WAL
requires all processes on one host ("WAL does not work over a
network filesystem"); a database file separated from a live `-wal`
file loses transactions or corrupts; the WAL file is normally
deleted on last close. Because Edda checkpoints on every drop, an
idle `.edda/ledger.db` is a self-contained snapshot **[V]**; a Git
snapshot taken mid-crash (persisting `-wal`) or from a live writer
would not be **[I]**. Network filesystems and cross-machine sharing
of one ledger are out of contract; Edda's fleet features are
explicitly same-machine — "Truth stays home", fan-out over local
paths in the user registry, with "repo not on this machine" as a
first-class miss *(main)* **[V]**.

Strata's mirrored costs, admitted: multi-file atomicity and
concurrent sequence allocation are unsolved (dragon 1 documents
branch-concurrent sequence collisions; Edda's single-writer lock
plus SQLite transactions simply do not have this problem); querying
scans the filesystem; validation is bespoke (`doctor`); plain text
guarantees legibility, not good information architecture **[V]**.

The axis unbundled **[J]**: both tools are *open format* and
*human-readable at rest* (Edda via JSON-in-SQLite plus generated
Markdown). They separate at *directly editable* (editing Edda's
ledger breaks the hash chain by design; editing Strata's files is
the intended write path), *Git-native canonical representation*
(Edda's own convention excludes its canonical store from version
control; Strata's canonical store **is** the versioned content),
and *tool-independent semantic intelligibility* (a Strata corpus
reads as documents; an Edda ledger reads as an event log that must
be replayed or projected to answer "what is settled"). Operational
convenience runs the other way: Edda gets transactions, indexes,
fast structured queries, and safe concurrent local writers for
free. "Plaintext good, database bad" is not the finding; the
finding is that each tool made the other's tradeoff deliberately.

### Product-boundary matrix

| Axis | Edda (v0.2.1 / main) | Strata (`01bdaeb`) |
|---|---|---|
| Primary user | Operator running coding agents; memory captured from sessions | Contributors and agents curating repo history by hand |
| Core job | Record/retrieve/inject session and decision memory automatically | Durable, reviewable project archaeology: why things are as they are |
| Canonical store | `.edda/ledger.db` SQLite event ledger; files are projections | Markdown artifacts in `archaeology/`; indexes would be projections |
| Ontology | Events: notes, key=value decisions, commits, tasks *(main)*, approvals, telemetry | Documents: decisions, dragons, ideas, sprints, tasks with narrative bodies |
| Decision shape | `key=value` + scope globs + status; ratification tier *(main)* | Prose record: context, decision, consequences, typed edges |
| Unresolved-risk artifact | None (notes/tags at best) | Dragons, first-class with lifecycle |
| Proposal artifact | Drafts with approval workflow | Ideas, never load-bearing by invariant |
| Human vs agent authority | Agents record, operator ratifies *(main)*; RBAC, tool tiers, draft approvals; identity self-asserted | No enforcement layer; authority is Git review and repo convention |
| Capture | 12 Claude Code hooks, transcript ingestion, auto-digest; manual CLI/MCP too | Manual CLI only |
| Retrieval/injection | Deterministic SessionStart pack with budgets; `ask`; Tantivy FTS; MCP | `list`/`show`/`fortune`; no search, no injection |
| Git relationship | Optional, opportunistic (branch names, diffs); ledger gitignored by convention | Optional at core; corpus is designed to be committed, diffed, merged |
| Repo-local vs user-global | Both: `.edda/` + `~/.edda` registry, indexes, coordination | Repo-local only |
| Branch/merge | Internal ledger branches; Git merge of the ledger impossible (verified); `sync` pulls decisions *(main)* | Git branches/merges operate on the records; sequence collisions unresolved (dragon 1) |
| Coordination | Same-machine peers: heartbeats, claims, leases *(task rail, main)*, fleet reads over local registry | None (explicit non-goal for now) |
| Integrations | Claude Code (deep), Codex/Cursor/Hermes/OpenClaw (varying), MCP, HTTP API, TUI, notifications | None |
| Deterministic vs LLM | Core loop deterministic, zero-egress; LLM strictly opt-in, budget-capped | Fully deterministic, no LLM anywhere |
| Install/release | v0.2.1 binaries ×5 targets, Homebrew tap, curl installer, crates.io, 3-OS CI | Unreleased; local cargo build |
| Corpus validation | Bridge-only `doctor`; chain verify unwired | `doctor` validates corpus invariants (84 artifacts, in dogfood use) |

All rows **[V]** except where marked *(main)*.

### Classifying Strata's apparent advantages

1. **Canonical records are ordinary repository files, versioned with
   the code** — *durable architectural differentiator* **[V]/[J]**.
   Edda cannot adopt this without inverting its consistency
   contract; its README, export headers, and gitignore convention
   all commit to the opposite.
2. **Git review/blame/branch/merge operate directly on the memory**
   — *durable differentiator, honestly qualified* **[V]**: the
   binary-merge experiment shows Edda's ledger cannot do this, but
   dragon 1 means Strata's merges are textual, not yet
   collision-safe.
3. **"Not held hostage" / legible without the tool** — *mostly
   vocabulary as a differentiator* **[V]/[J]**. Edda is open-format,
   unencrypted, plaintext-recoverable with stock `sqlite3`, and
   maintains readable log projections. Strata's honest form of this
   claim is about *direct editability and review*, not readability.
   Decision 1's invariant survives; the marketing sentence built on
   it must be sharpened.
4. **Narrative ontology (dragons, ideas, prose decisions)** —
   *partly durable differentiator, partly current lead* **[J]**.
   Edda's key=value decisions are wired deeply into its guard, ask,
   and pack machinery; unresolved-risk and never-load-bearing-idea
   artifacts have no Edda equivalent. But nothing architectural
   stops Edda from adding richer payloads.
5. **Corpus-invariant `doctor`** — *current implementation lead*
   **[V]**: Edda's doctor checks bridges only and its chain
   verifier is unwired at the pinned commit.
6. **Small, auditable surface** — *deliberate tradeoff* **[J]**.
7. **Deterministic core** — *not a differentiator* **[V]**: Edda's
   core loop is equally deterministic and zero-egress; LLM use is
   opt-in. Strata must not claim this as distinguishing.
8. **Concurrency and atomicity** — *current Strata deficit* **[V]**:
   Edda gets both from SQLite plus a workspace lock; Strata has
   dragon 1 and no locking.
9. **Capture, retrieval, injection, integrations, release** —
   *current Strata deficits / Edda leads* **[V]**, maturity gaps
   rather than boundaries, except where closing them Edda's way
   would violate decision 1.

**Edda already covers most of Strata's parked future territory**
**[V]**: ratification with a recorded-versus-binding distinction
*(main)*, draft proposals with approvals, policies (RBAC and tool
tiers), tasks with leases *(main)*, deterministic context packs,
agent bridges, same-machine peer coordination, and plan
orchestration. The semantics differ from Strata's sketches —
event-derived state, key=value decisions, per-machine scope,
convention-based identity — but the jobs are occupied. Ideas 20,
22, 27, and 31 now have live prior art to cite rather than blank
space **[I]**.

### What each side wins

**Edda demonstrably better, today**: automatic capture from real
agent sessions; deterministic budgeted context injection; full-text
search; MCP; multi-agent same-machine coordination; cross-repo
fleet reads *(main)*; governance surfaces (ratification *(main)*,
drafts, RBAC, tool tiers); transactions, indexes, and fast queries;
installability and release engineering; Windows support **[V]**.

**Strata different for a defensible reason**: the corpus is the
repository — reviewed in PRs, blamed, branched, merged, cloned,
readable and *editable* as first-class content, with no per-machine
or user-global residue; the ontology is built for curated
understanding (why/risk/proposal) rather than session telemetry
**[V]/[J]**.

**Strata simply behind**: everything in the capture/retrieval/
integration/release column above; concurrency safety; any
search **[V]**.

**Where to borrow (later)**: recorded-versus-ratified provenance
semantics; budgeted deterministic pack assembly; scoped decisions
consulted at edit time; watermark-gated incremental indexing
**[J]**.

**Where interoperability beats competition**: the tools can coexist
in one repository today without touching each other — `.edda/` is
gitignored machine-state, `archaeology/` is committed content; an
Edda session note could cite a Strata decision by stable id, and a
Strata decision could adopt what an Edda session surfaced **[I]**.

**Where matching Edda would violate Strata's boundary**: a canonical
SQLite ledger, hash-chained events, hook bridges, transcript
ingestion, MCP/TUI/HTTP surfaces, LLM assists, and coordination are
either decision-1 violations or standing bootstrap/sprint non-goals
**[V]**.

### Steelmen and adjudication

**The strongest case that Edda makes Strata unnecessary** **[J]**:
for the job both tools name — decision memory that reaches coding
agents — Edda ships today what Strata has only parked as ideas:
capture without ceremony, injection without prompting, search,
coordination, governance, binaries on five platforms. Its ledger is
open SQLite with readable projections, so the hostage argument
mostly dissolves; its determinism matches Strata's; its category
momentum (crates.io name, releases, integrations) means by the time
Strata ships retrieval, Edda's will be a year deeper. What remains
of Strata is an opinion about file format plus an unreleased
five-collection CRUD tool — an ADR folder with a numbering scheme.

**The strongest rebuttal** **[J]**: the two tools remember different
things for different lifetimes. Edda records what sessions did on
one machine, in a store its own conventions keep out of version
control; clone the repo and the memory stays behind, merge two
histories and the ledger cannot follow. Strata records what the
project settled, as content of the repository itself: it survives
`git clone` by definition, its changes are code-reviewable line by
line, and in fifty years a tarball of the repo still explains
itself with no executable, no schema replay, and no export step
that someone remembered to run. Edda optimizes the session loop;
Strata optimizes the artifact that outlives every session, every
machine, and both tools.

**Adjudication** **[J]**: both steelmen are substantially true
because they describe different centers of gravity. The tools are
**partly overlapping — substitutes only at the shallow end** (if
all you want is "agents can recall a few decisions on my machine",
Edda is the better tool today and Strata should not pretend
otherwise), **complements at the deep end** (machine-local session
memory beside repo-native curated archaeology, which is also the
concrete interop seam). The product thesis survives on the two
axes Edda deliberately occupies the other side of: canonical files
in Git, and curation over capture. **Judgment: continue, narrowed**
— not kill (the demonstrated Git-native boundary is real and
architecturally uncopyable by Edda), not unchanged (several
imagined differentiators did not survive contact, and the shipped
gap is wide). "More ambitious" appears nowhere in this verdict.

### One sentence, and sixty seconds

**One-sentence distinction**: Strata keeps a project's decisions,
risks, and history as canonical human-written files inside the
repository — reviewed, diffed, blamed, and merged like the code
they explain — while Edda keeps an append-only SQLite event ledger
beside the repository, captured automatically from agent sessions
and projected into read-only files on demand.

**Skeptical 60-second version** **[J]**: "Several young tools now do
'repo-local memory for coding agents'; Edda is a good one — it
hooks into your agent, records events into a local SQLite ledger,
and injects context back. Strata makes a different bet. Its records
are ordinary Markdown files committed in your repository: a
decision is a document you can read, a risk is a file with a
lifecycle, and changing one is a diff a human reviews. Nothing
lives outside the repo, so every clone carries the whole memory and
losing the tool loses nothing. The cost is honesty about what it
doesn't do yet: no automatic capture, no search, no context
injection, no agent hooks — you (or your agent) write records
deliberately, and the tool enforces structure, identity, and
validity. If you want ambient session memory on your machine, use
Edda. If you want the repository itself to stay explainable —
reviewed at the same bar as code — that is what Strata is for, and
the two don't actually fight: one is machine-state, the other is
repository content."

### Claims Strata should not make

- Not "the only" (or first) repo-local, Git-friendly, or
  agent-aware project memory tool — the category is crowded and
  forming **[V]**.
- Not "other tools lock your history in an opaque database" —
  Edda's store is open, unencrypted, plaintext-recoverable, with
  readable projections **[V]**. The defensible claim is direct
  editability and Git-native review, not sole readability.
- Not "deterministic, no LLM" as a *differentiator* — true of
  Strata, but equally true of Edda's core loop **[V]**.
- Not "safe for concurrent agents" — dragon 1 is open and there is
  no locking; Edda is currently *ahead* here **[V]**.
- Not "memory for coding agents" as the headline job — that
  promises capture and injection Strata does not ship **[V]**.
- No tamper-evidence/audit-trail claims — Strata has no chain, and
  Git history is mutable by force-push **[V]**.
- Avoid "plaintext good, database bad" framing generally; the
  honest statement is a tradeoff both sides chose **[J]**.

### Implication buckets

**Required before `v0.1.0`** — no new work item; one constraint on
existing sprint deliverables: the README/quickstart must position
Strata as Git-native, reviewable project archaeology (curation,
files-as-canon) and must not use the retired claims above —
folded into the existing "repository presents coherently" success
criterion. No architectural, packaging, or storage blocker to
First Contact was found **[J]**.

**Post-release opportunities** (recorded here; deliberately not new
idea artifacts — each attaches to an existing parked idea or is
retrievable from this Result): cite Edda's recorded-versus-ratified
provenance as prior art when idea 22 (reviewable mutation intents)
or idea 20 (review holds) is designed; cite its budgeted
deterministic pack assembly for idea 27 (context packs) and idea 31
(agent operating manual); cite scoped decisions consulted at edit
time for idea 12 (relevance surfacing); cite watermark-gated
incremental indexing for idea 18; run the coexistence experiment
(Edda beside Strata in one repository, disjoint stores) if
interop positioning ever needs evidence.

**Deliberate non-goals** (parity temptations rejected): canonical
SQLite ledger or any hash-chained event store (decision 1);
agent-session hooks, transcript ingestion, and auto-capture; MCP
server, TUI, HTTP API, notifications (standing bootstrap
non-goals); same-machine peer coordination and fleet topology
(sprint 8 non-goal: no agent scheduling); LLM-assisted anything;
key=value decision reshaping. None of these were implemented, and
no decision artifact was created — adjudication of positioning
language remains Henry's.

### Primary links (all pinned to `c581b85`, retrieved 2026-07-26)

- Bundled SQLite: github.com/fagemx/edda/blob/c581b85366/crates/
  edda-ledger/Cargo.toml#L21
- Pragmas and truncate-on-drop: …/crates/edda-ledger/src/
  sqlite_store/mod.rs#L54-L68
- Append-only invariant INV-02: …/docs/architecture/
  consistency-contract.md#L155
- Export is a projection: …/crates/edda-cli/src/cmd_export.rs#L6
- Hooks into repo `.claude/settings.local.json`: …/crates/
  edda-bridge-claude/src/admin.rs#L46
- `.edda/` gitignored by its own repo and demo: …/.gitignore#L2,
  …/examples/demo-project/.gitignore#L1
- Ratify (main-only): …/crates/edda-cli/src/main.rs#L113; absent
  from v0.2.1 (`git show v0.2.1:crates/edda-cli/src/main.rs`)
- WAL portability: sqlite.org/wal.html ("All processes using a
  database must be on the same host computer…")
- Release/binaries: github.com/fagemx/edda/releases/tag/v0.2.1;
  crates.io/api/v1/crates/edda (0.2.1, 2026-07-13)

### Workflow observation

This task was intentionally selected and executed while the
release-identity thread (task 40 and its possible moniker
follow-up) remained open territory owned elsewhere. The boundary
that kept the slices disjoint was explicit in the commissioning
prompt: task 40 untouched, naming adjudication not awaited, work
only from an independently clean checkout of `origin/main`. In
practice: the worktree was clean and equal to `origin/main` at
start, no other worker's state was encountered, and no artifact,
sequence, or commit collision occurred — task 41's sequence
allocated cleanly after task 40. The one concurrency incident of
the session was self-inflicted and internal to this task (a build
step briefly re-checked-out the shared *scratch* clone of Edda
under two readers; caught, isolated into a separate worktree, and
all affected citations re-verified). Evidence: one clean data point
**for** safe non-linear task selection when ownership and handoff
are explicit, and a reminder that the same discipline applies to
scratch resources, not only the repository.

### Addendum (2026-07-26): storage authority, continuability, and the workflow boundary

Synthesized from a 2026-07-26 follow-up discussion between Henry
Filgueiras and Lux (OpenAI/ChatGPT); edited and verified against
Task 41 by Claude. The task stays closed and the research above
stands as written: this addendum refines two of its judgments
rather than rewriting them, and records the architectural
boundaries the comparison exposed. No new claim about Edda,
SQLite, Temporal, or distributed systems is asserted here as
verified fact — every **[V]** below points back to something
already verified in the Result above.

#### Two judgments refined

**1. "Mostly vocabulary" was too dismissive** (refines advantage 3
in *Classifying Strata's apparent advantages*). Edda deserves full
credit for openness: its SQLite database is unencrypted, its event
payloads are readable JSON text, and it maintains Markdown
projections **[V]**. Someone without the Edda binary can recover
their history.

The property Strata actually holds is not recoverability but
**continuability**: its canonical records can be directly edited,
reviewed, repaired, branched, merged, and evolved with generic
repository tools. Edda's projections are explicitly read-only,
direct ledger edits violate its hash-chain contract, and
independently diverged databases do not Git-merge — all verified
above **[V]**. Recoverable and continuable are materially
different properties, and only the second is Strata's claim
**[J]**.

**2. "Architecturally uncopyable by Edda" was too absolute**
(refines the *Adjudication*). Edda could add or invert its storage
contract. That would be a foundational architectural change and
would trade away advantages it currently holds — transactional
mutation, indexes, cheap structured queries, safe local
concurrency — but it is not impossible **[J]**. The honest framing
is a durable difference between the two projects' *current*
contracts, not an uncopyable moat. The narrowed verdict does not
depend on the stronger claim.

#### The storage comparison, stated narrowly

> SQLite is superior inside one mutable, query-heavy authority.
> Files plus Git are superior when canonical state must fork,
> travel, merge, and remain directly reviewable.

Neither substrate is universally superior, and nothing here
suggests Edda chose wrongly **[J]**. Its workload — high-volume
event capture, structured queries, mutable projections, search,
hooks, and local concurrent access **[V]** — is well matched to
SQLite. Strata's workload — comparatively sparse, curated,
human-reviewed project records intended to travel with the
repository — is well matched to canonical files and Git **[J]**.

Finer points worth keeping **[J]** except where marked:

- SQLite supplies transactions, indexes, foreign keys, mature
  storage behavior, and well-understood local concurrency.
- Foreign keys prove *relational* facts, such as the existence of
  a referenced row. They do not automatically prove narrative or
  lifecycle semantics — whether a dragon was resolved by an
  *authorized* decision is not a constraint a schema can express.
- Strata's `doctor` is usefully understood as **deferred integrity
  enforcement at the publication/commit boundary**. That is weaker
  during mutation than a transaction, but natural for multi-file
  edits and branch-based work.
- Authoritative edges should ordinarily be stored once; reverse
  references and search structures should stay disposable
  projections.
- Sequence collisions
  ([[drg-bootstrap-branch-collisions|dragon 1]]) are primarily a
  topology problem, not a storage-engine problem. Stable IDs carry
  identity; human-friendly sequence numbers can be reconciled or
  renumbered during integration without inventing a distributed
  gapless allocator.
- Plain files expose many of Strata's dragons in code, diffs, and
  validation rules instead of hiding them inside a storage engine.
  SQLite removes a real class of storage-engine dragons, but
  neither substrate removes schema evolution, application
  invariants, projection consistency, divergent histories, or
  failures involving external effects.

#### Where workflow authority begins

The comparison exposed a boundary that belongs to neither project
specifically. The test **[J]**:

> State is a disposable projection only if it can be deleted and
> deterministically reconstructed without deciding whether an
> external effect happened. If losing it can duplicate, orphan,
> cancel, or authorize work, it is workflow authority.

SQLite may therefore be an excellent implementation for indexes,
caches, or local structured state. But once state answers
questions such as "was this worker dispatched?", "should this
attempt retry?", or "was this result already promoted?", a
home-grown loop around it is becoming a workflow engine. No
database transaction can atomically cover launching an agent,
pushing a branch, calling an external API, and surviving an
ambiguous network failure **[J]**. This is recorded as a general
boundary, **not** as an accusation: no evidence was gathered that
Edda crosses it.

If Strata ever participates in multi-agent orchestration, four
planes should stay distinct **[J]**:

| Plane | Likely authority |
|---|---|
| Durable project memory and accepted results | Git plus Strata |
| Live retries, timers, cancellation, in-flight execution | An established durable-workflow system |
| Provisional task results | Isolated worker branches or clones |
| Search, indexes, dashboards, cached observations | Disposable projections, possibly SQLite |

Temporal is the illustrative prior art for the second row — an
example only, not a selected dependency and not an architectural
decision. The promising future role is that **Strata may become a
workflow ABI for commissions, constraints, authority boundaries,
and accepted receipts; it should not become the durable workflow
runtime** **[J]**. This remains speculative and post-release: no
implementation task, idea, dragon, or decision is created from it.

#### The multi-worker topology this makes coherent

Git-native canon supports a future experiment that a
machine-local ledger does not **[J]**: commission attempts from a
pinned base commit; give each worker an isolated branch or clone;
treat worker completion as a *proposal* rather than canonical task
closure; give a single integrator authority over the promoted
sprint branch; revalidate each result against the latest accepted
head; integrate continuously to limit drift; and retain task,
attempt, worker, base-SHA, generation, and result-commit
provenance. This does not make overlapping task effects safe
automatically, and none of it is a currently shipped Strata
capability.

#### Product verdict and engineering-evidence verdict are separate

The product judgment above stands: **continue, narrowed**. Edda is
substantially ahead for automatically captured, locally queryable
agent-session memory **[V]**; Strata remains distinct as curated,
repository-governed project memory; the two have similar outer
shapes and partly overlap, but optimize different canonical
authorities and lifetimes **[J]**.

Independently of the product thesis, a case-study conclusion
**[J]**: discovering close or superior prior art does not erase
the value of this repository. The corpus is evidence of a real
engineering exercise — invariants stated progressively, dragons
left visible, adversarial comparison actually performed, decisions
revised after evidence, and the tool repeatedly used to preserve
its own development history. Task 41 is particularly strong
dogfooding evidence *because* the process found a serious
neighbor, retired weak claims, corrected overstatements, and
narrowed the thesis rather than manufacturing novelty. That
evidence justifies preserving the repository and its archaeology
even if the eventual market thesis weakens. It does **not** prove
product-market fit, justify feature parity, or provide an
unfalsifiable excuse to keep expanding the product.

> Keep the project and its archaeology. Continue testing the
> narrow product thesis honestly. Even if later evidence kills
> that thesis, the progressively reasoned and self-dogfooded
> engineering record remains a legitimate outcome rather than
> failed residue.
