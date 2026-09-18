use std::num::ParseIntError;

use crate::domain::solution::year_2021::day_02::direction::{Direction, InvalidDirection};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidInstruction {
    #[error("too few parts")]
    TooFewParts,
    #[error("too many parts")]
    TooManyParts,
    #[error(transparent)]
    Direction(#[from] InvalidDirection),
    #[error(transparent)]
    Distance(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub direction: Direction,
    pub distance: i32,
}

impl TryFrom<&str> for Instruction {
    type Error = InvalidInstruction;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let (Some(dir), Some(dist)) = (parts.next(), parts.next()) else {
            return Err(InvalidInstruction::TooFewParts);
        };
        if parts.next().is_some() {
            return Err(InvalidInstruction::TooManyParts);
        }
        let direction = Direction::try_from(dir)?;
        let distance: i32 = dist.parse()?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_try_from_str_ok() {
        let instruction = Instruction::try_from("forward 1").unwrap();
        assert!(matches!(instruction.direction, Direction::Forward));
        assert_eq!(instruction.distance, 1);
    }

    #[test]
    fn instruction_try_from_str_err() {
        assert!(matches!(
            Instruction::try_from("foo"),
            Err(InvalidInstruction::TooFewParts)
        ));
        assert!(matches!(
            Instruction::try_from("foo bar baz"),
            Err(InvalidInstruction::TooManyParts)
        ));
        assert!(matches!(
            Instruction::try_from("left 2"),
            Err(InvalidInstruction::Direction(_))
        ));
        assert!(matches!(
            Instruction::try_from("up one"),
            Err(InvalidInstruction::Distance(_))
        ));
    }
}
