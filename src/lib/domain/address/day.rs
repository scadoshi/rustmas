use crate::domain::address::{OutOfRange, year::FIRST_YEAR, year::Year};

/// A validated day within a validated [`Year`], which it always carries.
#[derive(Debug, Clone, Copy)]
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

    /// Every published puzzle day, narrowed by the filters.
    ///
    /// `None` means all of them, so `each(None, None)` walks every day of every
    /// event and `each(Some(2015), Some(1))` yields one.
    pub fn each(
        year: Option<i32>,
        day: Option<i32>,
    ) -> impl Iterator<Item = Result<Day, OutOfRange>> {
        (FIRST_YEAR..=Year::latest())
            // Every year in that range is valid by construction, so the filter only
            // drops what cannot happen.
            .filter_map(|y| Year::new(y).ok())
            .filter(move |y| year.is_none_or(|want| want == y.value()))
            .flat_map(move |y| {
                (1..=y.days_in())
                    .filter(move |d| day.is_none_or(|want| want == *d))
                    .map(move |d| Day::new(d, y))
            })
    }

    /// Validates `year`, then rejects any day outside [`Year::days_in`].
    pub fn new(day: i32, year: Year) -> Result<Self, OutOfRange> {
        if day < 1 || day > year.days_in() {
            return Err(OutOfRange);
        }
        Ok(Self { value: day, year })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn each_walks_every_published_day() {
        let all: Vec<_> = Day::each(None, None).map(Result::unwrap).collect();
        let expected: i32 = (FIRST_YEAR..=Year::latest())
            .filter_map(|y| Year::new(y).ok())
            .map(|y| y.days_in())
            .sum();
        assert_eq!(all.len(), expected as usize);
    }

    #[test]
    fn each_filters_are_independent() {
        let year: Vec<_> = Day::each(Some(2015), None).map(Result::unwrap).collect();
        assert_eq!(year.len(), 25);
        assert!(year.iter().all(|day| day.year() == 2015));

        let day: Vec<_> = Day::each(None, Some(1)).map(Result::unwrap).collect();
        assert_eq!(day.len(), (Year::latest() - FIRST_YEAR + 1) as usize);
        assert!(day.iter().all(|day| day.value() == 1));

        let both: Vec<_> = Day::each(Some(2015), Some(1)).map(Result::unwrap).collect();
        assert_eq!(both.len(), 1);
    }

    /// Day 25 exists in most years but not in 2025, so a day-only filter has to
    /// skip the years that never had it rather than erroring.
    #[test]
    fn each_skips_years_without_that_day() {
        let days: Vec<_> = Day::each(None, Some(25)).map(Result::unwrap).collect();
        assert!(days.iter().all(|day| day.year() != 2025));
        assert_eq!(days.len(), (Year::latest() - FIRST_YEAR) as usize);
    }
}
