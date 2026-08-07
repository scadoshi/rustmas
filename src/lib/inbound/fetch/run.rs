use crate::{
    domain::calendar::{Day, FIRST_YEAR, days_in_year, latest_year},
    inbound::fetch::{
        args::FetchArgs,
        utils::{download_input, ensure_dir},
    },
    outbound::client::AocClient,
};
use std::path::PathBuf;

const INPUT_PATH: &str = "inputs";

/// Downloads published puzzle inputs into `inputs/<year>/<NN>.txt`.
///
/// `--year` and `--day` are filters, so omitting one means all of them. Existing
/// files count as cached, so re-running only fills gaps. One failed download
/// aborts the rest.
pub fn run(args: &FetchArgs) -> anyhow::Result<()> {
    let project_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let input_path = project_path.join(INPUT_PATH);
    ensure_dir("inputs", &input_path)?;

    let client = AocClient::from_env()?;
    for year in FIRST_YEAR..=latest_year() {
        if args.year.is_some_and(|y| y != year) {
            continue;
        }
        let year_path = input_path.join(year.to_string());
        for day in 1..=days_in_year(year) {
            if args.day.is_some_and(|d| d != day) {
                continue;
            }
            // Made here rather than per year, so a filter that matches nothing
            // in this year leaves no empty directory behind.
            ensure_dir(&format!("year {year}"), &year_path)?;
            let day = Day::new(day, year)?;
            let day_path = year_path.join(format!("{:02}.txt", day.value()));
            download_input(&client, &day, &day_path)?;
        }
    }
    Ok(())
}
