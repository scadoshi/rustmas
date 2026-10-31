//! Day 2: Red-Nosed Reports. A report is safe when its levels step by 1 to 3, all up or all down. Part two also accepts a report that turns safe with one level removed, trying each removal in turn.

mod report;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2024::day_02::report::Report;

/// The reactor's unusual data, counted for safe reports.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One report per input line.
    reports: Vec<Report>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            reports: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.reports
                .iter()
                .filter(|r| r.is_safe())
                .count()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.reports
                .iter()
                .filter(|r| r.is_safe_dampened())
                .count()
                .to_string(),
        ))
    }
}
