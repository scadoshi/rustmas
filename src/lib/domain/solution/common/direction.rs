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

#[cfg(test)]
mod tests {
    use super::*;

    /// [`Direction`] has no `PartialEq`, and the tests should not dictate that.
    fn is(direction: Direction, expected: Direction) -> bool {
        use Direction::*;
        matches!(
            (direction, expected),
            (Up, Up) | (Right, Right) | (Down, Down) | (Left, Left)
        )
    }

    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    #[test]
    fn turns_clockwise() {
        assert!(is(Direction::Up.turn_right(), Direction::Right));
        assert!(is(Direction::Right.turn_right(), Direction::Down));
        assert!(is(Direction::Down.turn_right(), Direction::Left));
        assert!(is(Direction::Left.turn_right(), Direction::Up));
    }

    #[test]
    fn turns_anticlockwise() {
        assert!(is(Direction::Up.turn_left(), Direction::Left));
        assert!(is(Direction::Left.turn_left(), Direction::Down));
        assert!(is(Direction::Down.turn_left(), Direction::Right));
        assert!(is(Direction::Right.turn_left(), Direction::Up));
    }

    /// Catches an ordering mistake that four single steps can each survive.
    #[test]
    fn four_turns_return_to_the_start() {
        for start in ALL {
            let mut turned = start;
            for _ in 0..4 {
                turned = turned.turn_right();
            }
            assert!(is(turned, start));
        }
    }

    #[test]
    fn turned_follows_the_turn() {
        for start in ALL {
            assert!(is(start.turned(Turn::Left), start.turn_left()));
            assert!(is(start.turned(Turn::Right), start.turn_right()));
        }
    }

    #[test]
    fn reads_letters_and_words_in_either_case() {
        assert!(is(Direction::try_from("u").unwrap(), Direction::Up));
        assert!(is(Direction::try_from("UP").unwrap(), Direction::Up));
        assert!(is(Direction::try_from("Left").unwrap(), Direction::Left));
        assert!(is(Direction::try_from('R').unwrap(), Direction::Right));
        assert!(is(Direction::try_from('d').unwrap(), Direction::Down));
    }

    #[test]
    fn refuses_anything_else() {
        assert!(Direction::try_from("north").is_err());
        assert!(Direction::try_from("").is_err());
        assert!(Direction::try_from('x').is_err());
    }

    /// The message has to name what was read, since the reply is the only clue
    /// about which line of the input was wrong.
    #[test]
    fn the_error_says_what_it_read() {
        let error = Direction::try_from("north").unwrap_err();
        assert!(error.to_string().contains("north"));
    }
}
