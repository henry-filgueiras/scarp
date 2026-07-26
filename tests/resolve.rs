//! Integration tests for `strata resolve` (task 38): batch reference
//! resolution with positional pairing and all-or-nothing stdout.

use std::fs;
use std::path::Path;
use std::process::Output;

fn strata_in(dir: &Path, args: &[&str]) -> Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_strata"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run strata binary")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A repository with one dragon, two ideas, and one decision, all with
/// known stable ids.
fn seeded_repo() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let out = strata_in(tmp.path(), &["init"]);
    assert!(out.status.success(), "init failed:\n{}", stderr(&out));
    let write = |dir: &str, name: &str, content: &str| {
        let dir = tmp.path().join(dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), content).unwrap();
    };
    write(
        "archaeology/dragons",
        "0001-risk.md",
        "---\nid: drg-risk\nsequence: 1\nkind: dragon\nstatus: open\ncreated: 2026-07-20\n---\n\n# A risk\n",
    );
    write(
        "archaeology/ideas",
        "0001-first.md",
        "---\nid: ide-first\nsequence: 1\nkind: idea\nstatus: parked\ncreated: 2026-07-20\n---\n\n# First idea\n",
    );
    write(
        "archaeology/ideas",
        "0002-second.md",
        "---\nid: ide-second\nsequence: 2\nkind: idea\nstatus: parked\ncreated: 2026-07-20\n---\n\n# Second idea\n",
    );
    write(
        "archaeology/decisions",
        "0001-choice.md",
        "---\nid: dec-choice\nsequence: 1\nkind: decision\nstatus: accepted\ncreated: 2026-07-20\n---\n\n# A choice\n",
    );
    tmp
}

#[test]
fn resolves_references_in_argument_order_one_id_per_line() {
    let tmp = seeded_repo();

    let out = strata_in(tmp.path(), &["resolve", "idea:2", "dragon:1", "idea:1"]);

    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout(&out), "ide-second\ndrg-risk\nide-first\n");
    assert_eq!(stderr(&out), "", "stderr must stay clean on success");
}

#[test]
fn duplicate_inputs_resolve_independently_and_repeat() {
    let tmp = seeded_repo();

    let out = strata_in(tmp.path(), &["resolve", "idea:1", "idea:1"]);

    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout(&out), "ide-first\nide-first\n");
}

#[test]
fn mixed_sugar_and_stable_id_inputs_pair_positionally() {
    // A stable id verifies existence and echoes back: normalization is
    // idempotent.
    let tmp = seeded_repo();

    let out = strata_in(
        tmp.path(),
        &["resolve", "dec-choice", "idea:2", "drg-risk", "decision:1"],
    );

    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(
        stdout(&out),
        "dec-choice\nide-second\ndrg-risk\ndec-choice\n"
    );
}

#[test]
fn any_failure_empties_stdout_and_reports_every_failing_input() {
    let tmp = seeded_repo();

    let out = strata_in(
        tmp.path(),
        &["resolve", "idea:1", "dragon:99", "idea:2", "ide-stale"],
    );

    assert_eq!(out.status.code(), Some(7), "{}", stderr(&out));
    assert_eq!(
        stdout(&out),
        "",
        "stdout must emit nothing when any input fails"
    );
    let err = stderr(&out);
    let lines: Vec<&str> = err.lines().collect();
    assert_eq!(lines.len(), 2, "one line per failure:\n{err}");
    assert!(
        lines[0].starts_with("error[artifact-not-found]: ") && lines[0].contains("dragon:99"),
        "failures must report in input order:\n{err}"
    );
    assert!(
        lines[1].starts_with("error[artifact-not-found]: ") && lines[1].contains("ide-stale"),
        "every failure must be reported:\n{err}"
    );
}

#[test]
fn a_stale_stable_id_is_a_resolution_failure_not_a_pass_through() {
    let tmp = seeded_repo();

    let out = strata_in(tmp.path(), &["resolve", "drg-deleted-long-ago"]);

    assert_eq!(out.status.code(), Some(7), "{}", stderr(&out));
    assert_eq!(stdout(&out), "", "no in-band sentinel may reach stdout");
    assert!(
        stderr(&out).contains("drg-deleted-long-ago"),
        "{}",
        stderr(&out)
    );
}

#[test]
fn json_emits_one_record_per_input_in_input_order() {
    let tmp = seeded_repo();

    let out = strata_in(tmp.path(), &["resolve", "idea:2", "drg-risk", "--json"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let expected = concat!(
        "[",
        "{\"input\":\"idea:2\",\"kind\":\"idea\",\"id\":\"ide-second\",",
        "\"sequence\":2,\"reference\":\"idea:2\",",
        "\"path\":\"archaeology/ideas/0002-second.md\",",
        "\"title\":\"Second idea\"},",
        "{\"input\":\"drg-risk\",\"kind\":\"dragon\",\"id\":\"drg-risk\",",
        "\"sequence\":1,\"reference\":\"dragon:1\",",
        "\"path\":\"archaeology/dragons/0001-risk.md\",",
        "\"title\":\"A risk\"}",
        "]\n"
    );
    assert_eq!(stdout(&out), expected);
}

#[test]
fn json_failure_mode_matches_the_human_one() {
    let tmp = seeded_repo();

    let out = strata_in(tmp.path(), &["resolve", "idea:1", "idea:99", "--json"]);

    assert_eq!(out.status.code(), Some(7), "{}", stderr(&out));
    assert_eq!(stdout(&out), "");
    assert!(stderr(&out).contains("idea:99"), "{}", stderr(&out));
}

#[test]
fn no_references_is_a_usage_error() {
    let tmp = seeded_repo();

    let out = strata_in(tmp.path(), &["resolve"]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
}

#[test]
fn a_duplicated_sequence_is_ambiguous_not_first_match() {
    let tmp = seeded_repo();
    fs::write(
        tmp.path().join("archaeology/ideas/0002-twin.md"),
        "---\nid: ide-twin\nsequence: 2\nkind: idea\nstatus: parked\ncreated: 2026-07-20\n---\n\n# Twin\n",
    )
    .unwrap();

    let out = strata_in(tmp.path(), &["resolve", "idea:2"]);

    assert_eq!(out.status.code(), Some(8), "{}", stderr(&out));
    assert_eq!(stdout(&out), "");
    assert!(
        stderr(&out).starts_with("error[ambiguous-reference]: "),
        "{}",
        stderr(&out)
    );
}
