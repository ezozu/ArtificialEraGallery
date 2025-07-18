// src/main.rs
/*
 * Main executable for ArtificialEraGallery
 */

use clap::Parser;
use artificialeragallery::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialEraGallery - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
