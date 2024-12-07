use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// TODO: Add open file function
// TODO: Add tests

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Command line arguments
/// catr - Rust cat
/// version: 0.1.0
#[derive(Parser, Debug)]
#[clap(name = "catr", version = "0.1.0")]
#[group(multiple = true)]
struct Args {
    // Files to read
    #[arg(default_value = "-")]
    files: Vec<String>,

    /// Number all output lines
    #[arg(short = 'n', long = "number")]
    #[arg(conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Number nonempty output lines, overrides -n
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

/// Configuration
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();

    Ok(Config {
        files: args.files,
        number_lines: args.number_lines,
        number_nonblank_lines: args.number_nonblank_lines,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut line_num = 1;

    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                // Enumerate returns a tuple containing the index and value of an iterable element
                for line_result in file.lines() {
                    let line = line_result?;

                    // number_lines is true
                    if config.number_lines {
                        println!("{:6}\t{line}", line_num);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            println!("{:6}\t{line}", line_num);
                        } else {
                            println!();

                            // If a line is blank, the line number is decremented and the line number is not added.
                            line_num -= 1;
                        }
                    } else {
                        println!("{line}");
                    }
                    line_num += 1;
                }
            }
        }
    }

    Ok(())
}
