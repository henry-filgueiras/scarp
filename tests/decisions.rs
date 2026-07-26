//! Integration tests for the managed decisions collection (task 32)
//! through the compiled binary: creation, listing, the full addressing
//! matrix for `show`, and the lifecycle-verb refusals.
//!
//! Every invocation pins its working directory to a fresh temporary
//! directory so discovery can never walk up into a real repository.

use std::fs;
use std::path::Path;
use std::process::Output;

const DECISIONS_DIR: &str = "archaeology/decisions";

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

fn init_repo() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let out = strata_in(tmp.path(), &["init"]);
    assert!(out.status.success(), "init failed:\n{}", stderr(&out));
    tmp
}

fn decision_markdown(id: &str, sequence: u32, title: &str) -> String {
    format!(
        "---\nid: {id}\nsequence: {sequence}\nkind: decision\nstatus: accepted\ncreated: 2026-07-20\n---\n\n# {title}\n\n## Context\n"
    )
}

fn seed_decision(root: &Path, name: &str, content: &str) {
    let dir = root.join(DECISIONS_DIR);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(name), content).unwrap();
}

/// A repository holding one legacy-id decision (the existing corpus
/// shape) and one freshly created decision with a generated `dec_` id.
/// Returns the legacy artifact's exact bytes and the generated id.
fn seed_mixed_corpus(root: &Path) -> (String, String) {
    let legacy = decision_markdown("dec-bootstrap-files-canonical", 1, "Files are canonical");
    seed_decision(root, "0001-files-are-canonical.md", &legacy);
    let out = strata_in(root, &["new", "decision", "Adopt the spec engine"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let created = fs::read_to_string(
        root.join(DECISIONS_DIR)
            .join("0002-adopt-the-spec-engine.md"),
    )
    .unwrap();
    let id = created
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .expect("created decision carries an id")
        .to_string();
    (legacy, id)
}

#[test]
fn new_decision_creates_an_accepted_artifact_and_reports_it() {
    let tmp = init_repo();

    let out = strata_in(tmp.path(), &["new", "decision", "Files are canonical"]);

    assert!(out.status.success(), "{}", stderr(&out));
    assert!(
        stdout(&out).contains("created decision:1"),
        "{}",
        stdout(&out)
    );
    let content = fs::read_to_string(
        tmp.path()
            .join(DECISIONS_DIR)
            .join("0001-files-are-canonical.md"),
    )
    .unwrap();
    for needle in [
        "kind: decision",
        "status: accepted",
        "# Files are canonical",
        "## Context",
        "## Decision",
        "## Consequences",
    ] {
        assert!(content.contains(needle), "missing `{needle}`:\n{content}");
    }
}

#[test]
fn new_decision_json_pins_the_creation_record() {
    let tmp = init_repo();

    let out = strata_in(tmp.path(), &["new", "decision", "Adopt X", "--json"]);

    assert!(out.status.success(), "{}", stderr(&out));
    let value: serde_json::Value = serde_json::from_str(&stdout(&out)).unwrap();
    assert_eq!(value["kind"], "decision");
    assert_eq!(value["sequence"], 1);
    assert_eq!(value["reference"], "decision:1");
    assert_eq!(value["path"], "archaeology/decisions/0001-adopt-x.md");
    let id = value["id"].as_str().unwrap();
    assert!(id.starts_with("dec_"), "{id}");
}

#[test]
fn list_decisions_orders_the_corpus_by_sequence_in_both_projections() {
    let tmp = init_repo();
    let (_, generated_id) = seed_mixed_corpus(tmp.path());

    let human = strata_in(tmp.path(), &["list", "decisions"]);
    assert!(human.status.success(), "{}", stderr(&human));
    let text = stdout(&human);
    let first = text.find("decision:1").expect(&text);
    let second = text.find("decision:2").expect(&text);
    assert!(first < second, "sequences must ascend:\n{text}");
    assert!(text.contains("accepted"), "{text}");

    let json = strata_in(tmp.path(), &["list", "decisions", "--json"]);
    assert!(json.status.success(), "{}", stderr(&json));
    let value: serde_json::Value = serde_json::from_str(&stdout(&json)).unwrap();
    let items = value.as_array().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["id"], "dec-bootstrap-files-canonical");
    assert_eq!(items[0]["kind"], "decision");
    assert_eq!(items[0]["status"], "accepted");
    assert_eq!(items[1]["id"], generated_id.as_str());
}

#[test]
fn show_resolves_every_address_kind_in_raw_and_json_forms() {
    // Task 32's addressing matrix: `decision:N`, a legacy stable id, and
    // a generated `dec_` stable id, each shown raw and as JSON.
    let tmp = init_repo();
    let (legacy, generated_id) = seed_mixed_corpus(tmp.path());
    let generated = fs::read_to_string(
        tmp.path()
            .join(DECISIONS_DIR)
            .join("0002-adopt-the-spec-engine.md"),
    )
    .unwrap();

    for (address, expected) in [
        ("decision:1", &legacy),
        ("dec-bootstrap-files-canonical", &legacy),
        (generated_id.as_str(), &generated),
    ] {
        let raw = strata_in(tmp.path(), &["show", address]);
        assert!(raw.status.success(), "show {address}:\n{}", stderr(&raw));
        assert_eq!(
            stdout(&raw),
            *expected,
            "raw show must reproduce the file exactly for `{address}`"
        );

        let json = strata_in(tmp.path(), &["show", address, "--json"]);
        assert!(
            json.status.success(),
            "show {address} --json:\n{}",
            stderr(&json)
        );
        let value: serde_json::Value = serde_json::from_str(&stdout(&json)).unwrap();
        assert_eq!(value["kind"], "decision", "for `{address}`");
        assert_eq!(value["status"], "accepted", "for `{address}`");
        assert_eq!(
            value["content"],
            serde_json::Value::String(expected.clone()),
            "JSON content must be the canonical bytes for `{address}`"
        );
    }
}

#[test]
fn lifecycle_verbs_refuse_decisions_with_truthful_guidance() {
    // Task 32: the refusal names the operation, the artifact, and why
    // decisions have no such transition — never a generic parse or
    // not-found error — for both address forms of every verb.
    let tmp = init_repo();
    seed_mixed_corpus(tmp.path());

    for (verb, args) in [
        ("close", &["close", "decision:1"] as &[&str]),
        ("close", &["close", "dec-bootstrap-files-canonical"]),
        ("reopen", &["reopen", "decision:1"]),
        ("reopen", &["reopen", "dec-bootstrap-files-canonical"]),
        ("adopt", &["adopt", "decision:1"]),
        ("adopt", &["adopt", "dec-bootstrap-files-canonical"]),
        ("reject", &["reject", "decision:1"]),
        ("reject", &["reject", "dec-bootstrap-files-canonical"]),
    ] {
        let out = strata_in(tmp.path(), args);
        assert_eq!(out.status.code(), Some(2), "{args:?}:\n{}", stderr(&out));
        let err = stderr(&out);
        assert!(
            err.starts_with("error[invalid-invocation]: "),
            "{args:?}:\n{err}"
        );
        assert!(
            err.contains(&format!("cannot {verb} `{}`", args[1])),
            "must name the operation and artifact for {args:?}:\n{err}"
        );
        assert!(
            err.contains("no lifecycle transitions"),
            "must say why for {args:?}:\n{err}"
        );
        assert!(
            !err.contains("not-found") && !err.contains("artifact-not-found"),
            "must not decay into not-found for {args:?}:\n{err}"
        );
    }

    // Nothing was mutated by any refusal.
    let legacy = fs::read_to_string(
        tmp.path()
            .join(DECISIONS_DIR)
            .join("0001-files-are-canonical.md"),
    )
    .unwrap();
    assert_eq!(
        legacy,
        decision_markdown("dec-bootstrap-files-canonical", 1, "Files are canonical"),
        "refusals must leave decisions byte-identical"
    );
}

#[test]
fn doctor_validates_decisions_and_stays_green_on_a_conformant_corpus() {
    let tmp = init_repo();
    seed_mixed_corpus(tmp.path());

    let healthy = strata_in(tmp.path(), &["doctor"]);
    assert!(healthy.status.success(), "{}", stderr(&healthy));
    assert!(
        stdout(&healthy).contains("2 artifact(s) checked, no problems found"),
        "{}",
        stdout(&healthy)
    );

    seed_decision(
        tmp.path(),
        "0003-broken.md",
        "---\nid: dec-broken\nsequence: 3\nkind: decision\nstatus: proposed\ncreated: 2026-07-20\n---\n\n# Broken\n",
    );

    let sick = strata_in(tmp.path(), &["doctor"]);
    assert_eq!(sick.status.code(), Some(9), "{}", stderr(&sick));
    assert!(
        stdout(&sick).contains("archaeology/decisions/0003-broken.md"),
        "{}",
        stdout(&sick)
    );
}

#[test]
fn resolved_by_still_binds_a_dragon_to_a_managed_decision() {
    // Typed-edge resolution through the claimant catalog is unchanged by
    // managing decisions: `close --resolved-by decision:N` binds and the
    // repository stays healthy.
    let tmp = init_repo();
    seed_mixed_corpus(tmp.path());
    let out = strata_in(tmp.path(), &["new", "dragon", "A risk"]);
    assert!(out.status.success(), "{}", stderr(&out));

    let closed = strata_in(
        tmp.path(),
        &["close", "dragon:1", "--resolved-by", "decision:1"],
    );

    assert!(closed.status.success(), "{}", stderr(&closed));
    let dragon = fs::read_to_string(
        tmp.path()
            .join("archaeology/dragons")
            .join("0001-a-risk.md"),
    )
    .unwrap();
    assert!(
        dragon.contains("resolved-by: \"[[dec-bootstrap-files-canonical|Files are canonical]]\""),
        "{dragon}"
    );
    let doctor = strata_in(tmp.path(), &["doctor"]);
    assert!(doctor.status.success(), "{}", stderr(&doctor));
}
