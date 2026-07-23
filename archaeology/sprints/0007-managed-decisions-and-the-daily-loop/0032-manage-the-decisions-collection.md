---
id: tsk_01KY7S6Q7VF46RFDNFTCY9E5B2
sequence: 32
kind: task
status: pending
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-23
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
