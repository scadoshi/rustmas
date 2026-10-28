//! The argument parser, and the dispatch from subcommand to mode.

use clap::{Parser, Subcommand};

use crate::inbound::{fetch, solve};

/// What the user asked for.
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum Command {
    /// Download puzzle inputs
    Fetch(fetch::args::FetchArgs),
    /// Run solutions, optionally validating and submitting them
    Solve(solve::args::SolveArgs),
}

impl Command {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Fetch(args) => fetch::run(args),
            Self::Solve(args) => solve::run(args),
        }
    }
}

/// The command line. One binary, one subcommand per mode.
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(name = "rustmas")]
#[command(about = "Advent of Code tooling in Rust")]
#[command(version)]
pub struct Cli {
    /// The mode to run.
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    /// Runs whichever subcommand was given.
    ///
    /// # Errors
    ///
    /// A `--year` or `--day` outside the published days, a cache read or write
    /// that fails, a malformed `COOKIE`, a missing one when something has to
    /// be downloaded or submitted, a download or submission that fails, a
    /// `CONTACT` or `REPO_URL` that cannot go in a header, and a confirmation
    /// prompt that cannot read stdin. The first one ends the run. A day whose
    /// solution fails is printed instead and the run goes on.
    pub fn run(&self) -> anyhow::Result<()> {
        self.command.run()
    }
}
