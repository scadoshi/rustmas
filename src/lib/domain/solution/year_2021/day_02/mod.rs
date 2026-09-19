pub mod direction;
pub mod instruction;
pub mod position;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    year_2021::day_02::{
        instruction::Instruction,
        position::{Position, aimed::Aimed},
    },
};

pub struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            instructions: input
                .as_ref()
                .lines()
                .map(Instruction::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let position = self
            .instructions
            .iter()
            .fold(Position::default(), |p, &i| p.saturating_moved(i));
        Ok(Answer::solved(position.value().to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let aimed = self
            .instructions
            .iter()
            .fold(Aimed::default(), |p, &i| p.saturating_moved(i));
        Ok(Answer::solved(aimed.position.value().to_string()))
    }
}
