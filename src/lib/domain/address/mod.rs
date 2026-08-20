//! Which puzzle we mean: [`year::Year`], [`day::Day`], [`part::Part`].
//!
//! `Day` wraps `Year`, both constructor-only with private fields, so holding a
//! `Day` means it names a real puzzle without re-checking. The same address
//! becomes a URL path (`/2015/day/1`) and a cache path (`cache/2015/01/`).

pub mod day;
pub mod filter;
pub mod part;
pub mod year;

use thiserror::Error;

pub use day::Day;
pub use filter::{Filter, InvalidFilter};
pub use part::Part;
pub use year::{FIRST_YEAR, Year};

/// Returned when a year falls outside the published events.
#[derive(Debug, Error)]
#[error("year {given} is outside {FIRST_YEAR}..={latest}")]
pub struct YearOutOfRange {
    pub given: i32,
    pub latest: i32,
}
/// Returned when a day falls outside its bound, which is the year's own day
/// count when a year is known and 25 otherwise.
#[derive(Debug, Error)]
#[error("day {given} is outside 1..={bound}")]
pub struct DayOutOfRange {
    pub given: i32,
    pub bound: i32,
}
