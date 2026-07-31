---
id: spr_01KYFRWF0B8QKN89NHVKQG2TQT
sequence: 8
kind: sprint
status: closed
created: 2026-07-26
closed: 2026-07-30
---

# First Contact

## Goal

Take the project from a working repository to its first genuinely
external encounter: settle a coherent distribution identity, produce
an installable `v0.1.0`, provide a deterministic 60-second
quickstart/demo, and verify installation and the quickstart from a
clean environment.

## Rationale

This sprint's scope was fixed by
[[spr_01KY7S6Q69YJ6HATZB48SZBRRM|sprint 7]]'s amendment and confirmed
by its retrospective: sprint 7 was the final pre-release dogfood
sprint, and completions plus the doctor commit gate were groundwork
for exactly this release work. The corpus is green, the daily loop is
exercised, and the remaining gap between "works here" and "works for
a stranger" is identity, packaging, and a first-run story.

Identity comes first because every other deliverable — crate name,
binary name, README, quickstart prose — embeds it. The known
`strata` crates.io and PATH collision breadcrumbs make "do nothing"
a choice that must itself be examined rather than defaulted into.

## Success criteria

- A distribution identity (project, package, binary, library) is
  researched, recommended, and adjudicated by Henry.
- `v0.1.0` is installable by a documented method on a machine that
  has never seen this repository.
- A quickstart exists that takes a newcomer from install to a
  meaningful first result in about 60 seconds, deterministically.
- Install and quickstart are verified from a clean environment, not
  only on the development machine.
- The repository presents coherently to a first-time external
  visitor: name, README, license, and quickstart agree.

## Non-goals

- Embeddings, context packs, or any new managed collection.
- Generic agent scheduling or commissioning work.
- A release-automation cathedral: the first release may be a
  recorded manual performance per the chore-ledger stance
  ([[idea-chore-artifacts|idea 7]]).
- A prebuilt-binary matrix; one honest install path beats six
  unverified ones.
- The standing bootstrap non-goals: daemon, watcher, index,
  embeddings, MCP, GraphQL.

## Amendment (2026-07-27): release topology

Identity is settled ([[dec_01KYJE2K3VRASS8A1X1E847S1B|decision 16]],
[[tsk_01KYJE2K3PK4F5XC81N8S6PBNA|task 42]]). The remaining four
success criteria were initially planned as two tasks, which left the
irreversible act — publication — without a task of its own and left
"verified from a clean environment" attached to work that happens
before anything is published. Both are corrected here.

The remaining work runs in a fixed order, and the order is
load-bearing rather than tidy:

1. [[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|Task 43]] — package
   construction, manifest metadata, README install and quickstart,
   MSRV determination, verification against the packaged artifact,
   and `cargo publish --dry-run`. Nothing is published.
2. [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|Task 46]] — repair of the two
   defects found in task 43's output after it reached GitHub CI: an
   unsafe quickstart under ordinary pasted-shell semantics, and an
   MSRV gate that installed one toolchain and ran on another.
3. [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|Task 44]] — the pre-publication
   claim and landing-surface audit, over the final package and
   README source and every surface that can be checked before a live
   page exists.
4. [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|Task 45]] — publication, tag,
   GitHub release, verification of the live registry and
   documentation surfaces, a genuinely clean-environment install and
   quickstart, and sprint closure.

Task 44 runs after 43 so that the install and quickstart prose task
43 writes falls inside the audit rather than outside it. Task 45 runs
last because the surfaces it inspects — the rendered crates.io page,
the docs.rs build, an install from the registry — do not exist until
publication has happened.

### Amendment (2026-07-27): task 46 sits between 43 and 44

Task 46 was allocated after 44 and 45 and therefore carries a higher
sequence, but the order above is the sprint's logical order and is
authoritative. Sequences record allocation, not precedence.

It belongs before 44 for the same reason 44 belongs after 43: 46
rewrites the quickstart, and the audit must run over the prose that
will actually ship rather than over prose already known to be
replaced. It belongs before 45 because `README.md` is part of the
crate payload, so leaving the defect in place would freeze it into
`0.1.0` and make it cost a version number to fix.

The division is not merely sequencing. Everything before task 45 is
reversible; everything task 45 performs is not. `cargo publish`
cannot be undone: a yank hides a version from future resolution but
never deletes or replaces it, so a defect discovered after
publication costs a version number rather than an amendment.

### The sixty-second target, measured precisely

The quickstart criterion measures **the documented quickstart after
installation** — from a working `scarp` binary to a meaningful first
result — and explicitly does not include Rust dependency
compilation. `cargo install scarp` compiles a dependency tree whose
duration depends on the machine, the toolchain, and a cold or warm
cargo cache, and is not something this project controls or can make
deterministic.

Install time is still measured and reported, because a stranger
experiences it; it is simply not what the sixty seconds is
budgeting. Any timing claim published anywhere must make the same
distinction rather than quietly folding install into the number.

## Retrospective (2026-07-30)

All five success criteria are met. `scarp 0.1.0` is on crates.io,
installable from a machine that has never seen this repository, with a
quickstart that runs in 5 ms against a ~60-second budget and a set of
landing surfaces that tell one story. `0.1.0` also shipped one cosmetic
defect, held by [[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|task 47]].

### What First Contact actually cost

Seven tasks, where the plan had four. Two of the three extra tasks were
not scope creep but consequences: task 42 existed because identity had
to be migrated once it was settled, and task 46 existed because task 43's
output was wrong in two ways that only became visible after it closed.

The shape worth noting is that **the irreversible act was the cheapest
part of the sprint.** Publication itself was one command and about
twenty seconds. Everything else — determining the MSRV by bisecting
toolchains, building a positive `include` allowlist and discovering the
packaging bug it exposed, resolving the clean-tree paradox three times,
auditing every claim on every landing surface, repairing a quickstart
that could initialise a stranger's own repository — was the work of
earning the right to run it.

That ratio is the sprint's central lesson and it is not a complaint.
Nothing later in this project will be as unamendable as `0.1.0`, so the
ratio should be expected to fall, not to repeat.

### What the identity detour taught

Retaining `strata` was never adjudicated as viable and could not have
been: crates.io `strata` squatted, `strata-rs` occupying both the binary
name and the library namespace, `strata-mcp` shipping a `strata` CLI at
this project's exact audience. Four collisions, discovered only because
[[tsk_01KYFRWF1X37N5TBJ139X7ZKA1|task 40]] treated "do nothing" as a
candidate that had to survive research rather than a default.

The transferable part is not the name. It is that **the control arm has
to be examined too.** A tournament that only scores alternatives against
an unexamined incumbent will keep the incumbent, because it never
priced it.

The rename was also the cheapest it will ever be, and taking it at the
last free moment is why [[dec_01KYJE2K3VRASS8A1X1E847S1B|decision 16]]
could make a hard marker cut with no dual-discovery path, no migration
command, and no compatibility shim. One week later there would have been
an installed base and all three would have been permanent.

### A small mechanism inside a large evidence shell

crates.io's actual contract is remarkably small: a tarball under 10 MiB,
a manifest, a version that can never be reused, and a token. Publishing
is one command. Nothing in the registry asks whether the thing you
uploaded is the thing you reviewed.

This project spent far more on the evidence shell around that mechanism
than on the mechanism — a clean-tree proof, a checksum triangulated
across the download, the sparse index, and the pre-publication package,
`.cargo_vcs_info.json` read out of the *registry* copy, a container that
had never seen the repository. That asymmetry was deliberate and it paid
once, concretely: the published artifact turned out **byte-identical**
to the package verified beforehand, which converted "the upload probably
matches the review" from an assumption into a fact.

The honest caveat is that byte-identity is an observation, not a
guarantee Cargo offers, and it held only because the `include` allowlist
means archaeology edits cannot move the payload. The general principle
survives the caveat: **the registry does not verify provenance, so if
provenance matters it has to be established outside the registry.**

### The verification blind spots

[[log_01KYK8RC0YEY51YP37RGV7M7N4|Log 3]] named the shape mid-sprint — a
verification is blind to any defect whose precondition was established
by the work being verified — and then the sprint produced one more
instance after the log was written.

Task 44 checked the `#quickstart` anchor and was **right**: it matched a
real heading, in the source and on GitHub. It was also structurally
incapable of catching the defect, because crates.io rewrites heading ids
and no crates.io page existed yet. The log's companion heuristic covers
it exactly: a passing check is not evidence that the mechanism it
documents will hold on a surface the check never ran against.

Two things follow. The log's claim was already appropriately modest —
that the shape is now named and cheap to recognise, not eliminated — and
this sprint is evidence for the modesty rather than against the log.
And [[ide_01KYK895PPE90CY8RAAFBV8B4P|idea 34]] would **not** have caught
this one; it tests that the quickstart's commands run, not that the
prose renders. Recording that keeps the idea from being credited with
coverage it does not have.

### What is worth mechanizing next

One thing, narrowly.

The quickstart's extract-and-compare — pull the fenced block out of
`README.md`, run it verbatim, diff the captured output against the
documented `console` block modulo the three fields the prose declares as
varying — was hand-performed **three times** in this sprint, in tasks
46, 44, and 45. Three performances by three different pieces of work is
real recurrence evidence rather than an anticipated need, which is
exactly the promotion signal
[[ide_01KY7S6GG3NAA35KBJTC6CA1TM|idea 23]]'s desire-path ledger is
about. It is already specified as [[ide_01KYK895PPE90CY8RAAFBV8B4P|idea
34]], and it belongs as a **test** — `README.md` is crate payload, so a
test can gate it — not as a release script.

Explicitly **not** worth mechanizing yet, despite being performed once
each and feeling scriptable: the checksum triangulation, the unpack-and-
diff, the CI log assertions, the container smoke run. Each was a first
performance. `CLAUDE.md`'s stance holds and was tested here rather than
merely quoted: a checked-in script makes an undated promise about an
external interface that can drift silently, while a dated command block
in a Result cannot lie about when it worked. The recurring form is a
chore ledger ([[idea-chore-artifacts|idea 7]]), one row per performance.

Sprint 8's non-goal — no release-automation cathedral — survives its own
release. The right next step is one test, not a pipeline.
