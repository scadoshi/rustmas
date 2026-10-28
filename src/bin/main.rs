use clap::Parser;
use rustmas::inbound::cli::Cli;

/// Runs the command line. An error prints as `Error: {e:?}` and exits 1.
fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
