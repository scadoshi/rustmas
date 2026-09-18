pub mod aimed;

use crate::domain::solution::year_2021::day_02::{direction::Direction, instruction::Instruction};

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
}

impl Position {
    pub fn saturating_moved(self, instruction: Instruction) -> Self {
        match instruction.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal.saturating_add(instruction.distance),
                ..self
            },
            Direction::Up => Self {
                depth: self.depth.saturating_sub(instruction.distance),
                ..self
            },
            Direction::Down => Self {
                depth: self.depth.saturating_add(instruction.distance),
                ..self
            },
        }
    }

    pub fn value(&self) -> i32 {
        self.horizontal.saturating_mul(self.depth)
    }
}
