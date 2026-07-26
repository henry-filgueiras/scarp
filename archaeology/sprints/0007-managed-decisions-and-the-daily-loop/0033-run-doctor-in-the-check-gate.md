---
id: tsk_01KY7S6Q9FVH11FYN5VM9VBE13
sequence: 33
kind: task
status: closed
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-23
closed: 2026-07-25
---

# Run doctor in the check gate

## Objective

Make archaeology validity a commit-gate fact: `scripts/check.sh` runs
`strata doctor` against this repository and fails when doctor reports
problems. The gate's claim is exactly doctor's claim — it catches
doctor-detectable errors in the managed collections, no more: it does
not promise to catch arbitrary corpus corruption.

## Acceptance criteria

- `strata doctor` exits nonzero when it reports problems — confirmed
  by existing behavior or added, with a test, in this task.
- `scripts/check.sh` invokes doctor against this repository and fails
  when doctor fails.
- The gate is demonstrated once against a scratch copy carrying a
  doctor-detectable error in a managed collection, and the evidence is
  recorded in this task's result.
- The added wall-clock cost of the step is noted in the result.

## Result

`scripts/check.sh` now ends with `cargo run --quiet -- doctor` under
the script's existing `set -euo pipefail`, so a doctor failure fails
the gate. No new exit-code work was needed: doctor already exits 9
(`unhealthy-repository`) on any error-severity finding, pinned by
existing tests; advice findings stay non-fatal by construction, so
the gate's claim is exactly doctor's claim.

Demonstration (2026-07-25): rsync'd this repository (sans `target/`
and `.git/`) to a scratch copy, corrupted one managed artifact
(`status: open` → `status: smoldering` in dragon 1), and ran
`scripts/check.sh` there with a shared `CARGO_TARGET_DIR`. fmt, test,
and clippy passed; the doctor step reported
`malformed-artifact archaeology/dragons/0001-…: front-matter status
is smoldering; dragons are open or closed` and the script exited 9.

Wall-clock cost of the added step in this repository: ~0.08 s
(95 artifacts; dominated by cargo-run dispatch, not the scan).
