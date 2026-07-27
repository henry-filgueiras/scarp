---
id: tsk_01KYJG0S7SYMYY1FEG7H4QQX8G
sequence: 44
kind: task
status: pending
sprint: spr_01KYFRWF0B8QKN89NHVKQG2TQT
created: 2026-07-27
---

# Audit the landing surfaces a first-time visitor sees

## Objective

Take a deliberate once-over of every surface a stranger encounters
before they have decided whether to care, and make them agree with
each other and with what the tool actually does. This is sprint 8's
"the repository presents coherently to a first-time external visitor"
criterion, run as its own pass rather than assumed as a side effect
of writing the README.

It runs after [[tsk_01KYJG0S7GY51W8M1WYFMEV7MQ|task 43]] so that the
install and quickstart prose that task writes is inside the audit's
scope rather than outside it.

The audit has a specific standard to apply, not merely taste.
[[tsk_01KYFYKAZRGEJPJYKAWV8W9BB4|Task 41]] compared Scarp against
Edda and retired a list of claims as unsupportable: not "the only" or
first repo-local, Git-friendly, or agent-aware memory tool; not
"other tools lock your history in an opaque database"; not
determinism as a differentiator; not "safe for concurrent agents";
not "memory for coding agents" as the headline job, since that
promises capture and injection Scarp does not ship; no
tamper-evidence or audit-trail claims. It also fixed the positive
framing: Git-native, reviewable project archaeology, whose honest
distinguishing property is *continuability* — records that can be
directly edited, reviewed, branched, and merged — rather than mere
readability.

Two specific suspicions motivated this task and should be resolved
either way, not assumed:

- the README's hero line reads "structured repository memory for
  humans and coding agents", which sits close to the headline claim
  task 41 retired;
- the README describes "safe writes" while
  [[drg-bootstrap-branch-collisions|dragon 1]] (branch sequence
  collisions) and dragon 4 (power-loss durability of mutations)
  are both open, and task 41 explicitly retired "safe for concurrent
  agents".

Neither suspicion is a finding yet. The audit may conclude the
existing wording is defensible; what it may not do is leave the
question unexamined.

## Acceptance criteria

- The audited surface list is explicit and complete, covering at
  least: `README.md` including the hero, positioning line, feature
  claims, status scoreboard, and the new quickstart; the rendered
  crates.io page; the GitHub About panel — description, homepage,
  topics; `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, and
  the issue and pull-request templates; the light and dark wordmarks;
  and the repository's own first screen. Any surface deliberately
  excluded is named with its reason.
- Every claim made on those surfaces is checked against what ships
  today, and each is classified as supported, overstated, retired by
  task 41, or aspirational-stated-as-present. Corrections are applied
  to the wording, not merely listed.
- Open dragons are reconciled against safety and reliability claims.
  Where an open risk contradicts a claim, either the claim changes or
  the surface acknowledges the risk; a claim is not left standing
  because the dragon is "probably fine in practice".
- The scoreboard and any feature list match the real command surface,
  verified by running the binary rather than by reading the source.
- The surfaces agree with each other: the name, the positioning line,
  the description, the license statement, and the quickstart tell one
  story. Contradictions between GitHub metadata, the crate page, and
  the README are resolved rather than tolerated because they live in
  different places.
- Rendering is verified where it actually renders, not only where it
  is authored: the crates.io view of the README is inspected as a
  reader sees it.
- Anything found that is real but out of scope becomes an idea or a
  dragon rather than an unrecorded observation or an opportunistic
  fix.
- `scripts/check.sh` passes, and the work is committed per the commit
  policy in `CLAUDE.md`.

## Result
