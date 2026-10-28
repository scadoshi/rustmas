pub mod hashing;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::year_2015::day_04::hashing::lowest_suffix;

/// [`lowest_suffix`], with running out of numbers as an error.
fn lowest(secret: &str, zeros: usize) -> anyhow::Result<usize> {
    lowest_suffix(secret, zeros)
        .ok_or_else(|| anyhow!("no suffix up to {} gives {zeros} zeros", usize::MAX))
}

#[derive(Debug)]
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
        Ok(Answer::solved(lowest(&self.secret, 5)?.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(lowest(&self.secret, 6)?.to_string()))
    }
}
