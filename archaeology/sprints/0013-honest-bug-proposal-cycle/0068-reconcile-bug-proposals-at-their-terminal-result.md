---
id: tsk_01KZCCE4PDVWPMM6VY1XC0X3FJ
sequence: 68
kind: task
status: closed
sprint: spr_01KZCCCPGW3V959HBDTZC56JAE
created: 2026-08-06
closed: 2026-08-06
---

# Reconcile bug proposals at their terminal result

## Objective

Generalize proposal lookup and reconciliation across ideas, maintenance,
and tasks, and gate the bug-derived half on a stronger invariant than the
one ideas need.

For an idea, default-branch existence remains sufficient: the idea is the
deliverable. For a maintenance item or a task, the filer is waiting on an
outcome, so reconciliation must prove the work reached its terminal
result before saying anything public.

The invariant is deliberately stronger than "the path exists remotely".

## Acceptance criteria

- Idea reconciliation is unchanged: creation-aware, citing the commit
  that introduced the artifact.
- For a maintenance item or task, reconciliation fetches the remote
  default-branch artifact, parses it, and proves the matching stable id,
  `kind`, `proposal:` URL, and `status: closed` are all present.
- A locally closed artifact whose remote copy is still pending refuses
  with `precondition-unmet` / exit 12, and neither comments nor closes.
- Remote contents are authoritative. The local status, a remote-tracking
  ref, a substring search, and mere path existence are each insufficient
  and none of them is relied on.
- The permalink and cited commit for a bug point to a revision proven to
  contain the terminal state, not merely the commit that introduced the
  pending file.
- Invalid, mismatched, absent, or nonterminal remote contents never
  produce a GitHub mutation.
- Reconciliation enforces the same exactly-one-recognized-label rule as
  realization.
- The recovery ordering is preserved: prove canonical remote state, then
  comment unless the reconciliation marker is already present, then
  close. A crash after commenting stays recoverable without posting
  twice, an already-closed recognized issue remains a no-op, and nothing
  is ever reopened or synchronized.
- The bug comment reads well cold: it cites the reference, stable id,
  pinned artifact path, and the proven terminal commit, and says the work
  "reached its terminal result". It never says "fixed", because the
  `Result` may conclude that no defect existed.
- Tests cover unchanged idea reconciliation; pending remote maintenance
  and pending remote task; local closed with remote pending; remote
  closed with matching identity and provenance; remote content mismatch
  and malformed front matter; comment-before-close ordering;
  commented-but-open recovery; already-closed idempotence; both and
  neither recognized label; wording that claims a terminal result but not
  a fix; and a pinned commit that actually contains the terminal state.
- Tests use the existing pure-planning seam and a fake `gh` where a
  process boundary is needed. No unit test depends on the network.
- `scripts/check.sh` passes and the slice is committed with its
  archaeology.

## Result

Done. Reconciliation now generalizes across ideas, maintenance, and
tasks, and the bug half waits for a terminal result it proves rather than
assumes.

**The invariant, precisely.** For an idea, default-branch existence is
still the whole story. For a maintenance item or a task, reconciliation
fetches the default branch's copy of the artifact, parses its front
matter, and requires the stable id, `kind`, `proposal:` URL, and
`status: closed` to all be present and to match. Anything less refuses.

Four things it deliberately does not rely on, each of which would have
been cheaper and wrong:

- the **local status**, which says only what this disk believes — this is
  the exact case a local check waves through, and the one the headline
  test pins;
- a **remote-tracking ref**, which answers a question about this
  machine's last fetch;
- a **substring search**, which cannot tell `status: closed` in front
  matter from the same words quoted inside a `Result`. There is a test
  filing precisely that body;
- **mere path existence**, which was sufficient for an idea and is not
  sufficient here.

**The cited commit changed, and had to.** A bug artifact arrives
`pending` and becomes `closed` later, so the introducing commit is a
revision that contradicts the sentence the permalink sits beside.
`observe_remote` picks the *newest* commit touching the path for a
terminal claim and the *introducing* one for an idea, then re-fetches the
file at that exact revision and proves the terminal state there. It also
compares that copy against the default-branch copy: they must be
identical, since no later commit touched the path, and a difference means
the branch moved mid-run — a reason to stop, not to pick one. Three API
reads where ideas need two.

**Shape.** The remote observation became a value — `Remote::Absent`,
`Proven`, `Unproven { observed, remedy }` — so `plan` stays pure and every
refusal ordering is decided in one place with no network. `Unproven`
carries its own diagnosis because the interesting failures are all
different sentences: still pending, wrong id, wrong kind, wrong proposal,
unreadable front matter, branch moved. `prove` is a separate pure
function over the raw file contents.

Contents are fetched with `Accept: application/vnd.github.raw`, so the
bytes arrive as bytes and no base64 decode sits between GitHub's answer
and the proof drawn from it.

**Wording.** `comment_body` branches on class. The idea comment is
byte-identical to before. The bug comment says the work "reached its
terminal result", cites the reference, stable id, pinned path, and the
proven terminal commit, and then spends a paragraph saying what it is
*not* claiming: an investigation can end in a confirmed defect, in
behavior that turned out to be intended, in a report nobody could
reproduce, in a duplicate, or in a considered decision not to act. A test
asserts the rendered body contains none of `fix`, `resolved`, `the bug`,
or `confirmed the`. The `Result` this comment points at may conclude that
no defect existed, and nothing ever comes back to correct a comment.

Recovery ordering is unchanged and now asserted end to end rather than
only in the planner: prove, comment unless the marker is already there,
close.

## Verification

`scripts/check.sh` — passes. 284 lib tests, doctor 153 artifacts clean.

Ten new unit tests in `src/proposal.rs`: `prove` over a matching closed
artifact, a pending one, id/kind/proposal mismatches, unreadable content,
and the quoted-substring body; `newest_commit` beside
`introducing_commit`; the `Unproven` refusal and its distinctness from
`Absent`; the dual-label refusal; and the comment's wording.

Ten new integration tests in `tests/proposal_cycle.rs`, all through the
compiled binary against the fake `gh`, which logs every invocation and
every mutation separately. That second log is what makes the important
assertion possible: **every refusal test asserts `mutations()` is
empty.** Covered: unchanged idea reconciliation citing the introducing
commit; local-closed/remote-pending; remote closed for both a maintenance
item and a sprint task, each asserting the cited commit is the terminal
one and that the pinned revision was actually fetched; remote identity
mismatch; malformed remote content; absent remote artifact; the
commented-but-open recovery closing without commenting twice;
already-closed idempotence; and both/neither recognized label.

The harness proves the sequence of `gh` calls Scarp makes, not that
GitHub answers them as assumed. Nothing here is live evidence.
