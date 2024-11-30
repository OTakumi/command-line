use clap::Parser;

/// echor - Rust echo
/// version: 0.1.0
#[derive(Parser, Debug)]
#[command(version, about)]
#[group(required = true)]
struct Cli {
    /// Input text
    text: Vec<String>,

    /// Do not output the trailing newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    // let cli = Cli::parse();
    let cli = Cli::parse();

    // Check if omit_newline is true
    // If true, set ending to empty string
    // textの1つ目が-nだった場合、omit_newlineをtrueにする
    print!(
        "{}{}",
        cli.text.join(" "),
        if cli.omit_newline { "" } else { "\n" }
    );
}
