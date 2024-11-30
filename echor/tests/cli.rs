use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
/// This test checks that help is displayed when executed without any command arguments
fn no_args() -> TestResult {
    Command::cargo_bin("echo")?.assert().failure();

    Ok(())
}

/// This helper function is used to execute multiple test patterns
/// using the same test execution method
fn run(args: &[&str], expexted_file: &str) -> TestResult {
    let expected = fs::read_to_string(expexted_file)?;
    Command::cargo_bin("echo")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello_1() -> TestResult {
    let outfile = "tests/expected/hello_1.txt";
    run(&["Hello there"], outfile)
}

#[test]
fn hello_2() -> TestResult {
    let outfile = "tests/expected/hello_2.txt";
    run(&["Hello", "there"], outfile)
}

#[test]
fn hello1_no_newline() -> TestResult {
    let outfile = "tests/expected/hello_1_n.txt";
    run(&["Hello  there", "-n"], outfile)
}

#[test]
fn hello2_no_newline() -> TestResult {
    let outfile = "tests/expected/hello_2_n.txt";
    run(&["-n", "Hello", "there"], outfile)
}
