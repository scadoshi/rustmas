use crate::inbound::command::Command;
use clap::Parser;

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
