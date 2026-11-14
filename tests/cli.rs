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

#[test]
fn invalid_warmup_string_errors_without_a_camera() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(&["-w", "not-a-number"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse warmup"));
}

#[test]
fn non_finite_warmup_errors_without_a_camera() {
    for warmup in ["NaN", "inf", "-inf"] {
        Command::cargo_bin("imagesnap")
            .unwrap()
            .args(&["-w", warmup])
            .assert()
            .failure()
            .stderr(predicate::str::contains("Warmup must be between 0 and 10"));
    }
}

#[test]
fn negative_warmup_errors_without_a_camera() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(&["--warmup=-1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Warmup must be between 0 and 10"));
}

#[test]
fn incompatible_help_and_list_flags_error() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(&["-h", "-l"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid combination of arguments"));
}
