use crate::domain::solution::{Solution, answer::Answer};
use anyhow::Context;
use std::cmp::Reverse;

pub struct Puzzle {
    groups: Vec<Vec<i32>>,
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
                    .map(|s| s.parse())
                    .collect::<Result<Vec<i32>, _>>()
            })
            .collect::<Result<Vec<Vec<i32>>, _>>()?;
        Ok(Self { groups })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        match self.groups.iter().map(|g| g.iter().sum::<i32>()).max() {
            Some(m) => Ok(Answer::solved(m.to_string())),
            None => Ok(Answer::None),
        }
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut totals: Vec<i32> = self.groups.iter().map(|g| g.iter().sum::<i32>()).collect();
        totals.sort_unstable_by_key(|&n| Reverse(n));
        let top = totals
            .get(..3)
            .with_context(|| format!("need three groups to sum, input had {}", totals.len()))?;
        Ok(Answer::solved(top.iter().sum::<i32>().to_string()))
    }
}
