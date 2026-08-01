//! Integration tests for the GitHub proposal integration through the
//! compiled binary.
//!
//! The property under test is the one the design promises and the one a
//! consumer will actually rely on: **the integration is optional, never
//! load-bearing.** When `gh` is missing, or the repository is not on
//! GitHub, `scarp proposal` refuses with a typed, specific error and every
//! ordinary command keeps working.
//!
//! The success path is deliberately not tested here. It needs a live,
//! authenticated GitHub repository, and a test that shells out to a real
//! forge would be neither hermetic nor honest about what it proves; that
//! path is verified by dated performance in task 54's Result instead.

use std::path::Path;
use std::process::{Command, Output};

/// Run the binary in `dir` with a `PATH` that deliberately excludes `gh`,
/// simulating an operator who has never installed the GitHub CLI.
fn scarp_without_gh(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_scarp"))
        .args(args)
        .current_dir(dir)
        .env("PATH", "/nonexistent-for-tests")
        .output()
        .expect("failed to run scarp binary")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn init_repo() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("temp dir");
    let out = scarp_without_gh(tmp.path(), &["init"]);
    assert!(out.status.success(), "init failed: {}", stderr(&out));
    tmp
}

/// `init` deliberately does not pre-create empty collection directories,
/// since Git cannot track them (dragon 2), so a test seeding a file by
/// hand must materialize the directory the way creation would.
fn ideas_dir(tmp: &tempfile::TempDir) -> std::path::PathBuf {
    let dir = tmp.path().join("archaeology/ideas");
    std::fs::create_dir_all(&dir).expect("ideas dir");
    dir
}

#[test]
fn proposal_commands_refuse_cleanly_without_gh() {
    let tmp = init_repo();

    for args in [vec!["proposal", "list"], vec!["proposal", "realize", "1"]] {
        let out = scarp_without_gh(tmp.path(), &args);

        assert_eq!(
            out.status.code(),
            Some(11),
            "`scarp {}` must exit with the integration-unavailable code: {}",
            args.join(" "),
            stderr(&out)
        );
        let message = stderr(&out);
        assert!(
            message.starts_with("error[integration-unavailable]:"),
            "must lead with the stable machine token: {message}"
        );
        // A refusal that does not say what to do is a dead end.
        assert!(
            message.contains("scarp new idea"),
            "must name the way to get the same artifact without gh: {message}"
        );
        assert!(
            stdout(&out).is_empty(),
            "an unavailable integration must not write to stdout"
        );
    }
}

/// The load-bearing guarantee: a repository whose operator has no `gh` is
/// a completely ordinary Scarp repository. If this ever fails, an optional
/// integration has become mandatory.
#[test]
fn every_ordinary_command_works_without_gh() {
    let tmp = init_repo();

    let created = scarp_without_gh(tmp.path(), &["new", "idea", "Works without gh", "--json"]);
    assert!(created.status.success(), "{}", stderr(&created));
    assert!(stdout(&created).contains("\"sequence\":1"));

    for args in [
        vec!["list", "ideas"],
        vec!["show", "idea:1"],
        vec!["doctor"],
        vec!["fortune"],
        vec!["resolve", "idea:1"],
    ] {
        let out = scarp_without_gh(tmp.path(), &args);
        assert!(
            out.status.success(),
            "`scarp {}` must work without gh: {}",
            args.join(" "),
            stderr(&out)
        );
    }

    let closed = scarp_without_gh(tmp.path(), &["reject", "idea:1"]);
    assert!(closed.status.success(), "{}", stderr(&closed));
}

/// A realized artifact is an ordinary parked idea. Nothing marks it as
/// second-class, and its lifecycle verbs are the same ones every other
/// idea has — checked here because the front-matter stamp is the one
/// thing that could have made it special.
#[test]
fn a_proposal_stamped_idea_keeps_the_ordinary_lifecycle() {
    let tmp = init_repo();
    let ideas = ideas_dir(&tmp);
    std::fs::write(
        ideas.join("0001-realized.md"),
        "---\nid: ide_TEST\nsequence: 1\nkind: idea\nstatus: parked\n\
         proposal: https://github.com/o/r/issues/2\ncreated: 2026-08-01\n---\n\n\
         # Realized\n\n## Problem\n\nFrom a proposal.\n",
    )
    .expect("seed idea");

    let doctor = scarp_without_gh(tmp.path(), &["doctor"]);
    assert!(doctor.status.success(), "{}", stderr(&doctor));

    let shown = scarp_without_gh(tmp.path(), &["show", "idea:1", "--json"]);
    assert!(shown.status.success(), "{}", stderr(&shown));
    assert!(
        stdout(&shown).contains("https://github.com/o/r/issues/2"),
        "the stamp must survive into the read model: {}",
        stdout(&shown)
    );

    let adopted = scarp_without_gh(tmp.path(), &["adopt", "idea:1"]);
    assert!(adopted.status.success(), "{}", stderr(&adopted));
}

/// Two artifacts claiming one proposal is corruption, and it is the case
/// no single realization run can catch: each branch realized once, and
/// only the merge produced the duplicate.
#[test]
fn doctor_reports_a_proposal_realized_twice() {
    let tmp = init_repo();
    let ideas = ideas_dir(&tmp);
    for (sequence, name) in [(1, "0001-first.md"), (2, "0002-second.md")] {
        std::fs::write(
            ideas.join(name),
            format!(
                "---\nid: ide_TEST{sequence}\nsequence: {sequence}\nkind: idea\n\
                 status: parked\nproposal: https://github.com/o/r/issues/2\n\
                 created: 2026-08-01\n---\n\n# Idea {sequence}\n\n## Problem\n\nx\n"
            ),
        )
        .expect("seed idea");
    }

    let out = scarp_without_gh(tmp.path(), &["doctor"]);

    assert_eq!(out.status.code(), Some(9), "{}", stderr(&out));
    let report = stdout(&out);
    assert!(report.contains("duplicate-proposal"), "{report}");
    assert!(report.contains("0001-first.md"), "{report}");
    assert!(report.contains("0002-second.md"), "{report}");
}

/// A malformed stamp is corruption rather than something to tolerate: a
/// value nobody can follow looks like provenance without being any.
#[test]
fn doctor_reports_an_unusable_proposal_value() {
    let tmp = init_repo();
    std::fs::write(
        ideas_dir(&tmp).join("0001-bad-stamp.md"),
        "---\nid: ide_TEST\nsequence: 1\nkind: idea\nstatus: parked\n\
         proposal: owner/repo#2\ncreated: 2026-08-01\n---\n\n# Bad stamp\n\n\
         ## Problem\n\nx\n",
    )
    .expect("seed idea");

    let out = scarp_without_gh(tmp.path(), &["doctor"]);

    assert_eq!(out.status.code(), Some(9), "{}", stderr(&out));
    assert!(
        stdout(&out).contains("malformed-artifact"),
        "{}",
        stdout(&out)
    );
}
