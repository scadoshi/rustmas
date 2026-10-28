//! Unsigned row and column positions on a grid.

use crate::domain::solution::common::direction::Direction;

/// An unsigned grid index, rows counting down from the top-left.
///
/// So `Up` decreases the row. For signed coordinates on an unbounded plane,
/// use [`Point`](super::point::Point).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Cell {
    column: usize,
    row: usize,
}

impl Cell {
    /// A cell `column` across from the left edge and `row` down from the top,
    /// both counted from 0.
    pub fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    /// Counted from 0 at the left edge.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Counted from 0 at the top edge, so it grows downward.
    pub fn row(&self) -> usize {
        self.row
    }

    /// The same row, at `column`.
    #[must_use]
    pub fn with_column(self, column: usize) -> Self {
        Self { column, ..self }
    }

    /// The same column, at `row`.
    #[must_use]
    pub fn with_row(self, row: usize) -> Self {
        Self { row, ..self }
    }

    /// Moves `distance` in `direction`, `None` if that would leave the grid.
    pub fn checked_moved(self, direction: Direction, distance: usize) -> Option<Self> {
        Some(Self {
            column: match direction {
                Direction::Left => self.column.checked_sub(distance)?,
                Direction::Right => self.column.checked_add(distance)?,
                _ => self.column,
            },
            row: match direction {
                Direction::Up => self.row.checked_sub(distance)?,
                Direction::Down => self.row.checked_add(distance)?,
                _ => self.row,
            },
        })
    }

    /// Moves `distance` in `direction`, clamping at the edges of the grid.
    #[must_use]
    pub fn saturating_moved(self, direction: Direction, distance: usize) -> Self {
        Self {
            column: match direction {
                Direction::Left => self.column.saturating_sub(distance),
                Direction::Right => self.column.saturating_add(distance),
                _ => self.column,
            },
            row: match direction {
                Direction::Up => self.row.saturating_sub(distance),
                Direction::Down => self.row.saturating_add(distance),
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
            Cell::new(1, 0)
                .checked_moved(Direction::Left, 1)
                .unwrap()
                .column,
            0
        );
        assert_eq!(
            Cell::default()
                .checked_moved(Direction::Right, 1)
                .unwrap()
                .column,
            1
        );
        assert_eq!(
            Cell::new(0, 1).checked_moved(Direction::Up, 1).unwrap().row,
            0
        );
        assert_eq!(
            Cell::default()
                .checked_moved(Direction::Down, 1)
                .unwrap()
                .row,
            1
        );
    }

    #[test]
    fn cell_checked_moved_out_of_bounds() {
        assert!(Cell::default().checked_moved(Direction::Left, 1).is_none());
        assert!(
            Cell::new(usize::MAX, 0)
                .checked_moved(Direction::Right, 1)
                .is_none()
        );
        assert!(Cell::default().checked_moved(Direction::Up, 1).is_none());
        assert!(
            Cell::new(0, usize::MAX)
                .checked_moved(Direction::Down, 1)
                .is_none()
        );
    }

    #[test]
    fn cell_saturated_moved_in_bounds() {
        assert_eq!(
            Cell::new(1, 0).saturating_moved(Direction::Left, 1).column,
            0
        );
        assert_eq!(
            Cell::default().saturating_moved(Direction::Right, 1).column,
            1
        );
        assert_eq!(Cell::new(0, 1).saturating_moved(Direction::Up, 1).row, 0);
        assert_eq!(Cell::default().saturating_moved(Direction::Down, 1).row, 1);
    }

    #[test]
    fn cell_saturated_moved_left_out_of_bounds() {
        assert_eq!(
            Cell::default().saturating_moved(Direction::Left, 1).column,
            0
        );
        assert_eq!(
            Cell::new(usize::MAX, 0)
                .saturating_moved(Direction::Right, 1)
                .column,
            usize::MAX
        );
        assert_eq!(Cell::default().saturating_moved(Direction::Up, 1).row, 0);
        assert_eq!(
            Cell::new(0, usize::MAX)
                .saturating_moved(Direction::Down, 1)
                .row,
            usize::MAX
        );
    }
}
