pub mod password;

use crate::domain::solution::{Solution, answer::Answer, year_2020::day_02::password::Password};

pub struct Puzzle {
    passwords: Vec<Password>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            passwords: input
                .as_ref()
                .lines()
                .map(Password::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Value(
            self.passwords
                .iter()
                .filter(|p| p.valid_count())
                .count()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Value(
            self.passwords
                .iter()
                .filter(|p| p.valid_position())
                .count()
                .to_string(),
        ))
    }
}
