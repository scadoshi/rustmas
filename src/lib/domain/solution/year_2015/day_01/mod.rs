use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
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
        let floor = self
            .input
            .chars()
            .try_fold(0_i64, |floor, c| floor.checked_add(step(c)))
            .context("the floor passes the i64 range")?;
        Ok(Answer::solved(floor.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut floor = 0_i64;
        for (position, c) in (1_usize..).zip(self.input.chars()) {
            floor = floor
                .checked_add(step(c))
                .context("the floor passes the i64 range")?;
            if floor == -1 {
                return Ok(Answer::solved(position.to_string()));
            }
        }
        Ok(Answer::None)
    }
}

/// How far `c` moves Santa: up one for `(`, down one for `)`, else nowhere.
fn step(c: char) -> i64 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
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
}
