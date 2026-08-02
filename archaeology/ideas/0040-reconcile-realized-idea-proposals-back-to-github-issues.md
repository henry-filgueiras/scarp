---
id: ide_01KYZXGDY8YAFXMP1FV931ZB0M
sequence: 40
kind: idea
status: parked
proposal: https://github.com/henry-filgueiras/scarp/issues/3
created: 2026-08-01
---

# Reconcile realized idea proposals back to GitHub issues

## Problem

Sprint 10's Option B deliberately makes GitHub Issues the durable remote proposal surface while keeping canonical realization on the operator's trusted machine. That solves the authority problem cleanly, but it leaves the lifecycle visibly incomplete: after an issue is realized into a canonical Scarp idea, the originating GitHub issue remains open unless a human manually notices, finds the new artifact, comments with the durable reference, and closes the proposal.

That manual cleanup is not canonical mutation and carries much less authority than realization itself. The repository already knows the truth after the idea lands on `main`; GitHub is merely stale.

The asymmetry matters:

- GitHub -> Scarp realization should remain explicit and operator-authorized.
- Scarp -> GitHub reconciliation can potentially be automated because it only reports canonical state that already exists.

Without a reconciliation step, proposal issues accumulate even after they have served their purpose, and a later reader cannot tell at a glance whether an open proposal is pending, abandoned, or already represented in canon.

## Sketch

Add a small repository-owned reconciliation workflow that runs after changes land on `main` and detects newly-added canonical ideas that explicitly identify the GitHub proposal they realized.

A possible flow:

```text
proposal issue #42
    ↓
operator realizes locally
    ↓
canonical idea committed and pushed normally
    ↓
main changes
    ↓
reconciliation workflow inspects newly-added ideas
    ↓
find explicit proposal provenance
    ↓
comment on issue with durable artifact + commit reference
    ↓
close issue as completed
```

The workflow should never infer linkage by title similarity or prose matching. Realization should leave an explicit machine-readable-enough provenance marker in the canonical artifact's authored prose or another already-supported durable surface, for example conceptually:

```text
GitHub-Proposal: henry-filgueiras/scarp#42
```

The exact representation should follow Scarp's existing provenance conventions rather than invent hidden front matter that doctor ignores.

The reconciliation invariant should be:

> A proposal issue is closed only because canonical `main` contains an artifact explicitly claiming realization of that exact proposal.

The GitHub comment should make the authority relationship obvious, e.g. identify the realized `idea:<sequence>` / stable ID, link the artifact and landing commit, and state that the Scarp artifact is now the canonical record.

Idempotency should be boring:

- already-closed issue -> no-op;
- reconciliation comment already present -> no duplicate comment;
- workflow re-run -> no duplicate side effects;
- malformed/missing provenance -> do nothing and report/refuse clearly;
- multiple newly-landed ideas -> reconcile independently.

## Boundaries

- This is not issue-to-artifact synchronization. GitHub does not remain authoritative after realization, and later issue edits never mutate canon.
- The workflow must not create, edit, reject, or otherwise mutate canonical archaeology.
- The workflow must not realize proposals itself. GitHub -> Scarp remains operator-driven under Sprint 10 Option B.
- No heuristic matching by title, body similarity, timestamps, or sequence proximity.
- No requirement for a generic event bus, daemon, MCP service, or external webhook endpoint.
- Prefer a post-merge/push-to-main observation of established canonical truth rather than coupling reconciliation into the local realization command unless evidence shows that is simpler and equally reliable.
- Do not add opaque provenance front matter that Scarp does not understand merely to make automation easy.
- Keep this ideas-only until another proposal kind has a real consumer and equivalent lifecycle semantics.

## Evidence

The desire path appeared immediately after exercising Sprint 10's first real remote proposal. A structured idea issue can now be filed remotely and realized locally, but the original proposal remains open after the durable artifact lands unless a person performs cleanup by hand.

This is exactly the kind of friction the project has been using as promotion evidence: perform the workflow manually, observe which repeated step is mechanical, then automate only that step.

The security shape is also materially cheaper than automated realization. By the time reconciliation runs, the canonical mutation has already passed through the existing trusted local path and landed on `main`. The workflow's authority is limited to updating GitHub's derived view of that fact — commenting and closing an issue — rather than writing repository contents.

The pattern may also be a useful concrete instance of a broader principle: automation that projects established canonical truth outward can be granted more freely than automation that changes canon. This idea should stay narrow and prove that principle with the proposal lifecycle before generalizing it.
