use std::collections::HashMap;

use crate::domain::solution::{Solution, answer::Answer};
use anyhow::anyhow;

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
                    .ok_or(anyhow!("invalid input line: {:?}", l))?;
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
        let counts: HashMap<i32, i32> = self.list_2.iter().fold(HashMap::new(), |mut acc, n| {
            *acc.entry(*n).or_default() += 1;
            acc
        });
        let total: i32 = self
            .list_1
            .iter()
            .map(|n| {
                n.checked_mul(*counts.get(n).unwrap_or(&0)).ok_or(anyhow!(
                    "list_1 num multiplied by its list_2 counts exceeded i32::MAX"
                ))
            })
            .sum::<anyhow::Result<i32>>()?;
        Ok(Answer::solved(total.to_string()))
    }
}
