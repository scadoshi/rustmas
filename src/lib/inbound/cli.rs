use crate::inbound::{fetch, solve};
use clap::Parser;
use clap::Subcommand;

/// What the user asked for.
#[derive(Subcommand)]
pub enum Command {
    /// Download puzzle inputs
    Fetch(fetch::args::FetchArgs),
    /// Run solutions, optionally validating and submitting them
    Solve(solve::args::SolveArgs),
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Fetch(args) => fetch::run::run(args),
            Self::Solve(args) => solve::run::run(args),
        }
    }
}

/// The command line. One binary, one subcommand per mode.
#[derive(Parser)]
#[command(name = "rustmas")]
#[command(about = "Advent of Code tooling in Rust")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    /// Runs whichever subcommand was given.
    pub fn run(&self) -> anyhow::Result<()> {
        self.command.run()
    }
}
