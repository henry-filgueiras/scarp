---
id: tsk_01KY7S6Q7VF46RFDNFTCY9E5B2
sequence: 32
kind: task
status: closed
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-23
closed: 2026-07-25
---

# Manage the decisions collection

## Objective

Make decisions the fifth managed collection: creation, listing,
showing, and doctor coverage over the existing fifteen-file corpus,
without modifying that corpus. If the implementation reveals that the
fifth collection requires a wholesale further copy of collection
mechanics, stop and surface [[idea-declarative-collection-specs|idea
10]] as a decision point before proceeding — neither copy nor extract
silently.

## Acceptance criteria

- `strata new decision "<title>"` creates a correctly sequenced,
  slugged decision in `archaeology/decisions/` with status `accepted`
  and a Context / Decision / Consequences scaffold; generated decision
  identities are `dec_` followed by an uppercase ULID, matching the
  `drg_`/`ide_`/`spr_`/`tsk_` pattern.
- `strata list decisions` and `strata list decisions --json` list the
  full corpus in sequence order.
- `strata show`, in both raw and `--json` forms, is tested against all
  three address kinds: `decision:N`, a legacy stable id (for example
  `dec-bootstrap-files-canonical`), and a generated `dec_` stable id.
- The lifecycle verbs (`close`, `reopen`, `adopt`, `reject`) refuse
  decisions, and the refusal is truthful guidance: it names the
  operation, the artifact, and why decisions have no such transition —
  not a generic parse or not-found error.
- Global invariants hold across the widened managed set, with tests:
  duplicate-stable-id and duplicate-sequence detection cover
  decisions, and typed-edge resolution through the claimant catalog is
  unchanged for existing edges targeting decision ids.
- `doctor` applies the same structural invariants to decisions as to
  the other managed collections and stays green on the unmodified
  existing corpus.
- README's collection scoreboard and managed-collection claims are
  updated in the same slice, so no projection claims decisions are
  unmanaged after they are managed.
- No existing decision artifact changes in this task: none of the
  fifteen files in `archaeology/decisions/` is modified.
- Temp-directory tests cover creation, discovery, sequence allocation,
  and malformed metadata for decisions; `scripts/check.sh` passes.

## Result

Decisions are the fifth managed collection: `new`, `list`, `show`
(all address kinds), and doctor coverage landed without touching any
of the fifteen corpus files. The `DECISION` collection value admits
the single state `accepted` with an empty transition table; creation
scaffolds Context / Decision / Consequences and stamps `dec_` +
uppercase-ULID identities.

The idea 10 stop-condition did not trigger, and the evidence is now
concrete: a fifth flat collection cost one `Collection` static, one
id prefix, one three-line `create_decision` wrapper, and enumeration
entries (CLI vocabulary, doctor's flat-scan loop, the catalog's
canonical-parse positions, the bare-id unions in `show`/`close`).
The interpreting machinery — `scan`, `parse_artifact`, `create`,
doctor's `scan_dir`, the transition splicer — was reused unmodified.
Flat collections are already spec-driven in the idea 10 sense; the
divergent cost lives only in the containment layouts, as decision 11
predicted.

Lifecycle refusals are a dedicated guidance path, not the generic
wrong-collection message: every verb, in both address forms, answers
"cannot <verb> `<target>`: it is a decision, and decisions have no
lifecycle transitions — … a new decision that supersedes it". Bare
ids reach this through a best-effort decision-scan probe that
upgrades the would-be not-found; a corrupt decisions corpus degrades
to the old diagnostic rather than blocking other collections'
lifecycles.

Managing decisions flipped their claimant disposition from
`Unassessed` to canonical-parse territory, so five tests that had
used `archaeology/decisions` as their *unmanaged* specimen were
repointed at genuinely unmanaged trees (`logs/`, `notes/`) to
preserve their intent; typed-edge resolution through the catalog is
behaviorally unchanged for existing edges, now with a stronger
verdict attached to decision targets.

Corpus verdict: doctor is green over 95 artifacts (80 + 15
decisions), zero advisories — decision prose entered the
dangling-reference scan without surfacing anything. README's
scoreboard and lifecycle diagram now claim five managed collections.
`scripts/check.sh` passes; 7 new integration tests
(`tests/decisions.rs`) plus unit coverage for creation, discovery,
sequence allocation, malformed metadata, and the widened duplicate
checks.
