//! Validated puzzle coordinates: [`year::Year`] -> [`day::Day`].
//!
//! [`day::Day`] wraps [`year::Year`], both constructor-only with private
//! fields. Hold a `Day` and you can trust it names a real puzzle without
//! re-checking.

pub mod day;
pub mod year;

use thiserror::Error;

pub use day::{Day, days_in_year};
pub use year::{FIRST_YEAR, Year, latest_year};

/// Returned when a year or day falls outside the bounds AOC supports.
#[derive(Debug, Error)]
#[error("out of range")]
pub struct OutOfRange;
