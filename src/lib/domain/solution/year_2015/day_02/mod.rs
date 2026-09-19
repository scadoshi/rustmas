pub mod dimensions;

use crate::domain::solution::{
    Solution, answer::Answer, common::parse, year_2015::day_02::dimensions::Dimensions,
};

pub struct Puzzle {
    input: Vec<Dimensions>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.input
                .iter()
                .map(dimensions::Dimensions::wrapping_paper_required)
                .sum::<u32>()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.input
                .iter()
                .map(dimensions::Dimensions::ribbon_required)
                .sum::<u32>()
                .to_string(),
        ))
    }
}
