//! The `fetch` subcommand: fill the cache without solving anything.

pub mod args;

use crate::domain::address::{Day, Filter};
use crate::inbound::fetch::args::FetchArgs;
use crate::inbound::input;
use crate::outbound::client::aoc_client::LazyAocClient;

/// Downloads puzzle inputs and instructions into `cache/<year>/<NN>/`.
///
/// Filters omitted means all. Files on disk are left alone, and one failed
/// download aborts the rest.
pub(crate) fn run(args: &FetchArgs) -> anyhow::Result<()> {
    // Built on first download, so a fully cached run needs no cookie.
    let mut aoc = LazyAocClient::default();

    for day in Day::matching(Filter::new(args.year, args.day)?) {
        input::ensure_entry(&mut aoc, &day)?;
    }
    Ok(())
}
