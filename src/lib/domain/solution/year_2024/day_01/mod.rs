//! Day 1: Historian Hysteria. Both lists are sorted when parsed. Part one pairs them smallest with smallest and adds up the gaps; part two counts each number in the right list and scores the left list against those counts.

use std::collections::HashMap;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

/// The two lists of location ids, side by side in the input.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// The left column, sorted ascending.
    left: Vec<i32>,
    /// The right column, sorted ascending.
    right: Vec<i32>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
            .as_ref()
            .trim()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (left, right) = line.split_once("   ").ok_or_else(|| {
                    anyhow!("expected two numbers three spaces apart, read {line:?}")
                })?;
                Ok((left.parse::<i32>()?, right.parse::<i32>()?))
            })
            .collect::<Result<Vec<(i32, i32)>, anyhow::Error>>()?
            .into_iter()
            .unzip();
        left.sort_unstable();
        right.sort_unstable();
        Ok(Self { left, right })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let total = self
            .left
            .iter()
            .zip(&self.right)
            .try_fold(0_u32, |total, (left, right)| {
                total.checked_add(left.abs_diff(*right))
            })
            .ok_or_else(|| anyhow!("the distances add up past u32"))?;
        Ok(Answer::solved(total.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for n in &self.right {
            let count = counts.entry(*n).or_default();
            *count = count.checked_add(1).ok_or_else(|| {
                anyhow!("a number appears in the right list more than i32::MAX times")
            })?;
        }
        let total = self.left.iter().try_fold(0_i32, |total, n| {
            n.checked_mul(*counts.get(n).unwrap_or(&0))
                .and_then(|score| total.checked_add(score))
                .ok_or_else(|| anyhow!("the similarity score passes the i32 range"))
        })?;
        Ok(Answer::solved(total.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's example for both parts.
    #[test]
    fn example() {
        let puzzle = Puzzle::new("3   4\n4   3\n2   5\n1   3\n3   9\n3   3").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("11"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("31"));
    }

    #[test]
    fn distances_past_u32_are_an_error() {
        let puzzle = Puzzle::new("-2147483648   2147483647\n-2147483648   2147483647").unwrap();
        assert!(puzzle.part_one().is_err());
    }
}
