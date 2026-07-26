---
id: ide_01KYE05MPV39B4FYQCTA013PRR
sequence: 29
kind: idea
status: parked
created: 2026-07-25
---

# Protocols: repeatable operational procedures for participants

## Problem

The archaeology has a parked home for durable heuristics
([[ide_01KYDZVN858BK52A35KJ3ZY5BP|principles]]) but none for
repeatable operational procedures — how participants, human or agent,
engage with the project. Examples that already operate here: complete
the work, verify with `scripts/check.sh`, commit, stop; never push.

These are not principles. A principle shapes a choice and carries
counterpressure ("prefer X unless Y wins"); a protocol prescribes an
execution sequence with an entry condition, ordered steps, and an
exit condition. Overriding a principle is a signal worth surfacing;
skipping a protocol step is usually just an error.

Today protocols live in CLAUDE.md's "Archaeology workflow" and
"Commit policy" sections — a hand-maintained protocol registry with
no identity, lifecycle, or citability, the exact gap idea 28 named
for the "Core invariants" section. A task result cannot cite the
protocol it followed, and a procedural change cannot be recorded as a
dated amendment to an addressable artifact.

## Sketch

A `protocol` collection whose artifacts carry ordered execution
steps, preconditions, exit conditions, allowed transitions, authority
requirements, and failure recovery. Near term that is a structured
checklist; long term protocols may harden into explicit state
machines, which is a representation change, not a semantics change —
the artifact stays a description either way.

Typed relationships could eventually support reasoning such as "push
requires the release protocol", "commit belongs to the implement-task
protocol", "verify precedes commit". Edge vocabulary is introduced
only with first consumers, per the rule
[[idea-capability-constrained-work|capability-constrained work]]
already borrows from dragon 3.

The provocative end state: generate CLAUDE.md, AGENTS.md, and
GEMINI.md as projections from protocol (and principle) artifacts
rather than treating prompt files as canonical. That inverts today's
arrangement — the prompt file becomes a disposable, regenerable
projection per "derived projections are not canonical", and per-agent
dialects become render targets of one corpus. It also inherits that
invariant's obligation: a generated CLAUDE.md must be clearly marked
as generated, and the projection must never become the only place a
procedure is recorded.

## Boundaries

Overlaps examined at parking time:

- **Principles (idea 28).** Adjacent, not absorbing: principles
  advise decisions, protocols script engagement. A protocol may cite
  the principles it operationalizes. If both are adopted they are
  separate collections with a typed edge, not one kind with a flag.
- **Commissioning (idea 21).** Its recovered ten-step sequence is the
  corpus's richest protocol specimen, and its open question — can the
  minimum useful protocol live in existing artifact kinds? — is this
  idea's adoption gate too. Commissioning governs staged authority;
  protocols are the representation such a sequence would be written
  in. Neither absorbs the other.
- **Chores (idea 7).** A chore records that an obligation recurs and
  ledgers its performances; a protocol records how a performance is
  conducted. A chore may reference the protocol its performances
  follow.
- **Authority (ideas 15 and 22).** "Authority requirements" must
  reuse the affordance vocabulary of
  [[idea-capability-constrained-work|idea 15]] and the
  formulate-versus-authorize split of
  [[ide_01KY7R7CA8FNBRH3DFKFZW8V6J|idea 22]], not grow a parallel
  authority model. "Push requires Release protocol" is a statement a
  reader or agent consults, not an ACL Strata enforces.
- **Single-invocation commits (idea 9).** That flag would mechanize
  one protocol fragment; a protocol artifact is the citable
  description of the sequence the flag automates.
- **Amendments (idea 26).** Procedural evolution is the amendment
  story of [[ide_01KY7S6GMN26BFTEVGGKZHN4ZC|idea 26]] applied to a
  new kind; nothing protocol-specific is designed here.

Standing limits: protocols advise; Strata must not become a workflow
engine that blocks operations because a protocol says so, and doctor
checks at most well-formedness, never conformance. No collection work
is justified until protocols are being cited from real task results
or a procedural drift incident shows the prompt-file registry
failing. Prompt-file generation is strictly second-stage: it requires
the collection to exist and prove itself first.

## Evidence

Origin: Henry, 2026-07-25, in conversation, including the working
name, the checklist-to-state-machine trajectory, and the prompt-file
generation possibility. The corpus already contains executed
specimens: CLAUDE.md's workflow and commit-policy sections govern
every session, and the sprint 5 post-merge review executed a full
adversarial protocol end to end
([[ide_01KY7QF5FKX30PHTQ320MG4QXS|idea 21]] recovered its stages).
Prior art: operations runbooks and incident playbooks (SRE practice,
PagerDuty), aviation and surgical checklists with entry criteria and
abort conditions (Gawande), workflow/state-machine notations (BPMN,
statecharts) as the mature end of the checklist-to-state-machine
path, and literate configuration generators as precedent for
rendering enforced docs from canonical sources.
