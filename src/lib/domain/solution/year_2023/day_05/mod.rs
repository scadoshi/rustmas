mod almanac;
mod range;
mod seeds;

use std::str::FromStr;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::year_2023::day_05::almanac::Almanac;
#[derive(Debug)]
pub(crate) struct Puzzle {
    almanac: Almanac,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            almanac: Almanac::from_str(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.almanac
                .seeds()
                .iter()
                .map(|s| self.almanac.maps().map(*s))
                .min()
                .ok_or_else(|| anyhow!("expected at least one seed"))?
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.almanac
                .seeds()
                .to_ranges()?
                .into_iter()
                .flat_map(|r| {
                    self.almanac
                        .maps()
                        .map_range(r)
                        .into_iter()
                        .map(|r| r.start)
                })
                .min()
                .ok_or_else(|| anyhow!("expected a seed range that is not empty"))?
                .to_string(),
        ))
    }
}
