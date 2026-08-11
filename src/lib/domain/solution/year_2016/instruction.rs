use crate::domain::solution::common::direction::{Direction, InvalidDirection};
use std::{num::ParseIntError, ops::Deref, vec::IntoIter};
use thiserror::Error;

/// The ways a single instruction can fail to parse.
#[derive(Debug, Error)]
pub enum InvalidInstruction {
    #[error(transparent)]
    Direction(#[from] InvalidDirection),
    #[error(transparent)]
    Distance(#[from] ParseIntError),
    #[error("given string was too short")]
    TooShort,
}

/// One step of a puzzle input: which way to go and how far.
///
/// Parses from a direction letter followed by a number, such as `R2` or `L3`.
pub(super) struct Instruction {
    pub direction: Direction,
    pub distance: i32,
}

impl TryFrom<&str> for Instruction {
    type Error = InvalidInstruction;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let direction = Direction::try_from(value.get(0..1).ok_or(InvalidInstruction::TooShort)?)?;
        let distance: i32 = value
            .get(1..)
            .ok_or(InvalidInstruction::TooShort)?
            .parse()?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

/// A whole puzzle input. Parses from comma-separated instructions, trimming
/// each one, and fails as a whole if any single instruction fails.
///
/// Derefs to the underlying [`Vec`] for reading, and consumes into an
/// iterator of [`Instruction`].
pub(super) struct Instructions(Vec<Instruction>);

impl Deref for Instructions {
    type Target = Vec<Instruction>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Instructions {
    type Item = Instruction;
    type IntoIter = IntoIter<Instruction>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<&str> for Instructions {
    type Error = InvalidInstruction;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let instructions: Vec<Instruction> = value
            .split(',')
            .map(|s| Instruction::try_from(s.trim()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(instructions))
    }
}
