use crate::solutions::{answer::Answer, solution::Solution};

pub struct Puzzle {
    input: &'static str,
}

impl Solution for Puzzle {
    fn new(input: &'static str) -> anyhow::Result<Self> {
        Ok(Self { input })
    }

    fn input(&self) -> &str {
        self.input
    }

    fn part_one(&self) -> Answer {
        Answer::None
    }

    fn part_two(&self) -> Answer {
        Answer::None
    }
}
