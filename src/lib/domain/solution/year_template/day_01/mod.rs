use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    // Keep this only if the parts read the raw text; drop it if you parse.
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
