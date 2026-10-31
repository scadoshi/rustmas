//! Rock Paper Scissors: total the score from a strategy guide whose second column is read first as the shape to throw and then as the result to aim for.

mod game;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2022::day_02::game::RawGame;

/// The encrypted strategy guide.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One round per line, the second column left unread until a part decides what it means.
    games: Vec<RawGame>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            games: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let total: u32 = self.games.iter().try_fold(0_u32, |total, g| {
            total
                .checked_add(g.to_game_by_play()?.player_score())
                .context("the scores add up past u32")
        })?;
        Ok(Answer::solved(total.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let total: u32 = self.games.iter().try_fold(0_u32, |total, g| {
            total
                .checked_add(g.to_game_by_result()?.player_score())
                .context("the scores add up past u32")
        })?;
        Ok(Answer::solved(total.to_string()))
    }
}
