//! Which puzzle we mean: [`year::Year`], [`day::Day`], [`part::Part`].
//!
//! The three together name one puzzle unambiguously, which is the job they
//! exist for. Year and day being calendar-shaped is incidental.
//!
//! `Day` wraps `Year`, both constructor-only with private fields, so holding a
//! `Day` means it names a real puzzle without re-checking. The same address
//! becomes a URL path (`/2015/day/1`) and a file path
//! (`inputs/2015/01.txt`).

pub mod day;
pub mod part;
pub mod year;

use thiserror::Error;

pub use day::{Day, days_in_year};
pub use part::Part;
pub use year::{FIRST_YEAR, Year, latest_year};

/// Returned when a year or day falls outside the bounds AOC supports.
#[derive(Debug, Error)]
#[error("out of range")]
pub struct OutOfRange;

/// Every published puzzle day, narrowed by the filters.
///
/// `None` means all of them, so `each(None, None)` walks every day of every
/// event and `each(Some(2015), Some(1))` yields one.
pub fn each(year: Option<i32>, day: Option<i32>) -> impl Iterator<Item = Result<Day, OutOfRange>> {
    (FIRST_YEAR..=latest_year())
        .filter(move |y| year.is_none_or(|want| want == *y))
        .flat_map(move |y| {
            (1..=days_in_year(y))
                .filter(move |d| day.is_none_or(|want| want == *d))
                .map(move |d| Day::new(d, y))
        })
}
