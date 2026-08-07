use crate::{
    domain::address,
    inbound::{fetch::args::FetchArgs, input::ensure_input},
};

/// Downloads puzzle inputs into `inputs/<year>/<NN>.txt`.
///
/// `--year` and `--day` are filters, so omitting one means all of them. Files
/// already on disk are left alone. One failed download aborts the rest.
pub fn run(args: &FetchArgs) -> anyhow::Result<()> {
    // Built on first download, so a fully cached run needs no cookie.
    let mut client = None;

    for day in address::each(args.year, args.day) {
        ensure_input(&mut client, &day?)?;
    }
    Ok(())
}
