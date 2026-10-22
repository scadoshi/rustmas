pub mod password;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2020::day_02::password::Password;

pub struct Puzzle {
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
