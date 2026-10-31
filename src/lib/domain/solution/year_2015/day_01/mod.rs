//! Day 1, Not Quite Lisp: Santa climbs and descends floors by parentheses. Each character becomes a [`Step`] worth plus or minus one floor, summed for part one and walked until the basement for part two.

use anyhow::Context;
use thiserror::Error;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

/// A character in the directions that is neither parenthesis. Carries the character.
#[derive(Debug, Error)]
#[error("expected `(` or `)`, read {0:?}")]
struct InvalidStep(char);

/// One character of Santa's directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    /// `(`, up one floor.
    Up,
    /// `)`, down one floor.
    Down,
}

impl TryFrom<char> for Step {
    type Error = InvalidStep;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Self::Up),
            ')' => Ok(Self::Down),
            other => Err(InvalidStep(other)),
        }
    }
}

impl Step {
    /// How many floors the step moves Santa.
    fn delta(self) -> i64 {
        match self {
            Self::Up => 1,
            Self::Down => -1,
        }
    }
}

/// Santa's directions, parsed.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One per character, in the order Santa follows them.
    steps: Vec<Step>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let steps = input
            .as_ref()
            .trim()
            .chars()
            .map(Step::try_from)
            .collect::<Result<_, _>>()?;
        Ok(Self { steps })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let floor = self
            .steps
            .iter()
            .try_fold(0_i64, |floor, step| floor.checked_add(step.delta()))
            .context("the floor passes the i64 range")?;
        Ok(Answer::solved(floor.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut floor = 0_i64;
        for (position, step) in (1_usize..).zip(&self.steps) {
            floor = floor
                .checked_add(step.delta())
                .context("the floor passes the i64 range")?;
            if floor == -1 {
                return Ok(Answer::solved(position.to_string()));
            }
        }
        Ok(Answer::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn answers(input: &str) -> (Answer, Answer) {
        let puzzle = Puzzle::new(input).unwrap();
        (puzzle.part_one().unwrap(), puzzle.part_two().unwrap())
    }

    /// The puzzle's examples for both parts.
    #[test]
    fn examples() {
        assert_eq!(answers("(())").0, Answer::solved("0"));
        assert_eq!(answers("))(((((").0, Answer::solved("3"));
        assert_eq!(answers(")())())").0, Answer::solved("-3"));
        assert_eq!(answers(")").1, Answer::solved("1"));
        assert_eq!(answers("()())").1, Answer::solved("5"));
        assert_eq!(answers("((").1, Answer::None);
    }

    #[test]
    fn anything_but_parentheses_is_rejected() {
        assert!(Puzzle::new("(()x)").is_err());
        assert!(Puzzle::new("(( ))").is_err());
    }
}
