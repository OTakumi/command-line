use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/input/empty.txt";
const FOX: &str = "tests/input/fox.txt";
const SPIDERS: &str = "tests/input/spiders.txt";
const BUSTLE: &str = "tests/input/the-bustle.txt";

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
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/input/empty.txt")
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/input/fox.txt")
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], "tests/input/spiders.txt")
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/input/the-bustle.txt")
}
