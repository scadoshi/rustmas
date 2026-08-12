use std::collections::HashSet;

use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    nums: Vec<i32>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let nums = input
            .as_ref()
            .trim()
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Self { nums })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.nums.iter().sum::<i32>().to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut current = 0;
        let mut seen = HashSet::<i32>::from([current]);
        for num in self.nums.iter().cycle() {
            current += num;
            if !seen.insert(current) {
                return Ok(Answer::solved(current.to_string()));
            }
        }
        Ok(Answer::None)
    }
}
