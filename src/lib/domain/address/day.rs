use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::domain::address::filter::Filter;
use crate::domain::address::year::Year;

/// The last day of an event through 2024, and the highest day any event has.
pub(crate) const FINAL_DAY: i32 = 25;

/// Returned when a day falls outside its bound: the days its year has
/// unlocked when a year is known, and 25 otherwise.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("day {given} is outside 1..={bound}")]
pub struct DayOutOfRange {
    pub given: i32,
    pub bound: i32,
}

/// A validated day within a validated [`Year`], which it always carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Day {
    value: i32,
    year: Year,
}

impl Day {
    pub fn value(&self) -> i32 {
        self.value
    }

    /// The year this day belongs to.
    pub fn year(&self) -> i32 {
        self.year.value()
    }

    /// Whether this day's second star is a puzzle rather than a reward.
    ///
    /// The event's final day, [`Year::days_in`], gives its second star for
    /// holding every other one, so nothing fetches it.
    pub fn has_second_puzzle(&self) -> bool {
        self.value != self.year.days_in()
    }

    /// Every unlocked puzzle day that `filter` allows.
    pub fn matching(filter: Filter) -> impl Iterator<Item = Day> {
        Day::all().filter(move |d| {
            filter
                .year()
                .is_none_or(|desired_year| desired_year.value() == d.year())
                && filter
                    .day()
                    .is_none_or(|desired_day| desired_day == d.value())
        })
    }

    /// Every unlocked puzzle day, in year then day order.
    pub fn all() -> impl Iterator<Item = Self> {
        Self::all_at(Utc::now())
    }

    /// [`Day::all`] as of `now`.
    pub(crate) fn all_at(now: DateTime<Utc>) -> impl Iterator<Item = Self> {
        Year::all_at(now).flat_map(move |year| {
            (1..=year.unlocked_days_at(now)).map(move |value| Day { value, year })
        })
    }

    /// Rejects any day of `year` outside `1..=year.days_in()` or not yet
    /// unlocked.
    pub fn new(day: i32, year: Year) -> Result<Self, DayOutOfRange> {
        Self::new_at(day, year, Utc::now())
    }

    /// [`Day::new`] as of `now`.
    pub(crate) fn new_at(day: i32, year: Year, now: DateTime<Utc>) -> Result<Self, DayOutOfRange> {
        let bound = year.unlocked_days_at(now);
        if day < 1 || day > bound {
            return Err(DayOutOfRange { given: day, bound });
        }
        Ok(Self { value: day, year })
    }
}

impl Display for Day {
    /// `year 2015 day 1`, as every message names a day.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "year {} day {}", self.year(), self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::address::Filter;
    use crate::domain::address::year::FIRST_YEAR;

    fn year(value: i32) -> Year {
        Year::new(value).expect("test years are published events")
    }

    /// 2025 ran twelve days, so day 13 is out of range for that year alone.
    #[test]
    fn rejects_days_the_year_never_had() {
        assert!(Day::new(13, year(2025)).is_err());
        assert!(Day::new(12, year(2025)).is_ok());
        assert!(Day::new(13, year(2015)).is_ok());
        assert!(Day::new(0, year(2015)).is_err());
        assert!(Day::new(26, year(2015)).is_err());
    }

    #[test]
    fn carries_its_year() {
        let day = Day::new(3, year(2016)).unwrap();
        assert_eq!(day.year(), 2016);
        assert_eq!(day.value(), 3);
    }

    #[test]
    fn all_walks_every_published_day() {
        let now: DateTime<Utc> = "2026-10-28T12:00:00Z".parse().unwrap();
        let expected: i32 = (FIRST_YEAR..=Year::latest_at(now))
            .filter_map(|y| Year::new_at(y, now).ok())
            .map(|y| y.days_in())
            .sum();
        assert_eq!(Day::all_at(now).count(), expected as usize);
    }

    fn filter(year: Option<i32>, day: Option<i32>) -> Filter {
        Filter::new(year, day).expect("test filters are in range")
    }

    #[test]
    fn matching_filters_are_independent() {
        let year: Vec<_> = Day::matching(filter(Some(2015), None)).collect();
        assert_eq!(year.len(), 25);
        assert!(year.iter().all(|day| day.year() == 2015));

        let day: Vec<_> = Day::matching(filter(None, Some(1))).collect();
        assert_eq!(day.len(), (Year::latest() - FIRST_YEAR + 1) as usize);
        assert!(day.iter().all(|day| day.value() == 1));

        let mut both = Day::matching(filter(Some(2015), Some(1)));
        let only = both.next().expect("one day matches a full pair");
        assert_eq!((only.year(), only.value()), (2015, 1));
        assert!(both.next().is_none());
    }

    /// The final day's second star is awarded for holding every other star,
    /// so there is no text to fetch and nothing to keep asking for. From 2025
    /// on that day is 12, not 25.
    #[test]
    fn only_the_final_day_lacks_a_second_puzzle() {
        let now: DateTime<Utc> = "2026-10-28T12:00:00Z".parse().unwrap();
        let day = |year, value| {
            let year = Year::new_at(year, now).unwrap();
            Day::new_at(value, year, now).unwrap()
        };
        assert!(!day(2025, 12).has_second_puzzle());
        assert!(!day(2024, 25).has_second_puzzle());
        assert!(day(2025, 11).has_second_puzzle());
        assert!(day(2024, 12).has_second_puzzle());
        assert!(day(2015, 24).has_second_puzzle());
    }

    /// A day-only filter skips years that never had that day rather than
    /// erroring, since 2025 stopped at twelve.
    #[test]
    fn matching_skips_years_without_that_day() {
        let days: Vec<_> = Day::matching(filter(None, Some(25))).collect();
        assert!(days.iter().all(|day| day.year() != 2025));
        assert_eq!(days.len(), (Year::latest() - FIRST_YEAR) as usize);
    }

    #[test]
    fn displays_as_year_then_day() {
        let day = Day::new(1, year(2015)).unwrap();
        assert_eq!(day.to_string(), "year 2015 day 1");
    }

    /// An unfiltered walk in December stops at the last unlocked day, since a
    /// locked one has no page or input to fetch.
    #[test]
    fn all_stops_at_the_last_unlocked_day() {
        let now: DateTime<Utc> = "2026-12-03T12:00:00Z".parse().unwrap();
        let last = Day::all_at(now).last().expect("past events are unlocked");
        assert_eq!((last.year(), last.value()), (2026, 3));
    }
}
