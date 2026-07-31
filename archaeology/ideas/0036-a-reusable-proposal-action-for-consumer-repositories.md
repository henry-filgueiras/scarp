---
id: ide_01KYX31AG163NY0EQPCTXAQ066
sequence: 36
kind: idea
status: parked
created: 2026-07-31
---

# A reusable proposal action for consumer repositories

## Problem

Sprint 10 delivers a proposal channel — an authorized GitHub issue
realized as a canonical idea by Scarp — and deliberately delivers it as
a *recipe*: an Issue Form and a workflow file a reader copies into
their own repository, documented in
[[tsk_01KYX1WJ3P25528P5YTXJAJA4P|task 55]].

Copying has a known decay mode. Every copy forks: a security fix to the
authorization check, a correction to the injection handling, or a
change to Scarp's CLI reaches the original and none of the copies. The
people least able to notice are exactly the people who copied a
workflow they did not write in order to avoid understanding it.

## Sketch

Package the channel as something a consumer references rather than
copies — a composite action or a reusable workflow published from this
repository, referenced by tag:

```yaml
uses: henry-filgueiras/scarp/.github/actions/propose@v1
```

with the consumer supplying only what is genuinely theirs: the pinned
Scarp version, and whatever authorization policy differs from the
default.

The extraction should be cheap by construction. Sprint 10's workflow is
written to contain nothing specific to this repository beyond its name,
and its Scarp invocation is one command against a published binary, so
the difference between "our workflow" and "the action" is mostly
parameterization.

## Boundaries

- Publishing a versioned action is a maintenance commitment with a
  compatibility surface, and it is one this project should take on
  deliberately rather than drift into.
- It must not become the *only* way to use the channel. The copyable
  recipe stays, because a reader who wants to understand what runs in
  their repository should be able to, and because an action that
  disappears must not strand anyone — the same reasoning that keeps a
  Scarp repository editable without the Scarp executable.
- Not a marketplace listing, not branding, not a general-purpose
  "Scarp Actions" suite. One action, for one operation.
- The grant sprint 10 records is ideas-only; an action must not quietly
  widen it by accepting a collection parameter.

## Promotion trigger

Not before there is a second real consumer. With N=1 the recipe is
strictly better: it costs nothing to maintain, it is legible, and it
does not commit this project to a compatibility surface for a channel
whose shape is a few weeks old. Extraction is also much easier from a
workflow that demonstrably works than from a design.

Concretely: promote when at least one repository that is not this one
is running the channel, and either it or this project has had to
hand-propagate a change between copies.

## Evidence

Henry raised the consumer question during sprint 10's decomposition
(2026-07-31), before any of the channel was built, which is why the
sprint's workflow is written to be portable rather than retrofitted
later — see [[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]]'s portability
criterion. The install cost that makes a packaged action more
attractive is [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]]. Prior art:
`actions/cache`, `dtolnay/rust-toolchain`, and `taiki-e/install-action`
— all used by this repository's own CI, all things nobody copies
because referencing them is obviously better.
