mod room;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2016::day_04::room::Room;

#[derive(Debug)]
pub(crate) struct Puzzle {
    rooms: Vec<Room>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            rooms: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let total = self
            .rooms
            .iter()
            .filter(|room| room.is_real())
            .try_fold(0_u32, |total, room| total.checked_add(room.id))
            .context("the real room ids add up past u32")?;
        Ok(Answer::solved(total.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        match self
            .rooms
            .iter()
            .find(|room| room.decrypted_name().contains("northpole"))
        {
            Some(room) => Ok(Answer::solved(room.id.to_string())),
            None => Ok(Answer::None),
        }
    }
}
