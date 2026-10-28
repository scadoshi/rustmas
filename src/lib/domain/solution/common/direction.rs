//! The four axis moves, and turning between them.

use std::str::FromStr;

use thiserror::Error;

use crate::domain::solution::common::turn::Turn;

/// Returned when text does not name a direction. Carries what was read.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("expected a direction like `up`, `u` or `^`, read {0:?}")]
pub struct InvalidDirection(String);

/// One of the four moves along an axis. Parses from a letter or the full word.
///
/// `Up` and `Down` mean opposite things to [`Point`](super::point::Point) and
/// [`Cell`](super::cell::Cell). Variants are declared clockwise, which is what
/// makes a turn one step along the list.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    /// A lower row on a [`Cell`](super::cell::Cell), a higher `y` on a
    /// [`Point`](super::point::Point).
    #[default]
    Up,
    /// A higher column or `x`.
    Right,
    /// A higher row on a [`Cell`](super::cell::Cell), a lower `y` on a
    /// [`Point`](super::point::Point).
    Down,
    /// A lower column or `x`.
    Left,
}

impl Direction {
    /// A quarter turn clockwise.
    #[must_use]
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    /// A quarter turn anticlockwise.
    #[must_use]
    pub fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    /// A quarter turn whichever way [`Turn`] says.
    #[must_use]
    pub fn turned(self, turn: Turn) -> Self {
        match turn {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = InvalidDirection;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'u' | '^' => Ok(Self::Up),
            'r' | '>' => Ok(Self::Right),
            'd' | 'v' => Ok(Self::Down),
            'l' | '<' => Ok(Self::Left),
            _ => Err(InvalidDirection(value.to_string())),
        }
    }
}

impl FromStr for Direction {
    type Err = InvalidDirection;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "up" | "u" | "^" => Ok(Self::Up),
            "right" | "r" | ">" => Ok(Self::Right),
            "down" | "d" | "v" => Ok(Self::Down),
            "left" | "l" | "<" => Ok(Self::Left),
            _ => Err(InvalidDirection(value.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    #[test]
    fn turns_clockwise() {
        assert_eq!(Direction::Up.turn_right(), Direction::Right);
        assert_eq!(Direction::Right.turn_right(), Direction::Down);
        assert_eq!(Direction::Down.turn_right(), Direction::Left);
        assert_eq!(Direction::Left.turn_right(), Direction::Up);
    }

    #[test]
    fn turns_anticlockwise() {
        assert_eq!(Direction::Up.turn_left(), Direction::Left);
        assert_eq!(Direction::Left.turn_left(), Direction::Down);
        assert_eq!(Direction::Down.turn_left(), Direction::Right);
        assert_eq!(Direction::Right.turn_left(), Direction::Up);
    }

    /// Catches an ordering mistake that four single steps can each survive.
    #[test]
    fn four_turns_return_to_the_start() {
        for start in ALL {
            let mut turned = start;
            for _ in 0..4 {
                turned = turned.turn_right();
            }
            assert_eq!(turned, start);
        }
    }

    #[test]
    fn turned_follows_the_turn() {
        for start in ALL {
            assert_eq!(start.turned(Turn::Left), start.turn_left());
            assert_eq!(start.turned(Turn::Right), start.turn_right());
        }
    }

    #[test]
    fn reads_letters_and_words_in_either_case() {
        assert_eq!("u".parse::<Direction>().unwrap(), Direction::Up);
        assert_eq!("UP".parse::<Direction>().unwrap(), Direction::Up);
        assert_eq!("<".parse::<Direction>().unwrap(), Direction::Left);
        assert_eq!(Direction::try_from('>').unwrap(), Direction::Right);
        assert_eq!("Left".parse::<Direction>().unwrap(), Direction::Left);
        assert_eq!(Direction::try_from('R').unwrap(), Direction::Right);
        assert_eq!(Direction::try_from('d').unwrap(), Direction::Down);
    }

    #[test]
    fn refuses_anything_else() {
        assert!("north".parse::<Direction>().is_err());
        assert!("".parse::<Direction>().is_err());
        assert!(Direction::try_from('x').is_err());
    }

    /// The message has to name what was read, since the reply is the only clue
    /// about which line of the input was wrong.
    #[test]
    fn the_error_says_what_it_read() {
        let error = "north".parse::<Direction>().unwrap_err();
        assert!(error.to_string().contains("north"));
    }

    #[test]
    fn the_error_keeps_the_original_case() {
        assert!(
            "North"
                .parse::<Direction>()
                .unwrap_err()
                .to_string()
                .contains("North")
        );
        assert!(
            Direction::try_from('X')
                .unwrap_err()
                .to_string()
                .contains('X')
        );
    }
}
