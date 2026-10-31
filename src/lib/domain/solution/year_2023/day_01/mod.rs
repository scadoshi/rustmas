//! Day 1: Trebuchet?! Each line's calibration value is its first and last digit as a two-digit number. Part two also reads spelled-out digits, scanning from every position so overlapping words like `eightwo` both count.

mod calibration;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::year_2023::day_01::calibration::Calibration;

/// The calibration document, summed once per part with a different reading of each line.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One line per calibration document entry, trimmed of the trailing blank.
    lines: Vec<String>,
}

impl Puzzle {
    /// Sums `f` over every line, naming the first that yields no number.
    fn solve_with(&self, f: fn(&str) -> Option<u32>) -> anyhow::Result<Answer> {
        let total: u32 =
            self.lines
                .iter()
                .zip(1_usize..)
                .try_fold(0_u32, |total, (line, number)| {
                    let value = f(line)
                        .with_context(|| format!("line {number} has no numbers: {line:?}"))?;
                    total
                        .checked_add(value)
                        .context("the calibration values add up past u32")
                })?;
        Ok(Answer::solved(total.to_string()))
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            lines: input.as_ref().trim().lines().map(str::to_owned).collect(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        self.solve_with(Calibration::calibration_value)
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        self.solve_with(Calibration::calibration_value_with_words)
    }
}
