use crate::domain::solution::common::direction::Direction;

type Dir = Direction;

/// An unsigned grid index, rows counting down from the top-left.
///
/// So `Up` decreases the row. For signed coordinates on an unbounded plane,
/// use [`Point`](super::point::Point).
#[derive(Debug, Clone, Copy, Default)]
pub struct Cell {
    column: usize,
    row: usize,
}

impl Cell {
    pub fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }

    /// Moves `distance` in `direction`, `None` if that would leave the grid.
    pub fn checked_moved(self, direction: Direction, distance: usize) -> Option<Self> {
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

    /// Moves `distance` in `direction`, clamping at the edges of the grid.
    pub fn saturating_moved(self, direction: Direction, distance: usize) -> Self {
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
            Cell::new(usize::MAX, 0)
                .checked_moved(Dir::Right, 1)
                .is_none()
        );
        assert!(Cell::default().checked_moved(Dir::Up, 1).is_none());
        assert!(
            Cell::new(0, usize::MAX)
                .checked_moved(Dir::Down, 1)
                .is_none()
        );
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
            Cell::new(usize::MAX, 0)
                .saturating_moved(Dir::Right, 1)
                .column,
            usize::MAX
        );
        assert_eq!(Cell::default().saturating_moved(Dir::Up, 1).row, 0);
        assert_eq!(
            Cell::new(0, usize::MAX).saturating_moved(Dir::Down, 1).row,
            usize::MAX
        );
    }
}
