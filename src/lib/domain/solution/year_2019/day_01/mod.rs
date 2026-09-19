use crate::domain::solution::{Solution, answer::Answer, common::parse};

pub struct Puzzle {
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
        Ok(Answer::solved(
            self.masses
                .iter()
                .map(|&mass| fuel_for(mass))
                .sum::<u32>()
                .to_string(),
        ))
    }

    /// Fuel has mass too, so each load needs fuel of its own until a load
    /// rounds to nothing.
    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.masses
                .iter()
                .flat_map(|&mass| {
                    std::iter::successors(Some(fuel_for(mass)), |&fuel| Some(fuel_for(fuel)))
                        .take_while(|&fuel| fuel > 0)
                })
                .sum::<u32>()
                .to_string(),
        ))
    }
}
