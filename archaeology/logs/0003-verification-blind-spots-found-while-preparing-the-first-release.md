---
id: log_01KYK8RC0YEY51YP37RGV7M7N4
sequence: 3
kind: log
created: 2026-07-27
---

# Verification blind spots found while preparing the first release

Sprint 8's packaging work produced four defects that look unrelated —
a broken test inside the crate, an install that could pass falsely, a
CI gate that ran on the wrong toolchain, and a quickstart that could
initialise a stranger's own repository. They share one shape, and the
shape is worth more than any of the four fixes.

This log exists because that conclusion needs a home a reader will
actually reach. It was first written into the Evidence section of
[[ide_01KYDZVN858BK52A35KJ3ZY5BP|idea 28]] as an argument for a
`principle` collection, which is a fine secondary use and a poor
primary one: the reader who needs this is not browsing parked ideas
about collections that do not exist. If that idea is ever adopted,
this becomes its first principle; if it is rejected, the reasoning
survives regardless.

## The shape

> **A verification is blind to any defect whose precondition was
> established by the work being verified.** Check in an environment
> the work did not touch, or the check proves only that the
> environment is already in the shape the work put it in.

**Rationale.** Doing the work leaves residue — installed toolchains,
warm caches, files present in the tree, a directory created by the
first run. To a local check, that residue is indistinguishable from a
property of the artifact under test.

**Application ordering.** Prefer an environment the work never touched
(fresh `CARGO_HOME`, an unpacked tarball, a clean container). Failing
that, snapshot the contaminating state and assert it did not change.
Failing that, record explicitly which precondition the check assumed,
rather than reporting it as clean.

**Counterpressure.** Isolation costs wall-clock time and can itself be
wrong — a container missing something every real user has proves the
opposite of what was intended. A fast local check that *names its
assumption* beats a thorough one that gets skipped. This argues for
stating the contamination, not for maximal isolation.

**Failure signals.** A check that has only ever run on the development
machine; a verification step positioned after the step that creates
its precondition; "it works here" standing in for evidence; a green
result whose mechanism was never distinguished from a plausible
alternative.

### The companion heuristic

> **A passing check is not evidence that its documented mechanism
> ran.**

Job `90151540325` on `d0a3775` was green. Its log contains, in order:
the workflow installing toolchain `1.88.0`; an evidence step printing
a toolchain list with no `1.88` in it; and cargo-hack emitting
`running 'rustup toolchain add 1.88 --no-self-update'` before running
the check on the toolchain it had just fetched for itself. The comment
above that job asserted the opposite. Everything needed to falsify the
claim was printed in a job nobody had reason to open.

## The four instances

1. **The working tree hid a packaging bug.** `tests/init.rs` read
   `$CARGO_MANIFEST_DIR/archaeology/.gitattributes` at runtime. Once
   `archaeology/` was excluded from the tarball, that test failed
   *inside the packaged crate* while the working tree stayed green.
   Caught only by running the suite from an unpacked `.crate`.
2. **A warm cache would have hidden an install bug.** Any
   `target/debug/scarp`, any populated `CARGO_HOME`, or any
   development binary on `PATH` could have satisfied the install test.
   [[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|Task 43]] pre-empted this with a
   fresh Cargo home, target directory, and install root, each verified
   empty first — the one instance where the principle was applied
   before the defect rather than after.
3. **Determining the MSRV created the state that hid the gate's
   defect.** Establishing that the floor was 1.88 required installing
   the toolchain named `1.88`. The local check of `cargo hack check
   --rust-version` then passed without cargo-hack needing to install
   anything — not because the workflow was correct, but because the
   machine had already been put into the shape the workflow was
   supposed to produce. The defect could only appear on a machine that
   had never determined the MSRV, i.e. any CI runner.
4. **Running the quickstart made the quickstart work.** The documented
   `mkdir /tmp/scarp-demo && cd /tmp/scarp-demo` fails when the
   directory already exists, leaving every later line to run in the
   reader's current directory. On the author's machine the directory
   exists precisely *because* the quickstart was run. The failure was
   reachable only by a first-time reader — the one person who could
   not report it.

Instance 3 is the sharpest because the contaminating step and the
blinded check were in the same task, minutes apart, and both were
performed correctly.

## What changed

- MSRV is read from `cargo metadata`, the toolchain is installed by
  the *name* cargo-hack invokes (`1.88`, not the version `1.88.0`),
  cargo-hack is pinned at `0.6.45`, and `rustup toolchain list` is
  diffed across the gate with the job failing on any change. The
  heuristic became a check that can fail rather than a comment
  claiming it cannot.
- The quickstart became a subshell over `mktemp -d` with a trap
  installed after creation, tested on the success, mid-failure, and
  setup-failure paths in three shells.
- Package verification against the unpacked artifact in a clean
  environment is now the recorded expectation for release work, not a
  precaution one task happened to take.

Both repairs are in [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]];
[[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|task 43]] carries a dated erratum
preserving the original conclusion alongside the evidence that
falsified it.

## What is still exposed

The README quickstart remains executable code that ships in the crate
payload with no test, which is instance 4 unfixed rather than absent —
nothing stops a future edit from reintroducing a fixed path. Parked as
[[ide_01KYK895PPE90CY8RAAFBV8B4P|idea 34]].

More generally, this log states a heuristic and cannot enforce one.
Every instance above was found by a human or agent choosing to look;
three of the four were found *after* the work they invalidated had
been recorded as verified. The honest claim is that the shape is now
named and cheap to recognise, not that it has been eliminated.
