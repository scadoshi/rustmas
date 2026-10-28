use std::iter;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;

#[derive(Debug)]
pub(crate) struct Puzzle {
    masses: Vec<u32>,
}

/// Fuel to lift `mass`: a third of it, less two, never negative.
fn fuel_for(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            masses: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let total = self
            .masses
            .iter()
            .try_fold(0_u32, |total, &mass| total.checked_add(fuel_for(mass)))
            .context("the fuel adds up past u32")?;
        Ok(Answer::solved(total.to_string()))
    }

    /// Fuel has mass too, so each load needs fuel of its own until a load
    /// rounds to nothing.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let total = self
            .masses
            .iter()
            .flat_map(|&mass| {
                iter::successors(Some(fuel_for(mass)), |&fuel| Some(fuel_for(fuel)))
                    .take_while(|&fuel| fuel > 0)
            })
            .try_fold(0_u32, u32::checked_add)
            .context("the fuel adds up past u32")?;
        Ok(Answer::solved(total.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_past_u32_is_an_error() {
        let max = u32::MAX.to_string();
        let puzzle = Puzzle::new([max.as_str(); 4].join("\n")).unwrap();
        assert!(puzzle.part_one().is_err());
        assert!(puzzle.part_two().is_err());
    }
}
