use clap::Parser;

#[derive(Parser)]
#[command(about = "Advent of Code inputs fetcher")]
#[command(version)]
pub struct Args {
    /// Year to fetch inputs for (omit for all)
    #[arg(short, long)]
    pub year: Option<i32>,
    /// Day to fetch inputs for (omit for all)
    #[arg(short, long)]
    pub day: Option<i32>,
}
