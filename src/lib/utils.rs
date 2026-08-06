use chrono::{Datelike, Utc};

pub const FIRST_YEAR: i32 = 2015;

/// Latest year with published puzzles. AOC drops a new event each December, so
/// before December the current calendar year has nothing to download yet.
pub fn latest_year() -> i32 {
    let now = Utc::now();
    let year = now.year();
    if now.month() == 12 { year } else { year - 1 }
}
