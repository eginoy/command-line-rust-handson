use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult{
    Command::cargo_bin("echor")?
    .args(args)
    .assert()
    .success()
    .stdout(fs::read_to_string(expected_file)?);

    Ok(())
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello")
        .assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello","there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1n() -> TestResult {
    run(&["Hello there","-n"],"tests/expected/hello1.n.txt")
}

#[test]
fn hello2n() -> TestResult {
    run(&["-n","Hello","there"],"tests/expected/hello2.n.txt")
}
