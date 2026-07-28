---
id: spr_01KYFRWF0B8QKN89NHVKQG2TQT
sequence: 8
kind: sprint
status: active
created: 2026-07-26
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
2. [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|Task 44]] — the pre-publication
   claim and landing-surface audit, over the final package and
   README source and every surface that can be checked before a live
   page exists.
3. [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|Task 45]] — publication, tag,
   GitHub release, verification of the live registry and
   documentation surfaces, a genuinely clean-environment install and
   quickstart, and sprint closure.

Task 44 runs after 43 so that the install and quickstart prose task
43 writes falls inside the audit rather than outside it. Task 45 runs
last because the surfaces it inspects — the rendered crates.io page,
the docs.rs build, an install from the registry — do not exist until
publication has happened.

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
