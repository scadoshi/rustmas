use crate::domain::solution::common::turn::{InvalidTurn, Turn};
use std::{num::ParseIntError, ops::Deref, vec::IntoIter};
use thiserror::Error;

/// The ways a single instruction can fail to parse.
#[derive(Debug, Error)]
pub enum InvalidInstruction {
    #[error(transparent)]
    Turn(#[from] InvalidTurn),
    #[error(transparent)]
    Distance(#[from] ParseIntError),
    #[error("given string was too short")]
    TooShort,
}

/// Which way to turn, then how far to walk. Parses from `R2` or `L3`.
///
/// The letter is a [`Turn`] rather than a direction, so `U3` is rejected
/// instead of quietly turning nowhere and walking three.
pub(super) struct Instruction {
    pub turn: Turn,
    pub distance: i32,
}

impl TryFrom<&str> for Instruction {
    type Error = InvalidInstruction;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let turn = Turn::try_from(value.get(0..1).ok_or(InvalidInstruction::TooShort)?)?;
        let distance: i32 = value
            .get(1..)
            .ok_or(InvalidInstruction::TooShort)?
            .parse()?;
        Ok(Self { turn, distance })
    }
}

/// A whole input: comma-separated instructions, trimmed, all-or-nothing.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_turn_and_a_distance() {
        let instruction = Instruction::try_from("R2").unwrap();
        assert_eq!(instruction.turn, Turn::Right);
        assert_eq!(instruction.distance, 2);

        let instruction = Instruction::try_from("L347").unwrap();
        assert_eq!(instruction.turn, Turn::Left);
        assert_eq!(instruction.distance, 347);
    }

    /// Back when the letter was a direction, this parsed and walked three.
    #[test]
    fn a_heading_is_not_a_valid_instruction() {
        for input in ["U3", "D3", "up3"] {
            assert!(matches!(
                Instruction::try_from(input),
                Err(InvalidInstruction::Turn(_))
            ));
        }
    }

    #[test]
    fn rejects_a_missing_or_unparseable_distance() {
        assert!(matches!(
            Instruction::try_from("R"),
            Err(InvalidInstruction::Distance(_))
        ));
        assert!(matches!(
            Instruction::try_from("Rx"),
            Err(InvalidInstruction::Distance(_))
        ));
        assert!(matches!(
            Instruction::try_from(""),
            Err(InvalidInstruction::TooShort)
        ));
    }

    #[test]
    fn splits_on_commas_and_trims_the_spaces() {
        let instructions = Instructions::try_from("R2, L3,R5").unwrap();
        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[1].turn, Turn::Left);
        assert_eq!(instructions[1].distance, 3);
    }

    /// A partly parsed walk would give a confidently wrong answer.
    #[test]
    fn one_bad_instruction_fails_them_all() {
        assert!(Instructions::try_from("R2, U3, L5").is_err());
    }
}
