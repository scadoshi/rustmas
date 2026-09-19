use crate::domain::solution::common::direction::Direction as PointDirection;
use std::str::FromStr;
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

impl FromStr for Direction {
    type Err = InvalidDirection;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
    fn direction_from_str_ok() {
        assert!(matches!(
            "forward".parse::<Direction>().unwrap(),
            Direction::Forward
        ));
        assert!(matches!(
            "ForWARD".parse::<Direction>().unwrap(),
            Direction::Forward
        ));
        assert!(matches!(
            "DoWN".parse::<Direction>().unwrap(),
            Direction::Down
        ));
        assert!(matches!(
            "down".parse::<Direction>().unwrap(),
            Direction::Down
        ));
        assert!(matches!("UP".parse::<Direction>().unwrap(), Direction::Up));
        assert!(matches!("up".parse::<Direction>().unwrap(), Direction::Up));
    }

    #[test]
    fn direction_from_str_err() {
        assert!(matches!("left".parse::<Direction>(), Err(InvalidDirection)));
    }
}
