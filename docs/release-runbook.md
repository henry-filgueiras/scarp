---
runme:
  version: v3
---

# Release runbook

How a Scarp release is cut. Every step below is **human-owned**: an agent
may prepare, verify, and read, but the publish, the tag push, and the
credential handling are yours. That boundary was drawn in
`archaeology/sprints/0008-first-contact/0045-publish-and-verify-v0-1-0.md`
and it has held for two releases.

The file is executable with [`runme`](https://runme.dev):

```text {"name":"how-to-run","excludeFromRunAll":"true"}
runme ls --filename docs/release-runbook.md    # list the named cells
runme run preflight                            # run one cell by name
```

Do **not** `runme run --all`. The irreversible cells are marked
`excludeFromRunAll`, but the ordering here depends on you reading output
between steps, not on a runner. Run one cell, read what it printed,
then run the next.

Blocks tagged `text` are not runnable. They are commands to adapt, or
things to do in a browser.

`cargo publish` cannot be undone. A yank hides a version from future
resolution; it never deletes it and never permits re-publishing that
number. Everything provable is therefore proved before step 5.

This runbook is not packaged in the crate — `docs/` is outside the
`include` allowlist in `Cargo.toml`, deliberately.

---

## 1. Prerequisites

### 1.1 Local, once per machine

- A Rust toolchain. The declared MSRV is in `Cargo.toml` (`rust-version`);
  releases are built with stable, and the MSRV is a CI gate, not a build
  requirement here.
- `gh`, authenticated, for the GitHub release.
- `docker`, if you want the clean-room install check in step 8 to run
  somewhere that has genuinely never seen this repository. An empty
  `CARGO_HOME` on this host is the cheaper substitute and is what 0.2.0
  used.

### 1.2 crates.io, per release, in a browser

The token is minted for one release and revoked at the end of it. This is
not ceremony: `archaeology/ideas/0039-*.md` exists because a standing
token in `~/.cargo/credentials.toml` has no expiry and can publish
anything the account owns, and the risk is invisible precisely between
releases. Until trusted publishing lands, short-lived scoped tokens are
the mitigation.

1. Sign in at <https://crates.io> with the GitHub account that owns the
   crate (`henry-filgueiras`).
2. Go to **Account Settings → API Tokens → New Token**.
3. Name it for this release, e.g. `scarp-0.3.0-publish`.
4. **Endpoint scopes: `publish-update` only.** Not `yank`, not
   `change-owners`, not `publish-new`, not `legacy`.
   - `publish-new` was correct for `0.1.0` and is wrong now: the crate
     exists, so this is an update.
   - Omitting `yank` is deliberate. It means a yank cannot happen by
     reflex — performing one would require minting another token, which
     is itself the severity checkpoint the contingency in step 10 asks
     for.
5. **Crate scope: `scarp`.**
6. Set the shortest expiry offered that covers today.
7. Copy the token. It is shown once.

Then log in locally. Paste the token at the prompt — **never** pass it as
an argument, where it would land in shell history and process listings:

```sh {"name":"cargo-login","interactive":"true","excludeFromRunAll":"true"}
cargo login --registry crates-io
```

---

## 2. Preflight

Establishes that the tree you are about to publish is the tree that was
reviewed. Read every line it prints; it deliberately refuses rather than
reporting.

```sh {"name":"preflight"}
set -euo pipefail
git fetch origin --tags --quiet

test -z "$(git status --porcelain -uall)" \
  || { echo "FAIL: worktree is dirty"; git status --short; exit 1; }

test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" \
  || { echo "FAIL: HEAD is not origin/main"; exit 1; }

echo "release source SHA: $(git rev-parse HEAD)"
echo "current version:    $(cargo pkgid | sed 's/.*[#@]//')"
echo
RUN="$(gh run list --commit "$(git rev-parse HEAD)" --workflow CI \
  --json databaseId --jq '.[0].databaseId')"
test -n "$RUN" || { echo "FAIL: no CI run for this commit"; exit 1; }
echo "CI run $RUN — https://github.com/henry-filgueiras/scarp/actions/runs/$RUN"
gh run view "$RUN" --json jobs --jq '.jobs[] | "  \(.name): \(.conclusion)"'
```

Both jobs — `check` and `MSRV` — must be `success` **on this commit**. A
green badge on the branch is not the same claim, and the job level is
where the MSRV gate's own self-check lives: it is asked for by name here
because a gate that silently installs its own toolchain is not a gate,
and this is the last point at which that can be caught.

---

## 3. Choose and set the version

Decide the number before running anything. The judgment belongs in the
maintenance item that commissioned the release, not here; what this step
owes is that the number is chosen rather than defaulted.

Scarp is pre-1.0, so under Cargo's semver rules the **minor** position
carries breaking changes and the patch position carries compatible ones.
The crate ships a library (`src/lib.rs`) as well as a binary, so the
library's public API counts.

Set the new version — runme will prompt, offering the value below as the
default:

```sh {"name":"bump-version","promptEnv":"true"}
export NEW_VERSION="0.3.0"
sed -i.bak -E 's/^version = "[0-9]+\.[0-9]+\.[0-9]+"$/version = "'"$NEW_VERSION"'"/' Cargo.toml
rm -f Cargo.toml.bak
cargo update -p scarp
git --no-pager diff --stat Cargo.toml Cargo.lock
git --no-pager diff Cargo.toml
```

The `sed` is line-anchored, so it matches only the `[package]` version
and never a dependency's inline `version = ` field. Confirm from the diff
that exactly one line in `Cargo.toml` changed.

`cargo update -p scarp` refreshes only this package's entry in
`Cargo.lock`; it does not bump dependencies. That matters because every
subsequent command uses `--locked`.

Commit the bump on its own before proceeding — this commit is the release
source:

```sh {"name":"commit-bump"}
git add Cargo.toml Cargo.lock
git commit -m "release: bump to $(cargo pkgid | sed 's/.*[#@]//')"
git --no-pager log -1 --format='release source: %H%n%s'
```

---

## 4. Package review

Nothing here uploads. Run all three and read the output.

```sh {"name":"package-list"}
cargo package --locked --list
```

Expect `src/`, `tests/`, `README.md`, both `LICENSE-*` files, both
`assets/logo*.svg`, and Cargo's automatic `Cargo.toml`,
`Cargo.toml.orig`, minimized `Cargo.lock`, and `.cargo_vcs_info.json`.

Expect **absent**: `archaeology/`, `docs/`, `scripts/`, `.github/`,
`.claude/`, `CLAUDE.md`, `.scarp.toml`, and the community health files.
An unpacked crate that is itself a Scarp repository is the defect the
allowlist exists to prevent.

For scale: `0.1.0` packaged 36 files (561.8 KiB, 123.9 KiB compressed);
`0.2.0` packaged 38 (616.6 KiB, 138.0 KiB). A large jump means something
got in that should not have.

```sh {"name":"package-build"}
cargo package --locked
```

```sh {"name":"publish-dry-run"}
cargo publish --dry-run --locked --registry crates-io
```

This must pass with **no extra flags**. `--allow-dirty` and `--no-verify`
exist precisely to defeat the guarantees this release depends on; if
either seems necessary, stop and fix the actual problem.

Now treat the `.crate` as the product under test rather than trusting the
listing — unpack it outside the checkout and prove the exclusions:

```sh {"name":"verify-crate-contents"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
WORK="$(mktemp -d)"
tar -xzf "target/package/scarp-${VERSION}.crate" -C "$WORK"
UNPACKED="$WORK/scarp-${VERSION}"

for forbidden in archaeology docs scripts .github .claude CLAUDE.md .scarp.toml CONTRIBUTING.md; do
  if [ -e "$UNPACKED/$forbidden" ]; then
    echo "FAIL: $forbidden was packaged"; exit 1
  fi
done
echo "exclusions hold"

echo "vcs info: $(cat "$UNPACKED/.cargo_vcs_info.json")"
echo "HEAD:     $(git rev-parse HEAD)"
echo "sha256:   $(shasum -a 256 "target/package/scarp-${VERSION}.crate" | cut -d' ' -f1)"
echo
echo "unpacked at: $UNPACKED"
```

The `.cargo_vcs_info.json` `sha1` must equal `HEAD` and must carry **no
`dirty` flag**. Keep the printed SHA-256 — step 7 compares the registry's
copy against it.

Optionally, build and test from the unpacked source alone, which proves
the packaged tree is self-sufficient:

```text {"name":"test-unpacked","excludeFromRunAll":"true"}
cd <unpacked path from above>
CARGO_TARGET_DIR=$(mktemp -d) cargo test --locked
```

---

## 5. Publish — irreversible

Last check that this version number is free. A 404 here is the good
answer:

```sh {"name":"check-version-free"}
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
curl -sS -A "scarp-release-runbook (henry-filgueiras)" \
  "https://crates.io/api/v1/crates/scarp/${VERSION}" \
  | head -c 400
echo
```

Then publish. This is the point of no return.

```sh {"name":"publish","excludeFromRunAll":"true","interactive":"true"}
cargo publish --locked --registry crates-io
```

Keep the output verbatim — the `Packaged N files, X KiB (Y KiB
compressed)` line should be identical to the dry run's, and that identity
is the first sign that what you uploaded is what was reviewed. It goes in
the maintenance item's `Result` as dated provenance.

---

## 6. Tag and release

The tag must point at the release-source commit — the one that was
published, not the later commit that records the publication. A record of
a publication cannot live inside the commit it publishes; the archaeology
closure is deliberately after the tag and deliberately not tagged.

```sh {"name":"tag"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
SHA="$(git rev-parse HEAD)"
git tag -a "v${VERSION}" "$SHA" -m "scarp v${VERSION}"
git --no-pager show --no-patch --format='tagging %H' "v${VERSION}^{commit}"
```

Pushing is always a human decision, and this runbook is where you make
it:

```sh {"name":"push-tag","excludeFromRunAll":"true"}
git push origin main --follow-tags
```

Verify on the remote that the tag is annotated and peels to the release
source:

```sh {"name":"verify-tag"}
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
git ls-remote --tags origin "v${VERSION}*"
echo "expected peeled commit: $(git rev-parse HEAD)"
```

Two lines, the second ending `^{}`, means annotated rather than
lightweight. The `^{}` line is the commit the tag names.

Write the release notes, then:

```sh {"name":"github-release","excludeFromRunAll":"true","interactive":"true"}
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
gh release create "v${VERSION}" --verify-tag --title "Scarp v${VERSION}" --notes-file -
```

`--verify-tag` aborts if the tag is not already on the remote. Without
it, `gh` invents a tag from the default branch, and a tag invented by the
release tool is not guaranteed to name the commit that was published.

Notes are curated, not autogenerated. No prebuilt binaries: one honest
install path beats an unverified matrix.

---

## 7. Confirm what the registry actually received

```sh {"name":"verify-registry"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
UA="scarp-release-runbook (henry-filgueiras)"

curl -sS -A "$UA" "https://crates.io/api/v1/crates/scarp/${VERSION}" \
  | python3 -c 'import json,sys; v=json.load(sys.stdin)["version"]; print("\n".join(f"{k}: {v[k]}" for k in ("num","created_at","yanked","crate_size","rust_version","license")))'

echo
echo "sparse index:"
curl -sS -A "$UA" "https://index.crates.io/sc/ar/scarp" | tail -1 | cut -c1-200
echo
echo "owners:"
curl -sS -A "$UA" "https://crates.io/api/v1/crates/scarp/owners" \
  | python3 -c 'import json,sys; [print(" ", o["login"], o["kind"]) for o in json.load(sys.stdin)["users"]]'
```

The owners endpoint is unauthenticated on purpose: it proves ownership
without depending on a credential that should not outlive this release.

Tie the published artifact to the commit rather than assuming it:

```sh {"name":"verify-registry-artifact"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
DL="$(mktemp -d)/scarp-${VERSION}.crate"
curl -sSL -A "scarp-release-runbook (henry-filgueiras)" \
  "https://crates.io/api/v1/crates/scarp/${VERSION}/download" -o "$DL"
echo "downloaded: $(shasum -a 256 "$DL" | cut -d' ' -f1)"
echo "local:      $(shasum -a 256 "target/package/scarp-${VERSION}.crate" | cut -d' ' -f1)"
```

Byte-identity is what held for `0.1.0`, but it is an observation, not a
guarantee Cargo makes. A difference is worth investigating before you
accept it.

---

## 8. Prove the shipped binary, not the local one

Install from the registry into an environment with no cache, and exercise
the surfaces this version exists to ship. Do not trust that what was
tested locally is what shipped.

```sh {"name":"clean-install","interactive":"true"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
export CARGO_HOME="$(mktemp -d)"
ROOT="$(mktemp -d)"
time cargo install scarp --version "$VERSION" --locked --root "$ROOT"
"$ROOT/bin/scarp" --version
echo "installed binary: $ROOT/bin/scarp"
```

`Installed package \`scarp vX.Y.Z\`` with **no path or Git source in the
parentheses** is how a registry install is distinguished from a local
one. Record the wall-clock install time for
`archaeology/ideas/0035-*.md`, with the caveat that the number describes
your machine and not a bound.

Then drive the shipped binary through this version's new surfaces in a
throwaway repository. Adapt the checks to what the release actually adds:

```text {"name":"shipped-surfaces","excludeFromRunAll":"true"}
cd $(mktemp -d)
<installed binary> init
<installed binary> new log "shipped-surface check"
<installed binary> new principle "shipped-surface check"
<installed binary> new maintenance "shipped-surface check"
<installed binary> list maintenance
printf 'closed from the runbook.\n' > /tmp/result.md
<installed binary> close maintenance:1 --body-file /tmp/result.md
<installed binary> doctor
<installed binary> proposal reconcile --help
```

Every command must succeed against the **installed** binary, and the
closure must have written a `## Result` section. A surface that only
works from `cargo run` in this checkout did not ship.

Finally, the surfaces that only exist once publication has happened:

```sh {"name":"verify-docs-rs"}
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
curl -sS -A "scarp-release-runbook (henry-filgueiras)" \
  "https://docs.rs/crate/scarp/${VERSION}/status.json"
echo
```

`{"doc_status": true}` is a successful terminal state. Queued is not
built: docs.rs builds on nightly in a network-blocked sandbox with a
fifteen-minute limit, so it can fail where a local `cargo doc` never
does. A failure is a finding, not a delay.

In a browser, on <https://crates.io/crates/scarp>, in both light and dark
theme:

```text {"name":"browser-checklist","excludeFromRunAll":"true"}
- the wordmark renders in both themes, no broken-image placeholder
- both Mermaid diagrams render
- every README link resolves — including the absolute links into
  archaeology/ and docs/, which are absolute precisely because those
  directories are not packaged
- the sidebar shows the right MSRV, license, size, and docs.rs link
- no stale product identity: no `Strata`, no `SCARP`, no `.strata.toml`
```

The last one has bitten before. `0.1.0` shipped a `#quickstart` anchor
that worked on GitHub and was dead on crates.io, because crates.io
prefixes heading ids with `user-content-` and does not rewrite
author-written same-page hrefs. Check anchors on the crates.io rendering
specifically; GitHub is not evidence for it.

---

## 9. Credential cleanup

Both halves. `cargo logout` removes only Cargo's local copy — a token
that outlives it is still live at the registry.

```sh {"name":"cargo-logout","excludeFromRunAll":"true"}
cargo logout --registry crates-io
```

Then, in the browser: **crates.io → Account Settings → API Tokens →
Revoke** the token minted in step 1.2.

---

## 10. If something is wrong after publication

Classify honestly before acting.

- **Yank only when the version is actively harmful**, not merely
  imperfect. A cosmetic README, logo, or metadata defect does not justify
  one. `0.1.0` shipped with a dead anchor and was not yanked; the defect
  was recorded and repaired in the next version.
- A yank hides the version from future resolution. It does not delete it,
  and it does not free the number.
- Open a remediation item for the fix. Do not rewrite the record to
  suggest the release was clean, and do not describe a flawed release as
  successful. The version stays in the record either way.

---

## 11. Close the archaeology

The closure commit comes **after** the tagged commit, necessarily: the
publish output, the registry checksum, the tag's own hash, and the
release URL could not truthfully exist inside the commit they describe.
This is an ordering consequence, not an oversight — do not "correct" the
tag to include it, which would make it name a tree that was never
published.

```sh {"name":"close-item","excludeFromRunAll":"true"}
scripts/check.sh
# then, with the release's Result written to a file:
# scarp close maintenance:N --body-file <path>
```

The `Result` carries the dated provenance this project asks for: the
release-source SHA, the publish output verbatim, the token's scopes, the
registry metadata, the install timing, and anything that had to be
adapted from this runbook. If a step here was wrong, fix this file in the
same change — that is the difference between a runbook and a transcript.
