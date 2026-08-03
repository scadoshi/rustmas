//! Validated puzzle coordinates: [`Year`] -> [`Day`] -> [`Part`].
//!
//! The three types form a cascade. Each wraps the one before it and can only be
//! built through a constructor that checks its own bounds, so a value can never
//! hold an out-of-range year, a day that doesn't exist for its year, or a day
//! without a year. Fields stay private: once you hold a [`Part`], downstream
//! code can trust it points at a real puzzle without re-validating.

use chrono::{Datelike, Utc};
use thiserror::Error;

/// Returned when a year, day, or part falls outside the bounds AOC supports.
#[derive(Debug, Error)]
#[error("out of range")]
pub struct OutOfRange;

/// A validated AOC event year (`2015..=` the current year).
#[derive(Debug)]
pub struct Year(u32);

impl Year {
    /// Returns the inner year value.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Constructs a [`Year`], rejecting anything before 2015 (the first AOC) or
    /// after the current calendar year.
    pub fn new(year: u32) -> Result<Self, OutOfRange> {
        let this_year = u32::try_from(Utc::now().year()).unwrap_or(u32::MAX);
        if year > this_year || year < 2015 {
            return Err(OutOfRange);
        }
        Ok(Self(year))
    }
}

/// Number of puzzle days published for `year`.
///
/// Most years run the full 1..=25. 2025 was a shorter, 12-day event.
pub fn days_in_year(year: u32) -> u32 {
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
    value: u32,
    year: Year,
}

impl Day {
    /// Returns the inner day value.
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Returns the value of the year this day belongs to.
    pub fn year(&self) -> u32 {
        self.year.value()
    }

    /// Constructs a [`Day`], first validating `year`, then rejecting any day
    /// outside `1..=` [`days_in_year`] for that year.
    pub fn new(day: u32, year: u32) -> Result<Self, OutOfRange> {
        let year = Year::new(year)?;
        if !(1..=days_in_year(year.value())).contains(&day) {
            return Err(OutOfRange);
        }
        Ok(Self { value: day, year })
    }
}

/// Which of a day's two puzzle parts.
#[derive(Debug, Clone, Copy)]
pub enum PartKind {
    One,
    Two,
}

/// A validated puzzle part: a [`Day`] plus which of its two parts.
///
/// This is the fully-specified coordinate downstream code works with, e.g. to
/// check an answer against the AOC solver.
#[derive(Debug)]
pub struct Part {
    day: Day,
    kind: PartKind,
}

impl Part {
    /// Returns the year value.
    pub fn year(&self) -> u32 {
        self.day.year.value()
    }

    /// Returns the day value.
    pub fn day(&self) -> u32 {
        self.day.value()
    }

    /// Returns which part this is.
    pub fn kind(&self) -> PartKind {
        self.kind
    }

    /// Constructs a [`Part`], validating `year` and `day` through [`Day::new`].
    pub fn new(year: u32, day: u32, kind: PartKind) -> Result<Self, OutOfRange> {
        let day = Day::new(day, year)?;
        Ok(Self { day, kind })
    }
}
