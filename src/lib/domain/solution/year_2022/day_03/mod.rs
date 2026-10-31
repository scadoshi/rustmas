//! Rucksack Reorganization: sum the priorities of the item shared by each rucksack's halves, then of the badge shared by each group of three.

mod rucksack;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2022::day_03::rucksack::{Items, Rucksack};

/// How many elves carry one badge between them.
const GROUP: usize = 3;

/// Every elf's rucksack.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One per line, in input order, so consecutive runs of three make a group.
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
        let total = self.rucksacks.iter().try_fold(0_u32, |total, sack| {
            let priority = sack.left.common(sack.right).single_priority()?;
            total
                .checked_add(priority)
                .ok_or_else(|| anyhow!("the priorities add up past u32"))
        })?;
        Ok(Answer::solved(total.to_string()))
    }

    /// The one item type all three elves in a group carry: their badge.
    fn part_two(&self) -> anyhow::Result<Answer> {
        if !self.rucksacks.len().is_multiple_of(GROUP) {
            return Err(anyhow!(
                "expected the elves to divide into groups of {GROUP}"
            ));
        }
        let total =
            self.rucksacks
                .as_chunks::<GROUP>()
                .0
                .iter()
                .try_fold(0_u32, |total, group| {
                    let priority = group
                        .iter()
                        .map(|sack| sack.all())
                        .reduce(Items::common)
                        .ok_or_else(|| anyhow!("expected a group to have elves in it"))?
                        .single_priority()?;
                    total
                        .checked_add(priority)
                        .ok_or_else(|| anyhow!("the badge priorities add up past u32"))
                })?;
        Ok(Answer::solved(total.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn the_example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("157"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("70"));
    }
}
