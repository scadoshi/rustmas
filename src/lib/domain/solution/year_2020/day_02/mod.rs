//! Password Philosophy: count passwords that satisfy their line's policy, read first as a letter count range and then as two positions.

mod password;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2020::day_02::password::Password;

/// The password database.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One per line, each with the policy written beside it.
    passwords: Vec<Password>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            passwords: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.passwords
                .iter()
                .filter(|p| p.valid_count())
                .count()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.passwords
                .iter()
                .filter(|p| p.valid_position())
                .count()
                .to_string(),
        ))
    }
}
