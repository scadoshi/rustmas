pub mod game;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2022::day_02::game::RawGame;

#[derive(Debug)]
pub struct Puzzle {
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
                .checked_add(g.to_game_other_is_player()?.player_score())
                .context("the scores add up past u32")
        })?;
        Ok(Answer::solved(total.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let total: u32 = self.games.iter().try_fold(0_u32, |total, g| {
            total
                .checked_add(g.to_game_other_is_result()?.player_score())
                .context("the scores add up past u32")
        })?;
        Ok(Answer::solved(total.to_string()))
    }
}
