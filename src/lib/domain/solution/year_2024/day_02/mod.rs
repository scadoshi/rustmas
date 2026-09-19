pub mod report;

use crate::domain::solution::{
    Solution, answer::Answer, common::parse, year_2024::day_02::report::Report,
};

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
