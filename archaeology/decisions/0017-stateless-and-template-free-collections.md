---
id: dec_01KZ74BQQJ1W5Q32GQS7RD4JCK
sequence: 17
kind: decision
status: accepted
created: 2026-08-04
---

# Stateless and template-free collections

## Context

Every managed collection until now declared at least one lifecycle state
and at least one template section, and the machinery treated both as
universal: `FrontMatter` required `status:` of every artifact, and
`Body::parse` refused any `## ` heading outside the collection's
template.

Adopting `archaeology/logs/` falsified both assumptions against a corpus
that predates the tool by eleven sprints.

**No status.** The three logs written before adoption carry `id`,
`sequence`, `kind`, and `created`, and nothing else. They are otherwise
fully conformant — filenames agree with sequences, titles extract, ids
are unique. The available moves were to stamp a synthetic state into
three canonical files so the parser would accept them, or to admit that
a collection can have no lifecycle. The first invents a lifecycle to
suit the implementation and rewrites history to make the tool's job
easier, which is the inverse of preserve-history; a log records
something that already happened and is not in any state.

**No template.** The same three logs share no section vocabulary. Log 1
is unbroken prose beneath its title; log 2 uses `Rejected alternatives`
and its siblings; log 3 uses `The shape`, `The four instances`, `What
changed`, `What is still exposed`. There is no template to derive, and
inventing one would have made every existing log non-conformant to the
collection it belongs to.

## Decision

A `Collection` with an **empty `states` list is stateless**, and a
`Collection` with an **empty section list is template-free**. Both are
ordinary collection data, not special cases in the machinery.

For a stateless collection:

- artifacts carry no `status:` front-matter line, and creation writes
  none;
- a `status:` line on such an artifact is a **malformed-artifact
  finding**, not an ignored field. Tolerating it would let a lifecycle
  vocabulary no code admits accumulate silently, and the whole reason
  state lives in front matter (decision 11) is that front matter is the
  one authority — an authority that may not carry claims the collection
  denies;
- `Summary::status` is `None`, and the `status` key is **omitted** from
  JSON projections rather than emitted as null, so every stateful
  collection's `--json` output is byte-identical to what it was before
  statelessness existed;
- the human `list` projection drops the status column entirely rather
  than printing a constant in it;
- every lifecycle verb refuses the collection with truthful guidance,
  which falls out of an empty transition table and needs no new code —
  the behaviour `decision` already had.

Symmetrically, `status:` is **required** of every collection that
declares states. It had been enforced by the deserializer, which made a
missing status an opaque serde error; it is now a checked invariant with
a message naming the collection's vocabulary.

For a template-free collection, `--body-file` content is written
**verbatim** beneath the title. Scarp owns no headings in such an
artifact, so it polices none: there is no section vocabulary to violate
and no template order to disturb. The guards that are not about sections
all still apply — the size limit, the control-character refusal, CRLF
normalization, and the refusal of a level-1 heading that would compete
with the title, which is fence-aware because title extraction is.

## Consequences

- Logs become a managed collection with no migration: not one byte of
  the pre-existing corpus changes, which is the strongest available
  evidence that the model was wrong rather than the files.
- `Status` gains no variant. A stateless collection is the absence of
  the vocabulary, not a new entry in it — which is what keeps this from
  being the `status: recorded` sentinel wearing different clothes.
- Two doctor tests that used `archaeology/logs/` as their example of an
  *unmanaged* claimant had to name a genuinely unmanaged kind instead.
  That is the expected cost of adopting a collection and is recorded
  here so the next adoption expects it.
- `doctor`'s canonical-status-carrier check is skipped for stateless
  artifacts. It exists so a transition can rewrite exactly one `status:`
  line; nothing ever rewrites one here, and a log that grew such a line
  is already a malformed-artifact finding from the parser.
- `transition::perform` refuses a stateless artifact rather than
  unwrapping its absent state. Unreachable through the CLI, which
  refuses the reference before dispatch, but a future caller must not be
  able to rewrite a lifecycle into a collection that has none.
- This does **not** relax the template guarantee for collections that
  have one. A collection either owns its headings or owns none of them;
  the middle case — author-authored sections interleaved with managed
  ones — remains open as idea 41, and a template-free collection is
  precisely the case that has no ordering question to answer.
