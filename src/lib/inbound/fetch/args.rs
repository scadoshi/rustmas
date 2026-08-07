use clap::Args;

#[derive(Args)]
pub struct FetchArgs {
    /// Year to fetch inputs for (omit for all)
    #[arg(short, long)]
    pub year: Option<i32>,
    /// Day to fetch inputs for (omit for all)
    #[arg(short, long)]
    pub day: Option<i32>,
}
