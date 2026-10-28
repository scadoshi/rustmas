mod wire;

use anyhow::{Context, anyhow};

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::year_2019::day_03::wire::Wire;

#[derive(Debug)]
pub(crate) struct Puzzle {
    first: Wire,
    second: Wire,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let mut wires = input.as_ref().lines().map(str::parse::<Wire>);
        let (Some(first), Some(second)) = (wires.next(), wires.next()) else {
            return Err(anyhow!("expected two wires, one per line"));
        };
        Ok(Self {
            first: first?,
            second: second?,
        })
    }

    /// The crossing nearest the origin, as the elf walks.
    fn part_one(&self) -> anyhow::Result<Answer> {
        match self
            .first
            .crossings(&self.second)
            .map(|(point, ..)| point.distance_from_origin())
            .min()
        {
            Some(distance) => Ok(Answer::solved(distance.to_string())),
            None => Ok(Answer::None),
        }
    }

    /// The crossing the two wires reach in the fewest combined steps.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut fewest: Option<usize> = None;
        for (_, first, second) in self.first.crossings(&self.second) {
            let steps = first
                .checked_add(second)
                .context("the combined steps pass usize")?;
            fewest = Some(fewest.map_or(steps, |fewest| fewest.min(steps)));
        }
        Ok(fewest.map_or(Answer::None, |steps| Answer::solved(steps.to_string())))
    }
}
