use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Naveen Pantra").assert().success();
    Ok(())
}

#[test]
fn test_hello1() -> TestResult {
    run(&["Hello World"], "tests/expected/hello1.txt")
}

#[test]
fn test_hello2() -> TestResult {
    run(&["Hello", "World"], "tests/expected/hello2.txt")
}

#[test]
fn test_hello3() -> TestResult {
    run(&["Hello World", "-n"], "tests/expected/hello3.txt")
}

#[test]
fn test_hello4() -> TestResult {
    run(&["-n", "Hello", "World"], "tests/expected/hello4.txt")
}

fn run(args: &[&str], file_path: &str) -> TestResult {
    let expected_output = fs::read_to_string(file_path)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected_output);
    Ok(())
}
