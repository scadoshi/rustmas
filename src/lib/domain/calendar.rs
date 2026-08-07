//! Which Advent of Code events exist.

use chrono::{Datelike, Utc};

/// The first Advent of Code.
pub const FIRST_YEAR: i32 = 2015;

/// The latest event that has actually been published.
///
/// A new event drops each December, so before then the current calendar year
/// has nothing in it yet.
pub fn latest_year() -> i32 {
    let now = Utc::now();
    let year = now.year();
    if now.month() == 12 { year } else { year - 1 }
}
