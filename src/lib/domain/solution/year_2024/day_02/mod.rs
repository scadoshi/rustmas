pub mod report;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2024::day_02::report::Report;

#[derive(Debug)]
pub struct Puzzle {
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
