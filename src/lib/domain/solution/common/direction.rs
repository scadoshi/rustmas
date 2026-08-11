use crate::domain::solution::common::turn::Turn;
use thiserror::Error;

/// Returned when text does not name a direction. Carries what was read.
#[derive(Debug, Error)]
#[error("invalid direction: {0:?}")]
pub struct InvalidDirection(String);

/// One of the four moves along an axis. Parses from a letter or the full word.
///
/// `Up` and `Down` mean opposite things to the two position types:
/// [`Point`](super::point::Point) counts `y` upward,
/// [`Cell`](super::cell::Cell) counts rows down from the top. Variants are
/// declared clockwise, which is what makes the turns one step along the list.
#[derive(Debug, Clone, Copy, Default)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// A quarter turn clockwise.
    pub fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    /// A quarter turn anticlockwise.
    pub fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    /// A quarter turn whichever way [`Turn`] says.
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
            'u' => Ok(Self::Up),
            'r' => Ok(Self::Right),
            'd' => Ok(Self::Down),
            'l' => Ok(Self::Left),
            other => Err(InvalidDirection(other.to_string())),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = InvalidDirection;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "up" | "u" => Ok(Self::Up),
            "right" | "r" => Ok(Self::Right),
            "down" | "d" => Ok(Self::Down),
            "left" | "l" => Ok(Self::Left),
            other => Err(InvalidDirection(other.to_owned())),
        }
    }
}
