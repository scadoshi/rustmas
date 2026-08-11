use crate::domain::solution::common::direction::Direction;
use thiserror::Error;

/// Returned when text does not name a turn. Carries what was read.
#[derive(Debug, Error)]
#[error("invalid turn: {0:?}")]
pub struct InvalidTurn(String);

/// A quarter turn, either way. Parses from `l`/`r` or the full words.
///
/// Separate from [`Direction`] so that nothing has to accept `Up` and `Down`
/// where they name no turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    /// The same quarter turn the other way.
    pub fn reversed(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Where you point after turning this way from `direction`.
    pub fn applied_to(self, direction: Direction) -> Direction {
        direction.turned(self)
    }
}

impl TryFrom<char> for Turn {
    type Error = InvalidTurn;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'l' => Ok(Self::Left),
            'r' => Ok(Self::Right),
            other => Err(InvalidTurn(other.to_string())),
        }
    }
}

impl TryFrom<&str> for Turn {
    type Error = InvalidTurn;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "left" | "l" => Ok(Self::Left),
            "right" | "r" => Ok(Self::Right),
            other => Err(InvalidTurn(other.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_letters_and_words_in_either_case() {
        assert_eq!(Turn::try_from('l').unwrap(), Turn::Left);
        assert_eq!(Turn::try_from('R').unwrap(), Turn::Right);
        assert_eq!(Turn::try_from("left").unwrap(), Turn::Left);
        assert_eq!(Turn::try_from("RIGHT").unwrap(), Turn::Right);
    }

    /// The reason the type exists: a heading is not a turn, so it does not
    /// parse as one.
    #[test]
    fn headings_are_not_turns() {
        for heading in ['u', 'd'] {
            assert!(Turn::try_from(heading).is_err());
        }
        for heading in ["up", "down", "u", "d", ""] {
            assert!(Turn::try_from(heading).is_err());
        }
    }

    #[test]
    fn the_error_says_what_it_read() {
        let error = Turn::try_from("sideways").unwrap_err();
        assert!(error.to_string().contains("sideways"));
    }

    #[test]
    fn reversing_twice_is_the_original_turn() {
        for turn in [Turn::Left, Turn::Right] {
            assert_eq!(turn.reversed().reversed(), turn);
            assert_ne!(turn.reversed(), turn);
        }
    }

    #[test]
    fn applying_a_turn_matches_turning_a_direction() {
        let direction = Direction::default();
        assert!(matches!(
            Turn::Right.applied_to(direction),
            Direction::Right
        ));
        assert!(matches!(Turn::Left.applied_to(direction), Direction::Left));
    }
}
