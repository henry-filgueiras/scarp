---
id: ide_01KYZVJ6XCK11DP67GVMC3M23C
sequence: 38
kind: idea
status: parked
proposal: https://github.com/henry-filgueiras/scarp/issues/2
created: 2026-08-01
---

# Machine-readable repository capability manifest for agent collaboration

## Problem

Scarp is increasingly becoming something agents collaborate *through*, not merely a CLI humans invoke. Today, however, an agent has to learn a repository's operating contract indirectly: project prompts, CLAUDE.md, README prose, remembered conventions, or prior conversation. That makes correct collaboration depend on bespoke prompt context and on the agent already knowing which operations are allowed.

Sprint 10 exposes the concrete version of the problem. A remote conversational agent can now produce durable mutation intent through a GitHub issue, while canonical realization remains operator-driven. But nothing machine-readable tells an arbitrary competent agent that this repository accepts idea proposals, that proposals are non-canonical, that Scarp alone realizes canonical artifacts, or that mutation authority stops at a particular boundary.

As more optional surfaces appear — Git-aware commands, GitHub proposal realization, future collection-specific operations — relying on prose discovery risks recreating tribal knowledge for machines.

The useful question is not "how do we teach Lux/Claude/Codex this repository?" but:

> How does an arbitrary agent discover what this repository permits, requires, and refuses without bespoke prior prompting?

## Sketch

Expose a small, machine-readable capability manifest owned by Scarp/the repository and derivable from canonical configuration rather than handwritten duplication.

A possible conceptual shape:

```json
{
  "schema_version": 1,
  "tool": "scarp",
  "artifact_kinds": ["idea", "dragon", "decision", "task", "sprint"],
  "operations": {
    "idea": ["create", "park", "reject"],
    "task": ["create", "close"]
  },
  "integrations": {
    "git": "optional",
    "github": {
      "available": true,
      "proposal_kinds": ["idea"],
      "realization": "operator"
    }
  },
  "validation": {
    "command": "scarp doctor"
  },
  "authority": {
    "remote_proposals": "intent-only",
    "canonical_mutation": "explicit-operator"
  }
}
```

The exact schema is intentionally unsettled. The important property is that agents can query a stable surface such as `scarp capabilities --json` (or equivalent) and learn *affordances and authority boundaries*, while ordinary repository files remain canonical.

Prefer deriving the output from existing Scarp knowledge/configuration so the manifest does not become a second policy file that drifts from reality.

A future agent workflow could begin with capability discovery before attempting mutation, analogous to API feature negotiation rather than README scraping.

## Boundaries

- Not an agent protocol, MCP server, daemon, or long-running service.
- Not a replacement for human-readable documentation, CLAUDE.md, or project-specific guidance.
- Not permission enforcement by itself. A manifest describes available/allowed surfaces; the actual command/workflow must still enforce them.
- Not a generic plugin framework or speculative ontology of every action an agent might ever take.
- Do not expose arbitrary shell commands or make "capability" synonymous with "anything executable".
- Do not create a handwritten checked-in manifest if the same facts can be derived from existing canonical configuration or command surfaces.
- Git/GitHub-specific capabilities must remain optional; a plain Scarp repository must not require a forge.
- Schema versioning should be explicit before external consumers depend on it, but compatibility machinery should wait until there is a real second consumer.

## Evidence

Sprint 10 is the forcing case. The remote-proposal design repeatedly had to distinguish transport, mutation intent, canonical realization, and authority. Those distinctions currently live in sprint/task prose and project instructions rather than a queryable contract.

Idea 37 explores first-class GitHub integration while deliberately keeping `gh` optional and bounded. A capability surface would let an agent discover that integration when present without assuming GitHub is universal.

The desire path has also appeared directly in design discussion: instead of maintaining a "Lux-specific Scarp prompt," the repository should ideally teach any competent agent how it can be collaborated with. That would reduce prompt folklore and make future agent integrations depend on explicit software interfaces rather than model-specific memory.

This should remain parked until at least one concrete agent workflow would consume it. Sprint 10 may provide that evidence; the idea is not a reason to widen Sprint 10 itself.
