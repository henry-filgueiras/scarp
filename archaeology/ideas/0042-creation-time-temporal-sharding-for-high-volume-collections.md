---
id: ide_01KZ73A671YV99APMXAWEQ20X9
sequence: 42
kind: idea
status: parked
created: 2026-08-04
---

# Creation-time temporal sharding for high-volume collections

## Problem

Every collection Scarp manages files flat, one directory per collection,
and browsing a flat directory degrades with size. Dragons, decisions,
and logs are naturally rate-limited — you do not write forty decisions a
month — but `maintenance`, commissioned in sprint 12, is the first
collection whose whole point is high-volume, low-value-per-item
recording. A repository that uses it as intended accumulates hundreds of
files in one directory with nothing but a four-digit prefix to order
them, and `ls` stops being a useful way to see what happened recently.

This is a scaling hypothesis, not a felt requirement. It is recorded
because the moment to notice it is before the corpus exists, and the
moment to act on it is after.

## Sketch

Shard a collection's canonical placement by immutable creation time:

```text
archaeology/maintenance/2026-08/0003-refresh-the-status-table.md
```

The bucket is storage topology and nothing else. It is derived from the
artifact's own `created:` date, so it never changes; closing or
transitioning an artifact never moves it, which keeps
[[dec-flat-placement|decision 11]]'s surviving rule intact — lifecycle
state is never encoded in placement and transitions never move files.
Monthly is the current candidate granularity; yearly is the obvious
cheaper alternative and quarterly the obvious compromise, and the right
answer depends on the rate a real corpus turns out to have.

Display sequences stay collection-global across buckets, exactly as task
sequences are global across sprint containment directories. That is the
property that makes this migratable: an artifact's sequence and stable
id are unchanged by the move, so adopting sharding later is `git mv`
over existing files and a scanner change, with no identity churn, no
renumbering, and no reference breakage — the same shape task 18's
placement migration took.

## Boundaries

- Not lifecycle, not planning, not semantic grouping. A bucket says when
  an artifact was created and nothing about what it is or how it is
  going.
- Not retroactive reorganisation of collections that are fine flat.
  Dragons, decisions, ideas, principles, and logs have no rate problem
  and would gain a topology for nothing.
- Not user-configurable granularity in a first slice. One granularity,
  chosen from measured corpus growth.
- Raw browsing must stay obvious to someone with no tooling: the bucket
  name has to be readable as a date at a glance, which is why
  `YYYY-MM` and not an opaque shard key.
- Adopting this means accepting a bucket-versus-`created:` agreement
  check in `doctor`, the same dual bookkeeping decision 11's amendment
  accepted for tasks and for the same reason: neither carrier changes
  during a transition, so only hand edits can desynchronise them.

## Evidence

Adjudicated 2026-08-04 while commissioning sprint 12. Monthly sharding
was the initial candidate placement for `maintenance`; measuring the
implementation against the code changed the trade. Flat placement reuses
`read::scan`, `doctor::scan_dir`, and `artifact::create` verbatim and
needs one `Collection` static, one directory constant, one template, one
CLI arm, and one `create_*` function. Sharding needs, on top of all of
that, a dedicated scanner, a dedicated creator, a dedicated `doctor`
walker, and a new misfiled-artifact check — and it would make
creation-time-derived physical placement a canonical invariant, a third
stable-containment topology after flat and sprint-containment, before
any maintenance corpus existed to demonstrate the need.

The deferral is cheap precisely because collection-global sequences and
stable ids are required either way, which is the argument for keeping
them that way. Prior art for the topology: dated log directories,
`YYYY/MM` archive layouts in mail and static-site tooling, and Git's own
fan-out of object storage by prefix — sharding chosen for directory
ergonomics rather than meaning.
