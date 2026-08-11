use thiserror::Error;

/// Returned when text does not name a direction. Carries what was read.
#[derive(Debug, Error)]
#[error("invalid direction: {0:?}")]
pub struct InvalidDirection(String);

/// One of the four moves along an axis.
///
/// What `Up` and `Down` do to a coordinate depends on the type being moved:
/// [`Point`](super::point::Point) counts `y` upward, while
/// [`Cell`](super::cell::Cell) counts rows downward from the top.
///
/// Parses from a `char` (`'l'`, `'r'`, `'u'`, `'d'`) or a `&str` (those
/// letters or the full words), either case.
///
/// Declared clockwise from `Up`, so [`Direction::turn_right`] is one step down
/// the list and [`Direction::turn_left`] one step up. `Up` is the default
/// because a puzzle that starts you facing somewhere usually starts you facing
/// north.
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
