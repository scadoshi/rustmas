use clap::Parser;
use rustmas::inbound::cli::Cli;

fn main() {
    if let Err(e) = Cli::parse().run() {
        eprintln!("Error: {e:?}");
    }
}
