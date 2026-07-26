---
id: tsk_01KYFRWF1X37N5TBJ139X7ZKA1
sequence: 40
kind: task
status: closed
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-26
closed: 2026-07-26
---

# Research a coherent release identity

## Objective

Research and recommend one coherent public identity for the project
ahead of `v0.1.0`: a single name evaluated jointly as project and
GitHub repository, crates.io package, installed executable,
normalized Rust library crate, and documentation/spoken usage. This
is a research and recommendation task only — the naming decision,
any rename, crate reservation, or repository-settings change is
Henry's subsequent adjudication and is out of scope here.

The name must fit the product truth: a Git-friendly tool for
durable, repository-local project memory whose useful boundary is
helping contributors and agents understand why a repository is the
way it is, what has been settled, and what remains uncertain. It is
not a scheduler, a graph database, an oracle, or an enforcement
engine, and it should not need a short alias to be pleasant — the
canonical executable name itself must be.

Method: a bounded naming tournament. Generate roughly 25–40 serious
candidates across at least six semantic families, at least half
genuinely new rather than mutations of prior seeds; include
retaining `strata` (and a qualified `strata` distribution identity)
as control candidates that a rename must materially beat; shallow
collision/semantic screen to about five finalists; deep, dated
research on those finalists only.

## Acceptance criteria

- A candidate pool of roughly 25–40 names spanning at least six
  semantic families is recorded, with at least half not derived from
  the prior seed list.
- Retaining `strata` is evaluated as a genuine control, including
  independent verification of the reported crates.io ownership and
  installed-binary PATH collision, with dated primary evidence.
- The funnel from pool to approximately five finalists is recorded
  with notable eliminations and their reasons, distinguishing exact
  package/executable conflicts, close software/product conflicts,
  ordinary search noise, and mere shared English words.
- Each finalist carries dated collision research: crates.io
  availability and binaries installed by similarly named crates,
  exact and close GitHub identities, general web identity, prominent
  package-manager or executable collisions, and obvious
  trademark/product conflicts framed as risk research, not legal
  clearance.
- Each finalist states its exact proposed project, package, binary,
  and library identity, and is auditioned in real command lines.
- The Result records a concise naming brief, the semantic
  territories explored, a finalist comparison table, and a ranked
  recommendation: preferred choice, strongest fallback, the
  retain-`strata` control, and the strongest argument against the
  preferred choice.
- Verified facts, reasonable inference, and aesthetic judgment are
  explicitly distinguished throughout.
- No rename is performed, no crate is published or reserved, no
  repository settings change, and no naming decision artifact is
  created.
- The Result is useful standalone, without this task's originating
  prompt or conversation.

## Result

All research performed 2026-07-26. Registry claims were verified
against primary sources (crates.io API, npm registry, PyPI JSON API,
Homebrew formulae API, GitHub API/search) on that date; the two
crates closest to the incumbent name were downloaded and their
manifests read directly. Web claims carry their URLs. Epistemic
labels used below: **[V]** verified against a primary source that
day, **[I]** reasonable inference from verified facts, **[A]**
aesthetic or ergonomic judgment.

### Naming brief

One name must serve as project and GitHub repository, crates.io
package, installed executable, normalized Rust library crate, and
the word people say aloud. The product is a Git-friendly CLI for
durable, repository-local project memory: it helps contributors and
agents understand why a repository is the way it is, what has been
settled, and what remains uncertain. The name may evoke memory,
navigation, preservation, provenance, or a meticulous collaborator;
it must not claim to do the thinking, schedule the fleet, be a
database, or enforce anything. The canonical executable must itself
be pleasant — no rescue aliases. Retaining `strata` was evaluated as
a control that any rename had to materially beat.

### The retain-`strata` control is not viable

- **[V]** crates.io `strata` is squatted: v0.1.1, "A unique search
  technology", last updated 2019-02-28, lib-only, no binary
  (crate downloaded, Cargo.toml read; crates.io/crates/strata).
- **[V]** crates.io `strata-rs` v0.4.4 (updated 2026-03-05) declares
  `[lib] name = "strata"` and `[[bin]] name = "strata"` — verified
  by downloading the .crate and reading Cargo.toml. `cargo install
  strata-rs` installs an executable named `strata`, and the `strata`
  library namespace is occupied in practice. Its GitHub home
  (github.com/Emagjby/Strata, 10 stars, quiet since 2026-03) is
  small but real.
- **[V]** Klavis AI's "Strata" MCP tool-aggregation server
  (PyPI `strata-mcp` 1.0.2, 2025-10-28; github.com/Klavis-AI/klavis,
  5,777 stars) installs a CLI executable named `strata` via
  `pipx install strata-mcp`, with `strata add/list/run` subcommands
  targeting Claude, Cursor, and VS Code — an exact executable-name
  conflict in exactly this project's audience.
- **[V]** npm `strata` exists with a `bin` entry (dead HTTP
  framework, last published 2013-03-28) — formally an executable
  conflict, practically abandoned.
- **[V]** The name is contested by at least four active software
  brands: Palo Alto Networks "Strata"/Strata Cloud Manager (release
  2026.R2.2), Strata Identity/strata.io (acquired by Rubrik,
  completed 2026-06-30), AWS-originated strata-org/Strata formal
  verification platform (Lean4, 213 stars, includes a repo literally
  named "Strata-CLI"), and OpenGamma/Strata (Java finance library,
  952 stars, pushed 2026-07-02). Alpen Labs renamed
  alpenlabs/strata to alpenlabs/alpen but keeps active
  `strata-bridge`/`strata-common` repos (2026-07-21).
- **[I]** Even the qualified fallback identity (project "Strata",
  crate `strata-cli` — free **[V]** — binary `strata`) is
  incoherent: the binary collides with two active tools, the library
  namespace is taken, and the natural search "strata cli" hits both
  Klavis's `strata` CLI and strata-org's Strata-CLI repo.
- **Conclusion [I]**: this is a package conflict, an executable
  conflict, a library conflict, and a search-identity conflict at
  once. Staying put means shipping a binary two other live projects
  already install. The control loses to any clean finalist.

### Territories and candidate funnel

Roughly 45 candidates across seven semantic families; 13 were prior
seeds, the rest new. Shallow screen = crates.io existence check
(all 48 lookups logged 2026-07-26) plus known-product knowledge.

1. **Geology/stratigraphy** (continuity with `strata`): strata,
   qualified-strata, stratigraph, outcrop, varve, tephra, karst,
   sherd, tell.
2. **Records, registers, books**: annal, acta, cartulary, colophon,
   palimpsest, almanac, docket, hansard, scholia, edda.
3. **Weaving/thread**: skein, weft, selvage, quipu, braid.
4. **Navigation and ship memory**: cairn, rutter, waymark,
   binnacle, portolan, purser, stele.
5. **Memory itself**: loci, memex, engram, mnemo, munin, mimir.
6. **Keeper characters**: clio, seshat, griot, terrier, docent,
   annalist.
7. **Seed leftovers**: ergo, lorekeep, canonforge, arcanum,
   strataledger, strataclerk, stratacorpus.

Notable eliminations, by conflict class:

- **Exact crate taken, active or prominent [V]**: skein (Skein hash,
  320k downloads), clio (1.48M-download CLI library, plus Clio legal
  SaaS), seshat (Matrix message logger, updated 2026-07-22), engram
  (backup VCS, 57k), memex (12k, agent-context tool; plus WorldBrain
  Memex product), stele (11k), docket (32k), almanac (13k), ergo
  (15k, plus the nearby sandover/ergo agent-backlog CLI), mimir
  (17k, plus Grafana Mimir), munin (plus the Munin monitoring
  system's `munin` executables), acta, weft, karst (reserved),
  quipu, purser, mnemo, waymark, colophon, palimpsest, hansard,
  selvage, arcanum, terrier (retry crate, plus the University of
  Glasgow Terrier IR platform), tell.
- **Squatted by trivial crates [V]** — unavailable in practice,
  since crates.io does not reassign names: cairn (v0.0.0
  experimental), rutter (v0.0.0 "Example"), loci (v0.0.0
  placeholder).
- **Taken by direct near-competitors [V]** (see landscape note
  below): edda, cartulary, braid, binnacle.
- **Free but eliminated on semantics or ergonomics [A]**:
  stratigraph (overstates "graph"; 11 characters), annal
  (nonstandard singular; one keystroke from an unfortunate word),
  tephra (ash reads as destruction), portolan ("port" prefix reads
  as networking in a CLI), griot (nonobvious pronunciation; borrows
  a living oral tradition's role-name), lorekeep (fantasy
  coloration, prior objection stands), canonforge (canonicity
  overclaim — parked ideas and open dragons are deliberately not
  canon), strataledger/strataclerk/stratacorpus (inherit every
  strata collision and add length; "ledger" pulls toward
  blockchain/accounting).

Finalists advanced to deep research: **outcrop**, **varve**,
**sherd**, **annalist**, **docent**, plus the retain-`strata`
control.

### Landscape note: the category is forming right now

The shallow screen surfaced a cluster of young tools occupying
near-identical territory, all created between 2025-11 and 2026-04
**[V]**: `edda` ("Decision memory for coding agents — CLI and TUI",
edda.sh, crate created 2026-02-18), `cartulary` ("The knowledge
layer of your project — decisions, issues, docs", 2026-04-29),
`braid` ("lightweight, repo-local, multi-agent capable issue
tracker", 2025-12-27), `binnacle` ("CLI tool for AI agents and
humans to track project graphs", 2026-01-23), `memex` ("agent-ready
context", 2025-11-01), `mnemo` ("personal knowledge vault for your
terminal", 2026-01-25). Every plain memory-word crate checked was
already taken. **[I]** Repo-local memory for agent-assisted
development is an actively forming category; this is urgency
evidence for First Contact, and it means the chosen name will soon
have to be distinguishable among semantic neighbors, not just
technically free.

### Finalist research

**outcrop** — where buried strata reach the surface and become
readable without excavation.

- **[V]** crates.io, npm, PyPI, and Homebrew names all free
  (404s from all four registries, 2026-07-26).
- **[V]** criccomini/outcrop: read-only Rust web dashboard for
  SlateDB, 12 stars, active June 2026, distributed via curl
  installer that places a binary named `outcrop` in
  `~/.local/bin`; not on any registry. Author (Chris Riccomini) is
  a visible infrastructure developer. Close conflict, borderline
  executable conflict for SlateDB users.
- **[V]** outcrop.app: "The knowledge base for software teams"
  (Notion/Confluence alternative), private beta with waitlist,
  repo outcropapp/outcropapp at 4 stars. The semantically closest
  discovered collision.
- **[V]** GitHub user handle `outcrop` squatted by Outcrop
  Communications Ltd. (Canadian marketing agency, zero repos),
  which owns outcrop.com and asserts marks on its own name/logo —
  different industry; risk observation only, not legal clearance.
- **[V]** Plain search is owned by the geology meaning; no software
  product owns the word.
- **[A]** Two syllables, spelled exactly as heard, no homophone.
  Continuous with the project's geological vocabulary: docs can
  say "an outcrop is where the strata show". Names the function
  (exposure for reading), not a grandiose claim.

**varve** — an annual sediment layer; a countable, datable stratum.

- **[V]** crates.io, npm, and Homebrew free (404s, 2026-07-26).
- **[V]** PyPI `varve` is taken and active: a Python
  pipeline-caching devtool (v0.5.0, 2026-07-16, moeakwak/varve,
  1 star) that installs a console script named `varve` — an exact
  cross-ecosystem executable conflict, tiny but alive.
- **[V]** Cardosaum/varvedb (11 stars, Rust append-only event
  store) is one suffix away; several zero-star `varve` repos
  exist; the GitHub user handle is squatted and empty.
- **[V]** No trademark-shaped entity found beyond a stale
  consultancy ("Varve IT"); plain search is entirely
  geology/paleoclimatology.
- **[A]** Semantically the sharpest candidate: layers you can
  date — which is what front-matter `created:`/`closed:` fields
  make literal. One syllable, five letters. Costs: near-homophone
  of "valve" in speech, and the word needs a one-line explanation
  forever.

**sherd** — a pottery fragment, the archaeologist's unit of find.

- **[V]** crates.io, npm, and Homebrew free (404s, including the
  sparse index, 2026-07-26).
- **[V]** PyPI `sherd` is taken (pottery-profile vectoriser GUI,
  releases June 2026); wheel inspected — it installs no console
  script, so no executable conflict.
- **[V]** No live exact-name software (a dead 2019 Crystal package
  manager and a dead 2010 mirroring tool); GitHub handle squatted
  since 2013, inactive. "Shard" is heavily crowded
  (Crystal `shards`, Elasticsearch shard tooling, database
  sharding) and Minecraft's "Pottery Sherd" generates high search
  volume.
- **[A]** The word names a fragment: it undersells exactly the
  coherence — settled decisions, typed edges, lifecycle — that
  distinguishes this tool. Users will type `shard` and search
  engines will offer it. Eliminated on semantics despite clean
  registries.

**annalist** — the keeper of year-by-year records.

- **[V]** crates.io, npm, and Homebrew free; PyPI `annalist` taken
  by the dormant gklyne linked-data notebook (frozen 2022-04-12,
  owns annalist.net). noctuid/annalist.el (45 stars, active, a
  dependency of the evil-collection Emacs ecosystem) holds the
  exact name in another package world. GitHub user handle taken.
- **[V]** Software-qualified searches surface the dormant gklyne
  project first; code-adjacent searches bleed into rust-analyzer.
- **[A]** Spoken, it is indistinguishable from "analyst" — a
  pun once, a spelling dictation forever. Eliminated.

**docent** — the guide who explains a collection.

- **[V]** crates.io and Homebrew free, but PyPI `docent` is the
  actively shipped SDK (v0.1.75, 2026-07-15) of Transluce's
  "Docent", an AI-agent transcript-analysis platform
  (TransluceAI/docent, 114 stars, pushed 2026-07-23;
  transluce.org/introducing-docent). npm `docent` is a dormant
  squat. A second AI product ("Docent" support-chatbot builder)
  also uses the name.
- **[I]** Transluce's Docent lives in the AI-agent tooling space —
  the same conversations, conferences, and search queries this
  project will inhabit. Taking the name means being "the other
  docent" in one's own niche from day one. Eliminated despite the
  aptness of the metaphor.

### Comparison table

| | outcrop | varve | sherd | annalist | docent | strata (control) |
|---|---|---|---|---|---|---|
| crates.io | free | free | free | free | free | squatted 2019 |
| Binary name clear on PATH | curl-installed `outcrop` (12★ tool) | pip `varve` exists (1★) | yes | yes | yes | no: strata-rs + Klavis `strata` |
| Library `use NAME::` | free | free | free | free | free | taken (strata-rs) |
| npm / PyPI / brew | all free | PyPI taken (active) | PyPI taken (no bin) | PyPI taken (dormant) | PyPI taken (active, same niche) | npm bin + PyPI squats |
| Close product conflict | outcrop.app (private beta) | none found | none live | dormant gklyne + annalist.el | Transluce Docent (active, same audience) | ≥4 active brands |
| Spoken/spelled | clean | "valve" adjacency | "shard" absorption | = "analyst" | clean | clean but diluted |
| Semantic fit [A] | exposure of layers for reading | datable layers | fragment (undersells) | record-keeper | explainer-guide | the layers themselves |
| Needs explanation | rarely | one line, forever | shard confusion | spelling, forever | no | no |

### Proposed identities

- **outcrop**: project "Outcrop"; repo `henry-filgueiras/outcrop`;
  crates.io `outcrop` (single crate, lib + bin); executable
  `outcrop`; `use outcrop::` in Rust; spoken "Outcrop".
- **varve**: project "Varve"; repo `henry-filgueiras/varve`;
  crates.io `varve` (lib + bin); executable `varve`;
  `use varve::`; spoken "Varve" (rhymes with "carve").
- **strata control (qualified)**: project "Strata"; repo stays
  `henry-filgueiras/strata`; crates.io `strata-cli`; executable
  `strata` (conflicted); library identity unavailable.

Audition **[A]** — the preferred finalist in real command lines:

```console
outcrop init
outcrop new idea "relevance surfacing"
outcrop list decisions
outcrop resolve idea:15 decision:14
outcrop close task:37
outcrop doctor
outcrop completions zsh
```

`varve doctor`, `varve list decisions`, `varve close task:37` read
equally cleanly; both survive the "say it aloud in a meeting" test,
with varve carrying the valve smudge.

### Ranked recommendation

1. **Preferred: `outcrop`.** The only finalist with all four
   registry names free, a pronounceable, spelled-as-heard English
   word, and a meaning that states the product's actual boundary:
   the place where accumulated layers become readable without
   digging. It preserves the project's geological self-description
   (the corpus is still strata; the tool is the outcrop) so
   existing archaeology vocabulary survives the rename intact.
2. **Strongest fallback: `varve`.** Cleanest search identity and
   the sharpest single-word semantics (datable layers), held back
   by the active-though-tiny PyPI executable of the same name and
   the spoken "valve" adjacency.
3. **Control: retain `strata`.** Materially beaten: package,
   binary, library, and search identity are all contested, two of
   them by active tools in this project's own ecosystem and
   audience. The prompt's bar — "a rename must materially beat
   staying put" — is met by both finalists above.

**Strongest argument against the preferred choice**: outcrop.app is
a private-beta "knowledge base for software teams" — of everything
found, the nearest semantic neighbor, and unlike varve's collisions
it could grow into direct mindshare competition; additionally,
SlateDB users who installed criccomini's dashboard already have an
`outcrop` on PATH. Both are small today **[V]**; betting on
`outcrop` is betting neither becomes large **[I]**.

Not done here, by design: no naming decision artifact, no rename,
no crate reservation, no repository-settings change. Adjudication
is Henry's.

### Addendum (2026-07-26): the evocative-moniker direction

After the tournament closed, a third direction was explored in
discussion: instead of a word from a relevant semantic family, a
short evocative moniker whose meaning is supplied by the project
description — the `git` strategy. Constraints: four letters,
unambiguously pronounceable in English, easy to type, pleasant as
the canonical executable.

Method **[V]**: a generator was built (now the `namegen` cantrip,
destined for the dev-env repository) that enumerates every 4-letter
string legal under a conservative model of English phonotactics
(patterns CVCV/CVCC/CCVC/VCVC/VCCV; no soft-c/soft-g forks, no
silent-e), ranks ~9.7k candidates by a dictionary-trained trigram
euphony model blended with QWERTY ergonomics, and probes registries
for collisions. The top 300 pseudowords plus 18 curated real words
were screened against the crates.io sparse index on 2026-07-26:
**243 of 318 were free**. Survivors were hand-filtered for sound
and accidental meaning **[A]**, and the shortlist was cross-checked
against all four registries the same day **[V]**:

| name | says | crates | pypi | brew | npm | notes [A unless noted] |
|---|---|---|---|---|---|---|
| nost | NOST | free | free | free | squat | Greek *nostos*, the homecoming — root of "nostalgia"; quietly about returning to a project and knowing it |
| forn | FORN | free | free | free | squat | Old Norse "ancient"; Icelandic for archaeology is *fornleifafræði*. n/m adjacency makes "form" typos and autocorrect a daily tax |
| held | HELD | free | free | free | squat | what the repository held; common-word ungoogleability, the full git tradeoff |
| prow | PROW | free | free | free | squat | ship's leading edge; forward-facing, slightly fights the memory mission |
| wold | rhymes "old" | free | free | free | free | only candidate free on all four registries **[V]**; rolling upland; spoken it can smear into "would" |
| torm | TORM | free | taken | free | squat | pure sound; minor D&D deity noise |
| vist | VIST | free | taken | free | squat | vista-adjacent, clean |
| marl | MARL | free | taken | free | squat | lime-rich mudstone and marled yarn — geology and weaving in one quiet word |
| lith | LITH | free | taken | free | squat | stone, as in lithic |
| knap | trap | free | taken | free | squat | flint-knapping; silent k invites "kuh-nap" |

Casualty: `wist` (archaic English "knew" — near-perfect semantics)
is taken on crates.io, npm, and PyPI **[V]**.

Discussion. npm is a graveyard for pronounceable 4-letter strings,
but npm is the one registry this project would never publish to;
crates.io, Homebrew, and the binary name are load-bearing, PyPI
only if a Python SDK ever exists **[I]**. The strategy's real
advantage follows from the tournament's own landscape finding: the
semantic neighborhoods (memory, records, layers) are exactly where
the 2025–26 near-competitor cluster is squatting, so a distinctive
pseudoword buys an uncontested search identity — "nost cli" would
match nothing but this project **[I]**. The cost is zero semantic
scaffolding: the name does no onboarding work, acceptable for a
tool met through a README rather than a search box **[A]**.

Within this direction the pick is **nost** — one pronunciation,
snappy to type (`nost doctor`, `nost new dragon`, `nost resolve
idea:15`), hidden etymology about return-and-recognition without
claiming anything **[A]**. `forn`'s archaeology easter egg is
better still, but the "form" typo gravity is real; `wold` is the
cleanest namespace with the muddiest mouth.

Scope caveat: moniker candidates received registry screening only —
not the deep pass (GitHub identities, products, trademark-shaped
risks) the tournament finalists received. If one becomes a serious
contender, run that pass before deciding. The ranked recommendation
above stands unchanged; this addendum adds a third option class to
the adjudication: `outcrop` (semantic), `varve` (semantic
fallback), or a moniker led by `nost`.

### Addendum (2026-07-26): five-letter extension and cross-round snapshot

A bounded five-letter round extended the moniker direction, using
the four-letter shortlist's texture (compact, consonant-rich,
tactile, one strong beat) as the aesthetic target. Method **[V]**:
the `namegen` cantrip gained a `--len 5` mode (patterns
CCVCC/CVCVC/CCVCV/CVCCV/VCCVC/VCVCC over the same onset/coda
tables, ~98.5k legal coinages); the euphony ranking alone favors
soft CVCV-coinages, so gritty-texture candidates were surfaced with
a per-pattern filter and hand curation. 64 candidates — curated
real/archaic/craft words, textured coinages, and raw high-scorers —
were screened against the crates.io sparse index on 2026-07-26;
24 crates-free candidates were surfaced in chat. This round used
crates.io as the only bulk gate; no cross-registry or deep pass.

Findings **[V]**: the stone-and-craft lexicon is heavily mined on
crates.io (`chert`, `spall`, `clast`, `karst`, `scree`, `swarf`,
`burin`, `quoin`, `cairn`, `riven` all taken); the survivors are
the obscurer corners of that vocabulary. `esker` is crates-free but
collides with Esker S.A. (esker.com), an established
document-automation software company — admired and excluded for
trademark risk. Coinage seeds `prow` (Kubernetes Prow) and `knap`
(a Neovim plugin) are name-occupied by developer tools.

Cross-round top ten, both lengths, one assistant's taste **[A]**
(registry columns are dated 2026-07-26 snapshots **[V]**; 5-letter
rows were probed on crates.io only):

| rank | name | len | registries | why it survives | strongest objection |
|---|---|---|---|---|---|
| 1 | scarp | 5 | crates free | the seed texture exactly, and an escarpment is where strata become readable — the product promise in one beat | one transposition from `scrap`; weakly googleable common word |
| 2 | orlop | 5 | crates free | a ship's deepest deck — the lowest stratum of the vessel; strange in the right amount, clean VCCVC | obscure enough to need its gloss every time; faintly comic "-lop" |
| 3 | nost | 4 | crates/pypi/brew free, npm squat | the four-letter round's pick stands: *nostos*, the homecoming, snappiest to type of any candidate | zero semantic scaffolding even by moniker standards; "-ost" coinage smell |
| 4 | quern | 5 | crates free | a hand mill — humble, tactile, daily-grind tool imagery; archaic without fantasy residue | `qu-` spelling/typing tax; spoken it drifts toward "kern" |
| 5 | sherd | 5 | crates free | archaeology's own word for a recovered fragment of the past | a lifetime of "you misspelled shard"; autocorrect pulls to "shred" |
| 6 | wold | 4 | free on all four | still the only candidate anywhere with a fully clean namespace | spoken it smears into "would" |
| 7 | marl | 4 | crates/brew free, pypi taken, npm squat | geology and weaving in one quiet word; effortless to say and type | least distinctive of the ten; marl/Marl place-name noise |
| 8 | lith | 4 | crates/brew free, pypi taken, npm squat | stone at its most compressed; reads like a tool | feels like a fragment of a longer word — "-lith" is a suffix |
| 9 | drost | 5 | crates free | best pure coinage across both rounds: consonant-dense, single beat; accidentally Dutch for a district steward — a keeper of records | common Dutch surname; one letter from Droste, reads borrowed |
| 10 | winze | 5 | crates free | a shaft sunk between mine levels — descent through the workings' layers; pleasant to type | spoken aloud it is "wins" |

Near-misses recorded for completeness: `sprag`, `thole`, `arris`,
`held`, `torm`, `prent` (Scots "imprint" — print-adjacency is the
same daily typo tax that sank `forn`), `wista`. Henry's early,
explicitly non-committal leanings at snapshot time: `scarp` and
`orlop`, with `esker` admired but set aside for the trademark wall
**[V]**. The scope caveat above applies unchanged: promotion of any
of these requires the deep pass the tournament finalists received.
