//! Day 3, No Matter How You Slice It: elves claim rectangles of one sheet of fabric. Counting how many claims cover each square inch answers both parts: the inches claimed more than once, and the one claim nothing overlaps.

mod claim;

use std::collections::HashMap;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::cell::Cell;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2018::day_03::claim::Claim;

/// The elves' claims.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One per non-blank line.
    claims: Vec<Claim>,
}

impl Puzzle {
    /// How many claims cover each square inch anyone claimed.
    fn coverage(&self) -> anyhow::Result<HashMap<Cell, usize>> {
        let mut coverage: HashMap<Cell, usize> = HashMap::new();
        for cell in self.claims.iter().flat_map(|claim| claim.cells()) {
            let count = coverage.entry(cell).or_default();
            *count = count
                .checked_add(1)
                .context("a square inch has more claims than usize counts")?;
        }
        Ok(coverage)
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            claims: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.coverage()?
                .values()
                .filter(|count| **count > 1)
                .count()
                .to_string(),
        ))
    }

    /// The one claim whose every square inch is claimed once, which is itself.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let coverage = self.coverage()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's example.
    #[test]
    fn example() {
        let puzzle = Puzzle::new("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("4"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("3"));
    }
}
