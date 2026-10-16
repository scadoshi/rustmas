pub mod input;
pub mod range;
pub mod seeds;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    year_2023::day_05::{input::Input, range::maps::RangeMaps, seeds::Seeds},
};
use anyhow::anyhow;
use std::str::FromStr;
pub struct Puzzle {
    input: Input,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: Input::from_str(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.input
                .seeds()
                .iter()
                .map(|s| self.input.maps().map(*s))
                .min()
                .ok_or(anyhow!("no min found"))?
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.input
                .seeds()
                .to_ranges()?
                .into_iter()
                .flat_map(|r| self.input.maps().map_range(r).into_iter().map(|r| r.start))
                .min()
                .ok_or(anyhow!("no min found"))?
                .to_string(),
        ))
    }
}
