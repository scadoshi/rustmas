use crate::inbound::{fetch, solve};
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
