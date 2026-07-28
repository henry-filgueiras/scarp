---
id: ide_01KYK895PPE90CY8RAAFBV8B4P
sequence: 34
kind: idea
status: parked
created: 2026-07-27
---

# Execute the README quickstart as a test

## Problem

The README quickstart is executable code that ships inside the crate
payload, is the first thing a stranger runs, and has no test.

Its failure mode is not hypothetical.
[[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|Task 46]] found that the shipped
quickstart could initialise a Scarp repository in a reader's own
working directory — `mkdir /tmp/scarp-demo && cd /tmp/scarp-demo`
followed by unguarded lines, where a failed `mkdir` leaves every
subsequent command running wherever the reader was standing — and that
its cleanup was a fixed-path `rm -rf` of a directory the quickstart
may not have created. Both survived
[[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|task 43]], which executed every
command exactly as documented and recorded real output. Executing the
happy path is not a test; it is a demonstration, and it creates the
conditions under which it keeps succeeding.

The repaired block is safer but no better defended. Nothing prevents a
future edit from reintroducing a fixed path, dropping the subshell, or
letting the recorded output drift from what the binary prints —
`0.1.0`'s output block was already stale once before task 43 refreshed
it, and [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]] still carries the
`See it work` section's staleness as an open suspect. Prose rots
silently because no gate reads it.

## Sketch

Extract the fenced block from `README.md` at test time and run it,
rather than maintaining a copy that can diverge from the prose. A
test that reads its own fixture out of the shipped documentation is
the only version that cannot drift.

Plausible first slice, deliberately small:

- locate the quickstart's ```sh block by a stable marker;
- execute it in a temporary directory with the built binary on `PATH`;
- assert it exits zero, that the caller's directory is unchanged
  afterwards, and that the directory it created no longer exists;
- assert the documented setup-failure containment: with an unusable
  `TMPDIR`, no `.scarp.toml` and no `archaeology/` appear where the
  block was invoked.

A second slice could compare the ```console block against captured
output modulo the fields the prose already declares as varying — the
temporary path, the ULID, and the date. Task 46 performed exactly
that comparison by hand and it passed line for line, which is evidence
the normalisation is tractable rather than a guess.

## Boundaries

- Not a documentation-testing framework. One block, one test file; if
  a second document ever wants this, that is when generality is
  earned.
- Not a rendering check. Whether crates.io displays the README
  correctly is unrelated and remains a live-surface question for
  [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]].
- Shell-dependent by nature, which is a real cost: the test asserts
  behaviour of `mktemp`, `trap`, and `set -e` as much as of Scarp. It
  should run under one declared shell and say so, not silently inherit
  whatever CI provides.
- `README.md` is packaged but `tests/` fixtures that reach outside the
  crate are not; the test must work from an unpacked tarball, which
  is where task 43's `.gitattributes` test broke for the analogous
  reason. Gate on the marker, or read the packaged README only.
- Does not replace human review of the prose. It checks that the
  commands work and stay contained, not that the words are true.

## Evidence

Two defects in one document in one sprint, neither caught by
execution: the containment failure above, and the `See it work`
section's output going stale against a corpus that moved on
(task 44's first suspect). Task 46 additionally verified the console
block against captured output by hand — a check that existed for
exactly one session and then evaporated.

Related: this is the narrow, mechanised form of the heuristic recorded
in [[log_01KYK8RC0YEY51YP37RGV7M7N4|log 3, verification blind spots]]
— that a verification is blind to a defect whose precondition the work
itself established. The author's machine always has the directory the
quickstart creates. Only a fresh run can fail.

Proposed by Claude while closing task 46, 2026-07-27.
