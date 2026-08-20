pub mod args;
pub mod utils;

use crate::{
    domain::address::{Day, Filter},
    inbound::{fetch::args::FetchArgs, input::ensure_entry},
};

/// Downloads puzzle inputs and instructions into `cache/<year>/<NN>/`.
///
/// Filters omitted means all. Files on disk are left alone, and one failed
/// download aborts the rest.
pub fn run(args: &FetchArgs) -> anyhow::Result<()> {
    // Built on first download, so a fully cached run needs no cookie.
    let mut client = None;

    for day in Day::matching(Filter::new(args.year, args.day)?) {
        ensure_entry(&mut client, &day)?;
    }
    Ok(())
}
