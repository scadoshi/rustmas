pub mod rating;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    year_2021::day_03::rating::{Keep, rate, rating_by_filter},
};
use anyhow::anyhow;

pub struct Puzzle {
    readings: Vec<u32>,
    /// How many bits each reading has, taken from the input rather than assumed.
    width: u32,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let lines: Vec<&str> = input.as_ref().lines().collect();
        let width = lines.first().map_or(0, |line| line.len());
        if lines.iter().any(|line| line.len() != width) {
            return Err(anyhow!("expected every line the same width"));
        }
        Ok(Self {
            readings: lines
                .iter()
                .map(|line| u32::from_str_radix(line, 2))
                .collect::<Result<_, _>>()?,
            width: u32::try_from(width)?,
        })
    }

    /// Gamma is the commonest bit everywhere, epsilon the rarest.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let gamma = rate(&self.readings, self.width, Keep::MostCommon);
        let epsilon = rate(&self.readings, self.width, Keep::LeastCommon);
        Ok(Answer::solved(
            (u64::from(gamma) * u64::from(epsilon)).to_string(),
        ))
    }

    /// Oxygen keeps the commonest bit at each step, CO2 the rarest.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let oxygen = rating_by_filter(&self.readings, self.width, Keep::MostCommon);
        let co2 = rating_by_filter(&self.readings, self.width, Keep::LeastCommon);
        match (oxygen, co2) {
            (Some(o), Some(c)) => Ok(Answer::solved((u64::from(o) * u64::from(c)).to_string())),
            _ => Ok(Answer::None),
        }
    }
}
