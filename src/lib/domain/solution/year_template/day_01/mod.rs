//! A day whose parts are both [`Answer::Unwritten`].

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

/// The parsed input. Replace `input` with the day's own types.
#[derive(Debug)]
pub struct Puzzle {
    // Keep this only if the parts read the raw text; drop it if you parse.
    #[expect(
        dead_code,
        reason = "a template: the unwritten parts read nothing until a day fills them in"
    )]
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
