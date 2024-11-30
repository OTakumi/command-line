use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

/// Config struct
#[derive(Parser, Debug)]
#[clap(name = "catr", version = "0.1.0", author = "Takumiooo")]
#[group(multiple = false)]
struct Args {
    // Files to read
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'n', long = "number")]
    number_lines: bool,

    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

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
    dbg!(config);
    Ok(())
}
