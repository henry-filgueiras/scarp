---
id: dec_01KYJE2K3VRASS8A1X1E847S1B
sequence: 16
kind: decision
status: accepted
created: 2026-07-27
---

# Adopt Scarp as the release identity

## Context

The project shipped nothing under the name it was built with.
[[tsk_01KYFRWF1X37N5TBJ139X7ZKA1|Task 40]] ran a bounded naming
tournament ahead of `v0.1.0` and found the retain-`strata` control
not viable: crates.io `strata` is squatted, `strata-rs` installs a
binary named `strata` and occupies the `strata` library namespace,
Klavis AI's `strata-mcp` installs a CLI named `strata` aimed at
exactly this project's audience, and at least four active software
brands contest the word. That is a package conflict, an executable
conflict, a library conflict, and a search-identity conflict at
once.

The tournament recommended `outcrop`, with `varve` as fallback, and
two addenda opened a third direction: short evocative monikers whose
meaning is supplied by the project description. A five-letter round
put `scarp` at the top of the cross-round snapshot — an escarpment
is where strata become readable — with Henry's early, explicitly
non-committal leaning already resting there.

Henry adjudicated on 2026-07-27, after that snapshot and a
subsequent deep screen of the moniker candidates. This decision
records the adjudication; it does not reopen it.

## Decision

### The identity

| Surface | Value |
|---|---|
| Project / display name | Scarp |
| GitHub repository | `henry-filgueiras/scarp` |
| crates.io package | `scarp` |
| Executable | `scarp` |
| Rust library crate | `scarp` (`use scarp::`) |
| Repository marker | `.scarp.toml` |

Positioning line:

> Scarp exposes the strata of a repository: what changed, why, and
> what remains unsettled.

### Style

The name is written **Scarp** in prose and `scarp` in code, paths,
and command lines. It is never **SCARP**: it is a word, not an
acronym, and nothing about it expands.

"Strata" survives as the geological and domain metaphor for the
accumulated repository record, written lowercase in prose — the
strata are what the scarp exposes. It no longer names the product,
the package, the executable, the library, or the marker. Every
remaining lowercase use must genuinely mean accumulated layers; the
metaphor is not a hiding place for a stale product reference.

### No compatibility executable

No `strata` executable ships — not as an alias, not as a shim, not
as a deprecation stub. The project is unreleased, so there is no
installed base to protect, and shipping a `strata` binary would
reintroduce the exact PATH collision with `strata-rs` and
`strata-mcp` that motivated the rename.

### A hard marker cut, not dual discovery

`.strata.toml` becomes `.scarp.toml`. Discovery recognizes one
marker filename. No fallback probe, no migration command, and no
"if the new marker is absent, try the old one" path is added.

Dual discovery would be permanent: every future reader would carry
both spellings, and the ambiguity of a directory containing both
files would need its own rules. The only repositories in the world
carrying `.strata.toml` are this one and any local experiment, all
of which are converted by renaming a file. A pre-release rename is
the last moment this cut is free; it is taken now.

Config schema `version = 1` is unchanged. The representation and
semantics of the config are identical — only the product-owned
filename moves. Bumping the version would falsely tell readers that
the document's contents changed.

### What this supersedes

This decision supersedes exactly one clause of
[[dec-bootstrap-repo-marker|decision 5]]: the spelling of the marker
filename, which depended on the product name. Everything else in
decision 5 remains controlling and is restated here as unchanged:
the marker must be a regular file, symlinks are rejected, the file
contains a TOML table with an integer `version` key, unknown keys
are tolerated within a supported version, an unsupported version is
a `malformed-artifact` error and is never overwritten, and the
init mutation-safety contract holds — existing files are never
modified, truncated, or replaced, and the config is written last via
an exclusive temporary file and an atomic no-clobber persist. Its
2026-07-21 update, defining repository validity by the marker alone
and closed under Git round-trip, is likewise unaffected.

### The historical record is not renamed

Existing artifact ids, filenames, titles, and prose are not
rewritten. `strata` commands and `.strata.toml` spellings in the
existing corpus were accurate at the commits where they were
written, and `scripts/bootstrap-inception.sh` is deliberately frozen
provenance of the original inception performance.

Retroactive renaming would make the archaeology lie about its own
history — the failure mode this project exists to prevent. The
boundary is between *current* surfaces, which must all say Scarp,
and *historical* statements, which must all continue to say what
they said.

### Residual objections, acknowledged

Adopting the name does not dissolve its known costs:

- `scarp` is one transposition from `scrap`, and the typo produces a
  real English word that spellcheck will not flag;
- an unrelated PyPI package named `scarp` exists;
- an unrelated archaeology consultancy uses the name.

None of these is a package, executable, or library collision on the
registries this project publishes to. No claim of legal clearance or
global uniqueness is made or implied here; this is risk research,
not a trademark opinion.

### The name is not a reservation

A GitHub repository rename does not reserve anything on crates.io.
crates.io `scarp` was verified absent on 2026-07-27, but the name
remains claimable by anyone until publication. The release task must
re-verify availability immediately before it publishes, and must
treat an occupied name at that moment as a release blocker rather
than a formality.

## Consequences

- [[tsk_01KYJE2K3PK4F5XC81N8S6PBNA|Task 42]] migrates every live
  repository-controlled surface and records the operator runbook for
  the human-owned GitHub cutover.
- The GitHub rename, remote updates, crate publication, tags, and
  releases are Henry's actions and are not performed by any agent
  task.
- The `v0.1.0` release work inherits one new obligation: recheck
  crates.io `scarp` immediately before publishing.
- The README and quickstart adopt the positioning line above, under
  the constraint already recorded by
  [[tsk_01KYFYKAZRGEJPJYKAWV8W9BB4|task 41]]: position the tool as
  Git-native, reviewable project archaeology, and do not make the
  claims that task retired.
- A repository carrying `.strata.toml` is no longer a valid
  repository. Any local experiment is converted by renaming that file
  to `.scarp.toml`; no tooling assistance is provided.
