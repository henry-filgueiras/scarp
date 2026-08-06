//! Realizing remote proposals into canonical artifacts.
//!
//! A GitHub issue carries **mutation intent**; it is a proposal, not the
//! artifact. Scarp alone realizes canonical state, and it does so only
//! when an operator asks, from a machine that already holds the authority
//! to commit. Nothing here runs unattended, nothing holds a credential,
//! and nothing pushes.
//!
//! # Why `gh`
//!
//! GitHub is reached by shelling out to the installed, authenticated `gh`
//! binary, exactly as narrowly scoped Git-aware features shell out to
//! `git`. Scarp therefore acquires no HTTP client, no API SDK, no token
//! storage, and no credential handling: it borrows the operator's
//! existing session or does nothing at all.
//!
//! # Optional, always
//!
//! Every operation here is unavailable — never broken — when `gh` is
//! absent, unauthenticated, or the repository has no GitHub remote. Those
//! are [`Error::IntegrationUnavailable`], and no ordinary Scarp command
//! is affected by any of them.
//!
//! # Not synchronization
//!
//! Realization is one-shot. The issue is never canonical, nothing mirrors
//! state back, and closing or deleting a proposal invalidates nothing.
//! The realized artifact records where it came from in its `proposal:`
//! front matter, which is provenance, not a live link.
//!
//! # Reconciliation projects outward, once
//!
//! [`reconcile`] runs in the opposite direction to [`realize`], and is a
//! weaker thing than it looks: it reports a fact the repository already
//! established, and writes nothing canonical. Its one invariant is that
//! the fact must actually be established — a proposal is closed only
//! because the branch a reader would see already contains the artifact
//! claiming it. That is why reconciliation cannot be folded into
//! `realize`, which runs when the artifact exists on exactly one disk.
//!
//! It is terminal in both directions. Nothing revisits the issue when the
//! idea is later adopted or rejected, and nothing here ever reopens,
//! edits, or re-reads what it closed.

use std::collections::BTreeMap;
use std::process::Command;

use serde::Serialize;

use crate::artifact::{self, NewArtifact};
use crate::error::Error;
use crate::read;

/// What a proposal issue asks the repository for, decided by the label
/// its issue form applied.
///
/// Two classes, enumerated rather than configured. A registry of labels
/// to collections would be a framework built for one more entry than
/// exists; what the two classes actually need is different *semantics*
/// — see [`Class::creation_aware`] — and that is not something a lookup
/// table would have expressed anyway.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    /// An `idea` proposal, realized as a parked idea. Ideas are never
    /// load-bearing, which is what made this the class that went first.
    Idea,
    /// A `bug` proposal, realized as bounded investigative work: a
    /// pending maintenance item, or a pending task when an active sprint
    /// is named.
    ///
    /// Realizing one accepts an obligation to investigate. It does not
    /// assert the reporter's diagnosis is correct, and nothing here ever
    /// says a defect was confirmed.
    Bug,
}

/// The recognized proposal labels, and what each realizes into. Order is
/// the order refusals name them in.
const RECOGNIZED: &[(&str, Class)] = &[("idea", Class::Idea), ("bug", Class::Bug)];

impl Class {
    /// The GitHub label that selects this class.
    fn label(self) -> &'static str {
        match self {
            Class::Idea => "idea",
            Class::Bug => "bug",
        }
    }

    /// The collection `realize` targets with no `--sprint`.
    fn default_target(self) -> &'static str {
        match self {
            Class::Idea => "idea",
            Class::Bug => "maintenance",
        }
    }

    /// Whether the artifact's *existence* on the default branch is what
    /// the filer was waiting for.
    ///
    /// For an idea it is: the idea is the deliverable, so creating it
    /// discharges the proposal. For a bug it is not — the filer is
    /// waiting on an outcome, and closing their issue to announce that a
    /// tracking item now exists would be worse than saying nothing. That
    /// is the whole asymmetry between the two classes, and it lives here
    /// rather than in reconciliation's control flow.
    fn creation_aware(self) -> bool {
        matches!(self, Class::Idea)
    }
}

/// Classify one issue from its labels, refusing anything underdetermined.
///
/// Unrelated labels are ignored — a proposal may also be tagged
/// `documentation` or `good first issue` without that meaning anything
/// here — but exactly one *recognized* label must remain. Both classes at
/// once is a real ambiguity about what the operator is asking for, and
/// guessing at it would create the wrong kind of artifact in the
/// canonical record.
///
/// Pure, and called before anything is allocated or written, so the
/// refusal ordering that keeps a stranger's report from becoming
/// canonical state by accident is testable without a network.
fn classify(number: u64, labels: &[String]) -> Result<Class, Error> {
    let found: Vec<Class> = RECOGNIZED
        .iter()
        .filter(|(name, _)| labels.iter().any(|label| label == name))
        .map(|(_, class)| *class)
        .collect();

    match found.as_slice() {
        [only] => Ok(*only),
        [] => Err(Error::InvalidInvocation {
            message: format!(
                "issue {number} is not a proposal: it carries neither the \
                 `idea` nor the `bug` label, and Scarp acts only on issues \
                 filed through a proposal form — check the number, or label \
                 the issue on GitHub if it really is one"
            ),
        }),
        several => Err(Error::InvalidInvocation {
            message: format!(
                "issue {number} carries {} recognized proposal labels ({}); \
                 exactly one decides what it realizes into, and Scarp will \
                 not guess between an idea and a bug report — remove the \
                 wrong label on GitHub, then retry",
                several.len(),
                several
                    .iter()
                    .map(|class| format!("`{}`", class.label()))
                    .collect::<Vec<_>>()
                    .join(" and ")
            ),
        }),
    }
}

/// Prefix for the title of work realized from a bug report.
///
/// The report's own title is a stranger's claim about what is wrong. The
/// artifact's title is the project's own statement of what it undertook,
/// and it undertook an investigation.
const INVESTIGATE_PREFIX: &str = "Investigate reported behavior: ";

/// GitHub's placeholder for an optional form field the filer left blank.
/// It is presentation, not content, and must never reach an artifact.
const NO_RESPONSE: &str = "_No response_";

/// Marks a reconciliation comment as Scarp's own.
///
/// Invisible when rendered, and it lives in a GitHub comment rather than
/// in canonical state — the artifact learns nothing from reconciliation,
/// so there is nothing to record on the Scarp side. Its whole job is
/// letting a re-run tell "I already commented" from "someone else did",
/// which prose matching could not do reliably.
const RECONCILED_MARKER: &str = "<!-- scarp:reconciled -->";

/// One open proposal, as listed.
#[derive(Debug, Serialize)]
pub struct ProposalSummary {
    /// The issue number, which is how an operator names it to `realize`.
    pub number: u64,
    /// The issue title, which becomes the artifact's title verbatim.
    pub title: String,
    /// The issue's canonical URL, stamped into a realized artifact.
    pub url: String,
    /// The collection `scarp proposal realize` would create from this
    /// proposal with no `--sprint`: `idea` or `maintenance`.
    ///
    /// Stated rather than left to be inferred. With one recognized label
    /// the target was derivable from the command itself; with two it is
    /// a property of the issue, and a listing that made the operator go
    /// look it up would be withholding the one fact that decides what
    /// `realize` does next.
    pub target: &'static str,
    /// Repository-relative path of the artifact already realized from
    /// this proposal, when there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realized_as: Option<String>,
}

/// One proposal's full payload.
struct Proposal {
    title: String,
    body: String,
    url: String,
    labels: Vec<String>,
}

/// Run `gh` with `args` and return its stdout.
///
/// Every failure mode is an unavailable integration rather than a crash:
/// a missing binary, an unauthenticated session, and a network problem
/// are all things the operator can fix, and none of them says anything
/// about the repository.
fn gh(operation: &str, args: &[&str]) -> Result<String, Error> {
    let unavailable = |reason: String, remedy: &str| Error::IntegrationUnavailable {
        operation: operation.to_string(),
        requirement: "an authenticated `gh`".to_string(),
        reason,
        remedy: remedy.to_string(),
    };

    let output = Command::new("gh").args(args).output().map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            unavailable(
                "`gh` is not on PATH".to_string(),
                "install the GitHub CLI (https://cli.github.com), or create \
                 the artifact directly with `scarp new idea`",
            )
        } else {
            unavailable(
                format!("`gh` could not be run: {source}"),
                "check that `gh` is executable",
            )
        }
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let reason = stderr.trim();
        let remedy = if reason.contains("gh auth login") || reason.contains("authentication") {
            "run `gh auth login`"
        } else {
            "check the reported `gh` failure, then retry"
        };
        return Err(unavailable(
            if reason.is_empty() {
                format!("`gh {}` failed", args.join(" "))
            } else {
                reason.to_string()
            },
            remedy,
        ));
    }

    String::from_utf8(output.stdout).map_err(|_| {
        unavailable(
            "`gh` returned output that is not UTF-8".to_string(),
            "retry, and report this if it persists",
        )
    })
}

/// Run `gh` where a missing resource is an answer rather than a failure.
///
/// `gh api` reports an absent path as an ordinary command failure, which
/// [`gh`] correctly reads as an unavailable integration. For the one
/// question reconciliation asks — *is this file on the default branch* —
/// "no" is the useful half of the answer, not an error.
fn gh_absent_is_none(operation: &str, args: &[&str]) -> Result<Option<String>, Error> {
    match gh(operation, args) {
        Ok(out) => Ok(Some(out)),
        Err(Error::IntegrationUnavailable { reason, .. }) if is_absent(&reason) => Ok(None),
        Err(other) => Err(other),
    }
}

/// Whether a `gh api` failure means the resource does not exist.
///
/// Matched on `gh`'s rendered message rather than a status code, because
/// `gh` exposes no exit code that distinguishes 404 from any other API
/// failure. Narrow on purpose: a false positive here would report an
/// artifact as unlanded during an outage, which refuses safely, whereas a
/// looser match risks reporting a real failure as a clean "not there".
fn is_absent(reason: &str) -> bool {
    reason.contains("HTTP 404")
}

/// The GitHub repository these commands act on.
struct Repo {
    /// `owner/name`, as GitHub knows it.
    name_with_owner: String,
    /// The branch a reader arriving at the repository sees. Asked rather
    /// than assumed to be `main`: the landing invariant is about what is
    /// canonical for readers, and a consumer's repository may not agree
    /// with this one about the name.
    default_branch: String,
}

/// Confirm the repository has a GitHub remote, and describe it.
///
/// Checked before anything else so a non-GitHub repository gets an
/// immediate, specific refusal rather than a confusing `gh` error.
fn github_repo(operation: &str) -> Result<Repo, Error> {
    let not_github = |reason: String| Error::IntegrationUnavailable {
        operation: operation.to_string(),
        requirement: "a GitHub repository".to_string(),
        reason,
        remedy: "proposals are a GitHub-only convenience; every other Scarp \
                 command works here, and `scarp new idea` creates the same \
                 artifact directly"
            .to_string(),
    };

    // `gh` reports "not a repository" and "no GitHub remote" as ordinary
    // failures. Re-frame them: the operator's `gh` is fine, the repository
    // simply is not on GitHub, and saying otherwise sends them to fix the
    // wrong thing.
    let raw = gh(
        operation,
        &["repo", "view", "--json", "nameWithOwner,defaultBranchRef"],
    )
    .map_err(|err| match &err {
        Error::IntegrationUnavailable { reason, .. }
            if reason.contains("not a git repository")
                || reason.contains("no git remotes")
                || reason.contains("none of the git remotes")
                || reason.contains("not found") =>
        {
            not_github(reason.clone())
        }
        _ => err,
    })?;

    let value: serde_json::Value = serde_json::from_str(&raw).map_err(|source| {
        not_github(format!("`gh repo view` returned unreadable JSON: {source}"))
    })?;
    let name_with_owner = value
        .get("nameWithOwner")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| not_github("this repository has no GitHub remote".to_string()))?;
    let default_branch = value
        .get("defaultBranchRef")
        .and_then(|v| v.get("name"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| not_github(format!("`{name_with_owner}` reports no default branch")))?;

    Ok(Repo {
        name_with_owner,
        default_branch,
    })
}

/// Map GitHub's rendered issue-form body onto Scarp's body grammar.
///
/// The form renders each field as `### <Label>`; `--body-file` expects
/// `## <Section>`. Only lines that are exactly `### ` plus one of the
/// collection's own section names are promoted — never a general
/// `###`-to-`##` rewrite, which would turn a filer's own subheading into
/// a forged section. Fenced code blocks are skipped entirely, so a
/// proposal that quotes Markdown is quoting, not declaring.
fn to_body(issue_body: &str, sections: &[&str]) -> String {
    let mut out = String::with_capacity(issue_body.len());
    let mut fence: Option<(char, usize)> = None;

    for line in issue_body.replace("\r\n", "\n").lines() {
        let trimmed = line.trim_start();
        let marker = trimmed
            .chars()
            .next()
            .filter(|c| *c == '`' || *c == '~')
            .map(|c| (c, trimmed.chars().take_while(|x| *x == c).count()))
            .filter(|(_, run)| *run >= 3);

        if let Some((ch, run)) = marker {
            match fence {
                None => fence = Some((ch, run)),
                Some((open, len))
                    if ch == open && run >= len && trimmed[run..].trim().is_empty() =>
                {
                    fence = None;
                }
                Some(_) => {}
            }
            out.push_str(line);
            out.push('\n');
            continue;
        }

        if fence.is_none()
            && let Some(name) = line.strip_prefix("### ")
            && sections.contains(&name.trim())
        {
            out.push_str("## ");
            out.push_str(name.trim());
            out.push('\n');
            continue;
        }

        // GitHub's blank-field placeholder is presentation. Dropping it
        // leaves the section empty, exactly as the bare template renders.
        if fence.is_none() && line.trim() == NO_RESPONSE {
            continue;
        }

        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Find the artifact already realized from `url`, if any.
///
/// Reads the managed `proposal:` front-matter field rather than searching
/// prose, so the answer is exact. It sees only what is on this branch;
/// the merge-time case is `doctor`'s `duplicate-proposal` finding.
///
/// Every collection the stamp can land on is searched, in a fixed order,
/// because the uniqueness rule is global: **one proposal realizes at most
/// one canonical artifact**, not one per collection. A per-collection
/// check would let the same report become both a maintenance item and a
/// task, which is exactly the duplicate the field exists to prevent.
fn realized_from(root: &std::path::Path, url: &str) -> Result<Option<read::Summary>, Error> {
    let stamped = |artifacts: Vec<read::Artifact>| {
        artifacts
            .into_iter()
            .find(|a| a.summary.proposal.as_deref() == Some(url))
            .map(|a| a.summary)
    };
    for artifacts in [
        read::scan(root, &read::IDEA)?,
        read::scan(root, &read::MAINTENANCE)?,
        read::scan_tasks(root)?,
    ] {
        if let Some(found) = stamped(artifacts) {
            return Ok(Some(found));
        }
    }
    Ok(None)
}

/// Nest a reporter's Markdown beneath the one canonical section that will
/// hold it.
///
/// A bug report is prose Scarp did not write, landing whole inside a
/// section Scarp owns. Every unindented ATX heading outside a fenced code
/// block is therefore demoted to level three, so no reporter-authored
/// line can become — or split — a `## ` section the template defines.
/// Unindented is the whole surface that needs covering: the body parser
/// recognizes a section only as a line beginning exactly `## `, so a
/// heading it would not read as one needs no rewriting.
///
/// Fenced code survives byte for byte. A report quoting Markdown is
/// quoting, and a shell transcript's `# comment` is a comment; those
/// lines are the evidence being reported, and rewriting them would
/// corrupt the report to protect a structure they were never going to
/// reach.
fn to_report(issue_body: &str) -> String {
    let mut out = String::with_capacity(issue_body.len());
    let mut fence: Option<(char, usize)> = None;

    for line in issue_body.replace("\r\n", "\n").lines() {
        if let Some((marker, run)) = artifact::fence_marker(line) {
            artifact::advance_fence(&mut fence, line, marker, run);
            out.push_str(line);
            out.push('\n');
            continue;
        }
        if fence.is_none() {
            // GitHub's blank-field placeholder is presentation, exactly as
            // it is on the idea path.
            if line.trim() == NO_RESPONSE {
                continue;
            }
            if let Some(demoted) = demote_heading(line) {
                out.push_str(&demoted);
                out.push('\n');
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out.trim().to_string()
}

/// Rewrite an unindented level-one or level-two ATX heading to level
/// three, or `None` when the line is not one.
///
/// `#hashtag` is not a heading — ATX requires the run to be followed by
/// space or end of line — and deeper headings are already safe.
fn demote_heading(line: &str) -> Option<String> {
    let hashes = line.chars().take_while(|c| *c == '#').count();
    if !(1..=2).contains(&hashes) {
        return None;
    }
    let rest = &line[hashes..];
    if !(rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t')) {
        return None;
    }
    Some(format!("###{rest}"))
}

/// Scarp's own framing of a realized bug report, which is not the
/// reporter's and must not read as though it were.
///
/// Mechanically generated, identical for every report, and deliberately
/// explicit that promotion is an obligation to look rather than a finding
/// — the artifact exists from the moment it is created, and someone will
/// read it long before anyone has investigated anything.
fn investigation_preamble(url: &str) -> String {
    format!(
        "Investigate the behavior reported in {url}.\n\
         \n\
         Realizing a report accepts an obligation to look into it; it does \
         not assert that the reporter's diagnosis is correct. Close this \
         with a `Result` stating the finding and its evidence — a confirmed \
         defect, working as intended, unreproducible, a duplicate, already \
         handled, or consciously declined are all honest terminal \
         findings.\n\
         \n\
         ### The report as filed\n"
    )
}

/// The maintenance body a realized bug report becomes: one `## Work`
/// section, Scarp's framing, then the nested report.
fn maintenance_body(report: &str, url: &str) -> String {
    format!("## Work\n\n{}\n{report}\n", investigation_preamble(url))
}

/// The task body a realized bug report becomes.
///
/// The acceptance criteria are generated rather than solicited: an
/// outside reporter has no way to know what this project considers done,
/// and the answer is the same for every promoted report — reproduce it or
/// record the attempt, conclude with evidence, and cover a real
/// behavioral change with a test that fails without the fix.
fn task_body(report: &str, url: &str) -> String {
    format!(
        "## Objective\n\n{}\n{report}\n\
         \n\
         ## Acceptance criteria\n\
         \n\
         - The reported behavior is reproduced, or the attempt to reproduce \
         it is recorded with the exact commands run and the output \
         observed.\n\
         - The task closes with a `Result` stating the terminal finding and \
         the evidence for it.\n\
         - If a confirmed defect changes behavior, a regression test \
         covering it lands with the change and fails without it.\n",
        investigation_preamble(url)
    )
}

/// Where and when an artifact reached the branch readers see.
struct Landing {
    /// The commit that *introduced* the artifact, not the most recent one
    /// to touch it. A later edit is not the landing.
    commit: String,
    /// Permalink to the artifact at that commit — pinned rather than
    /// branch-relative, because the comment citing it is never revisited.
    permalink: String,
    /// Permalink to the commit itself.
    commit_url: String,
}

/// The conventional abbreviation of a commit sha.
///
/// A full sha is forty characters of hex that no reader parses; the
/// abbreviation is what every other tool shows, and the link behind it
/// carries the exact identity for anyone who needs it.
fn short_commit(sha: &str) -> &str {
    let end = sha.char_indices().nth(7).map_or(sha.len(), |(i, _)| i);
    &sha[..end]
}

/// Pick the commit that introduced a path from `gh`'s newest-first list.
///
/// One sha per line, so the introducing commit is the last line. Kept
/// separate from the call that produces it so the selection — the part
/// with a rule worth stating — is testable without a network.
fn introducing_commit(shas: &str) -> Option<&str> {
    shas.lines().map(str::trim).rfind(|s| !s.is_empty())
}

/// Prove that `path` is on the branch a reader sees, and say what put it
/// there.
///
/// **GitHub is asked, not the local checkout.** The alternative — reading
/// the `origin/main` remote-tracking ref — is cheaper and fails safe when
/// stale, but it answers a question about this machine's last fetch while
/// the invariant is about what a reader would find. The cost that would
/// normally argue against a network call does not apply here:
/// reconciliation has to reach GitHub anyway to comment and close, so
/// there is no offline path being sacrificed, and no `git` shell-out has
/// to be introduced into a module that has none.
///
/// Two questions, deliberately separate. Presence answers the invariant —
/// a file added and later deleted has commits but is not there — and the
/// commit list answers what to cite.
fn landed(operation: &str, repo: &Repo, path: &str) -> Result<Option<Landing>, Error> {
    let contents = format!(
        "repos/{}/contents/{}?ref={}",
        repo.name_with_owner, path, repo.default_branch
    );
    if gh_absent_is_none(operation, &["api", &contents])?.is_none() {
        return Ok(None);
    }

    // `--jq` streams one sha per page-item, which sidesteps `--paginate`
    // emitting several concatenated arrays that are not themselves JSON.
    let commits = format!(
        "repos/{}/commits?path={}&sha={}&per_page=100",
        repo.name_with_owner, path, repo.default_branch
    );
    let shas = gh(
        operation,
        &["api", "--paginate", "--jq", ".[].sha", &commits],
    )?;

    Ok(introducing_commit(&shas).map(|commit| Landing {
        permalink: format!(
            "https://github.com/{}/blob/{commit}/{path}",
            repo.name_with_owner
        ),
        commit_url: format!(
            "https://github.com/{}/commit/{commit}",
            repo.name_with_owner
        ),
        commit: commit.to_string(),
    }))
}

/// Parse `gh` JSON output, reporting a malformed payload as an
/// unavailable integration rather than a repository problem.
fn parse_json<T: serde::de::DeserializeOwned>(
    operation: &str,
    command: &str,
    raw: &str,
) -> Result<T, Error> {
    serde_json::from_str(raw).map_err(|source| Error::IntegrationUnavailable {
        operation: operation.to_string(),
        requirement: "a GitHub repository".to_string(),
        reason: format!("`{command}` returned unreadable JSON: {source}"),
        remedy: "retry, and report this if it persists".to_string(),
    })
}

/// Collect one string field out of each element of a JSON array field —
/// label names, comment bodies — skipping anything shaped unexpectedly.
fn json_strings(value: &serde_json::Value, field: &str, key: &str) -> Vec<String> {
    value
        .get(field)
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.get(key).and_then(|v| v.as_str()))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// Open proposals of every recognized class, newest issue number first,
/// each annotated with its default target and with the artifact it has
/// already been realized as.
pub fn list(root: &std::path::Path) -> Result<Vec<ProposalSummary>, Error> {
    const OPERATION: &str = "`scarp proposal list`";
    github_repo(OPERATION)?;

    // One query per recognized label, deduplicated by issue number.
    //
    // Not one query with repeated `--label` flags: GitHub *intersects*
    // them, so a single call would return only the issues carrying every
    // recognized label at once — which is precisely the ambiguous set
    // that must be refused, and an empty result for the ordinary case.
    // A `BTreeMap` keyed by number also makes the union deterministic
    // regardless of what order `gh` returned either page in.
    let mut issues: BTreeMap<u64, serde_json::Value> = BTreeMap::new();
    for (label, _) in RECOGNIZED {
        let raw = gh(
            OPERATION,
            &[
                "issue",
                "list",
                "--label",
                label,
                "--state",
                "open",
                "--json",
                "number,title,url,labels",
                "--limit",
                "100",
            ],
        )?;
        for issue in parse_json::<Vec<serde_json::Value>>(OPERATION, "gh issue list", &raw)? {
            let number = issue.get("number").and_then(|v| v.as_u64()).unwrap_or(0);
            issues.insert(number, issue);
        }
    }

    let mut proposals = Vec::new();
    for (number, issue) in issues {
        // A dual-labeled issue refuses the whole listing rather than
        // appearing under a guessed target. A listing is what an operator
        // reads before deciding what to realize, so a confidently wrong
        // row in it is worse than no listing at all.
        let class =
            classify(number, &json_strings(&issue, "labels", "name")).map_err(|err| match err {
                Error::InvalidInvocation { message } => Error::InvalidInvocation {
                    message: format!("cannot list proposals: {message}"),
                },
                other => other,
            })?;
        let url = issue
            .get("url")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        proposals.push(ProposalSummary {
            number,
            title: issue
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            target: class.default_target(),
            realized_as: realized_from(root, url)?.map(|s| s.path),
            url: url.to_string(),
        });
    }
    proposals.sort_by_key(|p| std::cmp::Reverse(p.number));
    Ok(proposals)
}

/// Fetch one proposal, labels included.
///
/// The labels are what decide the target collection, so they are fetched
/// here rather than assumed. Before they were, any issue number at all
/// could be realized as an idea — including one that was never a
/// proposal.
fn view(number: u64) -> Result<Proposal, Error> {
    const OPERATION: &str = "`scarp proposal realize`";
    let raw = gh(
        OPERATION,
        &[
            "issue",
            "view",
            &number.to_string(),
            "--json",
            "number,title,body,url,labels",
        ],
    )?;
    let value: serde_json::Value = parse_json(OPERATION, "gh issue view", &raw)?;
    let field = |name: &str| {
        value
            .get(name)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    Ok(Proposal {
        title: field("title"),
        body: field("body"),
        url: field("url"),
        labels: json_strings(&value, "labels", "name"),
    })
}

/// Realize proposal `number` as the canonical artifact its label asks
/// for: a parked idea, a pending maintenance item, or — with `sprint` —
/// a pending task in that active sprint.
///
/// `sprint` is the explicit `--sprint` selection as `(selector, display)`,
/// carrying exactly the semantics `scarp new task --sprint` gives it. It
/// applies only to a `bug`; an idea is never a sprint's work.
///
/// Refuses rather than duplicating if this proposal already has an
/// artifact on this branch, in any collection. Nothing is committed: the
/// operator reviews the new file and commits it through the ordinary
/// workflow.
///
/// Every refusal below happens before a sequence is allocated or a path
/// is touched. A stranger's report becoming canonical state is the thing
/// this ordering exists to prevent, so classification comes first — Scarp
/// learns what an issue is before it does anything on its behalf.
pub fn realize(
    root: &std::path::Path,
    number: u64,
    sprint: Option<(read::Selector<'_>, &str)>,
) -> Result<NewArtifact, Error> {
    const OPERATION: &str = "`scarp proposal realize`";
    github_repo(OPERATION)?;
    let proposal = view(number)?;
    let class = classify(number, &proposal.labels)?;

    if class.creation_aware() && sprint.is_some() {
        return Err(Error::InvalidInvocation {
            message: format!(
                "cannot realize proposal {number} into a sprint: `--sprint` \
                 attaches a `bug` report to an active sprint as a task, and \
                 this issue is labeled `idea` — an idea is an uncommitted \
                 proposal, never a sprint's committed work, so drop \
                 `--sprint`"
            ),
        });
    }

    if proposal.url.is_empty() {
        return Err(Error::IntegrationUnavailable {
            operation: OPERATION.to_string(),
            requirement: "a GitHub repository".to_string(),
            reason: format!("issue {number} has no URL"),
            remedy: "check the issue number, then retry".to_string(),
        });
    }

    if let Some(existing) = realized_from(root, &proposal.url)? {
        let path = existing.path;
        return Err(Error::ArtifactConflict {
            path: std::path::PathBuf::from(&path),
            reason: format!(
                "proposal {} has already been realized as `{path}`; one \
                 proposal realizes at most one artifact, across every \
                 collection",
                proposal.url
            ),
        });
    }

    if proposal.title.trim().is_empty() {
        return Err(Error::InvalidInvocation {
            message: format!(
                "cannot realize proposal {number}: its title is empty, and the \
                 title becomes the artifact's — retitle the issue, then retry"
            ),
        });
    }

    match class {
        Class::Idea => {
            let body = to_body(&proposal.body, read::IDEA_SECTIONS);
            artifact::create_idea_from(root, &proposal.title, Some(&body), Some(&proposal.url))
        }
        Class::Bug => {
            let report = to_report(&proposal.body);
            let title = format!("{INVESTIGATE_PREFIX}{}", proposal.title.trim());
            match sprint {
                None => artifact::create_maintenance_from(
                    root,
                    &title,
                    Some(&maintenance_body(&report, &proposal.url)),
                    Some(&proposal.url),
                ),
                Some(selection) => artifact::create_task_from(
                    root,
                    &title,
                    Some(selection),
                    Some(&task_body(&report, &proposal.url)),
                    Some(&proposal.url),
                ),
            }
        }
    }
}

/// One proposal's state on GitHub, as observed at invocation.
struct Issue {
    number: u64,
    url: String,
    /// Every label the issue carries, classified by [`plan`] rather than
    /// here. An issue that is not a proposal is not this command's
    /// business, whatever else it says, and one that is two proposals at
    /// once is not either.
    labels: Vec<String>,
    open: bool,
    /// Whether Scarp has already commented here, by
    /// [`RECONCILED_MARKER`] rather than by reading prose.
    commented: bool,
}

/// What reconciliation should do about a proposal.
#[derive(Debug, PartialEq, Eq)]
enum Plan {
    /// Post the reconciliation comment, then close.
    CommentAndClose,
    /// The comment landed on an earlier run that failed before closing.
    CloseOnly,
    /// Nothing to do, and saying so is a success.
    Nothing,
}

/// Decide what to do from everything observed, or refuse with a reason.
///
/// Pure, and separate from the calls that gather its inputs, because the
/// ordering of these refusals *is* the safety property: an issue must be
/// proved to be a proposal before it is treated as one, and an artifact
/// must be proved to be on the default branch before anything public is
/// said about it. Testing that ordering should not require a network or
/// a repository.
fn plan(
    operation: &str,
    issue: &Issue,
    realized: Option<&read::Summary>,
    landing: Option<&Landing>,
) -> Result<Plan, Error> {
    // The same exactly-one-recognized-label rule realization enforces,
    // and enforced first for the same reason: commenting on an issue is
    // public and posted under the repository's name, so Scarp proves what
    // an issue *is* before it says anything on it.
    classify(issue.number, &issue.labels)?;

    // Closing is terminal, so an already-closed proposal is finished
    // whatever put it there — including a human who closed it as
    // unwanted. Reopening to reconcile would be synchronization, which
    // this is deliberately not.
    if !issue.open {
        return Ok(Plan::Nothing);
    }

    let unmet = |requirement: &str, observed: String, remedy: &str| Error::PreconditionUnmet {
        operation: operation.to_string(),
        requirement: requirement.to_string(),
        observed,
        remedy: remedy.to_string(),
    };

    let Some(realized) = realized else {
        return Err(unmet(
            "an artifact realized from this proposal",
            format!(
                "no idea on this branch records `{}` as its proposal",
                issue.url
            ),
            &format!(
                "realize it first with `scarp proposal realize {}`, \
                 or switch to the branch that realized it",
                issue.number
            ),
        ));
    };

    let Some(_) = landing else {
        return Err(unmet(
            "the realizing artifact on the default branch",
            format!(
                "`{}` exists here but is not on the branch a reader sees",
                realized.path
            ),
            "commit and push it, then retry — closing the proposal would \
             advertise an artifact nobody else can find",
        ));
    };

    Ok(if issue.commented {
        Plan::CloseOnly
    } else {
        Plan::CommentAndClose
    })
}

/// Render the comment that supersedes a proposal.
///
/// Written for someone arriving cold, months later, and never revised:
/// nothing revisits this comment, so it states a settled fact rather than
/// a status. Kept pure so its wording is testable.
fn comment_body(realized: &read::Summary, landing: &Landing) -> String {
    format!(
        "Realized as **{reference}** — this proposal is now part of the \
         canonical record.\n\
         \n\
         | | |\n\
         |---|---|\n\
         | Artifact | [`{path}`]({permalink}) |\n\
         | Stable id | `{id}` |\n\
         | Landed in | [`{short}`]({commit_url}) |\n\
         \n\
         The Scarp artifact is the canonical record; this issue is not. \
         Its lifecycle is tracked there, and nothing will update this \
         issue again — editing or reopening it does not change the \
         artifact.\n\
         \n\
         {RECONCILED_MARKER}\n",
        reference = realized.reference(),
        path = realized.path,
        permalink = landing.permalink,
        id = realized.id,
        short = short_commit(&landing.commit),
        commit_url = landing.commit_url,
    )
}

/// Fetch one proposal's current state.
fn issue_state(operation: &str, number: u64) -> Result<Issue, Error> {
    let raw = gh(
        operation,
        &[
            "issue",
            "view",
            &number.to_string(),
            "--json",
            "number,url,state,labels,comments",
        ],
    )?;
    let value: serde_json::Value = parse_json(operation, "gh issue view", &raw)?;

    Ok(Issue {
        number,
        url: value
            .get("url")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string(),
        labels: json_strings(&value, "labels", "name"),
        open: value.get("state").and_then(|v| v.as_str()) == Some("OPEN"),
        commented: json_strings(&value, "comments", "body")
            .iter()
            .any(|b| b.contains(RECONCILED_MARKER)),
    })
}

/// What reconciling one proposal did.
#[derive(Debug, Serialize)]
pub struct Reconciled {
    /// The issue number that was reconciled.
    pub number: u64,
    /// What happened: `reconciled`, `closed`, or `already-reconciled`.
    pub outcome: &'static str,
    /// The canonical artifact that supersedes the proposal, when the
    /// proposal was still open and had one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    /// That artifact's stable id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// That artifact's human reference, e.g. `idea:40`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// The commit that introduced the artifact on the default branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}

/// Close the loop on proposal `number`: say what realized it, then close.
///
/// Writes nothing canonical — it reads the repository and mutates only
/// GitHub. Refuses unless the realizing artifact is already on the branch
/// a reader sees, because closing a proposal is a public, terminal claim
/// that the record contains its replacement.
///
/// Ordering is the recovery story. The comment goes first, so a run that
/// dies between the two steps leaves a commented, open issue; the next
/// run sees the marker, skips the comment, and closes. There is no state
/// in which a proposal is closed without its explanation.
pub fn reconcile(root: &std::path::Path, number: u64) -> Result<Reconciled, Error> {
    const OPERATION: &str = "`scarp proposal reconcile`";
    let repo = github_repo(OPERATION)?;
    let issue = issue_state(OPERATION, number)?;

    // Gathered in refusal order, so a proposal that will be refused early
    // costs neither a repository scan nor two API calls. `plan` reaches
    // the same verdict either way: what was not looked for is absent.
    let recognized = classify(issue.number, &issue.labels).is_ok();
    let realized = if recognized && issue.open {
        realized_from(root, &issue.url)?
    } else {
        None
    };
    let landing = match &realized {
        Some(summary) => landed(OPERATION, &repo, &summary.path)?,
        None => None,
    };

    let done = |outcome| {
        let (realized, landing) = (realized.as_ref(), landing.as_ref());
        Ok(Reconciled {
            number,
            outcome,
            artifact: realized.map(|s| s.path.clone()),
            id: realized.map(|s| s.id.clone()),
            reference: realized.map(read::Summary::reference),
            commit: landing.map(|l| l.commit.clone()),
        })
    };

    match plan(OPERATION, &issue, realized.as_ref(), landing.as_ref())? {
        Plan::Nothing => return done("already-reconciled"),
        Plan::CommentAndClose => {
            let (summary, landing) = (
                realized.as_ref().expect("planned with an artifact"),
                landing.as_ref().expect("planned with a landing"),
            );
            gh(
                OPERATION,
                &[
                    "issue",
                    "comment",
                    &number.to_string(),
                    "--body",
                    &comment_body(summary, landing),
                ],
            )?;
        }
        Plan::CloseOnly => {}
    }

    gh(
        OPERATION,
        &[
            "issue",
            "close",
            &number.to_string(),
            "--reason",
            "completed",
        ],
    )?;
    done("reconciled")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECTIONS: &[&str] = &["Problem", "Sketch", "Boundaries", "Evidence"];

    #[test]
    fn known_form_headings_are_promoted_to_sections() {
        let issue = "### Problem\n\nIt hurts.\n\n### Evidence\n\nSprint 10.\n";

        let body = to_body(issue, SECTIONS);

        assert_eq!(
            body,
            "## Problem\n\nIt hurts.\n\n## Evidence\n\nSprint 10.\n"
        );
    }

    /// The filer's own subheadings are content. A general `###`-to-`##`
    /// rewrite would forge sections out of them.
    #[test]
    fn unknown_third_level_headings_are_left_alone() {
        let issue = "### Problem\n\n### A subheading of my own\n\ntext\n";

        let body = to_body(issue, SECTIONS);

        assert!(body.starts_with("## Problem\n"), "{body}");
        assert!(body.contains("### A subheading of my own"), "{body}");
    }

    /// A proposal quoting Markdown is quoting, not declaring.
    #[test]
    fn headings_inside_fenced_blocks_are_untouched() {
        let issue = "### Problem\n\n```\n### Sketch\n# not a title\n```\n";

        let body = to_body(issue, SECTIONS);

        assert!(body.contains("\n### Sketch\n"), "{body}");
        assert!(body.contains("\n# not a title\n"), "{body}");
        assert_eq!(body.matches("## Problem").count(), 1, "{body}");
    }

    /// GitHub's blank-field placeholder is presentation, and would
    /// otherwise land in the artifact as prose.
    #[test]
    fn the_no_response_placeholder_is_dropped() {
        let issue = "### Problem\n\nReal.\n\n### Sketch\n\n_No response_\n";

        let body = to_body(issue, SECTIONS);

        assert!(!body.contains("_No response_"), "{body}");
        assert!(body.contains("## Sketch"), "{body}");
    }

    #[test]
    fn crlf_input_is_normalized() {
        let issue = "### Problem\r\n\r\nWindows filed this.\r\n";

        let body = to_body(issue, SECTIONS);

        assert!(!body.contains('\r'), "{body:?}");
        assert!(body.starts_with("## Problem\n"), "{body}");
    }

    // Classification. Pure, and the first thing every command does, so
    // what an issue *is* is settled before anything acts on it.

    fn labels(names: &[&str]) -> Vec<String> {
        names.iter().map(|n| n.to_string()).collect()
    }

    #[test]
    fn one_recognized_label_decides_the_class() {
        assert_eq!(classify(1, &labels(&["idea"])).unwrap(), Class::Idea);
        assert_eq!(classify(1, &labels(&["bug"])).unwrap(), Class::Bug);
    }

    /// A proposal may be tagged anything else without that meaning a
    /// thing here.
    #[test]
    fn unrelated_labels_do_not_participate() {
        let noisy = labels(&["documentation", "bug", "good first issue", "P1"]);

        assert_eq!(classify(1, &noisy).unwrap(), Class::Bug);
    }

    #[test]
    fn no_recognized_label_is_not_a_proposal() {
        let error = classify(9, &labels(&["question"])).unwrap_err();

        assert_eq!(error.code(), "invalid-invocation");
        assert!(error.to_string().contains("is not a proposal"), "{error}");
    }

    /// The ambiguity that must never be guessed at: an idea is never
    /// load-bearing and a promoted report is work the project owes, so
    /// picking the wrong one puts the wrong kind of claim in the record.
    #[test]
    fn both_recognized_labels_refuse_and_name_them() {
        let error = classify(9, &labels(&["bug", "idea"])).unwrap_err();

        assert_eq!(error.code(), "invalid-invocation");
        let message = error.to_string();
        assert!(message.contains("`idea` and `bug`"), "{message}");
        assert!(message.contains("will not guess"), "{message}");
    }

    /// The asymmetry the whole sprint turns on, asserted where it lives.
    #[test]
    fn only_ideas_are_discharged_by_creation() {
        assert!(Class::Idea.creation_aware());
        assert!(!Class::Bug.creation_aware());
        assert_eq!(Class::Idea.default_target(), "idea");
        assert_eq!(Class::Bug.default_target(), "maintenance");
    }

    // Report normalization. A bug report is prose Scarp did not write,
    // landing whole inside a section Scarp owns.

    #[test]
    fn a_reporters_headings_are_nested_beneath_the_canonical_section() {
        let report = to_report("## Work\n\nForged.\n\n# Title\n\n### Already deep\n");

        assert!(report.starts_with("### Work\n"), "{report}");
        assert!(report.contains("\n### Title\n"), "{report}");
        assert!(report.contains("\n### Already deep"), "{report}");
        assert!(!report.contains("\n## "), "{report}");
    }

    /// A report quoting Markdown is quoting. Rewriting the quoted lines
    /// would corrupt the evidence to protect a structure they were never
    /// going to reach.
    #[test]
    fn fenced_code_in_a_report_survives_byte_for_byte() {
        let report = to_report("### Steps\n\n```console\n$ scarp doctor\n# a comment\n```\n");

        assert!(
            report.contains("```console\n$ scarp doctor\n# a comment\n```"),
            "{report}"
        );
    }

    #[test]
    fn a_report_drops_the_no_response_placeholder_and_normalizes_crlf() {
        let report =
            to_report("### Environment\r\n\r\n_No response_\r\n\r\n### Context\r\n\r\nx\r\n");

        assert!(!report.contains('\r'), "{report:?}");
        assert!(!report.contains(NO_RESPONSE), "{report}");
        assert!(report.contains("### Environment"), "{report}");
    }

    #[test]
    fn a_hashtag_is_not_a_heading() {
        assert_eq!(demote_heading("#hashtag"), None);
        assert_eq!(demote_heading("text # here"), None);
        assert_eq!(demote_heading("### already deep"), None);
        assert_eq!(demote_heading("## Work"), Some("### Work".to_string()));
        assert_eq!(demote_heading("#"), Some("###".to_string()));
    }

    /// The generated bodies must name every honest terminal finding, or
    /// the first person to close one as "not a bug" has no precedent to
    /// follow and invents a status instead.
    #[test]
    fn generated_bodies_frame_promotion_as_an_obligation_to_look() {
        let url = "https://github.com/o/r/issues/7";
        let work = maintenance_body("### Observed\n\nx", url);
        let task = task_body("### Observed\n\nx", url);

        for body in [&work, &task] {
            assert!(body.contains(url), "{body}");
            assert!(
                body.contains("does not assert that the reporter's diagnosis is correct"),
                "{body}"
            );
            assert!(body.contains("unreproducible"), "{body}");
            assert!(body.contains("### The report as filed"), "{body}");
        }
        assert!(work.starts_with("## Work\n"), "{work}");
        assert!(task.starts_with("## Objective\n"), "{task}");
        assert!(task.contains("\n## Acceptance criteria\n"), "{task}");
        assert!(task.contains("regression test"), "{task}");
        // The framing is Scarp's, and must never claim a defect exists.
        assert!(!work.contains("the bug"), "{work}");
    }

    // Reconciliation. Every refusal below is decided by `plan`, which is
    // pure precisely so this file needs neither a network nor a
    // repository to prove the ordering that keeps a public, terminal
    // claim honest.

    const OPERATION: &str = "`scarp proposal reconcile`";

    fn proposal_issue() -> Issue {
        Issue {
            number: 3,
            url: "https://github.com/o/r/issues/3".to_string(),
            labels: vec!["idea".to_string()],
            open: true,
            commented: false,
        }
    }

    fn summary() -> read::Summary {
        read::Summary {
            id: "ide_01ABC".to_string(),
            sequence: 40,
            kind: "idea".to_string(),
            status: Some(read::Status::Parked),
            title: "Reconcile realized proposals".to_string(),
            created: "2026-08-01".to_string(),
            sprint: None,
            proposal: Some("https://github.com/o/r/issues/3".to_string()),
            path: "archaeology/ideas/0040-reconcile.md".to_string(),
        }
    }

    fn landing() -> Landing {
        Landing {
            commit: "abc1234def5678".to_string(),
            permalink:
                "https://github.com/o/r/blob/abc1234def5678/archaeology/ideas/0040-reconcile.md"
                    .to_string(),
            commit_url: "https://github.com/o/r/commit/abc1234def5678".to_string(),
        }
    }

    #[test]
    fn a_landed_proposal_is_commented_then_closed() {
        let plan = plan(
            OPERATION,
            &proposal_issue(),
            Some(&summary()),
            Some(&landing()),
        );

        assert_eq!(plan.unwrap(), Plan::CommentAndClose);
    }

    /// The recovery case: a previous run commented and died before
    /// closing. Re-running must finish the job, not say it twice.
    #[test]
    fn an_already_commented_proposal_is_only_closed() {
        let issue = Issue {
            commented: true,
            ..proposal_issue()
        };

        let plan = plan(OPERATION, &issue, Some(&summary()), Some(&landing()));

        assert_eq!(plan.unwrap(), Plan::CloseOnly);
    }

    /// The headline refusal. An artifact that exists only on the
    /// operator's disk must not have a public claim made about it.
    #[test]
    fn an_unlanded_artifact_refuses_rather_than_closing() {
        let error = plan(OPERATION, &proposal_issue(), Some(&summary()), None).unwrap_err();

        assert_eq!(error.code(), "precondition-unmet");
        let message = error.to_string();
        assert!(
            message.contains("archaeology/ideas/0040-reconcile.md"),
            "{message}"
        );
        assert!(message.contains("push"), "{message}");
    }

    /// Distinguishable from the unlanded case by message, and from a
    /// mistyped command by code: nothing is wrong, it is merely early.
    #[test]
    fn an_unrealized_proposal_refuses_and_names_realize() {
        let error = plan(OPERATION, &proposal_issue(), None, None).unwrap_err();

        assert_eq!(error.code(), "precondition-unmet");
        assert!(
            error.to_string().contains("scarp proposal realize 3"),
            "{error}"
        );
    }

    /// An issue that is not a proposal is a wrong argument rather than a
    /// state that will come true later, and gets the other exit code.
    #[test]
    fn a_non_proposal_issue_is_an_invalid_invocation() {
        let issue = Issue {
            labels: Vec::new(),
            ..proposal_issue()
        };

        let error = plan(OPERATION, &issue, Some(&summary()), Some(&landing())).unwrap_err();

        assert_eq!(error.code(), "invalid-invocation");
        assert_eq!(error.exit_code(), 2);
    }

    /// Closing is terminal. A proposal closed by hand — as unwanted, say
    /// — stays closed; reopening it to reconcile would be the
    /// synchronization this design refuses.
    #[test]
    fn a_closed_proposal_is_a_no_op_and_not_an_error() {
        let issue = Issue {
            open: false,
            ..proposal_issue()
        };

        let plan = plan(OPERATION, &issue, None, None);

        assert_eq!(plan.unwrap(), Plan::Nothing);
    }

    /// Order matters: a closed issue that never was a proposal must not
    /// be reported as already reconciled.
    #[test]
    fn a_closed_non_proposal_still_refuses() {
        let issue = Issue {
            labels: Vec::new(),
            open: false,
            ..proposal_issue()
        };

        let error = plan(OPERATION, &issue, None, None).unwrap_err();

        assert_eq!(error.code(), "invalid-invocation");
    }

    /// `gh` lists commits newest first, so the landing is the last one.
    #[test]
    fn the_introducing_commit_is_the_oldest_listed() {
        let shas = "ccc333\nbbb222\naaa111\n";

        assert_eq!(introducing_commit(shas), Some("aaa111"));
    }

    #[test]
    fn a_path_with_no_commits_has_not_landed() {
        assert_eq!(introducing_commit(""), None);
        assert_eq!(introducing_commit("\n \n"), None);
    }

    /// A missing file is an answer; every other API failure is not, and
    /// must keep surfacing as an unavailable integration.
    #[test]
    fn only_a_404_reads_as_absent() {
        assert!(is_absent("gh: Not Found (HTTP 404)"));
        assert!(!is_absent("gh: Bad credentials (HTTP 401)"));
        assert!(!is_absent("gh: Server Error (HTTP 500)"));
        assert!(!is_absent("dial tcp: lookup api.github.com: no such host"));
    }

    /// The comment is never revised, so it has to carry everything a
    /// cold reader needs and say plainly which record wins.
    #[test]
    fn the_comment_cites_the_artifact_and_claims_no_authority_for_the_issue() {
        let body = comment_body(&summary(), &landing());

        assert!(body.contains("idea:40"), "{body}");
        assert!(body.contains("ide_01ABC"), "{body}");
        assert!(
            body.contains("archaeology/ideas/0040-reconcile.md"),
            "{body}"
        );
        assert!(
            body.contains("canonical record; this issue is not"),
            "{body}"
        );
    }

    /// Found by reading the first real reconciliation comment rendered on
    /// GitHub: a bare forty-character sha is a wall of hex nobody parses,
    /// and it was not even a link. The abbreviation is what a reader
    /// wants; the link behind it keeps the exact identity.
    #[test]
    fn the_landing_commit_is_an_abbreviated_link_not_a_wall_of_hex() {
        let body = comment_body(&summary(), &landing());

        assert!(
            body.contains("[`abc1234`](https://github.com/o/r/commit/abc1234def5678)"),
            "{body}"
        );
        assert!(
            !body.contains("| abc1234def5678 |"),
            "the full sha must not appear bare: {body}"
        );
    }

    #[test]
    fn a_short_commit_is_seven_characters_and_never_panics() {
        assert_eq!(short_commit("abc1234def5678"), "abc1234");
        assert_eq!(short_commit("abc"), "abc");
        assert_eq!(short_commit(""), "");
    }

    /// The marker is what makes a re-run idempotent, so it has to
    /// survive into the comment `issue_state` will later read back.
    #[test]
    fn the_comment_carries_the_marker_that_detects_it() {
        let body = comment_body(&summary(), &landing());

        assert!(body.contains(RECONCILED_MARKER), "{body}");
        assert!(
            body.trim_end().ends_with(RECONCILED_MARKER),
            "the marker belongs at the end, out of the reader's way: {body}"
        );
    }
}
