pub mod calibration;

use crate::domain::solution::{
    Solution, answer::Answer, year_2023::day_01::calibration::Calibration,
};
use anyhow::Context;

pub struct Puzzle {
    /// One line per calibration document entry, trimmed of the trailing blank.
    input: Vec<String>,
}

impl Puzzle {
    /// Sums `f` over every line, naming the first that yields no number.
    fn solve_with(&self, f: fn(&str) -> Option<u32>) -> anyhow::Result<Answer> {
        let total: u32 = self
            .input
            .iter()
            .enumerate()
            .map(|(i, s)| f(s).with_context(|| format!("line {} has no numbers: {s:?}", i + 1)))
            .sum::<anyhow::Result<u32>>()?;
        Ok(Answer::solved(total.to_string()))
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input
                .as_ref()
                .trim()
                .lines()
                .map(|s| s.to_owned())
                .collect(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        self.solve_with(Calibration::calibration_value)
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        self.solve_with(Calibration::calibration_value_with_words)
    }
}
