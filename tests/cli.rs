//! Integration tests for the bootstrap command surface and error contract.

use std::process::Output;

fn strata(args: &[&str]) -> Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_strata"))
        .args(args)
        .output()
        .expect("failed to run strata binary")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn help_lists_bootstrap_commands() {
    let out = strata(&["--help"]);
    assert!(out.status.success(), "--help must exit 0");
    let help = stdout(&out);
    for command in [
        "init",
        "new",
        "list",
        "show",
        "doctor",
        "close",
        "reopen",
        "fortune",
        "completions",
    ] {
        assert!(
            help.contains(command),
            "help output missing `{command}`:\n{help}"
        );
    }
}

#[test]
fn completions_emit_a_script_per_shell_outside_any_repository() {
    // Generation is pure clap-definition output: no repository discovery,
    // so it must succeed anywhere. Each shell's script carries a
    // recognizable opening.
    for (shell, needle) in [
        ("zsh", "#compdef strata"),
        ("bash", "_strata"),
        ("fish", "strata"),
    ] {
        let out = strata(&["completions", shell]);
        assert!(
            out.status.success(),
            "completions {shell}:\n{}",
            stderr(&out)
        );
        assert!(
            stdout(&out).contains(needle),
            "completions {shell} must contain `{needle}`"
        );
    }
}

#[test]
fn completions_reject_an_unknown_shell_as_a_usage_error() {
    let out = strata(&["completions", "csh"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("csh"), "{}", stderr(&out));
}

#[test]
fn zsh_completion_script_loads_cleanly_when_zsh_is_available() {
    // The acceptance bar for task 35: the emitted script sources without
    // error under `zsh -f` with compinit loaded. Skipped silently where
    // zsh is not installed.
    if !std::process::Command::new("zsh")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
    {
        return;
    }
    let out = strata(&["completions", "zsh"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("_strata"), &out.stdout).unwrap();
    let load = std::process::Command::new("zsh")
        .args([
            "-f",
            "-c",
            &format!(
                "fpath=({dir} $fpath); autoload -Uz compinit; \
                 compinit -u -d {dir}/zcompdump; source {dir}/_strata",
                dir = dir.path().display()
            ),
        ])
        .output()
        .expect("failed to run zsh");
    assert!(
        load.status.success(),
        "zsh must load the script without error:\n{}",
        String::from_utf8_lossy(&load.stderr)
    );
}

#[test]
fn version_flag_reports_version() {
    let out = strata(&["--version"]);
    assert!(out.status.success());
    assert!(stdout(&out).contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn missing_subcommand_is_a_usage_error() {
    let out = strata(&[]);
    assert_eq!(out.status.code(), Some(2), "usage errors must exit 2");
    assert!(
        stderr(&out).contains("Usage"),
        "stderr should show usage:\n{}",
        stderr(&out)
    );
}

#[test]
fn unknown_subcommand_is_a_usage_error() {
    let out = strata(&["daemonize"]);
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn unknown_collection_is_rejected_with_guidance() {
    let out = strata(&["new", "widget", "A title"]);
    assert_eq!(out.status.code(), Some(2), "invalid invocation must exit 2");
    let err = stderr(&out);
    assert!(
        err.contains("widget"),
        "error should name the input:\n{err}"
    );
    assert!(
        err.contains("dragon"),
        "error should list valid collections:\n{err}"
    );
}

#[test]
fn malformed_artifact_reference_is_rejected() {
    let out = strata(&["show", "dragon:seven"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(
        stderr(&out).contains("positive integer"),
        "error should state the expected form:\n{}",
        stderr(&out)
    );
}

#[test]
fn unknown_collection_in_reference_is_rejected() {
    let out = strata(&["show", "widget:1"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(
        stderr(&out).contains("widget"),
        "error should name the input:\n{}",
        stderr(&out)
    );
}

#[test]
fn zero_sequence_reference_is_rejected() {
    let out = strata(&["show", "dragon:0"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("start at 1"), "{}", stderr(&out));
}
