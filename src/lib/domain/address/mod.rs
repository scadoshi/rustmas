//! Which puzzle we mean: [`year::Year`], [`day::Day`], [`part::Part`].
//!
//! The three together name one puzzle unambiguously, which is the job they
//! exist for. Year and day being calendar-shaped is incidental.
//!
//! `Day` wraps `Year`, both constructor-only with private fields, so holding a
//! `Day` means it names a real puzzle without re-checking. The same address
//! becomes a URL path (`/2015/day/1`) and a cache path (`cache/2015/01/`).

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_years_outside_published_events() {
        assert!(Year::new(FIRST_YEAR - 1).is_err());
        assert!(Year::new(latest_year() + 1).is_err());
        assert!(Year::new(FIRST_YEAR).is_ok());
        assert!(Year::new(latest_year()).is_ok());
    }

    /// 2025 ran twelve days, so day 13 is out of range for that year alone.
    #[test]
    fn rejects_days_the_year_never_had() {
        assert_eq!(days_in_year(2025), 12);
        assert_eq!(days_in_year(2015), 25);
        assert!(Day::new(13, 2025).is_err());
        assert!(Day::new(12, 2025).is_ok());
        assert!(Day::new(13, 2015).is_ok());
        assert!(Day::new(0, 2015).is_err());
        assert!(Day::new(26, 2015).is_err());
    }

    #[test]
    fn day_carries_its_year() {
        let day = Day::new(3, 2016).unwrap();
        assert_eq!(day.year(), 2016);
        assert_eq!(day.value(), 3);
    }

    #[test]
    fn each_walks_every_published_day() {
        let all: Vec<_> = each(None, None).map(Result::unwrap).collect();
        let expected: i32 = (FIRST_YEAR..=latest_year()).map(days_in_year).sum();
        assert_eq!(all.len(), expected as usize);
    }

    #[test]
    fn each_filters_are_independent() {
        let year: Vec<_> = each(Some(2015), None).map(Result::unwrap).collect();
        assert_eq!(year.len(), 25);
        assert!(year.iter().all(|day| day.year() == 2015));

        let day: Vec<_> = each(None, Some(1)).map(Result::unwrap).collect();
        assert_eq!(day.len(), (latest_year() - FIRST_YEAR + 1) as usize);
        assert!(day.iter().all(|day| day.value() == 1));

        let both: Vec<_> = each(Some(2015), Some(1)).map(Result::unwrap).collect();
        assert_eq!(both.len(), 1);
    }

    /// Day 25 exists in most years but not in 2025, so a day-only filter has to
    /// skip the years that never had it rather than erroring.
    #[test]
    fn each_skips_years_without_that_day() {
        let days: Vec<_> = each(None, Some(25)).map(Result::unwrap).collect();
        assert!(days.iter().all(|day| day.year() != 2025));
        assert_eq!(days.len(), (latest_year() - FIRST_YEAR) as usize);
    }
}
