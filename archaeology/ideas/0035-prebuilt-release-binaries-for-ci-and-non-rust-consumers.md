---
id: ide_01KYX31AE8WX1HMBFNRZ3XQK4V
sequence: 35
kind: idea
status: parked
created: 2026-07-31
---

# Prebuilt release binaries for CI and non-Rust consumers

## Problem

`cargo install scarp --locked` is the only documented install path, so
every environment that wants Scarp must first have a Rust toolchain and
then compile Scarp and its dependency graph. On a development machine
that is a one-time cost nobody notices. In continuous integration it is
a cost paid *per run*, and for a consumer whose repository is Python,
TypeScript, or Go, it means installing a Rust toolchain into their CI
purely to run a tool that ships as a single static-ish binary.

Sprint 10's proposal channel is the forcing case. It installs a pinned
published `scarp` on every proposal specifically so that this
repository's channel is identical to a consumer's — which means the
install cost is now on the critical path of a user-facing interaction,
not a background chore. The channel is meant to be copied into other
people's repositories, and the first thing a copier meets is a
multi-minute compile in a workflow whose actual work is one command.

The install was measured at 8.0 s in a `rust:1.88` container during
sprint 8's release verification, but that container already had the
toolchain and a warm registry. A cold consumer runner has neither.

## Sketch

Publish per-platform binaries as GitHub Release assets alongside each
crates.io publication, and document a fetch-and-verify install that
needs no toolchain. The pieces, roughly in order of value: a release
workflow that builds for the platforms worth supporting; checksums
published with the assets; and an install snippet short enough to paste
into a workflow.

Constraints this project already implies:

- A release remains a human-owned performance
  ([[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]'s boundary). Building
  assets may be automated; deciding to publish is not.
- Binaries are a distribution projection, not canonical anything. They
  must never become the only way to get Scarp, and `cargo install` must
  keep working.
- The recurring form of a release is a chore ledger
  ([[idea-chore-artifacts|idea 7]]), and adding assets adds rows to
  whatever that becomes rather than justifying a release cathedral —
  a non-goal sprint 8 recorded and sprint 9 kept.

## Boundaries

- Not a package-manager campaign. Homebrew, apt, Nix, and the rest are
  a separate question with separate maintenance costs; this idea is
  only about a downloadable artifact.
- Not an install script hosted at a URL. `curl | sh` is its own
  security conversation and this idea does not open it.
- Which platforms to build for is deliberately unsettled; guessing at a
  matrix before anyone has asked is how release automation becomes a
  cathedral.
- No auto-updating, no version checks phoning home.

## Evidence

Sprint 10's channel makes the cost concrete and recurring rather than
theoretical, and [[tsk_01KYX31ACH05NGA3GYH0TJA870|task 56]] is asked to
time a cold install precisely so this idea has a number rather than an
intuition. [[idea-launch-channels|Idea 16]] already treats `cargo
install` as itself a distribution channel and ranks placements that
assume a low-friction install. Prior art is unanimous among Rust CLIs
with non-Rust audiences — `ripgrep`, `fd`, `just`, and `gh` itself all
ship release binaries, and `taiki-e/install-action`, already used by
this repository's MSRV job, exists precisely because compiling tools in
CI is the thing everyone wants to stop doing.
