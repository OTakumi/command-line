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
