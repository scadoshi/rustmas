pub mod claim;

use crate::domain::solution::{
    Solution, answer::Answer, common::cell::Cell, year_2018::day_03::claim::Claim,
};
use std::collections::HashMap;

pub struct Puzzle {
    claims: Vec<Claim>,
}

impl Puzzle {
    /// How many claims cover each square inch anyone claimed.
    fn coverage(&self) -> HashMap<Cell, usize> {
        let mut coverage = HashMap::new();
        for cell in self.claims.iter().flat_map(|claim| claim.cells()) {
            *coverage.entry(cell).or_default() += 1;
        }
        coverage
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            claims: input
                .as_ref()
                .lines()
                .map(Claim::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.coverage()
                .values()
                .filter(|count| **count > 1)
                .count()
                .to_string(),
        ))
    }

    /// The one claim whose every square inch is claimed once, which is itself.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let coverage = self.coverage();
        match self
            .claims
            .iter()
            .find(|claim| claim.cells().all(|cell| coverage.get(&cell) == Some(&1)))
        {
            Some(claim) => Ok(Answer::solved(claim.id.to_string())),
            None => Ok(Answer::None),
        }
    }
}
