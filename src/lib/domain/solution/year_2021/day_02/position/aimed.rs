use crate::domain::solution::year_2021::day_02::{
    direction::Direction, instruction::Instruction, position::Position,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Aimed {
    pub aim: i32,
    pub position: Position,
}

impl Aimed {
    pub fn saturating_moved(self, instruction: Instruction) -> Self {
        match instruction.direction {
            Direction::Forward => Self {
                position: Position {
                    horizontal: self
                        .position
                        .horizontal
                        .saturating_add(instruction.distance),
                    depth: self
                        .position
                        .depth
                        .saturating_add(instruction.distance.saturating_mul(self.aim)),
                },
                ..self
            },
            Direction::Up => Self {
                aim: self.aim.saturating_sub(instruction.distance),
                ..self
            },
            Direction::Down => Self {
                aim: self.aim.saturating_add(instruction.distance),
                ..self
            },
        }
    }
}
