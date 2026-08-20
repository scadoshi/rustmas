use crate::domain::address::{
    DayOutOfRange,
    filter::Filter,
    year::{FIRST_YEAR, Year},
};

/// Christmas Day, whose second star is awarded rather than puzzled.
pub(crate) const FINAL_DAY: i32 = 25;

/// A validated day within a validated [`Year`], which it always carries.
#[derive(Debug, Clone, Copy)]
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
    /// Day 25's is given for holding every other star, so its text stays hidden
    /// until the year is finished and there is nothing to fetch before then.
    pub fn has_second_puzzle(&self) -> bool {
        self.value != FINAL_DAY
    }

    /// Every published puzzle day, narrowed by the filters. `None` means all.
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

    /// Every published puzzle day, in year then day order.
    pub fn all() -> impl Iterator<Item = Self> {
        (FIRST_YEAR..=Year::latest()).flat_map(|y| {
            let year = Year::new(y).expect("range is FIRST_YEAR..=latest");
            (1..=year.days_in()).map(move |d| Day::new(d, year).expect("d is within days_in"))
        })
    }

    /// Validates `year`, then rejects any day outside [`Year::days_in`].
    pub fn new(day: i32, year: Year) -> Result<Self, DayOutOfRange> {
        let bound = year.days_in();
        if day < 1 || day > bound {
            return Err(DayOutOfRange { given: day, bound });
        }
        Ok(Self { value: day, year })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::address::Filter;

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
        let expected: i32 = (FIRST_YEAR..=Year::latest())
            .filter_map(|y| Year::new(y).ok())
            .map(|y| y.days_in())
            .sum();
        assert_eq!(Day::all().count(), expected as usize);
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

        assert_eq!(Day::matching(filter(Some(2015), Some(1))).count(), 1);
    }

    /// Day 25's second star is awarded for the other 49, so there is no text
    /// to fetch and nothing to keep asking for.
    #[test]
    fn only_day_25_lacks_a_second_puzzle() {
        let year = Year::new(2015).unwrap();
        assert!(!Day::new(25, year).unwrap().has_second_puzzle());
        for value in [1, 12, 24] {
            assert!(Day::new(value, year).unwrap().has_second_puzzle());
        }
    }

    /// A day-only filter skips years that never had that day rather than
    /// erroring, since 2025 stopped at twelve.
    #[test]
    fn matching_skips_years_without_that_day() {
        let days: Vec<_> = Day::matching(filter(None, Some(25))).collect();
        assert!(days.iter().all(|day| day.year() != 2025));
        assert_eq!(days.len(), (Year::latest() - FIRST_YEAR) as usize);
    }
}
