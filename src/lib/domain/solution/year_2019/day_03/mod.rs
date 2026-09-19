pub mod wire;

use crate::domain::solution::{Solution, answer::Answer, year_2019::day_03::wire::Wire};
use anyhow::anyhow;

pub struct Puzzle {
    first: Wire,
    second: Wire,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let mut wires = input.as_ref().lines().map(Wire::try_from);
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
            .map(|(point, _)| point.distance_from_origin())
            .min()
        {
            Some(distance) => Ok(Answer::solved(distance.to_string())),
            None => Ok(Answer::None),
        }
    }

    /// The crossing the two wires reach in the fewest combined steps.
    fn part_two(&self) -> anyhow::Result<Answer> {
        match self
            .first
            .crossings(&self.second)
            .map(|(_, steps)| steps)
            .min()
        {
            Some(steps) => Ok(Answer::solved(steps.to_string())),
            None => Ok(Answer::None),
        }
    }
}
