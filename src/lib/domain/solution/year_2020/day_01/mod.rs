use crate::domain::solution::{Solution, answer::Answer};
use std::{cmp::Ordering, collections::HashMap};

const TARGET: i32 = 2020;

fn two_sum(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let map = nums
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect::<HashMap<i32, usize>>();
    for (i, &x) in nums.iter().enumerate() {
        let y = target - x;
        if map.get(&y).is_some_and(|j| i != *j) {
            return Some((x, y));
        }
    }
    None
}

fn three_sum(nums: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    let mut nums = nums.to_owned();
    nums.sort_unstable();
    for (i, &x) in nums.iter().enumerate() {
        let (mut lo, mut hi) = (i + 1, nums.len() - 1);
        while lo < hi {
            match (x + nums[lo] + nums[hi]).cmp(&target) {
                Ordering::Equal => return Some((x, nums[lo], nums[hi])),
                Ordering::Less => lo += 1,
                Ordering::Greater => hi -= 1,
            }
        }
    }
    None
}

pub struct Puzzle {
    nums: Vec<i32>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let nums: Vec<i32> = input
            .as_ref()
            .trim()
            .lines()
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Self { nums })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        match two_sum(&self.nums, TARGET) {
            Some((x, y)) => Ok(Answer::solved((x * y).to_string())),
            None => Ok(Answer::None),
        }
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        match three_sum(&self.nums, TARGET) {
            Some((x, y, z)) => Ok(Answer::solved((x * y * z).to_string())),
            None => Ok(Answer::None),
        }
    }
}
