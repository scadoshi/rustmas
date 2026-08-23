use chrono::{Datelike, Utc};
use thiserror::Error;

/// The first Advent of Code.
pub const FIRST_YEAR: i32 = 2015;

/// Returned when a year falls outside the published events.
#[derive(Debug, Error)]
#[error("year {given} is outside {FIRST_YEAR}..={latest}")]
pub struct YearOutOfRange {
    pub given: i32,
    pub latest: i32,
}

/// A validated event year, from [`FIRST_YEAR`] through [`Year::latest`].
#[derive(Debug, Clone, Copy)]
pub struct Year(i32);

impl Year {
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Bounded by the latest *published* event, not the calendar year.
    pub fn new(year: i32) -> Result<Self, YearOutOfRange> {
        let latest = Self::latest();
        if year > latest || year < FIRST_YEAR {
            return Err(YearOutOfRange {
                given: year,
                latest,
            });
        }
        Ok(Self(year))
    }

    /// The latest event actually published. Before December that is last year.
    pub fn latest() -> i32 {
        let now = Utc::now();
        if now.month() == 12 {
            now.year()
        } else {
            now.year() - 1
        }
    }

    /// Days published for `year`. Usually 25; 2025 was a 12-day event.
    pub fn days_in(&self) -> i32 {
        match self.0 {
            2025 => 12,
            _ => 25,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_years_outside_published_events() {
        assert!(Year::new(FIRST_YEAR - 1).is_err());
        assert!(Year::new(Year::latest() + 1).is_err());
        assert!(Year::new(FIRST_YEAR).is_ok());
        assert!(Year::new(Year::latest()).is_ok());
    }

    /// The message names the value and the live bounds, since it is the only
    /// clue the user gets about what to type instead.
    #[test]
    fn the_error_names_the_value_and_the_bounds() {
        let error = Year::new(2030).unwrap_err();
        assert_eq!(
            error.to_string(),
            format!("year 2030 is outside 2015..={}", Year::latest())
        );
    }

    /// 2025 ran twelve days rather than the usual twenty five.
    #[test]
    fn knows_how_long_each_event_ran() {
        assert_eq!(Year::new(2025).unwrap().days_in(), 12);
        assert_eq!(Year::new(2015).unwrap().days_in(), 25);
    }
}
