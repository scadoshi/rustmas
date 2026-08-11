use crate::domain::solution::{Solution, answer::Answer};
use std::collections::HashMap;

const TARGET: i32 = 2020;

fn two_sum(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let map = nums
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect::<HashMap<i32, usize>>();
    for (i, x) in nums.iter().enumerate() {
        let y = target - x;
        if map.get(&y).is_some_and(|j| i != *j) {
            return Some((*x, y));
        }
    }
    None
}

fn n_sum(nums: &[i32], target: i32, n: usize) -> Vec<i32> {
    for k in nums.iter().enumerate() {
        todo!();
    }
    todo!();
}

pub struct Puzzle {
    nums: Vec<i32>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let nums: Vec<i32> = input
            .as_ref()
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Self { nums })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        match two_sum(&self.nums, TARGET) {
            Some((x, y)) => Ok(Answer::Value((x * y).to_string())),
            None => Ok(Answer::None),
        }
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Unwritten)
    }
}
