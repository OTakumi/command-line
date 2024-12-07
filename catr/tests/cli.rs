use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

/// This function generates a random filename.
/// It will keep generating filenames until it finds one that doesn't exist.
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

/// This test checks for errors for non-existent files.
#[test]
fn skip_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);

    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

/// This helper function runs the program with the given arguments and checks the output.
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

/// This helper function
fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

// ---------- Bustle tests ----------
#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
}

#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/the-bustle.txt.n.stdin.out",
    )
}

#[test]
fn bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/expected/the-bustle.txt.b.stdin.out",
    )
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_n() -> TestResult {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

#[test]
fn bustle_b() -> TestResult {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

// ---------- Empty tests ----------
#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_stdin_n() -> TestResult {
    run_stdin(EMPTY, &["-n", "-"], "tests/expected/empty.txt.n.out")
}

#[test]
fn empty_stdin_b() -> TestResult {
    run_stdin(EMPTY, &["-b", "-"], "tests/expected/empty.txt.b.out")
}

// ---------- Fox tests ----------
#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_stdin_n() -> TestResult {
    run_stdin(FOX, &["-n", "-"], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox_stdin_b() -> TestResult {
    run_stdin(FOX, &["-b", "-"], "tests/expected/fox.txt.b.out")
}

// ---------- Spiders tests ----------
#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_n() -> TestResult {
    run(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
}

#[test]
fn spiders_stdin_b() -> TestResult {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

// ---------- Multiple file tests ----------
#[test]
fn all() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

#[test]
fn all_n() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

#[test]
fn all_b() -> TestResult {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
