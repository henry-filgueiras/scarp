# Security Policy

## Supported versions

Scarp is pre-1.0. Security fixes land on `main` and in the next published
version; there are no maintained release branches, and older versions are
not patched in place.

## Threat model, honestly stated

Scarp is a local command-line tool that reads and writes ordinary files
inside a repository you already control. It runs no daemon, opens no network
connections, and executes no code from the repositories it manages. The
interesting security surface is therefore narrow:

- parsing untrusted repository files (front matter, configuration) — panics,
  resource exhaustion, or logic errors triggered by malformed input;
- path handling — an artifact or configuration value that could induce reads
  or writes outside the repository root;
- content-loss bugs in mutation paths — see
  [decision 8](archaeology/decisions/0008-mutation-failure-classes.md) for
  the failure-class contract, which states plainly which classes are covered
  and which (power loss, kernel crashes) are deliberately out of scope.

If you find any of these — especially anything that lets a hostile
repository escape its own directory — please report it.

## Reporting a vulnerability

Use GitHub's private vulnerability reporting: **Security → Report a
vulnerability** on this repository. Please do not open a public issue for
suspected vulnerabilities.

You can expect an acknowledgment within a week. This is a single-maintainer
project; fixes will be prioritized honestly rather than on a promised SLA.
Credit is given in the fix's commit and release notes unless you ask
otherwise.
