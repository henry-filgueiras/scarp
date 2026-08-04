//! Integration tests for the managed maintenance collection (task 64).
//!
//! Maintenance is bounded repository work worth recording but not
//! commissioned in service of a sprint goal. The tests that matter are
//! the ones about what it deliberately lacks: no sprint, no recurrence,
//! no staleness, no ledger, and no placement topology of its own.

use std::fs;
use std::path::Path;
use std::process::Output;

const MAINTENANCE_DIR: &str = "archaeology/maintenance";

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

fn init_repo() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    run(tmp.path(), &["init"]);
    tmp
}

fn body(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    fs::write(&path, text).unwrap();
    path.to_str().unwrap().to_string()
}

fn item(tmp: &Path, name: &str) -> String {
    fs::read_to_string(tmp.join(MAINTENANCE_DIR).join(name)).unwrap()
}

#[test]
fn maintenance_is_created_without_any_sprint() {
    // The whole point: WitnessGlass had to commission a sprint to file
    // one piece of housekeeping. This repository has no sprint at all.
    let tmp = init_repo();

    let out = run(tmp.path(), &["new", "maintenance", "Refresh the table"]);

    assert_eq!(
        out,
        "created maintenance:1 at archaeology/maintenance/0001-refresh-the-table.md\n"
    );
    let written = item(tmp.path(), "0001-refresh-the-table.md");
    assert!(written.contains("status: pending"), "{written}");
    assert!(written.contains("id: mnt_"), "{written}");
    assert!(
        !written.contains("sprint:"),
        "maintenance carries no sprint membership:\n{written}"
    );
    // One creation section; `Result` arrives at close, never as a stub.
    assert!(
        written.ends_with("# Refresh the table\n\n## Work\n"),
        "{written}"
    );
}

#[test]
fn creating_maintenance_needs_no_sprint_even_when_one_is_active() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "sprint", "A sprint"]);

    run(
        tmp.path(),
        &["new", "maintenance", "Unrelated housekeeping"],
    );

    let written = item(tmp.path(), "0001-unrelated-housekeeping.md");
    assert!(!written.contains("sprint:"), "{written}");
}

#[test]
fn the_sprint_flag_is_refused_for_maintenance() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "sprint", "A sprint"]);

    let out = scarp_in(
        tmp.path(),
        &["new", "maintenance", "Housekeeping", "--sprint", "sprint:1"],
    );

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(stderr(&out).contains("`--sprint`"), "{}", stderr(&out));
}

#[test]
fn closing_stamps_the_date_and_writes_the_result_section() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "maintenance", "Refresh the table"]);
    let file = body(tmp.path(), "r.md", "Done; the table matches doctor now.\n");

    run(
        tmp.path(),
        &["close", "maintenance:1", "--body-file", &file],
    );

    let written = item(tmp.path(), "0001-refresh-the-table.md");
    assert!(written.contains("status: closed"), "{written}");
    assert!(written.contains("closed: "), "{written}");
    assert!(
        written.ends_with("## Result\n\nDone; the table matches doctor now.\n"),
        "{written}"
    );
}

#[test]
fn an_item_not_worth_doing_closes_with_a_result_saying_so() {
    // There is no `cancelled` state, deliberately: closing with a reason
    // is lossless and needs no new vocabulary.
    let tmp = init_repo();
    run(tmp.path(), &["new", "maintenance", "Rewrite the docs"]);
    let file = body(
        tmp.path(),
        "r.md",
        "Not done: the docs were rewritten by task 40 already.\n",
    );

    run(
        tmp.path(),
        &["close", "maintenance:1", "--body-file", &file],
    );

    let written = item(tmp.path(), "0001-rewrite-the-docs.md");
    assert!(written.contains("status: closed"), "{written}");
    assert!(written.contains("Not done:"), "{written}");
}

#[test]
fn maintenance_has_exactly_one_transition() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "maintenance", "Housekeeping"]);
    run(tmp.path(), &["close", "maintenance:1"]);

    let closed_twice = scarp_in(tmp.path(), &["close", "maintenance:1"]);
    assert_eq!(closed_twice.status.code(), Some(2));
    assert!(
        stderr(&closed_twice).contains("already closed"),
        "{}",
        stderr(&closed_twice)
    );

    let reopened = scarp_in(tmp.path(), &["reopen", "maintenance:1"]);
    assert_eq!(reopened.status.code(), Some(2));
    assert!(
        stderr(&reopened).contains("maintenance items close"),
        "{}",
        stderr(&reopened)
    );
}

#[test]
fn placement_is_flat_and_sequences_are_collection_global() {
    // Flat by adjudication (idea 42): the property that keeps a later
    // move to temporal buckets a pure `git mv` is that sequences and ids
    // owe nothing to placement.
    let tmp = init_repo();
    run(tmp.path(), &["new", "maintenance", "First"]);
    run(tmp.path(), &["new", "maintenance", "Second"]);
    run(tmp.path(), &["close", "maintenance:1"]);
    run(tmp.path(), &["new", "maintenance", "Third"]);

    let names: Vec<String> = fs::read_dir(tmp.path().join(MAINTENANCE_DIR))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(
        sorted,
        vec!["0001-first.md", "0002-second.md", "0003-third.md"],
        "every item files directly in the collection directory"
    );
    // Closing moved nothing.
    assert!(item(tmp.path(), "0001-first.md").contains("status: closed"));
}

#[test]
fn maintenance_carries_no_chore_semantics() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "maintenance", "Housekeeping"]);

    let written = item(tmp.path(), "0001-housekeeping.md");

    for absent in ["stale_after", "recurrence", "ledger", "performed"] {
        assert!(
            !written.contains(absent),
            "maintenance is not a chore (idea 7): found `{absent}` in\n{written}"
        );
    }
    // Nor does it join fortune's nagging pool.
    let fortune = run(tmp.path(), &["fortune"]);
    assert!(
        !fortune.contains("maintenance:1"),
        "maintenance is not surfaced by fortune: {fortune}"
    );
}

#[test]
fn list_and_show_use_the_mass_noun_spelling() {
    let tmp = init_repo();

    let empty = run(tmp.path(), &["list", "maintenance"]);
    assert!(
        empty.starts_with("no maintenance found"),
        "the mass noun must not pluralize: {empty}"
    );

    run(tmp.path(), &["new", "maintenance", "Housekeeping"]);
    let listed = run(tmp.path(), &["list", "maintenance"]);
    assert!(
        listed.contains("maintenance:1  pending  Housekeeping"),
        "{listed}"
    );
    assert!(
        run(tmp.path(), &["show", "maintenance:1"]).contains("# Housekeeping"),
        "show must resolve the reference"
    );
}

#[test]
fn a_bare_stable_id_closes_a_maintenance_item() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "maintenance", "Housekeeping"]);
    let id = item(tmp.path(), "0001-housekeeping.md")
        .lines()
        .find_map(|line| line.strip_prefix("id: ").map(str::to_string))
        .unwrap();

    run(tmp.path(), &["close", &id]);

    assert!(item(tmp.path(), "0001-housekeeping.md").contains("status: closed"));
}

#[test]
fn doctor_validates_maintenance_and_looks_for_no_sprint() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "maintenance", "Housekeeping"]);

    let report = run(tmp.path(), &["doctor"]);
    assert!(report.contains("1 artifact(s) checked"), "{report}");
    assert!(report.contains("no problems found"), "{report}");

    // A malformed item is a finding on the same terms as anywhere else.
    fs::write(
        tmp.path().join(MAINTENANCE_DIR).join("0007-mismatch.md"),
        "---\nid: mnt-x\nsequence: 3\nkind: maintenance\nstatus: pending\ncreated: 2026-08-04\n---\n\n# Mismatch\n",
    )
    .unwrap();
    let out = scarp_in(tmp.path(), &["doctor"]);
    assert_eq!(out.status.code(), Some(9));
    assert!(
        stdout(&out).contains("archaeology/maintenance/0007-mismatch.md"),
        "{}",
        stdout(&out)
    );
}

#[test]
fn sugar_binds_in_both_maintenance_write_paths() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "dragon", "A risk"]);
    let work = body(tmp.path(), "w.md", "## Work\n\nFollows [[dragon:1]].\n");
    run(
        tmp.path(),
        &["new", "maintenance", "Housekeeping", "--body-file", &work],
    );
    let result = body(tmp.path(), "r.md", "Done, and it touched [[dragon:1]].\n");

    run(
        tmp.path(),
        &["close", "maintenance:1", "--body-file", &result],
    );

    let written = item(tmp.path(), "0001-housekeeping.md");
    assert_eq!(
        written.matches("[[drg_").count(),
        2,
        "creation and closure both bind at the write boundary: {written}"
    );
    assert!(!written.contains("[[dragon:1]]"), "{written}");
}

#[test]
fn init_pre_creates_no_maintenance_directory() {
    // Git cannot track an empty directory (dragon 2), so the collection
    // directory appears on first use, like every other.
    let tmp = init_repo();

    assert!(
        !tmp.path().join(MAINTENANCE_DIR).exists(),
        "the directory must not be pre-created empty"
    );
}
