use clap::{Parser, Subcommand};
use std::io::Result;

mod amm;
mod keys;
mod dlmm;
mod setup;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    SwapAMM,
    SwapDLMM,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::SwapAMM => amm::swap(),
        Command::SwapDLMM => dlmm::swap(),
    }

    Ok(())
}
