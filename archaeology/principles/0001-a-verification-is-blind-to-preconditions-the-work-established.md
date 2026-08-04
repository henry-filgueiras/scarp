---
id: prn_01KZ76WRJ5QMEDGCPB6076HEAC
sequence: 1
kind: principle
status: active
created: 2026-08-04
---

# A verification is blind to preconditions the work established

## Statement

A verification is blind to any defect whose precondition was established
by the work being verified. Check in an environment the work did not
touch, or the check proves only that the environment is already in the
shape the work put it in.

A companion, narrower and sharper: **a passing check is not evidence
that its documented mechanism ran.**

## Rationale

Doing the work leaves residue — installed toolchains, warm caches, files
present in the tree, a directory created by the first run. To a local
check, that residue is indistinguishable from a property of the artifact
under test. The check does not lie; it answers a question about a
contaminated environment while appearing to answer one about the
artifact.

The companion follows from the same shape applied to evidence rather
than to state. A green result is consistent with the documented
mechanism having run and with several alternatives, and nothing
distinguishes them unless someone looks. Sprint 8 shipped a CI comment
asserting a mechanism whose own job log, in order, recorded the
mechanism not running.

Distilled from [[log_01KYK8RC0YEY51YP37RGV7M7N4|Verification blind spots found while preparing the first release]], which records how the shape was found: four
defects in one sprint that looked unrelated — a test that passed in the
working tree and failed inside the packaged crate, an install check a
warm cache would have satisfied, a CI gate whose defect was hidden by
the very step that determined the MSRV, and a quickstart that worked
only because it had already been run.

That log remains the canonical account, with its four instances, its
dates, and its closing admission that three of the four were found
*after* the work they invalidated had been recorded as verified. This
principle is the reusable claim extracted from it; the log is the
evidence, and neither replaces the other.

First applied prospectively rather than retrospectively in sprint 12,
task 65, which requires the consumer affordances to be exercised from a
repository that did not build them.

## Application ordering

1. Prefer an environment the work never touched: a fresh `CARGO_HOME`,
   an unpacked tarball, a clean container, a second repository.
2. Failing that, snapshot the contaminating state and assert it did not
   change across the check.
3. Failing that, record explicitly which precondition the check assumed,
   rather than reporting the result as clean.

Prefer a check that can fail over a comment claiming it cannot.

## Counterpressure

Isolation costs wall-clock time, and it can itself be wrong: a container
missing something every real user has proves the opposite of what was
intended. A fast local check that *names its assumption* beats a
thorough one that gets skipped.

So this principle argues for stating the contamination, not for maximal
isolation. Applying it maximally — demanding a pristine environment for
every verification — is a failure mode of its own, and one that
correlates with checks quietly disappearing from the routine.

## Failure signals

- A check that has only ever run on the development machine.
- A verification step positioned after the step that creates its
  precondition.
- "It works here" standing in for evidence.
- A green result whose mechanism was never distinguished from a
  plausible alternative.
- A comment asserting what a job log would refute.
