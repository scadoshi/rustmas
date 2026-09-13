//! Where a whole run's time went, gathered one day at a time.

use crate::domain::{
    address::{Day, Part},
    solution::Solved,
};
use std::{fmt::Display, time::Duration};

/// A run's time, totalled across every day it solved.
///
/// Parsing counts once per day and solving once per part, so the two means
/// have different denominators and the printed lines say which is which.
#[derive(Debug, Default)]
pub struct Totals {
    parsed: Duration,
    days: u32,
    solved: Duration,
    parts: u32,
    slowest: Option<Slowest>,
}

/// The part that took longest, and which one it was.
#[derive(Debug)]
struct Slowest {
    day: Day,
    part: Part,
    elapsed: Duration,
}

impl Totals {
    /// Folds in one day's run. Only parts that produced an answer count
    /// towards solving; the day counts towards parsing either way.
    pub fn add(&mut self, day: &Day, solved: &Solved) {
        self.parsed += solved.parsed_in;
        self.days += 1;

        for (part, outcome) in [(Part::One, &solved.part_one), (Part::Two, &solved.part_two)] {
            let Some(elapsed) = outcome.solve_time() else {
                continue;
            };
            self.solved += elapsed;
            self.parts += 1;
            if self.slowest.as_ref().is_none_or(|s| elapsed > s.elapsed) {
                self.slowest = Some(Slowest {
                    day: *day,
                    part,
                    elapsed,
                });
            }
        }
    }

    pub fn days(&self) -> u32 {
        self.days
    }

    /// `None` when no day ran, since there is nothing to divide by.
    pub fn parse_mean(&self) -> Option<Duration> {
        (self.days > 0).then(|| self.parsed / self.days)
    }

    /// `None` when no part produced an answer, which a run of stubs manages.
    pub fn solve_mean(&self) -> Option<Duration> {
        (self.parts > 0).then(|| self.solved / self.parts)
    }
}

impl Display for Totals {
    /// A block of whole lines, each one ended. Renders whatever it holds;
    /// when that is worth showing is the caller's to decide.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "total time spent parsing: {:?}", self.parsed)?;
        if let Some(mean) = self.parse_mean() {
            writeln!(f, "average parse time per day: {mean:?}")?;
        }
        writeln!(f, "total time spent solving: {:?}", self.solved)?;
        if let Some(mean) = self.solve_mean() {
            writeln!(f, "average solve time per part: {mean:?}")?;
        }
        if let Some(s) = &self.slowest {
            writeln!(
                f,
                "slowest part: year {} day {} part {} [{:?}]",
                s.day.year(),
                s.day.value(),
                s.part,
                s.elapsed
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        address::Year,
        solution::{answer::Answer, outcome::Outcome},
    };

    fn day(year: i32, value: i32) -> Day {
        Day::new(value, Year::new(year).expect("test year")).expect("test day")
    }

    fn micros(n: u64) -> Duration {
        Duration::from_micros(n)
    }

    /// Both parts answered, so both count.
    fn solved(parsed: Duration, one: Duration, two: Duration) -> Solved {
        Solved {
            parsed_in: parsed,
            part_one: Outcome::new(Ok(Answer::solved("a")), one),
            part_two: Outcome::new(Ok(Answer::solved("b")), two),
        }
    }

    #[test]
    fn nothing_run_has_no_means() {
        let totals = Totals::default();
        assert_eq!(totals.days(), 0);
        assert_eq!(totals.parse_mean(), None);
        assert_eq!(totals.solve_mean(), None);
    }

    /// The count a caller gates on. One day is not a summary of anything.
    #[test]
    fn one_day_is_one_day() {
        let mut totals = Totals::default();
        totals.add(&day(2015, 1), &solved(micros(2), micros(10), micros(30)));

        assert_eq!(totals.days(), 1);
        assert_eq!(totals.parse_mean(), Some(micros(2)));
        assert_eq!(totals.solve_mean(), Some(micros(20)));
    }

    #[test]
    fn totals_and_means_span_every_day() {
        let mut totals = Totals::default();
        totals.add(&day(2015, 1), &solved(micros(2), micros(10), micros(30)));
        totals.add(&day(2015, 2), &solved(micros(4), micros(50), micros(70)));

        assert_eq!(totals.parse_mean(), Some(micros(3)));
        assert_eq!(totals.solve_mean(), Some(micros(40)));
    }

    /// The whole reason for `solve_time`: a part with no answer did no work,
    /// so counting it would report a mean of a measurement of nothing.
    #[test]
    fn only_answered_parts_reach_the_solving_total() {
        let mut totals = Totals::default();
        totals.add(
            &day(2015, 25),
            &Solved {
                parsed_in: micros(2),
                part_one: Outcome::new(Ok(Answer::solved("a")), micros(10)),
                part_two: Outcome::new(Ok(Answer::None), micros(6)),
            },
        );

        assert_eq!(totals.solve_mean(), Some(micros(10)));
    }

    /// With nothing solved, the solving mean and slowest part lines are
    /// absent rather than zero.
    #[test]
    fn stub_days_count_towards_parsing_alone() {
        let mut totals = Totals::default();
        for year in [2016, 2017] {
            totals.add(
                &day(year, 2),
                &Solved {
                    parsed_in: micros(8),
                    part_one: Outcome::new(Ok(Answer::Unwritten), Duration::ZERO),
                    part_two: Outcome::new(Err(anyhow::anyhow!("broke")), micros(99)),
                },
            );
        }

        assert_eq!(totals.parse_mean(), Some(micros(8)));
        assert_eq!(totals.solve_mean(), None);
        assert_eq!(
            totals.to_string(),
            "total time spent parsing: 16µs\n\
             average parse time per day: 8µs\n\
             total time spent solving: 0ns\n"
        );
    }

    #[test]
    fn the_slowest_part_is_named() {
        let mut totals = Totals::default();
        totals.add(&day(2015, 1), &solved(micros(1), micros(10), micros(30)));
        totals.add(&day(2021, 4), &solved(micros(1), micros(90), micros(20)));
        totals.add(&day(2024, 7), &solved(micros(1), micros(15), micros(15)));

        assert!(
            totals
                .to_string()
                .ends_with("slowest part: year 2021 day 4 part one [90µs]\n")
        );
    }
}
