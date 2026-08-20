use crate::domain::address::{Day, DayOutOfRange, Year, YearOutOfRange, day::FINAL_DAY};
use thiserror::Error;

/// The ways a filter can name something outside the published range.
#[derive(Debug, Error)]
pub enum InvalidFilter {
    #[error(transparent)]
    YearOutOfRange(#[from] YearOutOfRange),
    #[error(transparent)]
    DayOutOfRange(#[from] DayOutOfRange),
}

/// A validated narrowing of the published days. `None` means all of them.
///
/// Validation happens here, eagerly, so expanding a filter cannot fail and an
/// impossible one errors up front rather than sweeping the range and matching
/// nothing in silence.
#[derive(Debug, Clone, Copy)]
pub struct Filter {
    year: Option<Year>,
    /// An `i32` rather than a [`Day`], since a day filter with no year is not
    /// an address: day 13 is valid for 2015 and not for 2025.
    day: Option<i32>,
}

impl Filter {
    /// Validates whatever was given: each side alone, and the pair strictly.
    pub fn new(year: Option<i32>, day: Option<i32>) -> Result<Filter, InvalidFilter> {
        let (year, day) = match (year, day) {
            (Some(y), Some(d)) => {
                let year = Year::new(y)?;
                let _day = Day::new(d, year)?;
                (Some(year), Some(d))
            }
            (Some(y), None) => (Some(Year::new(y)?), None),
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

    pub fn year(&self) -> Option<Year> {
        self.year
    }

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
}
