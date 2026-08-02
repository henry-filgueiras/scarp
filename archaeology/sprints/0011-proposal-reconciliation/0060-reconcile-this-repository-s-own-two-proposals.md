---
id: tsk_01KYZXTN71EDDR370MD3F00CK9
sequence: 60
kind: task
status: pending
sprint: spr_01KYZXP2MJ0EGR8KVPFZ1S8ZFX
created: 2026-08-01
---

# Reconcile this repository's own two proposals

## Objective

Run the finished lifecycle against the two proposals this repository has
actually filed, and close the loop on both.

This is the sprint's proof that the thing works for its purpose rather
than merely works — the same distinction sprint 10's retrospective drew
when idea 38 arrived through the channel instead of being tested through
it.

### The two instances

They are not the same case, which is why both are worth doing:

- **Issue [#2](https://github.com/henry-filgueiras/scarp/issues/2)** was
  realized as [[ide_01KYZVJ6XCK11DP67GVMC3M23C|idea 38]] and shipped in
  `0.2.0`. It has been landed and stale for a sprint. Reconciling it is
  the ordinary case, run against real history rather than a fixture.
- **Issue [#3](https://github.com/henry-filgueiras/scarp/issues/3)** was
  realized as [[ide_01KYZXGDY8YAFXMP1FV931ZB0M|idea 40]] — the proposal
  that asked for this sprint. It becomes reconcilable only after this
  sprint's own work reaches `main`, so it exercises the refusal first
  and the success path second, in that order, without contriving either.

Issue #3 completes a path with no hand transcription anywhere in it:
drafted on a phone, filed as a structured issue, realized by Scarp,
landed by an ordinary commit, and closed by Scarp against its own
canonical record.

### Sequencing and authority

Both reconciliations post publicly under the repository's identity, and
issue #3's requires this sprint's commits to be on `main` first. **Only
Henry pushes.** This task therefore waits on a human action it must not
attempt, and that dependency is the point rather than an inconvenience:
it is the same boundary the whole channel is built around.

### What to watch for

Record friction as first-class output, not as an aside — the comment's
wording read cold, anything the refusal failed to explain, whether the
landing proof behaved against real history the way it did against tests,
and any step still performed by hand.

## Acceptance criteria

- Issue #2 carries a reconciliation comment naming idea 38 and its
  landing commit, and is closed as completed.
- Issue #3 refuses reconciliation before this sprint's work lands,
  observed rather than assumed, and is reconciled and closed after.
- `scarp proposal list` reports no open realized proposals afterwards.
- Nothing about the canonical artifacts changed: `scarp doctor` is green,
  and ideas 38 and 40 are byte-identical to before.
- Observed friction is recorded in the Result, including anything that
  argues for or against task 58's outcome.

## Progress: issue #2 reconciled (2026-08-01)

Authorized by Henry in session. Half the task; issue #3 waits on the
push, and this task stays pending until then.

### The procedure, as performed

Dated provenance rather than automation: these are the exact commands
run on 2026-08-01 against the live repository, and they promise only
that they worked that day against that interface.

```sh
# 1. Confirm what is outstanding. #2 is realized and still open --
#    the stale state the whole sprint exists to end.
scarp proposal list

# 2. Reconcile. Comments the canonical artifact, then closes.
scarp proposal reconcile 2

# 3. Verify against GitHub rather than trusting the exit code.
gh issue view 2 --json state,stateReason,comments \
  --jq '{state,stateReason,comments:[.comments[].body]}'

# 4. Prove idempotency on the real issue, not just in tests.
scarp proposal reconcile 2
gh issue view 2 --json comments --jq '.comments|length'

# 5. Prove nothing canonical moved.
git status --porcelain     # empty
scarp doctor
```

### What was observed

- `scarp proposal reconcile 2` → `reconciled proposal #2: closed, citing
  idea:38`, exit 0.
- GitHub: `state: CLOSED`, `stateReason: COMPLETED`, one comment. The
  artifact permalink is pinned to the introducing commit
  (`ee8c611`), so it will still resolve after the file moves or changes.
- Re-running → `proposal #2 is already closed; nothing to do`, exit 0,
  comment count still 1. **The idempotency claim is now demonstrated
  rather than asserted** — [[tsk_01KYZXTN1EV8KKTK3Q75B8HSYR|task 57]]
  could only prove it against a pure function.
- `git status --porcelain` empty and `doctor` green afterwards:
  reconciliation touched nothing canonical, as designed.

### Friction: the comment read worse than it tested

**Real rendering found what the test could not.** The `Landed in` row
emitted a bare forty-character sha — `ee8c611f81efb403fe8c42233513572a168355e1`
— unlinked, in a table otherwise made of links. Every assertion about
that row passed, because a test asserting `body.contains("abc1234")`
cannot notice that a human would not read it.

Fixed before issue #3 gets a comment: the row is now an abbreviated
`[`ee8c611`](…/commit/…)` link. Issue #2's existing comment is
deliberately **not** edited — reconciliation is terminal, and going back
to touch a published comment would be the synchronization this design
refuses. The first comment stays as the slightly worse artifact it was,
which is the honest record.

This is sprint 10's lesson recurring exactly: *real payloads find what
synthetic ones cannot.* There, a real proposal body contained a fenced
code block no hand-written fixture had. Here, a real comment was read by
a person for the first time.

### For [[tsk_01KYZXTN3AMPNJ482J4Q13ACTW|task 58]]

Nothing here argues for automation. The operator path took one command,
returned in about a second, and the only judgment involved — *is this
comment good enough to publish under the repository's name* — is
precisely what an unattended workflow would have removed. Had this run
unattended, the bare-sha defect would have been published to every
proposal before anyone read one.

### Remaining

Issue #3 needs this sprint's commits on `main`. Pushing is Henry's, and
the refusal is already observed: `scarp proposal reconcile 3` refuses
with `precondition-unmet`, exit 12.
