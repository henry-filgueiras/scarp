---
id: tsk_01KYE1JV7X5NNYYMAH5400T5ZR
sequence: 38
kind: task
status: pending
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-25
---

# Batch reference resolution: strata resolve

## Objective

Add `strata resolve <REFERENCE>...`: map one or more references to
stable ids in a single invocation. Motivating friction (task 37's
report): authoring wikilinks for idea 29 meant grepping artifact
files for `id:` values because `show` answers one reference per
invocation. `resolve` is the read-only query primitive; the in-file
binding of prose sugar remains [[idea-links-bind-command|idea 1]]'s
separate mutation, which this verb would underpin, not replace.

## Semantics

- **Input**: one or more positional references, each either
  `kind:sequence` sugar (`idea:15`) or a stable id — the same
  selector grammar `show` accepts, over all managed collections.
  A stable id input is verified to exist and echoed back:
  normalization is idempotent, and a stale id is a resolution
  failure, not a pass-through.
- **Output**: exactly one line per input on stdout, in input order,
  each line the bare stable id and nothing else. Positional pairing
  is the contract: line k answers input k. Duplicate inputs resolve
  independently and repeat in the output.
- **Failure**: all-or-nothing on stdout. If any input fails, stdout
  emits nothing, every failure (not merely the first) is reported on
  stderr under the decision 4 error contract, and the exit code is
  nonzero. No in-band not-found sentinel, ever: a sentinel on stdout
  is exactly what a careless pipe consumer pastes into a canonical
  file, while an empty stdout composes safely with `&&` and `$(...)`.
  Resolution is read-only, so fix-and-rerun is the whole recovery
  story, and one run reports the complete typo list.
- **`--json`**: a deterministic array, one object per input in input
  order, carrying at least the input as given, kind, stable id,
  sequence, canonical reference, path, and title — the richer
  surface for consumers assembling `[[id|label]]` markers, where the
  title informs the label.
- **Not in scope**: a stdin mode. The demonstrated consumer passes
  arguments; `-`/no-args stdin reading waits for a real pipe
  consumer, per the first-consumer rule. The order-preserving,
  sentinel-free stdout is chosen so a future stdin mode composes
  without semantic change. Prior art: `git rev-parse` (batch
  ref-to-id, bare ids on stdout), `git cat-file --batch-check`.

## Acceptance criteria

- `strata resolve idea:15 idea:22` prints exactly two stable ids in
  argument order and exits zero, in and only in a repository where
  both resolve.
- A mixed invocation with one unresolvable reference prints nothing
  on stdout, reports every failing input on stderr, and exits
  nonzero.
- A stable id argument echoes itself when the artifact exists and
  fails resolution when it does not.
- `--json` emits the deterministic array described above; humans and
  automation get the same resolution semantics.
- The command appears in `--help` with no placeholder flags; no
  stdin mode ships.
- Tests cover order preservation, duplicates, mixed sugar/id input,
  the all-or-nothing failure mode, and stale stable ids.
- `scripts/check.sh` passes.
