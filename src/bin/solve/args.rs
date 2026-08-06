use clap::Parser;

#[derive(Parser)]
#[command(about = "Advent of Code solution runner")]
#[command(version)]
pub struct Args {
    /// Year to run (omit for all)
    #[arg(short, long)]
    pub year: Option<i32>,
    /// Day to run (omit for all)
    #[arg(short, long)]
    pub day: Option<i32>,
    /// Check answers against the third-party solver (one request per part)
    #[arg(short, long)]
    pub validate: bool,
    /// Submit answers to the official Advent of Code website (one request per part)
    #[arg(short, long)]
    pub submit: bool,
}
