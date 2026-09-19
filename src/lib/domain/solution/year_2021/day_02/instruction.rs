use std::{num::ParseIntError, str::FromStr};

use crate::domain::solution::year_2021::day_02::direction::{Direction, InvalidDirection};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidInstruction {
    #[error("expected a direction and a distance separated by whitespace")]
    TooFewParts,
    #[error("expected nothing after the distance")]
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

impl FromStr for Instruction {
    type Err = InvalidInstruction;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split_whitespace();
        let (Some(dir), Some(dist)) = (parts.next(), parts.next()) else {
            return Err(InvalidInstruction::TooFewParts);
        };
        if parts.next().is_some() {
            return Err(InvalidInstruction::TooManyParts);
        }
        let direction = dir.parse()?;
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
    fn instruction_from_str_ok() {
        let instruction = "forward 1".parse::<Instruction>().unwrap();
        assert!(matches!(instruction.direction, Direction::Forward));
        assert_eq!(instruction.distance, 1);
    }

    #[test]
    fn instruction_from_str_err() {
        assert!(matches!(
            "foo".parse::<Instruction>(),
            Err(InvalidInstruction::TooFewParts)
        ));
        assert!(matches!(
            "foo bar baz".parse::<Instruction>(),
            Err(InvalidInstruction::TooManyParts)
        ));
        assert!(matches!(
            "left 2".parse::<Instruction>(),
            Err(InvalidInstruction::Direction(_))
        ));
        assert!(matches!(
            "up one".parse::<Instruction>(),
            Err(InvalidInstruction::Distance(_))
        ));
    }
}
