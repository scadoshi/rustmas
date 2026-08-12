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
        Ok(Answer::solved(
            self.nums
                .windows(2)
                .filter(|w| w[0] < w[1])
                .count()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.nums
                .windows(4)
                .filter(|w| w[0] < w[3])
                .count()
                .to_string(),
        ))
    }
}
