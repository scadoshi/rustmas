use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    input: String,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input.as_ref().to_owned(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let nums: Vec<u32> = self.input.chars().filter_map(|c| c.to_digit(10)).collect();
        let mut total = 0;
        let mut p = 0;
        while p < nums.len() {
            let next = (p + 1) % nums.len();
            if nums[p] == nums[next] {
                total += nums[p];
            }
            p += 1;
        }
        Ok(Answer::Value(total.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let nums: Vec<u32> = self.input.chars().filter_map(|c| c.to_digit(10)).collect();
        let mut total = 0;
        let mut p = 0;
        while p < nums.len() {
            let next = (p + nums.len() / 2) % nums.len();
            if nums[p] == nums[next] {
                total += nums[p];
            }
            p += 1;
        }
        Ok(Answer::Value(total.to_string()))
    }
}
