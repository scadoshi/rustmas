//! Validated puzzle coordinates: [`Year`] -> [`Day`].
//!
//! [`Day`] wraps [`Year`], both are constructor-only with private fields. Hold
//! a [`Day`] and you can trust it names a real puzzle without re-checking.

use thiserror::Error;

use crate::calendar::{FIRST_YEAR, latest_year};

/// Returned when a year or day falls outside the bounds AOC supports.
#[derive(Debug, Error)]
#[error("out of range")]
pub struct OutOfRange;

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

/// Days published for `year`. Usually 25; 2025 was a 12-day event.
pub fn days_in_year(year: i32) -> i32 {
    match year {
        2025 => 12,
        _ => 25,
    }
}

/// A validated day within a validated [`Year`], which it always carries.
#[derive(Debug)]
pub struct Day {
    value: i32,
    year: Year,
}

impl Day {
    /// Returns the inner day value.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Returns the value of the year this day belongs to.
    pub fn year(&self) -> i32 {
        self.year.value()
    }

    /// Validates `year`, then rejects any day outside [`days_in_year`].
    pub fn new(day: i32, year: i32) -> Result<Self, OutOfRange> {
        let year = Year::new(year)?;
        if !(1..=days_in_year(year.value())).contains(&day) {
            return Err(OutOfRange);
        }
        Ok(Self { value: day, year })
    }
}
