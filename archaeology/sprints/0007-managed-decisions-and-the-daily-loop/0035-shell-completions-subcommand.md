---
id: tsk_01KY7S6QCN94NTTZ8CEW8RQS77
sequence: 35
kind: task
status: closed
sprint: spr_01KY7S6Q69YJ6HATZB48SZBRRM
created: 2026-07-23
closed: 2026-07-25
---

# Shell completions subcommand

## Objective

Add a `strata completions <shell>` subcommand that emits shell
completion scripts, lowering the cost of human CLI use — the surface
where desire-path data originates.

## Acceptance criteria

- `strata completions zsh` emits a completion script that loads
  without error in zsh; other shells supported by the completion
  generator may be included where they come free.
- The subcommand appears in `--help` with no placeholder flags.
- A brief installation note lands in the README.
- `scripts/check.sh` passes.

## Result

`strata completions <shell>` emits clap_complete-generated scripts
for bash, zsh, fish, elvish, and powershell — all five come free from
the one generator, no placeholder flags. Generation is pure command-
definition output with no repository discovery, so it works outside
any Strata repository; an unknown shell is a clap usage error naming
the input.

Verified: the zsh script loads without error under `zsh -f` with
compinit (both manually and in a self-skipping integration test that
runs wherever zsh is installed); bash/fish/zsh emissions and the
unknown-shell refusal are pinned in `tests/cli.rs`. README gained an
installation note. New dependency: `clap_complete` (the clap
project's own generator — within the spirit of the decided clap
dependency). `scripts/check.sh` passes.
