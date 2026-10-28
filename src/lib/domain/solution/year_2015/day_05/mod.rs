pub mod nice;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub struct Puzzle {
    lines: Vec<String>,
}

impl Puzzle {
    /// How many lines `rule` calls nice.
    fn count(&self, rule: impl Fn(&[u8]) -> bool) -> String {
        self.lines
            .iter()
            .filter(|line| rule(line.as_bytes()))
            .count()
            .to_string()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            lines: input.as_ref().lines().map(str::to_owned).collect(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(nice::is_nice)))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(nice::is_nicer)))
    }
}
