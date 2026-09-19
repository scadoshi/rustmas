use crate::domain::solution::common::turn::{InvalidTurn, Turn};
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

/// Returned when a line does not name a turn and a distance.
#[derive(Debug, Error)]
pub(super) enum InvalidInstruction {
    #[error("expected a turn letter followed by a distance")]
    TooFewParts,
    #[error(transparent)]
    Turn(#[from] InvalidTurn),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// A turn and how far to go, read from one line.
pub(super) struct Instruction {
    pub(super) turn: Turn,
    pub(super) distance: i32,
}

impl FromStr for Instruction {
    type Err = InvalidInstruction;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut chars = value.trim().chars();
        let turn = Turn::try_from(chars.next().ok_or(InvalidInstruction::TooFewParts)?)?;
        let distance = chars.as_str().parse::<i32>()?;
        Ok(Self { turn, distance })
    }
}

/// Every instruction in the input, in order.
pub(super) struct Instructions(Vec<Instruction>);

impl Instructions {
    /// Borrows each instruction in turn.
    pub(super) fn iter(&self) -> std::slice::Iter<'_, Instruction> {
        self.0.iter()
    }
}

impl FromStr for Instructions {
    type Err = InvalidInstruction;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            value
                .trim()
                .lines()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
