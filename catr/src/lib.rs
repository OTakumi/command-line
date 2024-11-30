use clap::Parser;
use std::error::Error;

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

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        println!("{}", filename);
    }

    Ok(())
}
