use std::cmp::Reverse;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub struct Puzzle {
    groups: Vec<Vec<i32>>,
}

impl Puzzle {
    /// What each elf carries in all.
    fn totals(&self) -> anyhow::Result<Vec<i32>> {
        self.groups.iter().map(|group| checked_sum(group)).collect()
    }
}

/// The sum of `calories`, or an error past the i32 range.
fn checked_sum(calories: &[i32]) -> anyhow::Result<i32> {
    calories
        .iter()
        .try_fold(0_i32, |total, &n| total.checked_add(n))
        .context("the calories add up past the i32 range")
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let groups: Vec<Vec<i32>> = input
            .as_ref()
            .trim()
            .split("\n\n")
            .map(|group_str| {
                group_str
                    .trim()
                    .lines()
                    .filter(|s| !s.is_empty())
                    .map(str::parse)
                    .collect::<Result<Vec<i32>, _>>()
            })
            .collect::<Result<Vec<Vec<i32>>, _>>()?;
        Ok(Self { groups })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        match self.totals()?.into_iter().max() {
            Some(m) => Ok(Answer::solved(m.to_string())),
            None => Ok(Answer::None),
        }
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut totals = self.totals()?;
        totals.sort_unstable_by_key(|&n| Reverse(n));
        let top = totals
            .get(..3)
            .with_context(|| format!("need three groups to sum, input had {}", totals.len()))?;
        Ok(Answer::solved(checked_sum(top)?.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_elf_past_i32_is_an_error() {
        let puzzle = Puzzle::new("2147483647\n1\n\n5").unwrap();
        assert!(puzzle.part_one().is_err());
    }
}
