use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidCard {
    #[error("expected `Card <id>: <winning numbers> | <numbers you have>`")]
    MissingColon,
    #[error("expected `Card` followed by an id")]
    MissingId,
    #[error("expected a `|` between the winning numbers and yours")]
    MissingBar,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug)]
pub struct Card {
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl FromStr for Card {
    type Err = InvalidCard;

    /// Parses `Card 1: 41 48 83 | 83 86 6`. The id is checked but not kept,
    /// since cards are numbered in the order they appear.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (head, rest) = value
            .trim()
            .split_once(':')
            .ok_or(InvalidCard::MissingColon)?;
        head.strip_prefix("Card ")
            .ok_or(InvalidCard::MissingId)?
            .trim()
            .parse::<u32>()?;
        let (winning, have) = rest.split_once('|').ok_or(InvalidCard::MissingBar)?;
        let numbers = |list: &str| -> Result<Vec<u32>, ParseIntError> {
            list.split_whitespace().map(str::parse).collect()
        };
        Ok(Self {
            winning: numbers(winning)?,
            have: numbers(have)?,
        })
    }
}

impl Card {
    /// How many of the numbers you have are winning numbers.
    pub fn matches(&self) -> usize {
        self.have
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    /// One point for the first match, doubling for each after.
    pub fn points(&self) -> u32 {
        match self.matches() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_from_str_ok() {
        let card: Card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .parse()
            .unwrap();
        assert_eq!(card.matches(), 4);
        assert_eq!(card.points(), 8);
    }

    #[test]
    fn card_from_str_err() {
        assert!(matches!(
            "Card 1 41 | 83".parse::<Card>(),
            Err(InvalidCard::MissingColon)
        ));
        assert!(matches!(
            "Ticket 1: 41 | 83".parse::<Card>(),
            Err(InvalidCard::MissingId)
        ));
        assert!(matches!(
            "Card 1: 41 83".parse::<Card>(),
            Err(InvalidCard::MissingBar)
        ));
        assert!(matches!(
            "Card 1: 41 | x".parse::<Card>(),
            Err(InvalidCard::ParseInt(_))
        ));
    }

    #[test]
    fn points_double_per_match_from_one() {
        let card = |have: &str| format!("Card 1: 1 2 3 4 | {have}").parse::<Card>().unwrap();
        assert_eq!(card("9").points(), 0);
        assert_eq!(card("1").points(), 1);
        assert_eq!(card("1 2").points(), 2);
        assert_eq!(card("1 2 3 4").points(), 8);
    }
}
