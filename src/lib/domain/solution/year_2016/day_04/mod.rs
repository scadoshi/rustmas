pub mod room;

use crate::domain::solution::{
    Solution, answer::Answer, common::parse, year_2016::day_04::room::Room,
};

pub struct Puzzle {
    rooms: Vec<Room>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            rooms: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.rooms
                .iter()
                .filter(|room| room.is_real())
                .map(|room| room.id)
                .sum::<u32>()
                .to_string(),
        ))
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
