pub mod utils;

use chrono::{Datelike, Utc};
use rustmas::{
    part::{Day, days_in_year},
    session::Session,
};
use std::path::PathBuf;

use crate::utils::{download_input, ensure_dir};

const INPUT_PATH: &str = "inputs";
const FIRST_YEAR: u32 = 2015;

/// Latest year with published puzzles. AOC drops a new event each December, so
/// before December the current calendar year has nothing to download yet.
fn latest_year() -> u32 {
    let now = Utc::now();
    let year = u32::try_from(now.year()).unwrap_or(u32::MAX);
    if now.month() == 12 { year } else { year - 1 }
}

/// Downloads every published puzzle input into `inputs/<year>/<NN>.txt`.
///
/// Walks each year from [`FIRST_YEAR`] through [`latest_year`] and every day in
/// it, fetching any input not already on disk. Existing files are treated as
/// cached and left untouched, so the command is safe to re-run and only fills
/// gaps. Returns an error if the project dir, session, or any single download
/// fails (a failure aborts the remaining downloads).
fn init() -> anyhow::Result<()> {
    let project_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let input_path = project_path.join(INPUT_PATH);
    ensure_dir("inputs", &input_path)?;

    let session = Session::from_env()?;
    for year in FIRST_YEAR..=latest_year() {
        let year_path = input_path.join(year.to_string());
        ensure_dir(&format!("year {year}"), &year_path)?;
        for day in 1..=days_in_year(year) {
            let day = Day::new(day, year)?;
            let day_path = year_path.join(format!("{:02}.txt", day.value()));
            download_input(&session, &day, &day_path)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = init() {
        eprintln!("Error: {e:?}");
    }
}
