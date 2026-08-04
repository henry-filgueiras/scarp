//! Integration tests for the managed principles collection (task 63,
//! adopting idea 28) and for sugar binding on the creation path.
//!
//! The binding tests live here because principle 1 is the specimen that
//! motivated extending the binder to `new --body-file`: its citation to
//! log 3 is authored as sugar rather than by copying a ULID.

use std::fs;
use std::path::Path;
use std::process::Output;

const PRINCIPLES_DIR: &str = "archaeology/principles";

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

/// Seed a log the way the pre-existing corpus writes one: no `status:`.
fn seed_log(root: &Path, sequence: u32, id: &str, title: &str) {
    let dir = root.join("archaeology/logs");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join(format!("{sequence:04}-a-log.md")),
        format!("---\nid: {id}\nsequence: {sequence}\nkind: log\ncreated: 2026-07-27\n---\n\n# {title}\n"),
    )
    .unwrap();
}

#[test]
fn new_principle_creates_an_active_artifact_with_idea_28s_template() {
    let tmp = init_repo();

    let out = run(tmp.path(), &["new", "principle", "Name the contamination"]);

    assert_eq!(
        out,
        "created principle:1 at archaeology/principles/0001-name-the-contamination.md\n"
    );
    let written = fs::read_to_string(
        tmp.path()
            .join(PRINCIPLES_DIR)
            .join("0001-name-the-contamination.md"),
    )
    .unwrap();
    assert!(written.contains("status: active"), "{written}");
    assert!(written.contains("id: prn_"), "{written}");
    for section in [
        "## Statement",
        "## Rationale",
        "## Application ordering",
        "## Counterpressure",
        "## Failure signals",
    ] {
        assert!(written.contains(section), "missing {section}:\n{written}");
    }
    // Template order is the idea's order.
    let statement = written.find("## Statement").unwrap();
    let signals = written.find("## Failure signals").unwrap();
    assert!(statement < signals, "{written}");
}

#[test]
fn principles_have_one_state_and_no_lifecycle_verbs() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "principle", "A principle"]);
    let path = tmp.path().join(PRINCIPLES_DIR).join("0001-a-principle.md");
    let before = fs::read_to_string(&path).unwrap();

    for verb in ["close", "reopen", "adopt", "reject"] {
        let out = scarp_in(tmp.path(), &[verb, "principle:1"]);

        assert_eq!(out.status.code(), Some(2), "{verb}: {}", stderr(&out));
        assert!(
            stderr(&out).contains("principles have no lifecycle verbs"),
            "{verb}: {}",
            stderr(&out)
        );
    }
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        before,
        "a refused verb must not touch the artifact"
    );
}

#[test]
fn a_principle_in_any_other_state_is_malformed() {
    let tmp = init_repo();
    let dir = tmp.path().join(PRINCIPLES_DIR);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("0001-retired.md"),
        "---\nid: prn-x\nsequence: 1\nkind: principle\nstatus: retired\ncreated: 2026-08-04\n---\n\n# Retired\n",
    )
    .unwrap();

    let out = scarp_in(tmp.path(), &["doctor"]);

    assert_eq!(out.status.code(), Some(9), "{}", stderr(&out));
    assert!(
        stdout(&out).contains("malformed-artifact") && stdout(&out).contains("active"),
        "the admitted vocabulary must be named: {}",
        stdout(&out)
    );
}

#[test]
fn doctor_validates_principles_and_judges_no_conformance() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "principle", "A principle"]);
    run(tmp.path(), &["new", "dragon", "A risk that ignores it"]);

    let report = run(tmp.path(), &["doctor"]);

    assert!(report.contains("2 artifact(s) checked"), "{report}");
    assert!(
        report.contains("no problems found"),
        "principles advise; conformance is never a finding: {report}"
    );
}

#[test]
fn sugar_in_a_creation_body_binds_at_the_write_boundary() {
    // The invariant task 63 established: authored prose that becomes
    // canonical through a Scarp write carries canonical markers, whichever
    // command performed the write.
    let tmp = init_repo();
    seed_log(
        tmp.path(),
        3,
        "log_01KYK8RC0YEY51YP37RGV7M7N4",
        "Blind spots",
    );
    let file = body(
        tmp.path(),
        "b.md",
        "## Statement\n\nCheck elsewhere.\n\n## Rationale\n\nFound in [[log:3]].\n",
    );

    run(
        tmp.path(),
        &["new", "principle", "A principle", "--body-file", &file],
    );

    let written =
        fs::read_to_string(tmp.path().join(PRINCIPLES_DIR).join("0001-a-principle.md")).unwrap();
    assert!(
        written.contains("[[log_01KYK8RC0YEY51YP37RGV7M7N4|Blind spots]]"),
        "sugar must bind to the stable id and the target's title: {written}"
    );
    assert!(!written.contains("[[log:3]]"), "{written}");
}

#[test]
fn an_explicit_label_survives_creation_binding() {
    let tmp = init_repo();
    seed_log(
        tmp.path(),
        3,
        "log-x",
        "A very long title nobody wants inline",
    );
    let file = body(
        tmp.path(),
        "b.md",
        "## Problem\n\nAs [[log:3|log 3]] records.\n",
    );

    run(
        tmp.path(),
        &["new", "idea", "An idea", "--body-file", &file],
    );

    let written = fs::read_to_string(tmp.path().join("archaeology/ideas/0001-an-idea.md")).unwrap();
    assert!(written.contains("[[log-x|log 3]]"), "{written}");
}

#[test]
fn unresolvable_sugar_refuses_creation_without_leaving_an_artifact() {
    let tmp = init_repo();
    let file = body(tmp.path(), "b.md", "## Statement\n\nSee [[log:99]].\n");

    let out = scarp_in(
        tmp.path(),
        &["new", "principle", "Doomed", "--body-file", &file],
    );

    assert_eq!(out.status.code(), Some(7), "{}", stderr(&out));
    assert!(stderr(&out).contains("[[log:99]]"), "{}", stderr(&out));
    assert!(
        !tmp.path().join(PRINCIPLES_DIR).exists()
            || fs::read_dir(tmp.path().join(PRINCIPLES_DIR))
                .unwrap()
                .next()
                .is_none(),
        "a refused creation must leave no artifact behind"
    );
}

#[test]
fn creation_binding_applies_to_every_templated_collection() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "dragon", "A risk"]);
    let file = body(tmp.path(), "b.md", "## Context\n\nAfter [[dragon:1]].\n");

    run(
        tmp.path(),
        &["new", "decision", "A decision", "--body-file", &file],
    );

    let written =
        fs::read_to_string(tmp.path().join("archaeology/decisions/0001-a-decision.md")).unwrap();
    assert!(written.contains("[[drg_"), "{written}");
    assert!(written.contains("|A risk]]"), "{written}");
}

#[test]
fn creation_binding_applies_to_a_template_free_collection() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "dragon", "A risk"]);
    let file = body(tmp.path(), "b.md", "Prose citing [[dragon:1]] verbatim.\n");

    run(tmp.path(), &["new", "log", "A log", "--body-file", &file]);

    let written = fs::read_to_string(tmp.path().join("archaeology/logs/0001-a-log.md")).unwrap();
    assert!(written.contains("|A risk]]"), "{written}");
}

#[test]
fn markers_in_creation_code_spans_stay_sugar() {
    let tmp = init_repo();
    run(tmp.path(), &["new", "dragon", "A risk"]);
    let file = body(
        tmp.path(),
        "b.md",
        "## Problem\n\nWrite `[[dragon:1]]` to cite it.\n",
    );

    run(
        tmp.path(),
        &["new", "idea", "An idea", "--body-file", &file],
    );

    let written = fs::read_to_string(tmp.path().join("archaeology/ideas/0001-an-idea.md")).unwrap();
    assert!(written.contains("`[[dragon:1]]`"), "{written}");
}
