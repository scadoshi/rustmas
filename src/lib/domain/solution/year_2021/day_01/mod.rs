//! Sonar Sweep: count depth increases, singly and then over three-reading windows.
//!
//! Two windows of three share their middle two readings, so part two compares readings three apart instead of summing.

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;

/// The sonar sweep report.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// Sea floor depths, one per line, in the order measured.
    nums: Vec<i32>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let nums: Vec<i32> = parse::lines(input.as_ref())?;
        Ok(Self { nums })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.nums
                .windows(2)
                .filter(|w| matches!(w, [a, b] if a < b))
                .count()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.nums
                .windows(4)
                .filter(|w| matches!(w, [a, _, _, d] if a < d))
                .count()
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's example for both parts.
    #[test]
    fn example() {
        let puzzle = Puzzle::new("199\n200\n208\n210\n200\n207\n240\n269\n260\n263").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("7"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("5"));
    }
}
