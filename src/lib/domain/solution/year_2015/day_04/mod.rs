pub mod hashing;

use crate::domain::solution::{
    Solution, answer::Answer, year_2015::day_04::hashing::lowest_suffix,
};

pub struct Puzzle {
    secret: String,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            secret: input.as_ref().trim().to_owned(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(lowest_suffix(&self.secret, 5).to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(lowest_suffix(&self.secret, 6).to_string()))
    }
}
