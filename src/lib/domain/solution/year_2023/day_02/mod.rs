pub mod game;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::parse,
    year_2023::day_02::game::{Game, Handful},
};

/// The bag part one asks about.
const BAG: Handful = Handful {
    red: 12,
    green: 13,
    blue: 14,
};

pub struct Puzzle {
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
        Ok(Answer::solved(
            self.games
                .iter()
                .filter(|game| game.handfuls.iter().all(|h| h.fits_in(BAG)))
                .map(|game| game.id)
                .sum::<u32>()
                .to_string(),
        ))
    }

    /// The power of each game's smallest possible bag.
    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.games
                .iter()
                .map(|game| game.smallest_bag().power())
                .sum::<u32>()
                .to_string(),
        ))
    }
}
