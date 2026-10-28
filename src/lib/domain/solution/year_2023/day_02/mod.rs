mod game;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2023::day_02::game::{Game, Handful};

/// The bag part one asks about.
const BAG: Handful = Handful {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug)]
pub(crate) struct Puzzle {
    games: Vec<Game>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            games: parse::lines(input.as_ref())?,
        })
    }

    /// The ids of the games every handful of which fits the bag.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let total = self
            .games
            .iter()
            .filter(|game| game.handfuls.iter().all(|h| h.fits_in(BAG)))
            .try_fold(0_u32, |total, game| total.checked_add(game.id))
            .ok_or_else(|| anyhow!("the game ids add up past {}", u32::MAX))?;
        Ok(Answer::solved(total.to_string()))
    }

    /// The power of each game's smallest possible bag.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let total = self
            .games
            .iter()
            .try_fold(0_u32, |total, game| {
                game.smallest_bag()
                    .power()
                    .and_then(|power| total.checked_add(power))
            })
            .ok_or_else(|| anyhow!("the powers add up past {}", u32::MAX))?;
        Ok(Answer::solved(total.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn the_example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("8"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("2286"));
    }

    #[test]
    fn ids_past_u32_are_an_error() {
        let puzzle = Puzzle::new(format!("Game {}: 1 red\nGame 1: 1 red", u32::MAX)).unwrap();
        assert!(puzzle.part_one().is_err());
    }

    /// 5000 of each color is a power of 125 billion.
    #[test]
    fn a_power_past_u32_is_an_error() {
        let puzzle = Puzzle::new("Game 1: 5000 red, 5000 green, 5000 blue").unwrap();
        assert!(puzzle.part_two().is_err());
    }
}
