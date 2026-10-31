//! The guard's patrol: the lab map, where she starts, and the walk she takes from there.

use std::collections::HashSet;
use std::str::FromStr;

use thiserror::Error;

use crate::domain::solution::common::cell::Cell;
use crate::domain::solution::common::direction::Direction;
use crate::domain::solution::common::grid::rectangle::{InvalidRectangle, Rectangle};
use crate::domain::solution::common::grid::{FixedWidth, Grid};

/// Why the input does not read as a lab map with a guard on it.
#[derive(Debug, Error)]
pub(super) enum InvalidPatrol {
    /// The map is empty or its rows differ in width.
    #[error(transparent)]
    Shape(#[from] InvalidRectangle),
    /// No `^`, `>`, `v` or `<` anywhere on the map.
    #[error("expected a guard, one of `^>v<`")]
    MissingGuard,
}

/// The lab map and where the guard begins her patrol.
#[derive(Debug)]
pub(super) struct Patrol {
    /// The lab as drawn, `#` for an obstacle. The guard's marker stays on her start cell.
    map: Rectangle<char>,
    /// The cell the guard starts on.
    start: Cell,
    /// The way she faces at the start.
    facing: Direction,
}

impl FromStr for Patrol {
    type Err = InvalidPatrol;

    /// Parses the map, locating the guard by her `^>v<` marker.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let map = Rectangle::new(
            value
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().chars().collect())
                .collect::<Vec<Vec<char>>>(),
        )?;
        let guard = (0..map.height())
            .flat_map(|row| (0..map.width()).map(move |column| Cell::new(column, row)))
            .find_map(|cell| {
                let facing = Direction::try_from(*map.at(&cell)?).ok()?;
                Some((cell, facing))
            });
        let (start, facing) = guard.ok_or(InvalidPatrol::MissingGuard)?;
        Ok(Self { map, start, facing })
    }
}

/// Where a walk went and whether it ever came back round on itself.
#[derive(Debug)]
pub(super) struct Walk {
    /// Every cell she stood on, the start included.
    pub(super) visited: HashSet<Cell>,
    /// True when she came back to a cell facing the same way, false when she walked off the map.
    pub(super) looped: bool,
}

impl Patrol {
    /// The cell the guard starts on, where the puzzle forbids an obstacle.
    pub(super) fn start(&self) -> Cell {
        self.start
    }

    /// Follows the guard's rule from the start until she leaves or loops:
    /// blocked ahead means turn right, otherwise step forward.
    ///
    /// `extra` is one more obstacle, so part two can try a placement without
    /// copying the map.
    pub(super) fn walk(&self, extra: Option<Cell>) -> Walk {
        let blocked = |cell: Cell| Some(cell) == extra || self.map.at(&cell) == Some(&'#');
        let mut seen = HashSet::new();
        let (mut at, mut facing) = (self.start, self.facing);
        loop {
            if !seen.insert((at, facing)) {
                return Walk {
                    visited: seen.into_iter().map(|(cell, _)| cell).collect(),
                    looped: true,
                };
            }
            let Some(ahead) = at
                .checked_moved(facing, 1)
                .filter(|c| self.map.contains_cell(c))
            else {
                return Walk {
                    visited: seen.into_iter().map(|(cell, _)| cell).collect(),
                    looped: false,
                };
            };
            if blocked(ahead) {
                facing = facing.turn_right();
            } else {
                at = ahead;
            }
        }
    }
}
