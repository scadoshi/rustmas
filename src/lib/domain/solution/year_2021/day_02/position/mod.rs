pub(super) mod aimed;

use crate::domain::solution::year_2021::day_02::direction::Direction;
use crate::domain::solution::year_2021::day_02::instruction::Instruction;

/// How far forward and how deep the submarine is.
///
/// Kept in [`i64`] so a product of two [`i32`]-sized moves still fits.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Position {
    horizontal: i64,
    depth: i64,
}

impl Position {
    /// Follows `instruction` the first way: up and down change depth directly.
    /// `None` if a coordinate leaves [`i64`].
    #[must_use]
    pub(super) fn checked_moved(self, instruction: Instruction) -> Option<Self> {
        let distance = i64::from(instruction.distance);
        Some(match instruction.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal.checked_add(distance)?,
                ..self
            },
            Direction::Up => Self {
                depth: self.depth.checked_sub(distance)?,
                ..self
            },
            Direction::Down => Self {
                depth: self.depth.checked_add(distance)?,
                ..self
            },
        })
    }

    /// Horizontal position times depth, or `None` if that leaves [`i64`].
    #[must_use]
    pub(super) fn value(&self) -> Option<i64> {
        self.horizontal.checked_mul(self.depth)
    }
}
