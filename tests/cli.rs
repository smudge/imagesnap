use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_displays_usage() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn invalid_warmup_errors() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(&["-w", "11"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Warmup must be between 0 and 10"));
}

#[test]
fn too_many_args_errors() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(&["file1", "file2"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid combination of arguments"));
}
