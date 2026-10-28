use crate::domain::solution::year_2021::day_02::direction::Direction;
use crate::domain::solution::year_2021::day_02::instruction::Instruction;
use crate::domain::solution::year_2021::day_02::position::Position;

/// A position plus the aim that steers how deep each forward move goes.
#[derive(Debug, Clone, Copy, Default)]
pub(in super::super) struct Aimed {
    pub(in super::super) aim: i64,
    pub(in super::super) position: Position,
}

impl Aimed {
    /// Follows `instruction` the second way: up and down change the aim, and
    /// forward dives by the distance times the aim. `None` if a value leaves
    /// [`i64`].
    #[must_use]
    pub(in super::super) fn checked_moved(self, instruction: Instruction) -> Option<Self> {
        let distance = i64::from(instruction.distance);
        Some(match instruction.direction {
            Direction::Forward => Self {
                position: Position {
                    horizontal: self.position.horizontal.checked_add(distance)?,
                    depth: self
                        .position
                        .depth
                        .checked_add(distance.checked_mul(self.aim)?)?,
                },
                ..self
            },
            Direction::Up => Self {
                aim: self.aim.checked_sub(distance)?,
                ..self
            },
            Direction::Down => Self {
                aim: self.aim.checked_add(distance)?,
                ..self
            },
        })
    }
}
