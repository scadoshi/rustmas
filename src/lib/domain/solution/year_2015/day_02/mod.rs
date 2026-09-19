pub mod dimensions;

use anyhow::Context;

use crate::domain::solution::{
    Solution, answer::Answer, year_2015::day_02::dimensions::Dimensions,
};

pub struct Puzzle {
    input: Vec<Dimensions>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input
                .as_ref()
                .trim()
                .lines()
                .map(Dimensions::try_from)
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| "Parsing dimensions")?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.input
                .iter()
                .map(|d| d.wrapping_paper_required())
                .sum::<u32>()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.input
                .iter()
                .map(|d| d.ribbon_required())
                .sum::<u32>()
                .to_string(),
        ))
    }
}
