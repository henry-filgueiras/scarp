//! Integration tests for terminal narrative on the close transition
//! (task 62, adopting idea 30's mechanism half).
//!
//! Two claims carry the task. The narrative and the transition are one
//! write, so neither can land without the other; and legal `[[kind:N]]`
//! sugar in the supplied narrative is bound at authorship time, so an
//! author closing an artifact never hand-copies a ULID.

use std::fs;
use std::path::Path;
use std::process::Output;

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

/// A repository with one active sprint, one pending task, one open dragon.
fn seeded_repo() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    run(tmp.path(), &["init"]);
    run(tmp.path(), &["new", "sprint", "A sprint"]);
    run(tmp.path(), &["new", "task", "A task"]);
    run(tmp.path(), &["new", "dragon", "A risk"]);
    tmp
}

fn body(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    fs::write(&path, text).unwrap();
    path.to_str().unwrap().to_string()
}

const TASK: &str = "archaeology/sprints/0001-a-sprint/0001-a-task.md";
const DRAGON: &str = "archaeology/dragons/0001-a-risk.md";
const SPRINT: &str = "archaeology/sprints/0001-a-sprint/sprint.md";

fn today() -> String {
    jiff::Zoned::now().strftime("%Y-%m-%d").to_string()
}

#[test]
fn closing_a_task_writes_the_result_section_and_the_transition_together() {
    let tmp = seeded_repo();
    let file = body(tmp.path(), "r.md", "Delivered as specified.\n");

    run(tmp.path(), &["close", "task:1", "--body-file", &file]);

    let written = fs::read_to_string(tmp.path().join(TASK)).unwrap();
    assert!(written.contains("status: closed"), "{written}");
    assert!(
        written.contains(&format!("closed: {}", today())),
        "{written}"
    );
    assert!(
        written.ends_with("## Result\n\nDelivered as specified.\n"),
        "{written}"
    );
}

#[test]
fn dragons_and_sprints_take_their_own_dated_headings() {
    // Read off the corpus: tasks carry a bare `## Result`, dragons and
    // sprints carry dated headings.
    let tmp = seeded_repo();
    let file = body(tmp.path(), "r.md", "The story.\n");

    run(tmp.path(), &["close", "dragon:1", "--body-file", &file]);
    let dragon = fs::read_to_string(tmp.path().join(DRAGON)).unwrap();
    assert!(
        dragon.ends_with(&format!("## Resolution ({})\n\nThe story.\n", today())),
        "{dragon}"
    );

    run(tmp.path(), &["close", "task:1", "--body-file", &file]);
    run(tmp.path(), &["close", "sprint:1", "--body-file", &file]);
    let sprint = fs::read_to_string(tmp.path().join(SPRINT)).unwrap();
    assert!(
        sprint.ends_with(&format!("## Retrospective ({})\n\nThe story.\n", today())),
        "{sprint}"
    );
}

#[test]
fn sugar_in_the_narrative_is_bound_at_authorship_time() {
    let tmp = seeded_repo();
    let dragon_id = fs::read_to_string(tmp.path().join(DRAGON))
        .unwrap()
        .lines()
        .find_map(|line| line.strip_prefix("id: ").map(str::to_string))
        .unwrap();
    let file = body(
        tmp.path(),
        "r.md",
        "It resolved [[dragon:1]] along the way.\n",
    );

    run(tmp.path(), &["close", "task:1", "--body-file", &file]);

    let written = fs::read_to_string(tmp.path().join(TASK)).unwrap();
    assert!(
        written.contains(&format!("[[{dragon_id}|A risk]]")),
        "sugar must bind to the stable id with the target's title: {written}"
    );
    assert!(
        !written.contains("[[dragon:1]]"),
        "the sugar form must not survive: {written}"
    );
}

#[test]
fn an_authors_explicit_label_survives_binding() {
    // The label was written beside a reference the tool then verified, so
    // the pairing is correct by construction; replacing it would flatten
    // the author's prose into a title.
    let tmp = seeded_repo();
    let file = body(
        tmp.path(),
        "r.md",
        "It resolved [[dragon:1|the risk we already knew about]].\n",
    );

    run(tmp.path(), &["close", "task:1", "--body-file", &file]);

    let written = fs::read_to_string(tmp.path().join(TASK)).unwrap();
    assert!(
        written.contains("|the risk we already knew about]]"),
        "{written}"
    );
    assert!(
        written.contains("[[drg_"),
        "the id must still bind: {written}"
    );
}

#[test]
fn already_bound_markers_pass_through_untouched() {
    let tmp = seeded_repo();
    let file = body(
        tmp.path(),
        "r.md",
        "See [[some-other-id|a label of my own]].\n",
    );

    run(tmp.path(), &["close", "task:1", "--body-file", &file]);

    let written = fs::read_to_string(tmp.path().join(TASK)).unwrap();
    assert!(
        written.contains("[[some-other-id|a label of my own]]"),
        "a bound marker is already canonical and is not this task's business: {written}"
    );
}

#[test]
fn markers_inside_code_are_mentions_and_are_left_alone() {
    let tmp = seeded_repo();
    let file = body(
        tmp.path(),
        "r.md",
        "Write `[[dragon:1]]` to cite it.\n\n```\n[[dragon:1]]\n```\n",
    );

    run(tmp.path(), &["close", "task:1", "--body-file", &file]);

    let written = fs::read_to_string(tmp.path().join(TASK)).unwrap();
    assert_eq!(
        written.matches("[[dragon:1]]").count(),
        2,
        "both the code span and the fenced block must survive: {written}"
    );
}

#[test]
fn unresolvable_sugar_refuses_the_whole_closure_without_touching_the_file() {
    let tmp = seeded_repo();
    let before = fs::read_to_string(tmp.path().join(TASK)).unwrap();
    let file = body(tmp.path(), "r.md", "See [[task:999]].\n");

    let out = scarp_in(tmp.path(), &["close", "task:1", "--body-file", &file]);

    assert_eq!(out.status.code(), Some(7), "{}", stderr(&out));
    assert!(
        stderr(&out).contains("[[task:999]]"),
        "the diagnostic must name the marker, not the closure's target: {}",
        stderr(&out)
    );
    assert_eq!(
        fs::read_to_string(tmp.path().join(TASK)).unwrap(),
        before,
        "a refused narrative must leave the artifact byte-identical"
    );
}

#[test]
fn a_body_that_writes_the_owned_heading_is_refused() {
    let tmp = seeded_repo();
    let before = fs::read_to_string(tmp.path().join(TASK)).unwrap();
    let file = body(tmp.path(), "r.md", "## Result\n\nDelivered.\n");

    let out = scarp_in(tmp.path(), &["close", "task:1", "--body-file", &file]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(
        stderr(&out).contains("Scarp writes that heading"),
        "{}",
        stderr(&out)
    );
    assert_eq!(fs::read_to_string(tmp.path().join(TASK)).unwrap(), before);
}

#[test]
fn an_empty_body_is_refused_rather_than_written_as_an_empty_section() {
    let tmp = seeded_repo();
    let file = body(tmp.path(), "r.md", "\n  \n");

    let out = scarp_in(tmp.path(), &["close", "task:1", "--body-file", &file]);

    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(stderr(&out).contains("is empty"), "{}", stderr(&out));
}

#[test]
fn closing_without_a_body_file_appends_nothing() {
    // A creation stub must never grow an empty terminal section, and the
    // old two-step flow must keep working unchanged.
    let tmp = seeded_repo();

    run(tmp.path(), &["close", "task:1"]);

    let written = fs::read_to_string(tmp.path().join(TASK)).unwrap();
    assert!(written.contains("status: closed"), "{written}");
    assert!(!written.contains("## Result"), "{written}");
}

#[test]
fn a_narrative_composes_with_resolved_by_in_the_one_write() {
    let tmp = seeded_repo();
    run(tmp.path(), &["new", "decision", "A decision"]);
    let file = body(tmp.path(), "r.md", "Settled by the decision.\n");

    run(
        tmp.path(),
        &[
            "close",
            "dragon:1",
            "--resolved-by",
            "decision:1",
            "--body-file",
            &file,
        ],
    );

    let written = fs::read_to_string(tmp.path().join(DRAGON)).unwrap();
    assert!(written.contains("resolved-by: \"[[dec_"), "{written}");
    assert!(written.contains("## Resolution ("), "{written}");
    assert!(written.contains("Settled by the decision."), "{written}");
}

#[test]
fn a_repository_closed_with_narrative_stays_doctor_green() {
    let tmp = seeded_repo();
    let file = body(
        tmp.path(),
        "r.md",
        "Delivered, and it touched [[dragon:1]].\n",
    );

    run(tmp.path(), &["close", "task:1", "--body-file", &file]);
    let report = run(tmp.path(), &["doctor"]);

    assert!(
        report.contains("no problems found") && !report.contains("advice"),
        "a bound marker written by Scarp must not be a finding: {report}"
    );
}
