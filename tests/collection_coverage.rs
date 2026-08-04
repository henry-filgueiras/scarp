//! The invariant the compiler cannot enforce (task 66).
//!
//! Adding a collection touches eight exhaustive `match` arms, and the
//! compiler names every one of them — a forgotten arm is a build failure,
//! not a defect. Three sites are *not* behind exhaustive matches, and one
//! of those fails silently: a collection missing from `doctor`'s
//! validated set is simply never checked, and the repository reports
//! healthy while an entire collection goes unread.
//!
//! This test closes that gap without extracting anything. It takes the
//! CLI's own advertised collection vocabulary as the authority, creates
//! one artifact in every collection, and asserts `doctor` accounts for
//! all of them. Adding a ninth collection means updating the advertised
//! list — which the first assertion detects — and then the second
//! assertion proves `doctor` sees it.

use std::path::Path;
use std::process::Output;

/// Every collection, in the order the CLI advertises them. Adding a
/// collection means adding it here; the first test below fails until you
/// do, because it compares this list against the CLI's own error text.
const COLLECTIONS: &[&str] = &[
    "dragon",
    "idea",
    "decision",
    "log",
    "principle",
    "maintenance",
    "sprint",
    "task",
];

fn scarp_in(dir: &Path, args: &[&str]) -> Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_scarp"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run scarp binary")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn run(dir: &Path, args: &[&str]) -> String {
    let out = scarp_in(dir, args);
    assert!(out.status.success(), "{args:?} failed:\n{}", stderr(&out));
    stdout(&out)
}

#[test]
fn the_advertised_collection_vocabulary_matches_this_list() {
    // Ties COLLECTIONS to the one place a user learns what exists, so the
    // coverage test below cannot silently fall behind the CLI.
    let tmp = tempfile::tempdir().unwrap();
    let out = scarp_in(tmp.path(), &["list", "nonesuch"]);

    let message = format!("{}{}", stdout(&out), stderr(&out));
    let (_, advertised) = message
        .split_once("collections are: ")
        .expect("the unknown-collection error must advertise the vocabulary");
    let advertised: Vec<String> = advertised
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .trim_end_matches(['`', '"', '.'])
        .split(',')
        .map(|name| name.trim().trim_end_matches('`').to_string())
        .filter(|name| !name.is_empty())
        .collect();

    assert_eq!(
        advertised, COLLECTIONS,
        "the CLI advertises a different set than this test covers; \
         add the new collection to COLLECTIONS and to doctor's validated set"
    );
}

#[test]
fn doctor_accounts_for_an_artifact_in_every_collection() {
    // The silent failure this pins: a collection absent from doctor's
    // validated set is never read, and the repository reports healthy
    // with an entire collection unchecked.
    let tmp = tempfile::tempdir().unwrap();
    run(tmp.path(), &["init"]);

    // Sprints must exist before a task can be created in one; the order
    // of COLLECTIONS puts sprint before task, so a single pass works.
    for collection in COLLECTIONS {
        run(tmp.path(), &["new", collection, "A coverage artifact"]);
    }

    let report = run(tmp.path(), &["doctor"]);

    let expected = format!("{} artifact(s) checked", COLLECTIONS.len());
    assert!(
        report.contains(&expected),
        "doctor must check one artifact per collection ({expected}), but reported: {report}"
    );
    assert!(report.contains("no problems found"), "{report}");
}

#[test]
fn a_bare_stable_id_resolves_in_every_collection() {
    // The second non-enforced list: `show`'s bare-id union. A collection
    // missing from it resolves by `kind:N` but not by stable id.
    let tmp = tempfile::tempdir().unwrap();
    run(tmp.path(), &["init"]);
    for collection in COLLECTIONS {
        run(tmp.path(), &["new", collection, "A coverage artifact"]);
    }

    for collection in COLLECTIONS {
        let reference = format!("{collection}:1");
        let shown = run(tmp.path(), &["show", &reference, "--json"]);
        let id = shown
            .split_once("\"id\":\"")
            .and_then(|(_, rest)| rest.split_once('"'))
            .map(|(id, _)| id.to_string())
            .expect("show --json must carry the stable id");

        let out = scarp_in(tmp.path(), &["show", &id]);

        assert!(
            out.status.success(),
            "`show {id}` ({collection}) must resolve by stable id: {}",
            stderr(&out)
        );
    }
}
