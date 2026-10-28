use std::collections::HashMap;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub struct Puzzle {
    list_1: Vec<i32>,
    list_2: Vec<i32>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let (mut list_1, mut list_2): (Vec<i32>, Vec<i32>) = input
            .as_ref()
            .trim()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (s1, s2) = l
                    .split_once("   ")
                    .ok_or_else(|| anyhow!("invalid input line: {l:?}"))?;
                Ok((s1.parse::<i32>()?, s2.parse::<i32>()?))
            })
            .collect::<Result<Vec<(i32, i32)>, anyhow::Error>>()?
            .into_iter()
            .unzip();
        list_1.sort_unstable();
        list_2.sort_unstable();
        Ok(Self { list_1, list_2 })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.list_1
                .iter()
                .zip(&self.list_2)
                .map(|(n1, n2)| n1.abs_diff(*n2))
                .sum::<u32>()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for n in &self.list_2 {
            let count = counts.entry(*n).or_default();
            *count = count
                .checked_add(1)
                .ok_or_else(|| anyhow!("a number appears in list_2 more than i32::MAX times"))?;
        }
        let total = self.list_1.iter().try_fold(0_i32, |total, n| {
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
}
