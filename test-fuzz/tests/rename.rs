use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn rename() {
    Command::new("cargo")
        .args(&[
            "test",
            "--workspace",
            "--package",
            "test-fuzz-examples",
            "--no-run",
            "--features",
            &("test-fuzz/serde_".to_owned() + test_fuzz::serde_format()),
        ])
        .assert()
        .success();

    Command::new("cargo")
        .args(&[
            "test",
            "--workspace",
            "--package",
            "test-fuzz-examples",
            "--no-run",
            "--features",
            &("test-fuzz/serde_".to_owned() + test_fuzz::serde_format()),
            "--features",
            "test-fuzz-examples/bar_fuzz",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the name `bar_fuzz` is defined multiple times",
        ));
}
