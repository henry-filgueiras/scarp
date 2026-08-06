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
`cargo publish`, the tag push, the GitHub release, and the credential
handling. The runbook encodes that split by marking every mutating cell
`excludeFromRunAll` and forbidding `run --all` operationally — an
exclusion keeps a cell out of a sweep, it does not make the cell harder
to run on purpose, and the runbook says so rather than implying a safety
catch it does not have.

Closure is the same handoff in reverse: Henry captures the release
provenance, and Claude writes the `Result` and performs the transitions.
The runbook deliberately has no closure cell, because a cell named
`close` that exits zero without closing anything is a lie in the shape
of a command.

This item subsumes [[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]], which asked for a version bump
before the next publish; close it here rather than separately.

### Proposed version: 0.3.0

Prior releases recorded the version as a judgment. This one is closer to
forced. Scarp ships a library as well as a binary, and pre-1.0 the minor
position is where Cargo's SemVer rules put a breaking change. Multiple
public surfaces broke, including:

- `read::Summary.status` changed from `Status` to `Option<Status>`, so
  stateless collections carry no lifecycle state;
- `transition::close_sprint` gained a terminal-narrative parameter, and
  `proposal::realize` gained a third parameter selecting the owning
  sprint;
- `read::Collection` gained a public `terminal` field,
  `cli::Command::Close` gained a public `body_file` field,
  `cli::ProposalCommand::Realize` gained a public `sprint` field, and
  `proposal::ProposalSummary` gained a public `target` field;
- three variants were added to `cli::Collection` (`Log`, `Principle`,
  `Maintenance`), one to `cli::ProposalCommand` (`Reconcile`), and one to
  `error::Error` (`PreconditionUnmet`).

The last two categories are breaking only because nothing here is
`#[non_exhaustive]`: a downstream `match` over `Error` or a struct
literal for `Collection` stops compiling. That is worth stating rather
than waving at, because it is also the cheapest thing to fix before 1.0
and nobody has decided to.

Purely additive beside those: `artifact::create_maintenance_from` and
`artifact::create_task_from` joined `create_idea_from`, and
`artifact::create_maintenance` and `create_task` now delegate to them
with unchanged signatures and behavior.

So 0.3.0 is what the library requires, and the accumulated command
surface says the same thing independently:

| Change | Surface |
|---|---|
| `log`, `principle`, `maintenance` collections | three new collections across `new`, `list`, `show`, `close`, `doctor` |
| `scarp close --body-file` | new CLI flag; the terminal narrative lands in the same atomic write as the transition |
| sugar binding at every write boundary | `[[kind:N]]` in authored bodies is bound to `[[stable-id\|label]]`, and unresolvable sugar refuses the write |
| `scarp proposal reconcile` | new command surface, closing a landed proposal on GitHub |
| `scarp proposal realize --sprint` | a second proposal source class: a `bug`-labeled issue realizes a maintenance item, or a task in a named active sprint |
| `precondition-unmet`, exit 12 | new entry in the error contract |

Nothing was removed from the CLI, and the `--json` contract is
deliberately additive: `status` is omitted rather than emitted as null
for the stateless collections, so every stateful collection's output is
unchanged byte for byte.

The behaviour claim has to be narrower than that, though. **One existing
invocation does change.** `new --body-file` shipped in 0.2.0 copying the
body through verbatim; it now binds resolvable `[[kind:N]]` sugar to
`[[stable-id|label]]` and refuses the write outright when the sugar
resolves to nothing or to more than one artifact. A body that 0.2.0
wrote through unchanged may now be rewritten, and one citing a
nonexistent artifact that 0.2.0 accepted is now rejected before a
sequence is allocated. Both are the intended repair — the alternative is
canonical prose carrying markers that were never checked — but they are
behaviour changes to an unchanged command line, not additions beside it.

The version number's only audience is someone deciding whether to look,
and 0.2.1 would tell them nothing happened.

### What the release owes beyond publishing

- `--version` must distinguish a binary that knows sprint 12's
  collections from one that does not. That is [[mnt_01KZ7A8KPX088RA46TKXG65N7G|Bump the version before the next publish]]'s whole
  complaint: [[tsk_01KZ738BNX70HQWFCBYV8CF9F1|Validate the consumer affordances in WitnessGlass]] had to test compatibility behaviourally, by
  probing for `unknown collection`, because the obvious identifier
  lied.
- The shipped binary — installed from crates.io, not built here — must
  create a log, a principle, and a maintenance item, close one with
  `--body-file`, and expose `proposal realize --sprint`. A surface that
  only works from `cargo run` in this checkout did not ship.
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
