use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::domain::solution::common::turn::{InvalidTurn, Turn};

/// Returned when a line does not name a turn and a distance.
#[derive(Debug, Error)]
pub(super) enum InvalidInstruction {
    #[error("expected a turn letter followed by a distance")]
    TooFewParts,
    #[error(transparent)]
    Turn(#[from] InvalidTurn),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    /// The distance is below zero; a turn's direction comes from its letter.
    #[error("expected a distance of zero or more, read {0}")]
    Negative(i32),
}

/// A turn and how far to go, read from one line.
#[derive(Debug)]
pub(super) struct Instruction {
    pub(super) turn: Turn,
    /// Never negative, checked when parsed.
    pub(super) distance: i32,
}

impl FromStr for Instruction {
    type Err = InvalidInstruction;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut chars = value.trim().chars();
        let turn = Turn::try_from(chars.next().ok_or(InvalidInstruction::TooFewParts)?)?;
        let distance = chars.as_str().parse::<i32>()?;
        if distance < 0 {
            return Err(InvalidInstruction::Negative(distance));
        }
        Ok(Self { turn, distance })
    }
}

/// Every instruction in the input, in order.
#[derive(Debug)]
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
