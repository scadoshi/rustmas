pub mod utils;

use crate::utils::{download_input, ensure_dir};
use rustmas::{
    day::{Day, days_in_year},
    session::Session,
    calendar::{FIRST_YEAR, latest_year},
};
use std::path::PathBuf;

const INPUT_PATH: &str = "inputs";

/// Downloads every published puzzle input into `inputs/<year>/<NN>.txt`.
///
/// Existing files count as cached, so re-running only fills gaps. One failed
/// download aborts the rest.
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
