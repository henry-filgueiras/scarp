---
id: mnt_01KZ7A8KPX088RA46TKXG65N7G
sequence: 2
kind: maintenance
status: pending
created: 2026-08-04
---

# Bump the version before the next publish

## Work

Bump the crate version before the next `cargo publish`. Released 0.2.0
and the working tree both report `scarp 0.2.0`, so `--version` cannot
distinguish a binary that knows sprint 12's collections from one that
does not.

Found in [[tsk_01KZ738BNX70HQWFCBYV8CF9F1|Validate the consumer affordances in WitnessGlass]] while verifying that an older Scarp still reads a
repository containing `archaeology/maintenance/`. That test had to be
made entirely behavioural — probing for `unknown collection` — because
the version string was useless for telling the two binaries apart. It
worked, but the next person doing a compatibility check should not have
to rediscover that the obvious identifier lies.

This is pending until the next release, deliberately. It is not a chore:
one performance, at a known trigger, and it retires when done.
