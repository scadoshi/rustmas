//! The `--year` and `--day` narrowing, validated before any day is walked.

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::domain::address::day::{DayOutOfRange, FINAL_DAY};
use crate::domain::address::year::YearOutOfRange;
use crate::domain::address::{Day, Year};

/// The ways a filter can name something outside the published range.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidFilter {
    /// `--year` names no published event.
    #[error(transparent)]
    YearOutOfRange(#[from] YearOutOfRange),
    /// `--day` is outside 1 through 25 alone, or outside what the given year
    /// had or has unlocked.
    #[error(transparent)]
    DayOutOfRange(#[from] DayOutOfRange),
}

/// A validated narrowing of the published days. `None` means all of them.
///
/// Validated eagerly, so expanding cannot fail and an impossible filter errors
/// up front rather than matching nothing in silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Filter {
    year: Option<Year>,
    /// An `i32` rather than a [`Day`], since day 13 with no year is not an
    /// address: valid for 2015, not for 2025.
    day: Option<i32>,
}

impl Filter {
    /// Validates whatever was given: each side alone, and the pair strictly.
    ///
    /// # Errors
    ///
    /// [`InvalidFilter::YearOutOfRange`] for a year outside the published
    /// events. [`InvalidFilter::DayOutOfRange`] for a day outside 1 through
    /// 25, or, with a year, outside the days that year had or has unlocked.
    pub fn new(year: Option<i32>, day: Option<i32>) -> Result<Filter, InvalidFilter> {
        Self::new_at(year, day, Utc::now())
    }

    /// [`Filter::new`] as of `now`.
    fn new_at(
        year: Option<i32>,
        day: Option<i32>,
        now: DateTime<Utc>,
    ) -> Result<Filter, InvalidFilter> {
        let (year, day) = match (year, day) {
            (Some(y), Some(d)) => {
                let year = Year::new_at(y, now)?;
                let _day = Day::new_at(d, year, now)?;
                (Some(year), Some(d))
            }
            (Some(y), None) => (Some(Year::new_at(y, now)?), None),
            (None, Some(d)) => {
                if !(1..=FINAL_DAY).contains(&d) {
                    return Err(InvalidFilter::DayOutOfRange(DayOutOfRange {
                        given: d,
                        bound: FINAL_DAY,
                    }));
                }
                (None, Some(d))
            }
            (None, None) => (None, None),
        };
        Ok(Filter { year, day })
    }

    /// The one year to run, or `None` for every year.
    pub fn year(&self) -> Option<Year> {
        self.year
    }

    /// The one day to run in each year, or `None` for every day. Not a
    /// [`Day`], since it may name a day only some years had.
    pub fn day(&self) -> Option<i32> {
        self.day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_what_the_range_allows() {
        assert!(Filter::new(None, None).is_ok());
        assert!(Filter::new(Some(2015), None).is_ok());
        assert!(Filter::new(None, Some(13)).is_ok());
        assert!(Filter::new(Some(2015), Some(25)).is_ok());
    }

    /// The reason the type exists: an impossible filter errors here, eagerly,
    /// rather than sweeping the range and matching nothing in silence.
    #[test]
    fn rejects_either_side_out_of_range() {
        assert!(matches!(
            Filter::new(Some(2030), None),
            Err(InvalidFilter::YearOutOfRange(_))
        ));
        assert!(matches!(
            Filter::new(None, Some(26)),
            Err(InvalidFilter::DayOutOfRange(_))
        ));
    }

    /// Day 13 is fine alone and fine in 2015, but 2025 stopped at twelve, so
    /// only the pair can be judged strictly.
    #[test]
    fn judges_the_pair_strictly_when_both_are_given() {
        assert!(Filter::new(None, Some(13)).is_ok());
        assert!(Filter::new(Some(2015), Some(13)).is_ok());
        assert!(matches!(
            Filter::new(Some(2025), Some(13)),
            Err(InvalidFilter::DayOutOfRange(_))
        ));
    }

    /// The message carries the bound that rejected it, which for a paired
    /// filter is the year's own day count.
    #[test]
    fn the_error_names_the_real_bound() {
        let error = Filter::new(Some(2025), Some(13)).unwrap_err();
        assert!(error.to_string().contains("1..=12"));

        let error = Filter::new(Some(2030), None).unwrap_err();
        assert!(error.to_string().contains("2030"));
    }

    /// A day of the running event that has not unlocked yet is out of range,
    /// with the unlocked count as the bound.
    #[test]
    fn rejects_a_day_that_has_not_unlocked() {
        let now: DateTime<Utc> = "2026-12-03T12:00:00Z".parse().unwrap();
        assert!(Filter::new_at(Some(2026), Some(3), now).is_ok());
        let error = Filter::new_at(Some(2026), Some(4), now).unwrap_err();
        assert_eq!(error.to_string(), "day 4 is outside 1..=3");
    }
}
