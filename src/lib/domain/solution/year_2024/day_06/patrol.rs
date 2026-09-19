use crate::domain::solution::common::{
    cell::Cell,
    direction::Direction,
    grid::{
        FixedWidth, Grid,
        rectangle::{InvalidRectangle, Rectangle},
    },
};
use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidPatrol {
    #[error(transparent)]
    Shape(#[from] InvalidRectangle),
    #[error("expected a guard, one of `^>v<`")]
    MissingGuard,
}

/// The lab map and where the guard begins her patrol.
pub struct Patrol {
    map: Rectangle<char>,
    start: Cell,
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
                let facing = Direction::try_from(*map.get_at(&cell)?).ok()?;
                Some((cell, facing))
            });
        let (start, facing) = guard.ok_or(InvalidPatrol::MissingGuard)?;
        Ok(Self { map, start, facing })
    }
}

/// Where a walk went and whether it ever came back round on itself.
pub struct Walk {
    pub visited: HashSet<Cell>,
    pub looped: bool,
}

impl Patrol {
    pub fn start(&self) -> Cell {
        self.start
    }

    /// Follows the guard's rule from the start until she leaves or loops:
    /// blocked ahead means turn right, otherwise step forward.
    ///
    /// `extra` is one more obstacle, so part two can try a placement without
    /// copying the map.
    pub fn walk(&self, extra: Option<Cell>) -> Walk {
        let blocked = |cell: Cell| Some(cell) == extra || self.map.get_at(&cell) == Some(&'#');
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
