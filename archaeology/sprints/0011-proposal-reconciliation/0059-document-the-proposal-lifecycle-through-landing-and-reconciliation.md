---
id: tsk_01KYZXTN547YF2R6YZBDV2ZYDM
sequence: 59
kind: task
status: closed
sprint: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
created: 2026-08-01
closed: 2026-08-01
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

## Result

`docs/remote-proposals.md` now covers propose → realize → land →
reconcile, and the README's command table lists the third verb.

### The doc gained a third authority, not a fourth step

The largest change was not the new step. It was that the page's central
section — previously "Proposal authority and mutation authority" — was
built on a two-way distinction that reconciliation does not fit. It
neither proposes a change nor realizes one; it reports one already made.

Rather than appending it, the section is now **"Three authorities, held
by different parties"**, with *projection authority* named as the
weakest: the right to report canonical state outward, held by something
that writes nothing canonical and could not. The trust-boundary table
follows the same repair — `main` is restated as authoritative "in both
directions", being what realization must not fake and what
reconciliation must confirm before speaking, and the comment is placed
as derived.

That reframing is the durable part. A later channel that reports rather
than mutates now has a named position to occupy instead of being argued
about from scratch.

### Refusals are documented as a table with exit codes

Including the distinction a reader most needs and would otherwise have
to infer: **exit 11 means something is broken or absent; exit 12 means
retrying later can succeed with nothing repaired.** That is the whole
reason `PreconditionUnmet` exists as its own category, and it was
invisible from the outside until stated here.

### What was deliberately not claimed

The setup recipe now says the GitHub half has been exercised in exactly
one repository — this one — and that the Scarp half was followed into an
unrelated project successfully. [[spr_01KYX1WAD7CC0RHVZY0V7VE4X1|Sprint
10]]'s retrospective was explicit that the GitHub half was never run in
a second repository because creating one needs `delete_repo` scope this
environment lacks. This sprint added a verb to that recipe and did not
re-run it elsewhere either, so the claim is unchanged rather than
quietly upgraded: *"expected to work" is a weaker claim than "was run",
and it is the accurate one.*

Also stated rather than hidden: **nothing reminds the operator that a
proposal is waiting.** It appears twice — in the declined-automation
section as the cost of that decision, and in the extension points as the
one step still carried by memory rather than by the tool. A reader who
finds that annoying is the evidence the promotion criterion wants.

### Both declined automations are documented, separately

Two sections rather than one merged treatment, because the reasons
differ and merging them would blur exactly what
[[tsk_01KYZXTN3AMPNJ482J4Q13ACTW|task 58]] was careful to keep apart.
Realization was declined on **authority and price**; reconciliation was
declined on **value alone**, with the doc saying in terms that had it
come down to authority the answer would have been yes. Each carries its
own promotion criterion.

### Friction

None new. The `## Result` section was hand-added again — the fourth
instance of [[ide_01KYZY233Z7GAKFPFSKEAF89ZD|idea 41]] in four
artifacts, which is now less a finding than a standing tax.
