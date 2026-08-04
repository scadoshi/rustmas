use clap::Parser;
use rustmas::solutions::Solution;

#[derive(Parser)]
#[command(about = "Advent of Code solution runner")]
#[command(version)]
pub struct Args {
    /// Year to run (omit for all)
    #[arg(short, long)]
    pub year: Option<u32>,
    /// Day to run (omit for all)
    #[arg(short, long)]
    pub day: Option<u32>,
}

pub fn solve<S: Solution>(input: &'static str) -> anyhow::Result<(Option<String>, Option<String>)> {
    let s = S::new(input)?;
    Ok((s.part_one(), s.part_two()))
}
