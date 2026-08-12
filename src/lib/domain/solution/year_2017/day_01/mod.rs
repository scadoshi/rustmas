use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    digits: Vec<u32>,
}

impl Puzzle {
    fn matching(&self, step: usize) -> u32 {
        let n = self.digits.len();
        (0..n)
            .filter(|&i| self.digits[i] == self.digits[(i + step) % n])
            .map(|i| self.digits[i])
            .sum()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let digits = input
            .as_ref()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        Ok(Self { digits })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.matching(1).to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.matching(self.digits.len() / 2).to_string(),
        ))
    }
}
