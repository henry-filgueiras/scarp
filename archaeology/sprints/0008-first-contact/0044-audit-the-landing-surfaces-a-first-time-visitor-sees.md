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

This is the **pre-publication** audit. It covers the final package
and README source and every surface that can be checked before a live
page exists. Inspection of the rendered crates.io page and the
docs.rs build belongs to [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]],
because neither exists until publication has happened.

### Suspects to resolve, verdicts not assumed

Each was observed on 2026-07-27 and is recorded here so the audit
starts from evidence rather than from scratch. None is a finding yet:
the audit may conclude a given wording is defensible. What it may not
do is leave one unexamined.

- **The README's "real output" is stale.** It shows `list dragons`
  with dragons 1, 2, and 3 all `open`; dragons 2 and 3 are now
  `closed`, and dragon 4 (power-loss durability) is absent entirely.
  The `jq '.[-1]'` example claims to show dragon 3, but the last
  element is now dragon 4. Prefer replacing corpus-dependent
  showcase output with output from the deterministic quickstart
  fixture: another snapshot of the live corpus would be accurate on
  the day it was pasted and wrong again within a sprint. Note also
  that the example depends on `jq`, which task 43 bars from the
  quickstart for the same reason it is questionable here.
- **The README presents Markdown, JSON, and JSONL as current payload
  formats**, though only Markdown CRUD ships.
  [[dec-bootstrap-payload-separation|Decision 3]] makes the multi-format
  architecture real as a design boundary, but the README's framing
  should not read as shipped capability.
- **"Safe writes", the hero's coding-agent framing, and the Cargo
  description** must be reconciled with task 41's retired claims and
  with open [[drg-bootstrap-branch-collisions|dragon 1]] (branch
  sequence collisions) and
  [[drg_01KY3C0S3JQKEMEB9BH6NVJ35F|dragon 4]] (power-loss durability
  of mutations). The hero currently reads "structured repository
  memory for humans and coding agents", close to the headline claim
  task 41 retired.
- **`CONTRIBUTING.md` names `archaeology/dragons/open/`**, a
  directory that does not exist — placement has been flat since
  decision 11 as amended. Confirmed 2026-07-27: `archaeology/dragons/`
  contains four files and no subdirectories.
- **`SECURITY.md` promises GitHub private vulnerability reporting,
  which is disabled.** `gh api
  repos/henry-filgueiras/scarp/private-vulnerability-reporting`
  returned `{"enabled":false}` on 2026-07-27. A security policy that
  directs people to a reporting channel that does not exist is a
  **release blocker**: it fails exactly when it matters, and
  publication is what brings strangers who might use it. Resolve it
  either by preparing the exact human-owned enablement command — the
  REST surface is `PUT
  /repos/{owner}/{repo}/private-vulnerability-reporting`, requiring
  repository admin and returning 204 on success — or by rewriting
  `SECURITY.md` to a reporting path that is actually open. **Do not
  silently execute the repository-setting mutation**; it is an
  external GitHub change and therefore Henry's.
- **`.github/ISSUE_TEMPLATE/idea.md` requests a nonexistent `idea`
  label.** `gh label list` on 2026-07-27 returned only the nine
  GitHub defaults: `bug`, `documentation`, `duplicate`,
  `enhancement`, `good first issue`, `help wanted`, `invalid`,
  `question`, `wontfix`. Either plan a human-owned label creation or
  deliberately use an existing label; a template referencing a label
  that does not exist applies no label at all.
- **GitHub detects a single license.** `gh api repos/…/license`
  reports Apache-2.0 from `LICENSE-APACHE`, and the repository's
  license badge says "Apache License 2.0", while
  [[dec-dual-mit-apache-licensing|decision 9]] establishes dual
  MIT/Apache-2.0. Confirm the README, the manifest's `license`
  field, the packaged files, and the repository's first screen make
  the actual dual contract unmistakable to a human reader. Do not
  contort source files merely to satisfy GitHub's detector — the
  detector's opinion is not the license.
- **GitHub About metadata** is inspected read-only: description,
  homepage, and topics. As of 2026-07-27 the description is decision
  16's positioning line, homepage is empty, and twelve topics carry
  no stale product name. Any change required is an external mutation
  and gets an exact operator runbook plus subsequent verification,
  not a silent edit.

## Acceptance criteria

- The audited surface list is explicit and complete, covering at
  least: `README.md` including the hero, positioning line, feature
  claims, status scoreboard, and the new quickstart; the packaged
  README source as it will be shipped; the GitHub About panel —
  description, homepage, topics — read only; `CONTRIBUTING.md`,
  `SECURITY.md`, `CODE_OF_CONDUCT.md`, and the issue and
  pull-request templates; the light and dark wordmarks; and the
  repository's own first screen. Any surface deliberately excluded is
  named with its reason.
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
- Every suspect listed above reaches a recorded verdict, including
  the ones the audit decides are fine. "Examined and defensible" is a
  finding; silence is not.
- The two external blockers — private vulnerability reporting and the
  missing `idea` label — leave this task either resolved in prose or
  carrying an exact human-owned runbook for Henry. Neither is
  executed here, and publication does not proceed with a security
  policy pointing at a closed channel.
- Live-surface verification is explicitly **out of scope** and
  deferred to [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]: the
  rendered crates.io page, the docs.rs build, and the installed
  package do not exist yet, and asserting anything about them here
  would be prediction rather than verification.
- Anything found that is real but out of scope becomes an idea or a
  dragon rather than an unrecorded observation or an opportunistic
  fix.
- `scripts/check.sh` passes, and the work is committed per the commit
  policy in `CLAUDE.md`.

## Result
