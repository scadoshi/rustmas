use crate::domain::calendar::OutOfRange;
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

/// A validated event year, from [`FIRST_YEAR`] through [`latest_year`].
#[derive(Debug)]
pub struct Year(i32);

impl Year {
    /// Returns the inner year value.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// The upper bound is the latest *published* event, not the current
    /// calendar year, so outside December those differ.
    pub fn new(year: i32) -> Result<Self, OutOfRange> {
        if year > latest_year() || year < FIRST_YEAR {
            return Err(OutOfRange);
        }
        Ok(Self(year))
    }
}
