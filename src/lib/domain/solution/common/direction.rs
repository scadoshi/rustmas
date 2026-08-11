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
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = InvalidDirection;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'l' => Ok(Self::Left),
            'r' => Ok(Self::Right),
            'u' => Ok(Self::Up),
            'd' => Ok(Self::Down),
            other => Err(InvalidDirection(other.to_string())),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = InvalidDirection;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "left" | "l" => Ok(Self::Left),
            "right" | "r" => Ok(Self::Right),
            "up" | "u" => Ok(Self::Up),
            "down" | "d" => Ok(Self::Down),
            other => Err(InvalidDirection(other.to_owned())),
        }
    }
}
