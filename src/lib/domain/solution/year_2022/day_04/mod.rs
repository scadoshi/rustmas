pub mod assignment;

use crate::domain::solution::{
    Solution, answer::Answer, common::parse, year_2022::day_04::assignment::Pair,
};

pub struct Puzzle {
    pairs: Vec<Pair>,
}

impl Puzzle {
    fn count(&self, rule: impl Fn(Pair) -> bool) -> String {
        self.pairs
            .iter()
            .filter(|pair| rule(**pair))
            .count()
            .to_string()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            pairs: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(Pair::one_contains_the_other)))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(Pair::overlap)))
    }
}
