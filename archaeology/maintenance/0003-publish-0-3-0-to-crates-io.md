---
id: mnt_01KZA6MH5SCW0MDEJTKKW26Y9G
sequence: 3
kind: maintenance
status: pending
created: 2026-08-05
---

# Publish 0.3.0 to crates.io

## Work

Publish current `main` to crates.io as the next Scarp release, following
`docs/release-runbook.md`, which was written for this item and is the
first release performed from a repo-tracked procedure rather than from
the prior release's task record.

Publication is human-owned per
[[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|Publish and verify v0.1.0]]'s boundary: an agent prepares and verifies, Henry runs
`cargo publish`, the tag push, and the credential handling. The runbook
encodes that split — the irreversible cells are marked
`excludeFromRunAll`, so a `runme run --all` cannot publish.

This item subsumes [[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]], which asked for a version bump
before the next publish; close it here rather than separately.

### Proposed version: 0.3.0

Prior releases recorded the version as a judgment. This one is closer to
forced. Scarp ships a library as well as a binary, and pre-1.0 the minor
position is where Cargo's semver rules put a breaking change. Two broke:

- `read::Summary.status` changed from `Status` to `Option<Status>`, so
  stateless collections carry no lifecycle state;
- `transition::close_sprint` gained a terminal-narrative parameter.

So 0.3.0 is what the library requires, and the accumulated surface says
the same thing independently:

| Change | Surface |
|---|---|
| `log`, `principle`, `maintenance` collections | three new collections across `new`, `list`, `show`, `close`, `doctor` |
| `scarp close --body-file` | new CLI flag; the terminal narrative lands in the same atomic write as the transition |
| sugar binding at every write boundary | `[[kind:N]]` in authored bodies is bound to `[[stable-id\|label]]`, and unresolvable sugar refuses the write |
| `scarp proposal reconcile` | new command surface, closing a landed proposal on GitHub |
| `precondition-unmet`, exit 12 | new entry in the error contract |

Nothing was removed from the CLI and no existing invocation changes
behaviour. The `--json` contract is deliberately additive: `status` is
omitted rather than emitted as null for the stateless collections, so
every stateful collection's output is unchanged byte for byte.

The version number's only audience is someone deciding whether to look,
and 0.2.1 would tell them nothing happened.

### What the release owes beyond publishing

- `--version` must distinguish a binary that knows sprint 12's
  collections from one that does not. That is [[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]]'s whole
  complaint: [[tsk_01KZ738BNX70HQWFCBYV8CF9F1|Validate the consumer affordances in WitnessGlass]] had to test compatibility behaviourally, by
  probing for `unknown collection`, because the obvious identifier
  lied.
- The shipped binary — installed from crates.io, not built here — must
  create a log, a principle, and a maintenance item, and close one with
  `--body-file`. A surface that only works from `cargo run` in this
  checkout did not ship.
- Cold-install timing goes to [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|Prebuilt release binaries for CI and non-Rust consumers]], with its usual caveat.
- The crates.io rendering is checked in a browser, anchors included.
  [[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|Repair the crates.io quickstart anchor]]'s defect was invisible on GitHub and dead on crates.io,
  and this release adds no new same-page anchors but does add README
  prose and two diagram nodes.

### Runbook, not transcript

The runbook is repo-tracked and executable, which is a change in kind
from the previous two releases: 0.1.0 and 0.2.0 both recorded their
commands as dated provenance inside the task that performed them, per
CLAUDE.md's first-performance policy. Three performances is where that
policy expects a durable form to appear.

It is still not automation. Nothing runs unattended, nothing holds a
credential, and the cells exist so the human running them reads output
between steps. If a step turns out to be wrong, repair
`docs/release-runbook.md` in the same change as the Result — that
repair is the difference between a runbook and a transcript.
