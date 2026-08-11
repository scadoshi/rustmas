use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    input: String,
}

impl Solution for Puzzle {
    fn new(input: impl Into<String>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input.into(),
        })
    }

    fn input(&self) -> &str {
        &self.input
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Unwritten)
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Unwritten)
    }
}
