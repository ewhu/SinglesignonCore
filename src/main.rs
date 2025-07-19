// src/main.rs
/*
 * Main executable for SinglesignonCore
 */

use clap::Parser;
use singlesignoncore::{Result, run};

#[derive(Parser)]
#[command(version, about = SinglesignonCore - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
