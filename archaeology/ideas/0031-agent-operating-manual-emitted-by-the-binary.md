---
id: ide_01KYFQKDDK6EG9MXQPWX3GK1KK
sequence: 31
kind: idea
status: parked
created: 2026-07-26
---

# Agent operating manual emitted by the binary

## Problem

Installing Strata into a new repository currently means copy-pasting
operating boilerplate into that repository's CLAUDE.md: what the
collections mean, the orientation ritual, lifecycle rules, the doctor
gate. Pasted instructions are a snapshot that drifts as the tool
evolves — the same staleness problem hand-written completion scripts
have, solved there by deriving the script from the binary (sprint 7,
task 35). There is no equivalent for agent-facing operating semantics:
the clap tree carries syntax only, and the semantics live in prose
that today must be duplicated per host repository.

## Sketch

A command (working name `strata agent-help`; naming open) that emits
the operating manual on stdout, shipped inside the binary so the
instructions are always version-locked to the installed tool. The
"install" in a host repository's CLAUDE.md collapses to one line:
"run `strata agent-help` for operating instructions." This is also
progressive disclosure: the manual costs an agent zero context until
it is actually pulled, matching the lazy-loading design of Claude
Code skills.

Design constraints surfaced at proposal time:

- emit to stdout only; never write or append to host-repository
  files — the mutating variant (`init --claude`) is the
  hostage-taking version;
- default output is markdown prose, since the consumer reads
  language; a `--json` command matrix is added only when a machine
  consumer exists, per the no-placeholder-flags rule;
- this is the static complement of the session-start hook and of
  [[ide_01KY7S6GHMQ8ZWNXPX7TX21X7N|idea 24]]: hook and `status`
  answer "what is the state right now", this answers "how does the
  tool work at all" — keep the manual/state split clean.

## Boundaries

- Not sprint 8 scope: First Contact is packaging and presentation;
  this is a feature.
- The manual is authored content distributed by the binary, not
  content derived from the clap tree; deriving the command matrix
  mechanically is a possible later refinement, not the point.
- Never load-bearing: host repositories must remain fully operable by
  agents that ignore the command and read the files directly.

## Evidence

Origin: Henry, 2026-07-26, during sprint 8 planning, from the
observation that completion generation (task 35) already treats the
CLI as data and asking whether the same move hands agents the shape
of the tool. Named first consumer: Henry's planned strata-managed
dev-env repository, whose install would otherwise be the first
boilerplate paste. Prior art: generated shell completions versus
hand-written ones (drift argument); Claude Code skills' lazy loading
(context-cost argument); llms.txt as a convention for tool-shipped,
agent-addressed documentation. Related:
[[idea-launch-channels|idea 16]] treats the README as the human
first-contact surface; this is the agent-facing counterpart.
