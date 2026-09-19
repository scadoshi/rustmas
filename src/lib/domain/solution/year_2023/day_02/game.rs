use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidGame {
    #[error("expected `Game <id>: <handfuls>`")]
    MissingColon,
    #[error("expected `Game` followed by an id")]
    MissingId,
    #[error("expected a count followed by a color, like `3 blue`")]
    MissingColor,
    #[error("expected red, green or blue, read {0:?}")]
    UnknownColor(String),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// One handful drawn from the bag: how many cubes of each color showed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Handful {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for Handful {
    type Err = InvalidGame;

    /// Parses `3 blue, 4 red`; a color not named counts as zero.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value
            .split(',')
            .try_fold(Self::default(), |mut handful, cubes| {
                let (count, color) = cubes
                    .trim()
                    .split_once(' ')
                    .ok_or(InvalidGame::MissingColor)?;
                let count = count.parse()?;
                match color.trim() {
                    "red" => handful.red = count,
                    "green" => handful.green = count,
                    "blue" => handful.blue = count,
                    other => return Err(InvalidGame::UnknownColor(other.to_owned())),
                }
                Ok(handful)
            })
    }
}

impl Handful {
    /// Whether this handful could have come out of `bag`.
    pub fn fits_in(self, bag: Self) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    /// The most of each color seen across `self` and `other`.
    pub fn max(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    /// The puzzle's "power": the counts multiplied together.
    pub fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub handfuls: Vec<Handful>,
}

impl FromStr for Game {
    type Err = InvalidGame;

    /// Parses `Game 1: 3 blue, 4 red; 1 red, 2 green`.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (head, handfuls) = value
            .trim()
            .split_once(':')
            .ok_or(InvalidGame::MissingColon)?;
        let id = head
            .strip_prefix("Game ")
            .ok_or(InvalidGame::MissingId)?
            .trim()
            .parse()?;
        Ok(Self {
            id,
            handfuls: handfuls
                .split(';')
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Game {
    /// The smallest bag every handful in this game fits in.
    pub fn smallest_bag(&self) -> Handful {
        self.handfuls
            .iter()
            .copied()
            .fold(Handful::default(), Handful::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_from_str_ok() {
        let game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse()
            .unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.handfuls.len(), 3);
        assert_eq!(
            game.handfuls[0],
            Handful {
                red: 4,
                green: 0,
                blue: 3
            }
        );
        assert_eq!(
            game.smallest_bag(),
            Handful {
                red: 4,
                green: 2,
                blue: 6
            }
        );
    }

    #[test]
    fn game_from_str_err() {
        assert!(matches!(
            "Game 1 3 blue".parse::<Game>(),
            Err(InvalidGame::MissingColon)
        ));
        assert!(matches!(
            "Round 1: 3 blue".parse::<Game>(),
            Err(InvalidGame::MissingId)
        ));
        assert!(matches!(
            "Game 1: blue".parse::<Game>(),
            Err(InvalidGame::MissingColor)
        ));
        assert!(matches!(
            "Game 1: 3 teal".parse::<Game>(),
            Err(InvalidGame::UnknownColor(_))
        ));
        assert!(matches!(
            "Game x: 3 blue".parse::<Game>(),
            Err(InvalidGame::ParseInt(_))
        ));
    }
}
