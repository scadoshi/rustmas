use crate::domain::solution::common::direction::Direction as PointDirection;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("expected forward, down or up")]
pub struct InvalidDirection;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl TryFrom<&str> for Direction {
    type Error = InvalidDirection;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => return Err(InvalidDirection),
        })
    }
}

impl From<Direction> for PointDirection {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Forward => PointDirection::Right,
            Direction::Down => PointDirection::Down,
            Direction::Up => PointDirection::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_try_from_str_ok() {
        assert!(matches!(
            Direction::try_from("forward").unwrap(),
            Direction::Forward
        ));
        assert!(matches!(
            Direction::try_from("ForWARD").unwrap(),
            Direction::Forward
        ));
        assert!(matches!(
            Direction::try_from("DoWN").unwrap(),
            Direction::Down
        ));
        assert!(matches!(
            Direction::try_from("down").unwrap(),
            Direction::Down
        ));
        assert!(matches!(Direction::try_from("UP").unwrap(), Direction::Up));
        assert!(matches!(Direction::try_from("up").unwrap(), Direction::Up));
    }

    #[test]
    fn direction_try_from_str_err() {
        assert!(matches!(Direction::try_from("left"), Err(InvalidDirection)));
    }
}
