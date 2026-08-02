---
id: tsk_01KYZXTN547YF2R6YZBDV2ZYDM
sequence: 59
kind: task
status: pending
sprint: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
created: 2026-08-01
---

# Document the proposal lifecycle through landing and reconciliation

## Objective

Extend `docs/remote-proposals.md` so it describes the whole lifecycle —
propose, realize, land, reconcile — instead of stopping where sprint 10
stopped.

The doc currently ends with realization and a section on why automated
realization was deferred. A reader following it today reaches a
canonical artifact and an issue that stays open forever, with nothing
telling them that is expected or what to do about it.

### What the reader needs

- The fourth step exists, and why it is separate: landing is an event
  `realize` cannot observe, so reconciliation cannot be folded into it.
- What authority each step holds. The existing "proposal authority and
  mutation authority" and "the trust boundary, enumerated" sections are
  the right home; reconciliation adds a third position — an actor that
  reads canonical state and writes only to GitHub — and the enumeration
  should place it rather than append to it.
- That reconciliation is terminal. It is not a status mirror: adopting
  or rejecting the idea later does not reopen or amend the issue, and
  editing the issue afterwards reaches nothing. The existing
  "realization is one-shot, not synchronization" section makes the same
  argument for the other direction and should not be duplicated.
- What to do when it refuses — artifact not landed, no realizing
  artifact, integration unavailable.
- How a consumer sets this up in their own repository, folded into the
  existing setup section rather than bolted after it.

### Constraints

- Reflect what shipped, including what task 58 declined. A doc that
  describes the workflow if the workflow was not built is worse than
  silence.
- The README's command table needs the new verb; keep the one-line
  framing consistent with the `list` / `realize` row.
- Do not restate the charter. The doc is for someone who never reads the
  archaeology.

## Acceptance criteria

- `docs/remote-proposals.md` covers propose → realize → land →
  reconcile, with the authority boundary stated at each step.
- The trust-boundary enumeration includes reconciliation's position,
  integrated rather than appended.
- Terminality is stated in both directions, without duplicating the
  existing one-shot section's argument.
- Refusal modes are documented with what the reader should do.
- The setup instructions cover a consumer repository, and claim nothing
  that was not exercised — sprint 10's retrospective is explicit that the
  GitHub half of the consumer proof was never run in a second
  repository, and this sprint should not quietly upgrade that claim.
- The README command table lists the new verb.
- If task 58 declined automation, the doc says the issue is closed by an
  operator command and why, rather than leaving a reader to wonder
  whether automation exists.
- `scripts/check.sh` passes.
