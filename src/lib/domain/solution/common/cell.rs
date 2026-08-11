use crate::domain::solution::common::direction::Direction;

type Dir = Direction;

/// A position in a grid, indexed by row and column.
///
/// The origin is the top-left cell, so `Up` decreases the row and `Down`
/// increases it. Both fields are unsigned: there is no cell above row 0 or
/// left of column 0. For signed coordinates on an unbounded plane, use
/// [`Point`](super::point::Point).
#[derive(Debug, Clone, Copy, Default)]
pub struct Cell {
    column: u32,
    row: u32,
}

impl Cell {
    /// Builds a cell at the given column and row.
    pub fn new(column: u32, row: u32) -> Self {
        Self { column, row }
    }

    /// Moves `distance` cells in `direction`, returning `None` if that would
    /// leave the grid by going past 0 or overflowing [`u32::MAX`].
    pub fn checked_moved(self, direction: Direction, distance: u32) -> Option<Self> {
        Some(Self {
            column: match direction {
                Dir::Left => self.column.checked_sub(distance)?,
                Dir::Right => self.column.checked_add(distance)?,
                _ => self.column,
            },
            row: match direction {
                Dir::Up => self.row.checked_sub(distance)?,
                Dir::Down => self.row.checked_add(distance)?,
                _ => self.row,
            },
        })
    }

    /// Moves `distance` cells in `direction`, clamping at the edges of the
    /// grid rather than failing. Going past 0 stops at 0, and going past
    /// [`u32::MAX`] stops at [`u32::MAX`].
    pub fn saturating_moved(self, direction: Direction, distance: u32) -> Self {
        Self {
            column: match direction {
                Dir::Left => self.column.saturating_sub(distance),
                Dir::Right => self.column.saturating_add(distance),
                _ => self.column,
            },
            row: match direction {
                Dir::Up => self.row.saturating_sub(distance),
                Dir::Down => self.row.saturating_add(distance),
                _ => self.row,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_checked_moved_in_bounds() {
        assert_eq!(
            Cell::new(1, 0).checked_moved(Dir::Left, 1).unwrap().column,
            0
        );
        assert_eq!(
            Cell::default().checked_moved(Dir::Right, 1).unwrap().column,
            1
        );
        assert_eq!(Cell::new(0, 1).checked_moved(Dir::Up, 1).unwrap().row, 0);
        assert_eq!(Cell::default().checked_moved(Dir::Down, 1).unwrap().row, 1);
    }

    #[test]
    fn cell_checked_moved_out_of_bounds() {
        assert!(Cell::default().checked_moved(Dir::Left, 1).is_none());
        assert!(
            Cell::new(u32::MAX, 0)
                .checked_moved(Dir::Right, 1)
                .is_none()
        );
        assert!(Cell::default().checked_moved(Dir::Up, 1).is_none());
        assert!(Cell::new(0, u32::MAX).checked_moved(Dir::Down, 1).is_none());
    }

    #[test]
    fn cell_saturated_moved_in_bounds() {
        assert_eq!(Cell::new(1, 0).saturating_moved(Dir::Left, 1).column, 0);
        assert_eq!(Cell::default().saturating_moved(Dir::Right, 1).column, 1);
        assert_eq!(Cell::new(0, 1).saturating_moved(Dir::Up, 1).row, 0);
        assert_eq!(Cell::default().saturating_moved(Dir::Down, 1).row, 1);
    }

    #[test]
    fn cell_saturated_moved_left_out_of_bounds() {
        assert_eq!(Cell::default().saturating_moved(Dir::Left, 1).column, 0);
        assert_eq!(
            Cell::new(u32::MAX, 0)
                .saturating_moved(Dir::Right, 1)
                .column,
            u32::MAX
        );
        assert_eq!(Cell::default().saturating_moved(Dir::Up, 1).row, 0);
        assert_eq!(
            Cell::new(0, u32::MAX).saturating_moved(Dir::Down, 1).row,
            u32::MAX
        );
    }
}
