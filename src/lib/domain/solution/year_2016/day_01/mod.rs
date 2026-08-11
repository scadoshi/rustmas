use crate::domain::solution::{Solution, answer::Answer, year_2016::instruction::Instructions};

pub struct Puzzle {
    input: String,
    /// Parsed once here rather than per part, so a bad input fails the day
    /// before either part runs.
    instructions: Instructions,
}

impl Solution for Puzzle {
    fn new(input: impl Into<String>) -> anyhow::Result<Self> {
        let input = input.into();
        let instructions = Instructions::try_from(input.as_str())?;
        Ok(Self {
            input,
            instructions,
        })
    }

    fn input(&self) -> &str {
        &self.input
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let _ = &self.instructions;
        Ok(Answer::None)
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::None)
    }
}
