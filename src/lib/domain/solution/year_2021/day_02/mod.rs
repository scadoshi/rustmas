pub mod direction;
pub mod instruction;
pub mod position;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2021::day_02::instruction::Instruction;
use crate::domain::solution::year_2021::day_02::position::Position;
use crate::domain::solution::year_2021::day_02::position::aimed::Aimed;

pub struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            instructions: parse::lines(input.as_ref())?,
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
