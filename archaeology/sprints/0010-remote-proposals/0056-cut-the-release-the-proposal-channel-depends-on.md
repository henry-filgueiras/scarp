---
id: tsk_01KYX31ACH05NGA3GYH0TJA870
sequence: 56
kind: task
status: closed
sprint: spr_01KYX1WAD7CC0RHVZY0V7VE4X1
created: 2026-07-31
closed: 2026-08-01
---

# Cut the release the proposal channel depends on

## Objective

Publish a Scarp release carrying [[tsk_01KYX1WHWDG6DBCXBQH2J7YJWN|task
51]]'s `--body-file` surface, and whatever else has earned a version
number by then.

*Removed from the critical path 2026-08-01 by
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s Option B adjudication.*
This task was created because Option A's workflow would have installed a
pinned published `scarp` in CI, making an unreleased flag an unusable
one. Option B has the operator run the Scarp they already have, so
nothing in the sprint is blocked by publication and
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] no longer waits on this.

The release keeps its own independent reasons, which are not
manufactured for this sprint:

- `--body-file` is shipped, tested, and useful to anyone who installs
  Scarp; leaving it unreleased means the published tool cannot do the
  thing this sprint is about.
- [[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|Task 47]] in sprint 9 holds a
  cosmetic defect frozen into `0.1.0`, and its notes say explicitly that
  it is waiting for another release-worthy change to batch with. That
  reason predates this sprint and survives it.

Ordering is therefore free. This task may run at any point, or slip past
the sprint entirely without blocking anything.

## Relationship to sprint 9

Batching [[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|task 47]] in is Henry's call,
recorded in task 47's own Result either way. What this task owes is the
coordination: ask the question before publishing rather than after,
since after is a version number too late.

## Acceptance criteria

- The release carries task 51's `--body-file` surface, and its version
  number is chosen deliberately — whether an additive CLI surface is a
  minor bump or rides a patch is a judgment recorded here, not a
  default.
- Task 47's batching question is put to Henry before publication and its
  answer recorded, in task 47's Result if it ships and here either way.
- The full package reverification applies as in
  [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]] and
  [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]]: package construction, file
  list, unpack outside the checkout, and `cargo publish --dry-run
  --locked`, with neither `--allow-dirty` nor `--no-verify`.
- Publication follows the human-owned boundary
  [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] established: an agent
  prepares the exact command and stops.
- The published binary is verified to provide `--body-file` by
  installing it from crates.io into a clean environment and running it,
  rather than trusting that what was tested locally is what shipped.
- The install is timed in that clean environment, because that number is
  the evidence [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]] needs.
- Nothing in the `0.1.0` record is rewritten to suggest it was clean.
- Per CLAUDE.md's first-performance policy, any non-obvious invocation is
  recorded as dated provenance. The recurring form of a release remains a
  chore ledger ([[idea-chore-artifacts|idea 7]]), not a script.
- The Result states plainly that this release was **not** a sprint 10
  dependency, so a later reader does not infer that Option B needed a
  publication.

## Prepared, awaiting the version decision (2026-08-01)

Everything an agent may do is done; publication and the version number
are Henry's, per [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]'s boundary.

**Verified today:** `cargo publish --dry-run --locked` succeeds with
neither `--allow-dirty` nor `--no-verify`, packaging 38 files
(616.6 KiB, 138.0 KiB compressed) from a clean worktree.

**What has accumulated since `0.1.0`:**

| Change | Surface |
|---|---|
| `scarp new --body-file` | new CLI flag on every narrative collection |
| fenced-code-block awareness in the body parser | fixes a refusal of legitimate input |
| `proposal:` managed front-matter field | new artifact-model vocabulary |
| `duplicate-proposal` doctor finding | new validation |
| `scarp proposal list` / `realize` | new command surface |
| `integration-unavailable`, exit 11 | new entry in the error contract |
| README status table, `docs/remote-proposals.md` | packaged README changes |

**The version question.** This is additive — nothing removed, no
behaviour changed for an existing caller — so semver permits a patch.
Against that: it adds a command surface, an artifact-model field, and an
exit code, which is more than "0.1.0 with a fix" communicates to someone
reading the changelog. Recorded as a judgment rather than defaulted.

**Task 47's batching question**, which its own notes ask to be put
before publication: the crates.io quickstart anchor defect is still
unrepaired, and this is the release it was waiting for. Repairing it
first is cheap; shipping without it means it waits for another version
number.

Note that this release re-runs task 47's risk in a new place: the README
gained a link into `docs/`, which is **not** in the crate's `include`
list. It was written absolutely (`https://github.com/.../blob/HEAD/...`)
rather than relatively for exactly that reason, matching the three
`archaeology/` links already in the file. Whoever ships this should
click it on the published crate page, per task 47's acceptance criteria.

**The exact commands**, to be run by a human:

```sh
# 1. set the version in Cargo.toml, then:
cargo update -p scarp            # refresh Cargo.lock to match
scripts/check.sh
cargo publish --dry-run --locked # must pass with no extra flags
cargo publish --locked
git tag -a vX.Y.Z -m "scarp vX.Y.Z"
```

Post-publication verification, per
[[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]] and
[[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]]: install from crates.io into a
clean container, run `scarp new idea --body-file` to confirm the shipped
binary carries the surface, time the cold install for
[[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]], and click the README's
`docs/` link on the live crate page.

## Result

`scarp 0.2.0` published 2026-08-02T00:05:54Z from `f770a54`, tagged
`v0.2.0`, tag pushed. Confirmed live against the crates.io API: not
yanked, and `0.1.0` untouched beside it.

### The version judgment

**0.2.0, not 0.1.1.** Everything since `0.1.0` is additive — nothing
removed, no behaviour changed for an existing caller — so semver
permitted a patch. It was declined because the release adds a command
surface (`scarp proposal list` / `realize`), a managed front-matter
field (`proposal:`), a doctor finding (`duplicate-proposal`), a CLI flag
(`--body-file`), and an entry in the error contract
(`integration-unavailable`, exit 11). A patch number would undersell
that to the only audience a version number has: someone deciding whether
to look. Henry's call, 2026-08-01.

### Task 47 batched in

Batched per Henry — "not worth an extra publishing raindance just for
that" — which is the disposition
[[tsk_01KYTS3BZDRHEFVG0H5FBK4RW5|task 47]]'s own Notes anticipated. It
is now closed, verified on both live surfaces, and sprint 9 closed with
it.

### Verification

Pre-publication, from a clean worktree, with neither `--allow-dirty` nor
`--no-verify`: `cargo publish --dry-run --locked` passed; 38 files,
138.0 KiB compressed. The `.crate` was unpacked outside the checkout and
`archaeology/`, `.scarp.toml`, `docs/`, `scripts/`, `.github/`, and
`CONTRIBUTING.md` were each confirmed **absent**. 410 tests pass from
the unpacked source alone. `.cargo_vcs_info.json` names `f770a54`,
matching `HEAD`.

Post-publication, installed from crates.io into an empty `CARGO_HOME`
and run there — not trusting that what was tested locally is what
shipped:

- `scarp --version` reports `0.2.0`;
- the **shipped** binary carries `--body-file` and the `proposal`
  subcommand;
- it created an idea from a body file containing a fenced block with a
  `#` comment, proving the fence fix is in the published build — that
  input was refused by `0.1.0`;
- `scarp doctor` green;
- `scarp proposal list` in a non-GitHub directory refused with
  `integration-unavailable`, exit 11.

**Cold install: 6 seconds wall clock**, empty `CARGO_HOME`, 18-core
laptop. Recorded for [[ide_01KYX31AE8WX1HMBFNRZ3XQK4V|idea 35]] with its
caveat: that is a fast machine with a warm OS-level network path, and a
CI runner with fewer cores pays proportionally more. It is a data point,
not a bound.

### Not a sprint dependency

Stated plainly so a later reader does not infer otherwise: **Option B
never needed this release.** The dependency existed only under Option A,
whose workflow would have installed a pinned published `scarp` in CI.
Under the chosen design the operator runs whatever Scarp they have, and
[[tsk_01KYX1WJ1XA2W1SWJYV96R3Y8H|task 54]] was completed and verified
against a local build days before this shipped. The release happened
because `--body-file` deserved publishing and task 47 was waiting — both
reasons that predate and outlive this sprint.

### Provenance

```sh
cargo update -p scarp            # after bumping Cargo.toml
scripts/check.sh
cargo publish --dry-run --locked
cargo publish --locked
git tag -a v0.2.0 -m "scarp v0.2.0"
git push origin main --follow-tags
```

Publication and tagging were performed by Henry, per
[[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]'s boundary: an agent prepares
the exact command and stops. Everything above it was prepared and
verified by an agent.
