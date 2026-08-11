use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    // Keep the raw text only if the parts read it. A day that parses into its
    // own types should hold those instead and drop this field.
    #[allow(dead_code)]
    input: String,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input.as_ref().to_owned(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Unwritten)
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Unwritten)
    }
}
