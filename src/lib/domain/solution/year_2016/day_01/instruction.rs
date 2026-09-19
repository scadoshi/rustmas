use crate::domain::solution::common::turn::{InvalidTurn, Turn};
use std::{num::ParseIntError, ops::Deref, str::FromStr, vec::IntoIter};
use thiserror::Error;

/// The ways a single instruction can fail to parse.
#[derive(Debug, Error)]
pub enum InvalidInstruction {
    #[error(transparent)]
    Turn(#[from] InvalidTurn),
    #[error(transparent)]
    Distance(#[from] ParseIntError),
    #[error("expected a turn letter followed by a distance")]
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

impl FromStr for Instruction {
    type Err = InvalidInstruction;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let turn = value
            .get(0..1)
            .ok_or(InvalidInstruction::TooShort)?
            .parse()?;
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

impl FromStr for Instructions {
    type Err = InvalidInstruction;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = value
            .split(',')
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(instructions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_turn_and_a_distance() {
        let instruction = "R2".parse::<Instruction>().unwrap();
        assert_eq!(instruction.turn, Turn::Right);
        assert_eq!(instruction.distance, 2);

        let instruction = "L347".parse::<Instruction>().unwrap();
        assert_eq!(instruction.turn, Turn::Left);
        assert_eq!(instruction.distance, 347);
    }

    /// Back when the letter was a direction, this parsed and walked three.
    #[test]
    fn a_heading_is_not_a_valid_instruction() {
        for input in ["U3", "D3", "up3"] {
            assert!(matches!(
                input.parse::<Instruction>(),
                Err(InvalidInstruction::Turn(_))
            ));
        }
    }

    #[test]
    fn rejects_a_missing_or_unparseable_distance() {
        assert!(matches!(
            "R".parse::<Instruction>(),
            Err(InvalidInstruction::Distance(_))
        ));
        assert!(matches!(
            "Rx".parse::<Instruction>(),
            Err(InvalidInstruction::Distance(_))
        ));
        assert!(matches!(
            "".parse::<Instruction>(),
            Err(InvalidInstruction::TooShort)
        ));
    }

    #[test]
    fn splits_on_commas_and_trims_the_spaces() {
        let instructions = "R2, L3,R5".parse::<Instructions>().unwrap();
        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[1].turn, Turn::Left);
        assert_eq!(instructions[1].distance, 3);
    }

    /// A partly parsed walk would give a confidently wrong answer.
    #[test]
    fn one_bad_instruction_fails_them_all() {
        assert!("R2, U3, L5".parse::<Instructions>().is_err());
    }
}
