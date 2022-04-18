use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    /// The path to the GPX file
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = fs::read_to_string(args.path)?;
    println!("{}", file);
    Ok(())
}
