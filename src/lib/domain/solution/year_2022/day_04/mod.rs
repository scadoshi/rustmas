//! Camp Cleanup: count elf pairs where one assignment contains the other, then pairs that overlap at all.

mod assignment;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2022::day_04::assignment::Pair;

/// The cleanup assignments.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One pair of elves per line.
    pairs: Vec<Pair>,
}

impl Puzzle {
    /// How many pairs pass `rule`, as the answer text.
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
