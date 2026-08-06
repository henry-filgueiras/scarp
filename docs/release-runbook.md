---
cwd: ..
shell: bash
runme:
  version: v3
---

# Release runbook

How a Scarp release is cut. Every step below is **human-owned**: an agent
may prepare, verify, and read, but the publish, the tag push, the
credential handling, and the GitHub release are yours. That boundary was
drawn in
`archaeology/sprints/0008-first-contact/0045-publish-and-verify-v0-1-0.md`
and it has held for two releases.

`cargo publish` cannot be undone. A yank hides a version from future
resolution; it never deletes it and never permits re-publishing that
number. Everything provable is therefore proved before section 5.

This runbook is not packaged in the crate — `docs/` is outside the
`include` allowlist in `Cargo.toml`, deliberately.

## Running it

The file is executable with [`runme`](https://runme.dev) 3.17.2. Runme
resolves a cell's working directory from the **document's** location, not
from where you invoke it, so this document sets `cwd: ..` in its front
matter to put every cell at the repository root. `shell: bash` is pinned
there too, because cells rely on `set -o pipefail`, `<<<`, and `[[`.

Always pass `--filename`; without it runme looks for `README.md`:

```text {"excludeFromRunAll":"true"}
RB=docs/release-runbook.md
rr() { runme run --filename "$RB" "$@"; }

runme ls --filename "$RB"
rr preflight
```

Every named cell is self-contained. None of them depends on a variable,
a temporary path, or a shell function left behind by an earlier cell —
each derives the version, the SHA, and its own scratch directories. You
can therefore run one, read its output, and come back tomorrow.

Do **not** `runme run --all`. Mutating cells carry `excludeFromRunAll`,
which keeps them out of a run-all sweep — it does **not** make them
harder to run directly, and `rr publish` will publish. The exclusion is
a backstop against a sweep, not a safety catch on the individual cell.
The ordering here also depends on you reading output between steps,
which no runner does.

Unnamed `text` blocks are not operations. They are commands to adapt by
hand, or transcripts of what to expect.

The full cell order is in section 12. Follow it rather than reading top
to bottom and guessing.

---

## 1. Prerequisites

- A Rust toolchain. The declared MSRV is in `Cargo.toml` (`rust-version`);
  releases are built with stable, and the MSRV is a CI gate, not a build
  requirement here.
- `gh`, authenticated, for the CI wait and the GitHub release.
- `bash`, `curl`, `tar`, `shasum`, and `python3` — all present on stock
  macOS.
- The commit you intend to release from must already be on `origin/main`
  with CI green. `preflight` refuses otherwise, and that refusal is the
  point: everything after it reasons about a commit GitHub has seen.

The crates.io token is **not** a prerequisite. It is minted in section 5,
immediately before the publish, and revoked immediately after. See the
reasoning there.

---

## 2. Preflight

Establishes that the tree you are about to act on is a pushed,
CI-proven commit. It refuses rather than reports.

Run this **twice**: once before the version bump, to confirm you are
starting from a clean proven base, and again after `wait-for-ci`, to
confirm the bump commit itself is now the proven one. Packaging must not
begin until the second run passes.

```sh {"name":"preflight"}
set -euo pipefail

git fetch origin --tags --quiet

if [[ -n "$(git status --porcelain -uall)" ]]; then
  echo "FAIL: worktree is dirty"
  git status --short
  exit 1
fi

SHA="$(git rev-parse HEAD)"
if [[ "$SHA" != "$(git rev-parse origin/main)" ]]; then
  echo "FAIL: HEAD is not origin/main — push it and let CI run first"
  echo "  HEAD:        $SHA"
  echo "  origin/main: $(git rev-parse origin/main)"
  exit 1
fi

echo "release source SHA: $SHA"
echo "manifest version:   $(cargo pkgid | sed 's/.*[#@]//')"
echo

RUN="$(gh run list --commit "$SHA" --workflow CI --json databaseId \
  --jq '.[0].databaseId' 2>/dev/null || true)"
if [[ -z "$RUN" ]]; then
  echo "FAIL: no CI run exists for $SHA"
  echo "  push the commit and run \`wait-for-ci\` before packaging"
  exit 1
fi
echo "CI run $RUN — https://github.com/henry-filgueiras/scarp/actions/runs/$RUN"

JOBS="$(gh run view "$RUN" --json jobs --jq '.jobs[] | "\(.name)\t\(.conclusion)"')"
if [[ -z "$JOBS" ]]; then
  echo "FAIL: CI run $RUN reports no jobs"
  exit 1
fi

fail=0
for required in check MSRV; do
  if ! grep -qE "^${required}"$'\t' <<<"$JOBS"; then
    echo "FAIL: required job \`${required}\` is absent from run $RUN"
    fail=1
  fi
done

while IFS=$'\t' read -r name conclusion; do
  [[ -n "$name" ]] || continue
  printf '  %s: %s\n' "$name" "$conclusion"
  case "$conclusion" in
    success) ;;
    ""|null) echo "FAIL: job \`$name\` has not concluded"; fail=1 ;;
    *)       echo "FAIL: job \`$name\` concluded \`$conclusion\`"; fail=1 ;;
  esac
done <<<"$JOBS"

if [[ "$fail" -ne 0 ]]; then
  echo "preflight refused: CI is not green on $SHA"
  exit 1
fi
echo "preflight passed: $SHA is pushed and CI-green"
```

The predicate is deliberately strict. A missing run fails; a missing
`check` or `MSRV` job fails; a `null` conclusion — the shape of a run
still in progress — fails; and `skipped`, `cancelled`, `timed_out`, and
`failure` all fail through the same catch-all. Only the exact string
`success`, on every job present, passes.

Asking for `MSRV` **by name** is not redundant with the run's overall
conclusion. A gate that silently installs its own toolchain is not a
gate, and this is the last point at which its absence can be caught.

---

## 3. Set the version, and make that commit the proven one

Decide the number before running anything. The judgment belongs in the
maintenance item that commissioned the release, not here; what this step
owes is that the number is chosen rather than defaulted.

Scarp is pre-1.0, so under Cargo's SemVer rules the **minor** position
carries breaking changes and the patch position carries compatible ones.
The crate ships a library (`src/lib.rs`) as well as a binary, so the
library's public API counts, and it counts exhaustively: no public type
here is `#[non_exhaustive]`, so a new enum variant or a new public struct
field breaks downstream code that matches or constructs it.

Set the new version — runme prompts, offering the value below as the
default:

```sh {"name":"bump-version","promptEnv":"true","excludeFromRunAll":"true"}
set -euo pipefail
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

Commit the bump on its own. **This commit is the release source** — the
one that gets published, tagged, and installed from:

```sh {"name":"commit-bump","excludeFromRunAll":"true"}
set -euo pipefail
git add Cargo.toml Cargo.lock
git commit -m "release: bump to $(cargo pkgid | sed 's/.*[#@]//')"
git --no-pager log -1 --format='release source: %H%n%s'
```

Now push it. This is the correction that matters most in this section:
CI proved the *pre-bump* commit, and the bump created a different tree.
Publishing that tree without pushing and re-proving it would publish a
commit no CI run has ever seen — and `.cargo_vcs_info.json` would name a
SHA that does not exist on the remote.

```sh {"name":"push-release-source","excludeFromRunAll":"true","interactive":"true"}
set -euo pipefail
git push origin main
git --no-pager log -1 --format='pushed release source: %H'
```

Then wait for CI to conclude on exactly that commit. GitHub takes a few
seconds to register a run after a push, so this polls for the run before
watching it:

```sh {"name":"wait-for-ci"}
set -euo pipefail
SHA="$(git rev-parse HEAD)"
echo "waiting for CI on $SHA"

RUN=""
for attempt in $(seq 1 30); do
  RUN="$(gh run list --commit "$SHA" --workflow CI --json databaseId \
    --jq '.[0].databaseId' 2>/dev/null || true)"
  if [[ -n "$RUN" ]]; then
    break
  fi
  echo "  no run registered yet (attempt $attempt/30); sleeping 10s"
  sleep 10
done

if [[ -z "$RUN" ]]; then
  echo "FAIL: no CI run appeared for $SHA within five minutes"
  echo "  check that the push landed: git ls-remote origin main"
  exit 1
fi

echo "watching run $RUN — https://github.com/henry-filgueiras/scarp/actions/runs/$RUN"
gh run watch "$RUN" --exit-status
```

`--exit-status` makes a failed run a failed cell rather than a report you
have to read carefully.

**Run `preflight` again now.** `wait-for-ci` proves the run concluded;
`preflight` proves the worktree is still clean, `HEAD` is still
`origin/main`, and both required jobs concluded `success`. Packaging
begins only after that second pass.

---

## 4. Package review

Nothing here uploads.

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
CRATE="target/package/scarp-${VERSION}.crate"
test -f "$CRATE" || { echo "FAIL: $CRATE missing — run package-build first"; exit 1; }

WORK="$(mktemp -d)"
tar -xzf "$CRATE" -C "$WORK"
UNPACKED="$WORK/scarp-${VERSION}"

fail=0
for forbidden in archaeology docs scripts .github .claude CLAUDE.md .scarp.toml CONTRIBUTING.md; do
  if [[ -e "$UNPACKED/$forbidden" ]]; then
    echo "FAIL: $forbidden was packaged"
    fail=1
  fi
done
[[ "$fail" -eq 0 ]] || exit 1
echo "exclusions hold"

echo "vcs info: $(cat "$UNPACKED/.cargo_vcs_info.json")"
echo "HEAD:     $(git rev-parse HEAD)"
echo "sha256:   $(shasum -a 256 "$CRATE" | cut -d' ' -f1)"
echo
echo "unpacked at: $UNPACKED"
```

The `.cargo_vcs_info.json` `sha1` must equal `HEAD` and must carry **no
`dirty` flag**. Keep the printed SHA-256 — section 7 compares the
registry's copy against it.

Optionally, build and test from the unpacked source alone, which proves
the packaged tree is self-sufficient. Substitute the path the cell above
printed:

```text {"excludeFromRunAll":"true"}
cd <unpacked path from above>
CARGO_TARGET_DIR=$(mktemp -d) cargo test --locked
```

---

## 5. Publish — irreversible

Last check that this version number is free. Only an exact HTTP 404
passes:

```sh {"name":"check-version-free"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
BODY="$(mktemp)"
trap 'rm -f "$BODY"' EXIT

CODE="$(curl -sS -o "$BODY" -w '%{http_code}' \
  -A 'scarp-release-runbook (henry-filgueiras)' \
  "https://crates.io/api/v1/crates/scarp/${VERSION}")"

case "$CODE" in
  404)
    echo "OK: scarp ${VERSION} is not published (HTTP 404)"
    ;;
  200)
    echo "FAIL: scarp ${VERSION} already exists on crates.io (HTTP 200)"
    echo "  a published version can never be republished, even after a yank"
    head -c 400 "$BODY"; echo
    exit 1
    ;;
  *)
    echo "FAIL: unexpected HTTP ${CODE} from crates.io for scarp ${VERSION}"
    echo "  this is not evidence the version is free — an auth failure, a"
    echo "  rate limit, a redirect, or an outage all land here"
    head -c 400 "$BODY"; echo
    exit 1
    ;;
esac
```

No `-L`: a redirect surfaces as a 3xx and fails rather than being
followed to something that merely looks like an answer. An absence must
be a genuine not-found, per decision 16.

### 5.1 Mint the token — now, not earlier

Everything provable without a credential has now been proved. The token
is minted here so its life is measured in minutes, not in the hour the
package review takes.

`archaeology/ideas/0039-*.md` exists because a standing token in
`~/.cargo/credentials.toml` has no expiry and can publish anything the
account owns, and the risk is invisible precisely between releases.
Until trusted publishing lands, a short-lived scoped token is the
mitigation, and "short-lived" is a property of *when you create it*.

In a browser:

1. Sign in at <https://crates.io> with the GitHub account that owns the
   crate (`henry-filgueiras`).
2. **Account Settings → API Tokens → New Token.**
3. Name it for this release, e.g. `scarp-0.3.0-publish`.
4. **Endpoint scopes: `publish-update` only.** Not `yank`, not
   `change-owners`, not `publish-new`, not `legacy`.
   - `publish-new` was correct for `0.1.0` and is wrong now: the crate
     exists, so this is an update.
   - Omitting `yank` is deliberate. A yank cannot then happen by reflex —
     performing one would require minting another token, which is itself
     the severity checkpoint section 10 asks for.
5. **Crate scope: `scarp`.**
6. Set the shortest expiry offered that covers today.
7. Copy the token. It is shown once.

Paste it at the prompt. **Never** pass a token as an argument, where it
lands in shell history and process listings:

```sh {"name":"cargo-login","interactive":"true","excludeFromRunAll":"true"}
cargo login --registry crates-io
```

### 5.2 Publish

The point of no return.

```sh {"name":"publish","interactive":"true","excludeFromRunAll":"true"}
cargo publish --locked --registry crates-io
```

Keep the output verbatim. The `Packaged N files, X KiB (Y KiB
compressed)` line should be identical to the dry run's, and that identity
is the first sign that what you uploaded is what was reviewed. It goes in
maintenance 3's `Result` as dated provenance.

### 5.3 Retire the credential immediately

Run this **whether the publish succeeded, failed, or failed ambiguously**
— a timeout or a half-reported upload is exactly when a live token is
most dangerous, and nothing after this section needs one. Tagging, the
GitHub release, registry confirmation, the shipped-binary proof, and
docs.rs are all either local, `gh`-authenticated, or unauthenticated
crates.io endpoints.

```sh {"name":"cargo-logout","excludeFromRunAll":"true"}
cargo logout --registry crates-io
```

Then, in the browser, **revoke that token now**: crates.io → Account
Settings → API Tokens → Revoke. Both halves are required. `cargo logout`
removes only Cargo's local copy; a token that outlives it is still live
at the registry.

Only after the revocation do you continue to section 6.

---

## 6. Tag and release

The tag must point at the release-source commit — the one that was
published, not the later commit that records the publication. A record of
a publication cannot live inside the commit it publishes; the archaeology
closure is deliberately after the tag and deliberately not tagged.

```sh {"name":"tag","excludeFromRunAll":"true"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
SHA="$(git rev-parse HEAD)"
git tag -a "v${VERSION}" "$SHA" -m "scarp v${VERSION}"
git --no-pager show --no-patch --format='tagging %H' "v${VERSION}^{commit}"
```

Push **only that tag**. `main` was already pushed and CI-proven in
section 3, so there is nothing else to carry; a broad
`--follow-tags` would sweep any unrelated local tag onto the remote as a
side effect of a release:

```sh {"name":"push-tag","excludeFromRunAll":"true"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
git push origin "v${VERSION}"
```

Verify on the remote that the tag is annotated and peels to the release
source:

```sh {"name":"verify-tag"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
git ls-remote --tags origin "v${VERSION}*"
echo "expected peeled commit: $(git rev-parse HEAD)"
```

Two lines, the second ending `^{}`, means annotated rather than
lightweight. The `^{}` line is the commit the tag names, and it must
equal the SHA printed beneath.

Write the release notes at the prompt, then `Ctrl-D`:

```sh {"name":"github-release","interactive":"true","excludeFromRunAll":"true"}
set -euo pipefail
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

Every endpoint here is unauthenticated on purpose: ownership is proved
without depending on a credential that was revoked in section 5.3.

Tie the published artifact to the commit rather than assuming it:

```sh {"name":"verify-registry-artifact"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
LOCAL="target/package/scarp-${VERSION}.crate"
test -f "$LOCAL" || { echo "FAIL: $LOCAL missing — nothing to compare against"; exit 1; }

DL="$(mktemp -d)/scarp-${VERSION}.crate"
curl -sSL -A "scarp-release-runbook (henry-filgueiras)" \
  "https://crates.io/api/v1/crates/scarp/${VERSION}/download" -o "$DL"
echo "downloaded: $(shasum -a 256 "$DL" | cut -d' ' -f1)"
echo "local:      $(shasum -a 256 "$LOCAL" | cut -d' ' -f1)"
```

Byte-identity is what held for `0.1.0`, but it is an observation, not a
guarantee Cargo makes. A difference is worth investigating before you
accept it.

---

## 8. Prove the shipped binary, not the local one

One self-contained cell: it derives the version, installs **that exact
version from crates.io** into isolated directories, and drives the
resulting binary through every surface this release exists to ship. It
never invokes a bare `scarp`, so nothing can silently resolve to this
checkout's build or to `~/.cargo/bin`.

```sh {"name":"verify-shipped-surfaces"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
echo "version under test: $VERSION"

CH="$(mktemp -d)"
ROOT="$(mktemp -d)"
PLAY="$(mktemp -d)"

echo "cold install from crates.io (isolated CARGO_HOME):"
( export CARGO_HOME="$CH"; time cargo install scarp --version "$VERSION" --locked --root "$ROOT" )

BIN="$ROOT/bin/scarp"
test -x "$BIN" || { echo "FAIL: no installed binary at $BIN"; exit 1; }
echo "installed binary: $BIN"

REPORTED="$("$BIN" --version)"
echo "reported: $REPORTED"
if [[ "$REPORTED" != "scarp $VERSION" ]]; then
  echo "FAIL: installed binary reports \`$REPORTED\`, expected \`scarp $VERSION\`"
  exit 1
fi

cd "$PLAY"
"$BIN" init
"$BIN" new log "shipped-surface check"
"$BIN" new principle "shipped-surface check"
"$BIN" new maintenance "shipped-surface check"
"$BIN" list maintenance

printf 'Closed from the release runbook as a shipped-surface check.\n' > "$PLAY/result.md"
"$BIN" close maintenance:1 --body-file "$PLAY/result.md"

if ! "$BIN" show maintenance:1 | grep -q '^## Result'; then
  echo "FAIL: close --body-file did not write a Result section"
  "$BIN" show maintenance:1
  exit 1
fi
echo "terminal narrative written"

"$BIN" doctor

# The proposal surfaces this release adds. `--help` is the whole check
# that can run here: exercising them for real would need an
# authenticated `gh` and a live issue, which is a dogfooding step and
# not a release gate.
"$BIN" proposal reconcile --help > /dev/null
echo "proposal reconcile present"

REALIZE="$("$BIN" proposal realize --help)"
if ! grep -q -- '--sprint' <<<"$REALIZE"; then
  echo "FAIL: installed binary has no \`proposal realize --sprint\`"
  echo "$REALIZE"
  exit 1
fi
echo "proposal realize --sprint present"

# The classification refusal, proven rather than assumed: with no `gh`
# on PATH the integration is unavailable (exit 11), which is also the
# proof that every ordinary command above ran without one.
if PATH=/nonexistent "$BIN" proposal list > /dev/null 2>&1; then
  echo "FAIL: \`proposal list\` succeeded with no \`gh\` on PATH"
  exit 1
fi
echo "proposal commands refuse cleanly without gh"

echo "shipped surfaces verified against $BIN"
```

`Installed package \`scarp vX.Y.Z\`` with **no path or Git source in the
parentheses** is how a registry install is distinguished from a local
one. The `time` output is the cold-install measurement for
`archaeology/ideas/0035-*.md`; record it with the usual caveat that the
number describes your machine and is not a bound.

A surface that works from `cargo run` here but fails above did not ship.

---

## 9. Live surfaces

```sh {"name":"verify-docs-rs"}
set -euo pipefail
VERSION="$(cargo pkgid | sed 's/.*[#@]//')"
curl -sS -A "scarp-release-runbook (henry-filgueiras)" \
  "https://docs.rs/crate/scarp/${VERSION}/status.json"
echo
```

`{"doc_status": true}` is a successful terminal state. Queued is not
built: docs.rs builds on nightly in a network-blocked sandbox with a
fifteen-minute limit, so it can fail where a local `cargo doc` never
does. A failure is a finding, not a delay.

Then, by hand, on <https://crates.io/crates/scarp> in both light and dark
theme:

- the wordmark renders in both themes, with no broken-image placeholder;
- both Mermaid diagrams render;
- every README link resolves — including the absolute links into
  `archaeology/` and `docs/`, which are absolute precisely because those
  directories are not packaged;
- the sidebar shows the right MSRV, license, size, and docs.rs link;
- no stale product identity: no `Strata`, no `SCARP`, no `.strata.toml`;
- every same-page anchor actually moves the viewport.

The last one has bitten before. `0.1.0` shipped a `#quickstart` anchor
that worked on GitHub and was dead on crates.io, because crates.io
prefixes heading ids with `user-content-` and does not rewrite
author-written same-page hrefs. Check anchors on the crates.io rendering
specifically; GitHub is not evidence for it.

---

## 10. If something is wrong after publication

Classify honestly before acting.

- **Yank only when the version is actively harmful**, not merely
  imperfect. A cosmetic README, logo, or metadata defect does not justify
  one. `0.1.0` shipped with a dead anchor and was not yanked; the defect
  was recorded and repaired in the next version.
- A yank hides the version from future resolution. It does not delete it,
  and it does not free the number.
- The publish token was revoked in section 5.3 and had no `yank` scope
  anyway, so yanking requires minting a new token. That friction is the
  severity checkpoint, not an obstacle to route around.
- Open a remediation item for the fix. Do not rewrite the record to
  suggest the release was clean, and do not describe a flawed release as
  successful. The version stays in the record either way.

---

## 11. Close the archaeology — a handoff, not a cell

There is no closure cell. Closing the maintenance items is a handoff:
Henry supplies the captured provenance, and Claude writes the `Result`
and performs the transitions. A cell named `close` that ran a test suite
and closed nothing would be a lie in the shape of a command.

Capture and hand over:

- the release-source SHA (the pushed bump commit);
- the `cargo publish` output verbatim;
- the token's endpoint and crate scopes, and confirmation that
  `cargo logout` ran and the token was revoked in the browser;
- the registry metadata and both SHA-256 lines from
  `verify-registry-artifact`;
- the cold-install timing from `verify-shipped-surfaces`;
- the browser checklist verdict from section 9;
- anything you had to adapt from this runbook.

Claude then closes maintenance 3 with that `Result`, and closes
maintenance 2, which maintenance 3 subsumes, in the same change.

The closure commit comes **after** the tagged commit, necessarily: the
publish output, the registry checksum, the tag's own hash, and the
release URL could not truthfully exist inside the commit they describe.
This is an ordering consequence, not an oversight — do not "correct" the
tag to include it, which would make it name a tree that was never
published.

If a step here turned out to be wrong, repair this file in the same
change as the `Result`. That repair is the difference between a runbook
and a transcript.

---

## 12. The itinerary

Run in this order. Read the output of each cell before starting the next.

```text {"excludeFromRunAll":"true"}
RB=docs/release-runbook.md
rr() { runme run --filename "$RB" "$@"; }

runme ls --filename "$RB"

rr preflight              # clean, pushed, CI-green base
rr bump-version           # prompts for the number
rr commit-bump            # this commit is the release source
rr push-release-source
rr wait-for-ci
rr preflight              # again: the bump commit is now the proven one

rr package-list
rr package-build
rr publish-dry-run
rr verify-crate-contents  # keep the printed SHA-256
rr check-version-free     # must report HTTP 404

# Browser: mint the shortest-lived scarp-scoped publish-update token (5.1).
rr cargo-login
rr publish                # irreversible; keep the output verbatim

# Run next regardless of how publish ended — cleanly or ambiguously.
rr cargo-logout
# Browser: revoke that token now (5.3). Nothing below needs it.

rr tag
rr push-tag               # only the version tag
rr verify-tag
rr github-release         # notes at the prompt, then Ctrl-D
rr verify-registry
rr verify-registry-artifact
rr verify-shipped-surfaces
rr verify-docs-rs

# Browser: the section 9 checklist on the live crates.io page.
# Then capture the section 11 provenance and hand it to Claude, who
# closes maintenance 3 and the maintenance 2 it subsumes.
```

Three points interrupt the sequence with browser work: minting the token
before `cargo-login`, revoking it after `cargo-logout`, and the live-page
checklist at the end. Nothing else leaves the terminal.
