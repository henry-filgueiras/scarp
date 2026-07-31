---
id: tsk_01KYTS3BZDRHEFVG0H5FBK4RW5
sequence: 47
kind: task
status: pending
sprint: spr_01KYTS35VYCFQ6BJN7HD2F0TK7
created: 2026-07-30
---

# Repair the crates.io quickstart anchor

## Objective

Make the README's one in-body same-page link work on the crates.io crate
page, where it is currently dead, without breaking it on GitHub, where
it currently works.

Found during [[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]]'s live-surface
inspection, after `0.1.0` was published. It is therefore frozen into
`0.1.0` and can only be fixed by a version number, which is why it is a
task rather than an opportunistic edit.

## The defect, as observed

The *See it work* section links `[quickstart](#quickstart)`.

crates.io's markdown pipeline prefixes every heading id with
`user-content-`, so the Quickstart heading is rendered as
`id="user-content-quickstart"`. It does **not** rewrite author-written
same-page hrefs to match, and it ships no client-side handler that maps
one to the other. The rendered README therefore contains a link to
`#quickstart` and no element with that id.

Verified on 2026-07-30 against the live page rather than inferred from
the HTML: clicking the link changed the URL to
`https://crates.io/crates/scarp/0.1.0#quickstart` and the viewport did
not move. The ten heading anchors crates.io generates for itself all
resolve, because it emits both the id and the href for those.

**It is not a defect on GitHub.** GitHub emits the same
`id="user-content-quickstart"`, and its own heading permalink also
points at `href="#quickstart"`; a client-side handler performs the
mapping. Confirmed against the live repository page the same day.

So the naive fix is wrong in an interesting way: changing the link to
`#user-content-quickstart` would repair crates.io and break GitHub,
trading one dead link for another.

## Acceptance criteria

- The link resolves for a reader on the crates.io crate page, verified
  by clicking it on the published page for whatever version ships the
  fix — not by reading HTML, and not by reasoning about the renderer.
- The link still resolves on the GitHub rendering of `README.md`,
  verified the same way. A fix that repairs one surface by breaking the
  other is not a fix.
- The chosen approach is recorded with its reasoning. At least these are
  considered, and the rejected ones say why: an absolute URL into the
  repository README anchor; removing the link and keeping the prose;
  restructuring so the two sections are adjacent and no link is needed.
  Duplicating the anchor id by hand is examined against crates.io's
  sanitiser rather than assumed to survive it.
- Whatever ships is checked on **both** surfaces after publication, and
  the check is recorded in a form the next release can repeat. This
  defect existed because a same-page anchor was verified against the
  source and against GitHub, and no live third surface existed yet.
- `README.md` is crate payload, so the full package reverification
  applies as in [[tsk_01KYK608A5Q5CAEPYYKW4YFQSH|task 46]] and
  [[tsk_01KYJG0S7SYMYY1FEG7H4QQX8G|task 44]]: package construction, file
  list, unpack outside the checkout, and `cargo publish --dry-run
  --locked`, with neither `--allow-dirty` nor `--no-verify`.
- If this ships as `0.1.1`, publication follows the same human-owned
  boundary task 45 established: an agent prepares the exact command and
  stops. `0.1.0` is not yanked for this, and nothing in the `0.1.0`
  record is rewritten to suggest it was clean.

## Notes

Batching is legitimate here. This is one dead link on one page, and
spending a version number on it alone is a poor trade; holding it until
another `0.1.1`-worthy change appears is a reasonable choice, and so is
shipping it immediately. That decision is Henry's and belongs in this
task's Result either way.

Related: [[ide_01KYK895PPE90CY8RAAFBV8B4P|idea 34]] proposes executing
the README quickstart as a test. It would not have caught this — it
tests that the commands run, not that the prose renders — which is worth
recording so the idea is not credited with coverage it does not have.

## Result
