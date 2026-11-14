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
fn help_takes_precedence_over_other_options() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(&["-h", "-l", "--device", "nonexistent-camera"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn help_takes_precedence_over_invalid_warmup() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(&["--help", "--warmup", "not-a-number"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn malformed_warmup_precedes_device_lookup() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(["--warmup", "not-a-number", "--device", "not-a-camera"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse warmup"));
}

#[test]
fn out_of_range_warmup_precedes_device_lookup() {
    Command::cargo_bin("imagesnap")
        .unwrap()
        .args(["--warmup", "11", "--device", "not-a-camera"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Warmup must be between 0 and 10"));
}

#[test]
fn non_finite_warmup_precedes_device_lookup() {
    for warmup in ["NaN", "inf", "-inf", "-1"] {
        let warmup_arg = format!("--warmup={warmup}");
        Command::cargo_bin("imagesnap")
            .unwrap()
            .args(["--device", "not-a-camera", warmup_arg.as_str()])
            .assert()
            .failure()
            .stderr(predicate::str::contains("Warmup must be between 0 and 10"));
    }
}

#[test]
fn invalid_argument_shapes_skip_device_lookup() {
    for args in [
        &["--list", "--device", "not-a-camera"][..],
        &["first.jpg", "second.jpg", "--device", "not-a-camera"][..],
    ] {
        Command::cargo_bin("imagesnap")
            .unwrap()
            .args(args)
            .assert()
            .failure()
            .stderr(predicate::str::contains("Invalid combination of arguments"));
    }
}
