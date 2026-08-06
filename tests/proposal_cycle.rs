//! End-to-end proof of the proposal cycle through the compiled binary,
//! against a fake `gh`.
//!
//! # What this proves, and what it deliberately does not
//!
//! It proves the half Scarp owns: classification from labels, target
//! selection, refusal ordering, the shape of the canonical artifacts
//! produced, the deterministic listing, and — for reconciliation — that
//! the sequence of `gh` invocations is exactly the one the recovery story
//! depends on, and that no mutation is attempted when a precondition
//! fails.
//!
//! It proves nothing about GitHub. The `gh` on `PATH` here is a shell
//! script reading canned files, so a passing run says the arguments Scarp
//! builds are the ones it intends to build, not that GitHub answers them
//! as assumed. That second claim can only come from a dated live
//! performance recorded in a task result, and this file must never be
//! cited as though it were one.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A temporary repository plus the canned-response directory the fake
/// `gh` on its `PATH` reads from.
struct Harness {
    tmp: tempfile::TempDir,
    gh_dir: PathBuf,
    bin_dir: PathBuf,
}

/// The fake `gh`.
///
/// Dispatch is on the subcommand and, where it matters, on the requested
/// `--json` fields, because `issue view` is called with two different
/// field sets for two different questions. Every invocation is appended
/// to `calls.log` and every mutating one to `mutations.log`, so a test
/// can assert on ordering and on the absence of a mutation — the property
/// that keeps an unproven claim off a public issue.
const FAKE_GH: &str = r#"#!/bin/sh
D="$FAKE_GH_DIR"
printf '%s\n' "$*" >> "$D/calls.log"
sub="$1"; shift
case "$sub" in
  repo)
    cat "$D/repo.json"
    ;;
  api)
    path=""
    for a in "$@"; do
      case "$a" in repos/*) path="$a" ;; esac
    done
    slug=$(printf '%s' "$path" | tr -c 'A-Za-z0-9' '_')
    if [ -f "$D/api/$slug" ]; then
      cat "$D/api/$slug"
    else
      echo "gh: Not Found (HTTP 404)" >&2
      exit 1
    fi
    ;;
  issue)
    verb="$1"; shift
    case "$verb" in
      list)
        label=""
        for a in "$@"; do
          if [ "$prev" = "--label" ]; then label="$a"; fi
          prev="$a"
        done
        if [ -f "$D/list-$label.json" ]; then cat "$D/list-$label.json"; else echo '[]'; fi
        ;;
      view)
        n="$1"; shift
        fields=""
        for a in "$@"; do
          if [ "$prev" = "--json" ]; then fields="$a"; fi
          prev="$a"
        done
        case "$fields" in
          *body*) cat "$D/issue-$n-view.json" ;;
          *)      cat "$D/issue-$n-state.json" ;;
        esac
        ;;
      comment)
        printf 'comment %s\n' "$1" >> "$D/mutations.log"
        ;;
      close)
        printf 'close %s\n' "$1" >> "$D/mutations.log"
        ;;
    esac
    ;;
esac
exit 0
"#;

impl Harness {
    fn new() -> Self {
        let tmp = tempfile::tempdir().expect("temp dir");
        let gh_dir = tmp.path().join(".fake-gh");
        let bin_dir = tmp.path().join(".fake-bin");
        fs::create_dir_all(gh_dir.join("api")).expect("gh dir");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh = bin_dir.join("gh");
        fs::write(&gh, FAKE_GH).expect("write fake gh");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&gh, fs::Permissions::from_mode(0o755)).expect("chmod");
        }
        let harness = Harness {
            tmp,
            gh_dir,
            bin_dir,
        };
        harness.canned(
            "repo.json",
            r#"{"nameWithOwner":"o/r","defaultBranchRef":{"name":"main"}}"#,
        );
        let out = harness.scarp(&["init"]);
        assert!(out.status.success(), "init failed: {}", stderr(&out));
        harness
    }

    fn root(&self) -> &Path {
        self.tmp.path()
    }

    /// Write one canned `gh` response.
    fn canned(&self, name: &str, body: &str) {
        fs::write(self.gh_dir.join(name), body).expect("write canned response");
    }

    fn scarp(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_scarp"))
            .args(args)
            .current_dir(self.tmp.path())
            .env("PATH", format!("{}:/usr/bin:/bin", self.bin_dir.display()))
            .env("FAKE_GH_DIR", &self.gh_dir)
            .output()
            .expect("run scarp")
    }

    /// Every `gh` invocation so far, one per line, in order.
    fn calls(&self) -> Vec<String> {
        read_log(&self.gh_dir.join("calls.log"))
    }

    /// Every mutating `gh` invocation so far, in order.
    fn mutations(&self) -> Vec<String> {
        read_log(&self.gh_dir.join("mutations.log"))
    }

    fn read(&self, relative: &str) -> String {
        fs::read_to_string(self.tmp.path().join(relative)).expect("read artifact")
    }

    /// Repository-relative paths of every file under `dir`, sorted.
    fn entries(&self, dir: &str) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(self.tmp.path().join(dir))
            .map(|entries| {
                entries
                    .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
                    .filter(|n| !n.starts_with('.'))
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        names
    }
}

fn read_log(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A canned `gh issue view --json number,title,body,url,labels` payload.
fn issue_view(number: u64, title: &str, body: &str, labels: &[&str]) -> String {
    serde_json::json!({
        "number": number,
        "title": title,
        "body": body,
        "url": format!("https://github.com/o/r/issues/{number}"),
        "labels": labels.iter().map(|l| serde_json::json!({"name": l})).collect::<Vec<_>>(),
    })
    .to_string()
}

/// A canned `gh issue list` element.
fn issue_listed(number: u64, title: &str, labels: &[&str]) -> serde_json::Value {
    serde_json::json!({
        "number": number,
        "title": title,
        "url": format!("https://github.com/o/r/issues/{number}"),
        "labels": labels.iter().map(|l| serde_json::json!({"name": l})).collect::<Vec<_>>(),
    })
}

const REPORT: &str = "### Observed behavior\n\n`scarp doctor` counts 12.\n\n\
                      ### Expected behavior\n\nIt should count 11.\n";

// ---------------------------------------------------------------------
// Realization: one row of the adjudicated table per test.
// ---------------------------------------------------------------------

#[test]
fn a_bug_realizes_a_pending_maintenance_item() {
    let h = Harness::new();
    h.canned(
        "issue-7-view.json",
        &issue_view(7, "Doctor miscounts artifacts", REPORT, &["bug"]),
    );

    let out = h.scarp(&["proposal", "realize", "7"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let path = "archaeology/maintenance/\
                0001-investigate-reported-behavior-doctor-miscounts-artifacts.md";
    assert!(stdout(&out).contains(path), "{}", stdout(&out));
    let content = h.read(path);

    // The title is the project's statement of what it undertook, not the
    // reporter's claim about what is wrong.
    assert!(
        content.contains("# Investigate reported behavior: Doctor miscounts artifacts"),
        "{content}"
    );
    assert!(content.contains("kind: maintenance"), "{content}");
    assert!(content.contains("status: pending"), "{content}");
    assert!(
        content.contains("proposal: https://github.com/o/r/issues/7"),
        "{content}"
    );
    // One canonical section, holding the report and the framing that says
    // promotion is an obligation to look rather than a finding.
    assert_eq!(content.matches("\n## ").count(), 1, "{content}");
    assert!(content.contains("\n## Work\n"), "{content}");
    assert!(
        content.contains("does not assert that the reporter's diagnosis is correct"),
        "{content}"
    );
    assert!(content.contains("### The report as filed"), "{content}");
    assert!(content.contains("### Observed behavior"), "{content}");

    // Realization is local. It reads GitHub and writes a file; nothing
    // it does is visible to anyone but the operator until they commit.
    assert!(h.mutations().is_empty(), "{:?}", h.mutations());
}

#[test]
fn a_bug_with_a_sprint_realizes_a_pending_task_in_it() {
    let h = Harness::new();
    let sprint = h.scarp(&["new", "sprint", "Bug fixing"]);
    assert!(sprint.status.success(), "{}", stderr(&sprint));
    h.canned(
        "issue-7-view.json",
        &issue_view(7, "Doctor miscounts artifacts", REPORT, &["bug"]),
    );

    let out = h.scarp(&["proposal", "realize", "7", "--sprint", "sprint:1"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let path = "archaeology/sprints/0001-bug-fixing/\
                0001-investigate-reported-behavior-doctor-miscounts-artifacts.md";
    let content = h.read(path);
    assert!(content.contains("kind: task"), "{content}");
    assert!(content.contains("status: pending"), "{content}");
    assert!(content.contains("sprint: spr_"), "{content}");
    assert!(
        content.contains("proposal: https://github.com/o/r/issues/7"),
        "{content}"
    );
    assert!(content.contains("\n## Objective\n"), "{content}");
    assert!(content.contains("### Observed behavior"), "{content}");

    // Acceptance criteria are generated, because an outside reporter has
    // no way to know what this project considers done.
    assert!(content.contains("\n## Acceptance criteria\n"), "{content}");
    assert!(content.contains("reproduced, or the attempt"), "{content}");
    assert!(content.contains("terminal finding"), "{content}");
    assert!(
        content.contains("regression test") && content.contains("fails without it"),
        "{content}"
    );
    assert!(
        h.entries("archaeology/maintenance").is_empty(),
        "a sprinted bug must not also create maintenance"
    );
}

#[test]
fn an_idea_realizes_a_parked_idea_exactly_as_before() {
    let h = Harness::new();
    h.canned(
        "issue-2-view.json",
        &issue_view(
            2,
            "Reconcile realized proposals",
            "### Problem\n\nThe loop is open.\n\n### Sketch\n\n_No response_\n",
            &["idea"],
        ),
    );

    let out = h.scarp(&["proposal", "realize", "2"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let content = h.read("archaeology/ideas/0001-reconcile-realized-proposals.md");
    // The title is verbatim: the investigate prefix belongs to bugs only.
    assert!(
        content.contains("# Reconcile realized proposals"),
        "{content}"
    );
    assert!(content.contains("status: parked"), "{content}");
    assert!(
        content.contains("\n## Problem\n\nThe loop is open.\n"),
        "{content}"
    );
    assert!(!content.contains("_No response_"), "{content}");
    // Every template section, in template order, exactly as an idea
    // authored by hand would have them.
    assert!(
        content.ends_with("## Sketch\n\n## Boundaries\n\n## Evidence\n"),
        "{content}"
    );
}

#[test]
fn an_idea_refuses_a_sprint() {
    let h = Harness::new();
    let sprint = h.scarp(&["new", "sprint", "Bug fixing"]);
    assert!(sprint.status.success(), "{}", stderr(&sprint));
    h.canned(
        "issue-2-view.json",
        &issue_view(2, "An idea", "", &["idea"]),
    );

    let out = h.scarp(&["proposal", "realize", "2", "--sprint", "sprint:1"]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    let message = stderr(&out);
    assert!(
        message.starts_with("error[invalid-invocation]:"),
        "{message}"
    );
    assert!(
        message.contains("never a sprint's committed work"),
        "{message}"
    );
    assert!(
        h.entries("archaeology/ideas").is_empty(),
        "a refused realization must write nothing"
    );
}

#[test]
fn neither_recognized_label_refuses_before_writing_anything() {
    let h = Harness::new();
    h.canned(
        "issue-9-view.json",
        &issue_view(9, "A discussion", REPORT, &["question", "help wanted"]),
    );

    let out = h.scarp(&["proposal", "realize", "9"]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(
        stderr(&out).contains("is not a proposal"),
        "{}",
        stderr(&out)
    );
    for dir in ["archaeology/ideas", "archaeology/maintenance"] {
        assert!(h.entries(dir).is_empty(), "{dir} must be untouched");
    }
}

#[test]
fn both_recognized_labels_refuse_and_name_the_conflict() {
    let h = Harness::new();
    h.canned(
        "issue-9-view.json",
        &issue_view(9, "Ambiguous", REPORT, &["idea", "bug"]),
    );

    let out = h.scarp(&["proposal", "realize", "9"]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    let message = stderr(&out);
    assert!(message.contains("`idea` and `bug`"), "{message}");
    assert!(message.contains("will not guess"), "{message}");
    for dir in ["archaeology/ideas", "archaeology/maintenance"] {
        assert!(h.entries(dir).is_empty(), "{dir} must be untouched");
    }
}

/// A proposal may be tagged anything else without that meaning a thing
/// here. Exactly one *recognized* label has to remain.
#[test]
fn unrelated_labels_are_ignored() {
    let h = Harness::new();
    h.canned(
        "issue-7-view.json",
        &issue_view(
            7,
            "Reported crash",
            REPORT,
            &["documentation", "bug", "good first issue"],
        ),
    );

    let out = h.scarp(&["proposal", "realize", "7"]);

    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(h.entries("archaeology/maintenance").len(), 1);
}

/// One proposal realizes at most one artifact **across collections**. A
/// per-collection check would let the same report become both a
/// maintenance item and a task.
#[test]
fn duplicate_detection_spans_ideas_maintenance_and_tasks() {
    let h = Harness::new();
    let sprint = h.scarp(&["new", "sprint", "Bug fixing"]);
    assert!(sprint.status.success(), "{}", stderr(&sprint));
    h.canned(
        "issue-7-view.json",
        &issue_view(7, "Reported crash", REPORT, &["bug"]),
    );

    let first = h.scarp(&["proposal", "realize", "7"]);
    assert!(first.status.success(), "{}", stderr(&first));

    // The same proposal, now asking for a task in a different collection.
    let second = h.scarp(&["proposal", "realize", "7", "--sprint", "sprint:1"]);

    assert_eq!(second.status.code(), Some(4), "{}", stderr(&second));
    let message = stderr(&second);
    assert!(message.contains("already been realized"), "{message}");
    assert!(message.contains("across every collection"), "{message}");
    assert_eq!(
        h.entries("archaeology/sprints/0001-bug-fixing"),
        vec!["sprint.md".to_string()],
        "no task may be created for an already-realized proposal"
    );
}

/// The reverse direction: an idea realized first blocks a later bug
/// realization of the same issue.
#[test]
fn an_idea_already_realized_blocks_a_maintenance_realization() {
    let h = Harness::new();
    h.canned(
        "issue-4-view.json",
        &issue_view(4, "Same issue", "", &["idea"]),
    );
    let first = h.scarp(&["proposal", "realize", "4"]);
    assert!(first.status.success(), "{}", stderr(&first));

    // Relabeled on GitHub afterwards; the stamp still says it is taken.
    h.canned(
        "issue-4-view.json",
        &issue_view(4, "Same issue", "", &["bug"]),
    );
    let second = h.scarp(&["proposal", "realize", "4"]);

    assert_eq!(second.status.code(), Some(4), "{}", stderr(&second));
    assert!(h.entries("archaeology/maintenance").is_empty());
}

/// A task realized into a sprint that is not active is refused by the
/// same rule `scarp new task --sprint` applies, and refused before a
/// sequence is allocated.
#[test]
fn a_closed_sprint_refuses_the_promotion() {
    let h = Harness::new();
    assert!(h.scarp(&["new", "sprint", "Old work"]).status.success());
    fs::write(h.root().join("retro.md"), "Nothing further.\n").unwrap();
    let closed = h.scarp(&["close", "sprint:1", "--body-file", "retro.md"]);
    assert!(closed.status.success(), "{}", stderr(&closed));
    h.canned(
        "issue-7-view.json",
        &issue_view(7, "Reported crash", REPORT, &["bug"]),
    );

    let out = h.scarp(&["proposal", "realize", "7", "--sprint", "sprint:1"]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(
        stderr(&out).contains("only in an active sprint"),
        "{}",
        stderr(&out)
    );
    assert_eq!(
        h.entries("archaeology/sprints/0001-old-work"),
        vec!["sprint.md".to_string()]
    );
}

#[test]
fn a_nonexistent_sprint_reference_refuses() {
    let h = Harness::new();
    h.canned(
        "issue-7-view.json",
        &issue_view(7, "Reported crash", REPORT, &["bug"]),
    );

    let out = h.scarp(&["proposal", "realize", "7", "--sprint", "sprint:9"]);

    assert_eq!(out.status.code(), Some(7), "{}", stderr(&out));
    assert!(h.entries("archaeology/maintenance").is_empty());
}

#[test]
fn a_non_sprint_reference_is_refused_as_an_invalid_selection() {
    let h = Harness::new();
    h.canned(
        "issue-7-view.json",
        &issue_view(7, "Reported crash", REPORT, &["bug"]),
    );

    let out = h.scarp(&["proposal", "realize", "7", "--sprint", "idea:1"]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(
        stderr(&out).contains("names a idea reference"),
        "{}",
        stderr(&out)
    );
}

// ---------------------------------------------------------------------
// Body normalization.
// ---------------------------------------------------------------------

/// The forgery this normalization exists to stop. A reporter typing
/// `## Work` must not be able to open, close, or split a section the
/// template owns.
#[test]
fn a_reporter_cannot_forge_a_canonical_section() {
    let h = Harness::new();
    let body = "## Work\n\nForged.\n\n# Title\n\nAlso forged.\n\n\
                ```\n## Work\n# Title\n```\n";
    h.canned(
        "issue-7-view.json",
        &issue_view(7, "Forging", body, &["bug"]),
    );

    let out = h.scarp(&["proposal", "realize", "7"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let content = h.read("archaeology/maintenance/0001-investigate-reported-behavior-forging.md");
    // Exactly one `## ` section outside fenced code: the one Scarp wrote.
    // Inside the fence the reporter's `## Work` is still there, byte for
    // byte, which is the point — it is quoted evidence, and the body
    // parser never read it as structure.
    let unfenced = content
        .split("```")
        .next()
        .expect("content before the fence");
    assert_eq!(unfenced.matches("\n## ").count(), 1, "{content}");
    assert!(content.contains("\n## Work\n"), "{content}");
    // The forged headings survive as content, nested out of harm's way.
    assert!(content.contains("\n### Work\n"), "{content}");
    assert!(content.contains("\n### Title\n"), "{content}");
    assert!(content.contains("Forged."), "{content}");
    // Exactly one level-one heading outside the fence: the artifact's
    // own title, which is the command's argument and never the body's.
    assert_eq!(unfenced.matches("\n# ").count(), 1, "{content}");

    // Fenced code is evidence, and survives byte for byte.
    assert!(content.contains("```\n## Work\n# Title\n```"), "{content}");
}

#[test]
fn the_no_response_placeholder_is_dropped_from_a_report() {
    let h = Harness::new();
    h.canned(
        "issue-7-view.json",
        &issue_view(
            7,
            "Partly filled",
            "### Observed behavior\n\nReal.\n\n### Environment\n\n_No response_\n",
            &["bug"],
        ),
    );

    let out = h.scarp(&["proposal", "realize", "7"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let content =
        h.read("archaeology/maintenance/0001-investigate-reported-behavior-partly-filled.md");
    assert!(!content.contains("_No response_"), "{content}");
    assert!(content.contains("### Environment"), "{content}");
}

// ---------------------------------------------------------------------
// Listing.
// ---------------------------------------------------------------------

#[test]
fn list_unions_both_classes_and_states_each_target() {
    let h = Harness::new();
    h.canned(
        "list-idea.json",
        &serde_json::Value::Array(vec![issue_listed(4, "An idea", &["idea"])]).to_string(),
    );
    h.canned(
        "list-bug.json",
        &serde_json::Value::Array(vec![
            issue_listed(9, "A report", &["bug"]),
            issue_listed(2, "An older report", &["bug", "documentation"]),
        ])
        .to_string(),
    );

    let out = h.scarp(&["proposal", "list"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let rendered = stdout(&out);
    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(lines.len(), 3, "{:?}", lines);
    // Newest issue number first, whatever order `gh` returned either
    // page in.
    assert!(
        lines[0].starts_with("#9  open      maintenance"),
        "{lines:?}"
    );
    assert!(lines[1].starts_with("#4  open      idea"), "{lines:?}");
    assert!(
        lines[2].starts_with("#2  open      maintenance"),
        "{lines:?}"
    );

    // Two queries, not one with repeated `--label`: GitHub intersects
    // those, so a single call would return only the ambiguous set.
    let calls = h.calls();
    let list_calls: Vec<&String> = calls
        .iter()
        .filter(|c| c.starts_with("issue list"))
        .collect();
    assert_eq!(list_calls.len(), 2, "{calls:?}");
    assert_eq!(
        list_calls
            .iter()
            .filter(|c| c.matches("--label").count() == 1)
            .count(),
        2,
        "{list_calls:?}"
    );
}

#[test]
fn list_json_carries_the_target_and_is_deterministic() {
    let h = Harness::new();
    h.canned(
        "list-idea.json",
        &serde_json::Value::Array(vec![issue_listed(4, "An idea", &["idea"])]).to_string(),
    );
    h.canned(
        "list-bug.json",
        &serde_json::Value::Array(vec![issue_listed(9, "A report", &["bug"])]).to_string(),
    );

    let first = h.scarp(&["proposal", "list", "--json"]);
    let second = h.scarp(&["proposal", "list", "--json"]);

    assert!(first.status.success(), "{}", stderr(&first));
    assert_eq!(stdout(&first), stdout(&second), "listing must be stable");
    let parsed: serde_json::Value = serde_json::from_str(stdout(&first).trim()).unwrap();
    assert_eq!(parsed[0]["number"], 9);
    assert_eq!(parsed[0]["target"], "maintenance");
    assert_eq!(parsed[1]["number"], 4);
    assert_eq!(parsed[1]["target"], "idea");
    // Absent until realized, rather than emitted as null.
    assert!(parsed[0].get("realized_as").is_none(), "{parsed}");
}

#[test]
fn list_annotates_an_already_realized_proposal() {
    let h = Harness::new();
    h.canned(
        "issue-9-view.json",
        &issue_view(9, "A report", REPORT, &["bug"]),
    );
    assert!(h.scarp(&["proposal", "realize", "9"]).status.success());
    h.canned(
        "list-bug.json",
        &serde_json::Value::Array(vec![issue_listed(9, "A report", &["bug"])]).to_string(),
    );

    let out = h.scarp(&["proposal", "list"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let line = stdout(&out);
    assert!(line.contains("realized"), "{line}");
    assert!(line.contains("archaeology/maintenance/0001-"), "{line}");
}

/// A listing is what an operator reads before deciding what to realize,
/// so a confidently wrong row in it is worse than no listing at all.
#[test]
fn list_refuses_rather_than_guessing_at_a_dual_labeled_issue() {
    let h = Harness::new();
    h.canned(
        "list-idea.json",
        &serde_json::Value::Array(vec![issue_listed(9, "Ambiguous", &["idea", "bug"])]).to_string(),
    );
    h.canned(
        "list-bug.json",
        &serde_json::Value::Array(vec![issue_listed(9, "Ambiguous", &["idea", "bug"])]).to_string(),
    );

    let out = h.scarp(&["proposal", "list"]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    let message = stderr(&out);
    assert!(message.contains("cannot list proposals"), "{message}");
    assert!(message.contains("issue 9"), "{message}");
    assert!(message.contains("`idea` and `bug`"), "{message}");
    assert!(
        stdout(&out).is_empty(),
        "a refused listing must emit no partial classification"
    );
}
