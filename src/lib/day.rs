//! Validated puzzle coordinates: [`Year`] -> [`Day`].
//!
//! The two types form a cascade. [`Day`] wraps [`Year`], and each can only be
//! built through a constructor that checks its own bounds, so a value can never
//! hold an out-of-range year, a day that doesn't exist for its year, or a day
//! without a year. Fields stay private: once you hold a [`Day`], downstream
//! code can trust it points at a real puzzle without re-validating.

use thiserror::Error;

use crate::utils::{FIRST_YEAR, latest_year};

/// Returned when a year or day falls outside the bounds AOC supports.
#[derive(Debug, Error)]
#[error("out of range")]
pub struct OutOfRange;

/// A validated AOC event year, from the first AOC through the latest event
/// that has actually been published (see [`latest_year`]).
#[derive(Debug)]
pub struct Year(i32);

impl Year {
    /// Returns the inner year value.
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Constructs a [`Year`], rejecting anything before [`FIRST_YEAR`] or after
    /// [`latest_year`]. Note the upper bound is the latest *published* event,
    /// not the current calendar year, so before December those differ.
    pub fn new(year: i32) -> Result<Self, OutOfRange> {
        if year > latest_year() || year < FIRST_YEAR {
            return Err(OutOfRange);
        }
        Ok(Self(year))
    }
}

/// Number of puzzle days published for `year`.
///
/// Most years run the full 1..=25. 2025 was a shorter, 12-day event.
pub fn days_in_year(year: i32) -> i32 {
    match year {
        2025 => 12,
        _ => 25,
    }
}

/// A validated day within a validated [`Year`].
///
/// A `Day` always carries its year, so it can never exist on its own or name a
/// day the year doesn't have.
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

    /// Constructs a [`Day`], first validating `year`, then rejecting any day
    /// outside `1..=` [`days_in_year`] for that year.
    pub fn new(day: i32, year: i32) -> Result<Self, OutOfRange> {
        let year = Year::new(year)?;
        if !(1..=days_in_year(year.value())).contains(&day) {
            return Err(OutOfRange);
        }
        Ok(Self { value: day, year })
    }
}
