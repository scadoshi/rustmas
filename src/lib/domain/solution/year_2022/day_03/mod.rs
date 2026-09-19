pub mod rucksack;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::parse,
    year_2022::day_03::rucksack::{Items, Rucksack},
};
use anyhow::anyhow;

/// How many elves carry one badge between them.
const GROUP: usize = 3;

pub struct Puzzle {
    rucksacks: Vec<Rucksack>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            rucksacks: parse::lines(input.as_ref())?,
        })
    }

    /// The one item type each elf packed in both compartments.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let total = self
            .rucksacks
            .iter()
            .map(|sack| sack.left.common(sack.right).single_priority())
            .sum::<Result<u32, _>>()?;
        Ok(Answer::solved(total.to_string()))
    }

    /// The one item type all three elves in a group carry: their badge.
    fn part_two(&self) -> anyhow::Result<Answer> {
        if !self.rucksacks.len().is_multiple_of(GROUP) {
            return Err(anyhow!(
                "expected the elves to divide into groups of {GROUP}"
            ));
        }
        let total = self
            .rucksacks
            .chunks_exact(GROUP)
            .map(|group| {
                group
                    .iter()
                    .map(|sack| sack.all())
                    .reduce(Items::common)
                    .ok_or(anyhow!("expected a group to have elves in it"))?
                    .single_priority()
                    .map_err(Into::into)
            })
            .sum::<anyhow::Result<u32>>()?;
        Ok(Answer::solved(total.to_string()))
    }
}
