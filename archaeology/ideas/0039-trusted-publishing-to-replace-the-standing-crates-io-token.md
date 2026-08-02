---
id: ide_01KYZWC2NBHCSPHFSWZZTB25MH
sequence: 39
kind: idea
status: parked
created: 2026-08-01
---

# Trusted publishing to replace the standing crates.io token

## Problem

Publishing today depends on a long-lived API token sitting in
`~/.cargo/credentials.toml` on the development machine, written during
the `0.1.0` publication on 2026-07-30. It has no expiry, it can publish
any crate the account owns, and anything that can read the home
directory can take it. The blast radius of one compromised laptop is
every Scarp release, indefinitely.

That cost is invisible because it is never paid at release time — the
token is already there, so publishing feels free. Nothing surfaces the
standing risk between releases, which is exactly when it exists.

## Sketch

crates.io supports **Trusted Publishing** (RFC 3691): a GitHub Actions
job requests a signed identity token from GitHub's OIDC provider,
`rust-lang/crates-io-auth-action` exchanges it with crates.io for an
access token valid for about thirty minutes, and the action's post step
revokes it when the job ends. No secret is stored anywhere.

Two halves:

- **On crates.io**, a Trusted Publisher configuration naming the owning
  user or organization, the repository, the workflow filename (which
  must live in `.github/workflows/`), and optionally a GitHub Actions
  environment.
- **In the workflow**, `permissions: id-token: write` plus the auth
  action, then an ordinary `cargo publish`.

The optional environment claim is the part worth having: a GitHub
Actions environment with required reviewers turns publication into an
explicit human approval on a specific run — both a gate and an audit
record, which a local `cargo publish` does not leave behind at all.

## Boundaries

**This moves publication into CI, which is the boundary
[[tsk_01KYK0PTQV9PGZTHRDAPG6YGYM|task 45]] deliberately drew** — an
agent prepares the exact command and stops, and a human runs it.
Adopting this has to answer that decision rather than route around it.

The honest distinction, and why the answer may differ from
[[tsk_01KYX1WHTGXMBCBA7NE27RM9CF|task 50]]'s: publishing is not a
canonical mutation. It writes nothing to the repository, creates no
artifact, and changes no archaeology. It uploads an immutable,
already-committed tree to a registry. Sprint 10 concluded that CI must
not *author canonical state*; a release upload is a different act, and
the reasoning that settled one does not automatically settle the other.

Also true, and cutting the other way: with a required-reviewer
environment the human decision becomes *more* explicit than it is today,
not less. Today the gate is Henry remembering to run a command; there it
is Henry approving a named run.

Further boundaries:

- Not release automation generally. Version choice, changelog, tagging,
  and the decision to publish stay human. This replaces one credential
  mechanism, nothing else.
- The recurring form of a release remains a chore ledger
  ([[idea-chore-artifacts|idea 7]]), not a script. Sprint 8's non-goal
  against a release cathedral survives.
- `cargo publish` from a laptop must keep working. A repository that
  cannot reach GitHub Actions must still be releasable.

## Evidence

Henry raised it on 2026-08-01 while preparing the release
[[tsk_01KYX31ACH05NGA3GYH0TJA870|task 56]] holds, asking whether the
token dance could be automated and half-remembering that crates.io
supported something like this. It does; the recollection was accurate.

Verified 2026-08-01 against primary sources: RFC 3691 states the
workflow needs `id-token: write`, and that the crate owner configures
the owning user or organization, the repository name, the workflow
filename, and an optional environment. `rust-lang/crates-io-auth-action`
is the published action performing the exchange, and its post step
revokes the token when the job completes.

Observed the same day: `~/.cargo/credentials.toml` exists, dated
2026-07-30, matching the `0.1.0` publication — so the standing token is
real rather than hypothetical, and no token needs minting for the next
release either way.

Promotion trigger: the credential-hygiene argument does not depend on
release frequency, so this does not need the project's rule of three.
What it needs is an answer to the task 45 boundary question, which is
Henry's; one more manually performed release would be reasonable
evidence about whether the ceremony is worth replacing at all.
