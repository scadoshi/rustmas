use clap::Args;

#[derive(Args)]
pub struct SolveArgs {
    /// Year to run (omit for all)
    #[arg(short, long)]
    pub year: Option<i32>,
    /// Day to run (omit for all)
    #[arg(short, long)]
    pub day: Option<i32>,
    /// Check answers against the third-party solver (one request per part)
    #[arg(short, long)]
    pub validate: bool,
    /// Submit answers to the official Advent of Code website (one request per
    /// part). Validates first and only submits what the solver agrees with.
    #[arg(short, long)]
    pub submit: bool,
    /// Skip the confirmation prompt when submitting unfiltered.
    ///
    /// No short flag: this one is worth typing out.
    #[arg(long)]
    pub yes: bool,
}

impl SolveArgs {
    /// True when a submit run would post every solved day, not just this one.
    pub fn submitting_everything(&self) -> bool {
        self.submit && self.year.is_none() && self.day.is_none()
    }
}
