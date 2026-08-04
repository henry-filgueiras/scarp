//! Integration tests for the managed logs collection (task 61) through
//! the compiled binary.
//!
//! Logs are the first stateless, template-free collection, so these tests
//! carry two burdens the other collections' suites do not: that a log's
//! absent lifecycle is visible in every projection, and that adopting a
//! collection whose artifacts predate the tool changes none of them.
//!
//! Every invocation pins its working directory to a fresh temporary
//! directory so discovery can never walk up into a real repository.

use std::fs;
use std::path::Path;
use std::process::Output;

const LOGS_DIR: &str = "archaeology/logs";

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

fn init_repo() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let out = scarp_in(tmp.path(), &["init"]);
    assert!(out.status.success(), "init failed:\n{}", stderr(&out));
    tmp
}

/// The exact front-matter shape of the three logs that predate the
/// collection: no `status:` line, and a hand-seeded id on the older two.
fn seeded_log(id: &str, sequence: u32, title: &str, body: &str) -> String {
    format!(
        "---\nid: {id}\nsequence: {sequence}\nkind: log\ncreated: 2026-07-20\n---\n\n# {title}\n\n{body}"
    )
}

fn seed_log(root: &Path, name: &str, content: &str) {
    let dir = root.join(LOGS_DIR);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(name), content).unwrap();
}

#[test]
fn new_log_creates_a_stateless_artifact_and_reports_it() {
    let tmp = init_repo();

    let out = scarp_in(tmp.path(), &["new", "log", "What the release taught us"]);

    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(
        stdout(&out),
        "created log:1 at archaeology/logs/0001-what-the-release-taught-us.md\n"
    );
    let written = fs::read_to_string(
        tmp.path()
            .join(LOGS_DIR)
            .join("0001-what-the-release-taught-us.md"),
    )
    .unwrap();
    assert!(
        !written.contains("status:"),
        "logs carry no lifecycle:\n{written}"
    );
    assert!(written.contains("kind: log\n"), "{written}");
    assert!(written.contains("id: log_"), "{written}");
    // No template: the artifact is its front matter and its title.
    assert!(
        written.ends_with("---\n\n# What the release taught us\n"),
        "{written}"
    );
}

#[test]
fn a_log_body_is_taken_verbatim_including_its_own_headings() {
    let tmp = init_repo();
    let body_file = tmp.path().join("body.md");
    fs::write(
        &body_file,
        "The shape.\n\n## An author-chosen heading\n\nWhy it matters.\n",
    )
    .unwrap();

    let out = scarp_in(
        tmp.path(),
        &[
            "new",
            "log",
            "Verbatim",
            "--body-file",
            body_file.to_str().unwrap(),
        ],
    );

    assert!(out.status.success(), "{}", stderr(&out));
    let written = fs::read_to_string(tmp.path().join(LOGS_DIR).join("0001-verbatim.md")).unwrap();
    assert!(
        written.ends_with(
            "# Verbatim\n\nThe shape.\n\n## An author-chosen heading\n\nWhy it matters.\n"
        ),
        "{written}"
    );
}

#[test]
fn list_and_show_resolve_seeded_logs_by_sequence_and_by_stable_id() {
    let tmp = init_repo();
    seed_log(
        tmp.path(),
        "0001-inception.md",
        &seeded_log(
            "log-bootstrap-inception",
            1,
            "Project inception",
            "Prose.\n",
        ),
    );
    seed_log(
        tmp.path(),
        "0002-blind-spots.md",
        &seeded_log(
            "log_01KYK8RC0YEY51YP37RGV7M7N4",
            2,
            "Verification blind spots",
            "## The shape\n\nProse.\n",
        ),
    );

    let listed = scarp_in(tmp.path(), &["list", "logs"]);
    assert!(listed.status.success(), "{}", stderr(&listed));
    let text = stdout(&listed);
    assert!(text.contains("log:1  Project inception"), "{text}");
    assert!(text.contains("log:2  Verification blind spots"), "{text}");

    for address in ["log:2", "log_01KYK8RC0YEY51YP37RGV7M7N4"] {
        let shown = scarp_in(tmp.path(), &["show", address]);
        assert!(shown.status.success(), "{address}: {}", stderr(&shown));
        assert!(
            stdout(&shown).contains("# Verification blind spots"),
            "{address} did not resolve"
        );
    }
}

#[test]
fn the_json_projection_omits_status_rather_than_emitting_null() {
    let tmp = init_repo();
    seed_log(
        tmp.path(),
        "0001-inception.md",
        &seeded_log(
            "log-bootstrap-inception",
            1,
            "Project inception",
            "Prose.\n",
        ),
    );

    let out = scarp_in(tmp.path(), &["list", "logs", "--json"]);

    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(
        stdout(&out).trim(),
        "[{\"id\":\"log-bootstrap-inception\",\"sequence\":1,\"kind\":\"log\",\
         \"title\":\"Project inception\",\"created\":\"2026-07-20\",\
         \"path\":\"archaeology/logs/0001-inception.md\"}]"
    );
}

#[test]
fn a_stateful_collections_json_projection_is_unchanged_by_the_optional_status() {
    // The other half of the compatibility claim: the key stays present,
    // in position, for every collection that has a lifecycle.
    let tmp = init_repo();
    let created = scarp_in(tmp.path(), &["new", "dragon", "A risk"]);
    assert!(created.status.success(), "{}", stderr(&created));

    let out = scarp_in(tmp.path(), &["list", "dragons", "--json"]);

    let text = stdout(&out);
    assert!(
        text.contains("\"kind\":\"dragon\",\"status\":\"open\",\"title\":\"A risk\""),
        "the status key must keep its place: {text}"
    );
}

#[test]
fn lifecycle_verbs_refuse_logs_with_truthful_guidance() {
    let tmp = init_repo();
    seed_log(
        tmp.path(),
        "0001-inception.md",
        &seeded_log(
            "log-bootstrap-inception",
            1,
            "Project inception",
            "Prose.\n",
        ),
    );

    for verb in ["close", "reopen", "adopt", "reject"] {
        let out = scarp_in(tmp.path(), &[verb, "log:1"]);

        assert_eq!(out.status.code(), Some(2), "{verb} must be a usage error");
        let err = stderr(&out);
        assert!(
            err.contains("logs have no lifecycle verbs"),
            "{verb}: {err}"
        );
        // The refusal must not have touched the file.
        let written =
            fs::read_to_string(tmp.path().join(LOGS_DIR).join("0001-inception.md")).unwrap();
        assert!(!written.contains("status:"), "{verb} mutated the log");
    }
}

#[test]
fn a_status_line_on_a_log_is_a_doctor_finding() {
    let tmp = init_repo();
    seed_log(
        tmp.path(),
        "0001-inception.md",
        "---\nid: log-x\nsequence: 1\nkind: log\nstatus: open\ncreated: 2026-07-20\n---\n\n# Inception\n",
    );

    let out = scarp_in(tmp.path(), &["doctor"]);

    assert_eq!(out.status.code(), Some(9), "{}", stderr(&out));
    let report = stdout(&out);
    assert!(
        report.contains("malformed-artifact")
            && report.contains("archaeology/logs/0001-inception.md")
            && report.contains("no lifecycle"),
        "{report}"
    );
}

#[test]
fn doctor_validates_logs_under_the_same_invariants_as_every_collection() {
    let tmp = init_repo();
    seed_log(
        tmp.path(),
        "0001-inception.md",
        &seeded_log(
            "log-bootstrap-inception",
            1,
            "Project inception",
            "Prose.\n",
        ),
    );
    seed_log(
        tmp.path(),
        "0002-second.md",
        &seeded_log("log-second", 2, "Second", "Prose.\n"),
    );

    let healthy = scarp_in(tmp.path(), &["doctor"]);
    assert!(healthy.status.success(), "{}", stderr(&healthy));
    assert!(
        stdout(&healthy).contains("2 artifact(s) checked"),
        "logs must be counted among checked artifacts: {}",
        stdout(&healthy)
    );

    // A sequence that disagrees with its filename is malformed here for
    // the same reason it is anywhere else.
    seed_log(
        tmp.path(),
        "0003-mismatched.md",
        &seeded_log("log-third", 7, "Mismatched", "Prose.\n"),
    );
    let unhealthy = scarp_in(tmp.path(), &["doctor"]);
    assert_eq!(unhealthy.status.code(), Some(9));
    assert!(
        stdout(&unhealthy).contains("archaeology/logs/0003-mismatched.md"),
        "{}",
        stdout(&unhealthy)
    );
}

#[test]
fn new_log_allocates_the_next_sequence_after_seeded_logs() {
    let tmp = init_repo();
    seed_log(
        tmp.path(),
        "0003-blind-spots.md",
        &seeded_log("log-third", 3, "Blind spots", "Prose.\n"),
    );

    let out = scarp_in(tmp.path(), &["new", "log", "The fourth"]);

    assert!(out.status.success(), "{}", stderr(&out));
    assert!(
        stdout(&out).contains("created log:4 at archaeology/logs/0004-the-fourth.md"),
        "{}",
        stdout(&out)
    );
}

#[test]
fn adopting_the_collection_leaves_pre_existing_logs_byte_identical() {
    // The load-bearing migration claim of task 61: the corpus was already
    // conformant, so managing it rewrites nothing.
    let tmp = init_repo();
    let original = seeded_log(
        "log-bootstrap-inception",
        1,
        "Project inception",
        "Prose with no sections at all.\n",
    );
    seed_log(tmp.path(), "0001-inception.md", &original);
    let path = tmp.path().join(LOGS_DIR).join("0001-inception.md");

    for args in [
        vec!["doctor"],
        vec!["list", "logs"],
        vec!["list", "logs", "--json"],
        vec!["show", "log:1"],
        vec!["show", "log-bootstrap-inception"],
        vec!["new", "log", "A second log"],
    ] {
        let out = scarp_in(tmp.path(), &args);
        assert!(out.status.success(), "{args:?}: {}", stderr(&out));
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            original,
            "{args:?} rewrote a pre-existing log"
        );
    }
}
