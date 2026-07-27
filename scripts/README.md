# Script: bootstrap-inception.sh

## Historical bootstrap

`scripts/bootstrap-inception.sh` records the one-time manual process used to
seed this repository's archaeology before the tool could manage it. It is
retained as historical documentation and a fixture reference, not as a
supported installer.

Its contents are frozen provenance and are deliberately **not** migrated to
the current identity. The script still says `strata`, writes `.strata.toml`,
and invokes commands under the pre-release product name, because that is what
the inception performance actually did. The current interface is `scarp`,
`.scarp.toml`, and `scarp <command>` (see decision 16). Read the script as a
dated record of what happened, never as instructions to run today.
