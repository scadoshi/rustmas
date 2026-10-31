//! Dive: follow the course and multiply horizontal position by depth, reading up and down first as depth changes and then as aim.

mod direction;
mod instruction;
mod position;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2021::day_02::instruction::Instruction;
use crate::domain::solution::year_2021::day_02::position::Position;
use crate::domain::solution::year_2021::day_02::position::aimed::Aimed;

/// The planned course.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One per line, followed in order from the surface.
    instructions: Vec<Instruction>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            instructions: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let value = self
            .instructions
            .iter()
            .try_fold(Position::default(), |position, &instruction| {
                position.checked_moved(instruction)
            })
            .and_then(|position| position.value())
            .context("the position passes the range of an i64")?;
        Ok(Answer::solved(value.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let value = self
            .instructions
            .iter()
            .try_fold(Aimed::default(), |aimed, &instruction| {
                aimed.checked_moved(instruction)
            })
            .and_then(|aimed| aimed.position.value())
            .context("the aimed position passes the range of an i64")?;
        Ok(Answer::solved(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's example for both parts.
    #[test]
    fn example() {
        let puzzle = Puzzle::new("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("150"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("900"));
    }

    /// Depth past [`i64`] is an error rather than a clamped answer.
    #[test]
    fn a_dive_past_i64_is_an_error() {
        let puzzle =
            Puzzle::new("down 2147483647\nforward 2147483647\nforward 2147483647").unwrap();
        assert!(puzzle.part_one().is_ok());
        assert!(puzzle.part_two().is_err());
    }
}
