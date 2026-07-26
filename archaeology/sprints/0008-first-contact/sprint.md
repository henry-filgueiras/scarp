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
