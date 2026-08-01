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

use std::process::Command;

use serde::Serialize;

use crate::artifact::{self, NewArtifact};
use crate::error::Error;
use crate::read;

/// The label a proposal issue carries. Applied automatically by the issue
/// form, so `list` sees exactly the issues filed as proposals.
const PROPOSAL_LABEL: &str = "idea";

/// GitHub's placeholder for an optional form field the filer left blank.
/// It is presentation, not content, and must never reach an artifact.
const NO_RESPONSE: &str = "_No response_";

/// One open proposal, as listed.
#[derive(Debug, Serialize)]
pub struct ProposalSummary {
    /// The issue number, which is how an operator names it to `realize`.
    pub number: u64,
    /// The issue title, which becomes the artifact's title verbatim.
    pub title: String,
    /// The issue's canonical URL, stamped into a realized artifact.
    pub url: String,
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

/// Confirm the repository has a GitHub remote, and name it `owner/repo`.
///
/// Checked before anything else so a non-GitHub repository gets an
/// immediate, specific refusal rather than a confusing `gh` error.
fn github_repo(operation: &str) -> Result<String, Error> {
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
    let raw =
        gh(operation, &["repo", "view", "--json", "nameWithOwner"]).map_err(|err| match &err {
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
    value
        .get("nameWithOwner")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| not_github("this repository has no GitHub remote".to_string()))
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
fn realized_from(root: &std::path::Path, url: &str) -> Result<Option<String>, Error> {
    Ok(read::scan(root, &read::IDEA)?
        .into_iter()
        .find(|a| a.summary.proposal.as_deref() == Some(url))
        .map(|a| a.summary.path))
}

/// Open proposals, newest issue number first, each annotated with the
/// artifact it has already been realized as.
pub fn list(root: &std::path::Path) -> Result<Vec<ProposalSummary>, Error> {
    const OPERATION: &str = "`scarp proposal list`";
    github_repo(OPERATION)?;
    let raw = gh(
        OPERATION,
        &[
            "issue",
            "list",
            "--label",
            PROPOSAL_LABEL,
            "--state",
            "open",
            "--json",
            "number,title,url",
            "--limit",
            "100",
        ],
    )?;
    let issues: Vec<serde_json::Value> =
        serde_json::from_str(&raw).map_err(|source| Error::IntegrationUnavailable {
            operation: OPERATION.to_string(),
            requirement: "a GitHub repository".to_string(),
            reason: format!("`gh issue list` returned unreadable JSON: {source}"),
            remedy: "retry, and report this if it persists".to_string(),
        })?;

    let mut proposals = Vec::new();
    for issue in issues {
        let url = issue
            .get("url")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        proposals.push(ProposalSummary {
            number: issue.get("number").and_then(|v| v.as_u64()).unwrap_or(0),
            title: issue
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            realized_as: realized_from(root, url)?,
            url: url.to_string(),
        });
    }
    proposals.sort_by_key(|p| std::cmp::Reverse(p.number));
    Ok(proposals)
}

/// Fetch one proposal.
fn view(number: u64) -> Result<Proposal, Error> {
    const OPERATION: &str = "`scarp proposal realize`";
    let raw = gh(
        OPERATION,
        &[
            "issue",
            "view",
            &number.to_string(),
            "--json",
            "number,title,body,url",
        ],
    )?;
    let value: serde_json::Value =
        serde_json::from_str(&raw).map_err(|source| Error::IntegrationUnavailable {
            operation: OPERATION.to_string(),
            requirement: "a GitHub repository".to_string(),
            reason: format!("`gh issue view` returned unreadable JSON: {source}"),
            remedy: "retry, and report this if it persists".to_string(),
        })?;
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
    })
}

/// Realize proposal `number` as a canonical idea.
///
/// Refuses rather than duplicating if this proposal already has an
/// artifact on this branch. Nothing is committed: the operator reviews
/// the new file and commits it through the ordinary workflow.
pub fn realize(root: &std::path::Path, number: u64) -> Result<NewArtifact, Error> {
    const OPERATION: &str = "`scarp proposal realize`";
    github_repo(OPERATION)?;
    let proposal = view(number)?;

    if proposal.url.is_empty() {
        return Err(Error::IntegrationUnavailable {
            operation: OPERATION.to_string(),
            requirement: "a GitHub repository".to_string(),
            reason: format!("issue {number} has no URL"),
            remedy: "check the issue number, then retry".to_string(),
        });
    }

    if let Some(path) = realized_from(root, &proposal.url)? {
        return Err(Error::ArtifactConflict {
            path: std::path::PathBuf::from(&path),
            reason: format!(
                "proposal {} has already been realized as `{path}`; \
                 one proposal realizes at most one artifact",
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

    let sections = read::IDEA_SECTIONS;
    let body = to_body(&proposal.body, sections);
    artifact::create_idea_from(root, &proposal.title, Some(&body), Some(&proposal.url))
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
}
