//! Regression coverage for the Scarp release identity (decision 16).
//!
//! The 2026-07-27 rename from Strata to Scarp touched the package, the
//! executable, the library, the repository marker, and every diagnostic. A
//! purely textual rename would pass a search and still ship the wrong
//! product name in generated completions or a stale marker probe, so these
//! tests pin the identity through observable behavior instead: what the
//! binary reports, what it generates, and what it accepts as a repository.
//!
//! The historical corpus is deliberately out of scope. Only current
//! surfaces are asserted here.

use std::fs;
use std::path::Path;
use std::process::Output;

const CONFIG_FILE: &str = ".scarp.toml";
const RETIRED_CONFIG_FILE: &str = ".strata.toml";

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

#[test]
fn version_output_names_the_scarp_executable() {
    let tmp = tempfile::tempdir().unwrap();
    let out = scarp_in(tmp.path(), &["--version"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(
        stdout(&out).trim(),
        format!("scarp {}", env!("CARGO_PKG_VERSION")),
        "--version must report the package name, not just the number"
    );
}

#[test]
fn help_output_names_the_scarp_executable_and_never_the_old_name() {
    let tmp = tempfile::tempdir().unwrap();
    let out = scarp_in(tmp.path(), &["--help"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let help = stdout(&out);
    assert!(
        help.contains("Usage: scarp"),
        "help must show the current executable name:\n{help}"
    );
    assert!(
        !help.to_lowercase().contains("strata"),
        "help must not carry the retired product name:\n{help}"
    );
}

#[test]
fn generated_completions_carry_the_scarp_identity_for_every_shell() {
    let tmp = tempfile::tempdir().unwrap();
    // Completion scripts embed the command name in shell-specific ways; the
    // zsh function name is what users install as `~/.zfunc/_scarp`.
    for shell in ["bash", "zsh", "fish", "elvish", "powershell"] {
        let out = scarp_in(tmp.path(), &["completions", shell]);
        assert!(
            out.status.success(),
            "completions {shell}:\n{}",
            stderr(&out)
        );
        let script = stdout(&out);
        assert!(
            script.contains("scarp"),
            "completions {shell} must name the scarp command"
        );
        assert!(
            !script.to_lowercase().contains("strata"),
            "completions {shell} must not carry the retired command name"
        );
    }

    let zsh = stdout(&scarp_in(tmp.path(), &["completions", "zsh"]));
    assert!(zsh.contains("#compdef scarp"), "zsh compdef header:\n{zsh}");
    assert!(zsh.contains("_scarp"), "zsh function name:\n{zsh}");
    let bash = stdout(&scarp_in(tmp.path(), &["completions", "bash"]));
    assert!(bash.contains("_scarp"), "bash function name:\n{bash}");
}

#[test]
fn init_writes_the_scarp_marker_and_never_the_retired_one() {
    let tmp = tempfile::tempdir().unwrap();
    let out = scarp_in(tmp.path(), &["init"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(
        fs::read_to_string(tmp.path().join(CONFIG_FILE)).unwrap(),
        "version = 1\n",
        "the marker keeps schema version 1: only its filename changed"
    );
    assert!(
        !tmp.path().join(RETIRED_CONFIG_FILE).exists(),
        "init must not write the retired marker"
    );
    assert!(
        stdout(&out).contains(CONFIG_FILE),
        "init must report the marker it wrote:\n{}",
        stdout(&out)
    );
}

#[test]
fn the_retired_marker_alone_does_not_mark_a_repository() {
    // Decision 16 takes a pre-release hard cut: discovery recognizes exactly
    // one marker filename. A directory carrying only `.strata.toml` is not a
    // repository, and no fallback probe rescues it.
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join(RETIRED_CONFIG_FILE), "version = 1\n").unwrap();

    let out = scarp_in(tmp.path(), &["list", "dragons"]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "the retired marker must not be discovered:\n{}",
        stderr(&out)
    );
    let err = stderr(&out);
    assert!(
        err.contains("error[missing-repository]"),
        "typed error category:\n{err}"
    );
    assert!(
        err.contains("scarp init"),
        "the remedy names the current executable:\n{err}"
    );
}

#[test]
fn discovery_mutation_and_doctor_all_run_through_the_scarp_marker() {
    let tmp = tempfile::tempdir().unwrap();
    assert!(scarp_in(tmp.path(), &["init"]).status.success());

    // Create from a nested directory: discovery walks up to the marker.
    let nested = tmp.path().join("src/deep");
    fs::create_dir_all(&nested).unwrap();
    let created = scarp_in(&nested, &["new", "dragon", "Marker discovery"]);
    assert!(created.status.success(), "{}", stderr(&created));

    let closed = scarp_in(&nested, &["close", "dragon:1"]);
    assert!(closed.status.success(), "{}", stderr(&closed));

    let doctor = scarp_in(&nested, &["doctor"]);
    assert!(doctor.status.success(), "{}", stderr(&doctor));
    assert!(
        stdout(&doctor).contains("no problems found"),
        "{}",
        stdout(&doctor)
    );

    // Renaming the marker away removes the repository entirely.
    fs::rename(
        tmp.path().join(CONFIG_FILE),
        tmp.path().join(RETIRED_CONFIG_FILE),
    )
    .unwrap();
    let out = scarp_in(&nested, &["doctor"]);
    assert_eq!(out.status.code(), Some(3), "{}", stderr(&out));
}
