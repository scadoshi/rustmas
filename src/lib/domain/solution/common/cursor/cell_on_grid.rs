use crate::domain::solution::common::cell::Cell;
use crate::domain::solution::common::cursor::CellOutOfBounds;
use crate::domain::solution::common::direction::Direction;
use crate::domain::solution::common::grid::{FixedWidth, Grid, VariableWidth};

/// A [`Cell`] paired with a borrow of the grid it indexes, always in bounds.
#[derive(Debug)]
pub struct CellOnGrid<'grid, G> {
    cell: Cell,
    grid: &'grid G,
}

// Written out rather than derived: `derive` would demand `G: Copy`, which a
// `&G` field never needs, and no grid type is `Copy`.
impl<G> Copy for CellOnGrid<'_, G> {}
impl<G> Clone for CellOnGrid<'_, G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'grid, G: Grid> CellOnGrid<'grid, G> {
    /// A cursor on `cell`, checked against `grid`.
    ///
    /// # Errors
    ///
    /// [`CellOutOfBounds`] if `cell` is not on `grid`.
    pub fn new(cell: Cell, grid: &'grid G) -> Result<Self, CellOutOfBounds> {
        if !grid.contains_cell(&cell) {
            return Err(CellOutOfBounds);
        }
        Ok(Self { cell, grid })
    }

    /// Skips the bounds check. The caller guarantees `cell` is on `grid`.
    fn new_trusted(cell: Cell, grid: &'grid G) -> Self {
        Self { cell, grid }
    }

    pub fn cell(&self) -> Cell {
        self.cell
    }

    pub fn grid(&self) -> &'grid G {
        self.grid
    }

    /// Moves `distance` in `direction`, `None` if that would leave the grid.
    pub fn checked_moved(self, direction: Direction, distance: usize) -> Option<Self> {
        self.cell
            .checked_moved(direction, distance)
            .filter(|c| self.grid.contains_cell(c))
            .map(|c| Self::new_trusted(c, self.grid))
    }

    /// The item under the cursor.
    pub fn value(&self) -> &'grid G::Item {
        self.grid
            .at(&self.cell)
            .expect("a cursor is always in bounds")
    }
}

impl<G: Grid + FixedWidth> CellOnGrid<'_, G> {
    /// Moves `distance` in `direction`, clamping at the edges of a grid whose
    /// rows are all one width.
    #[must_use]
    pub fn saturating_moved_on_fixed_width(self, direction: Direction, distance: usize) -> Self {
        let cell = {
            let unclamped = self.cell.saturating_moved(direction, distance);
            let width_clamp = self.grid.width().saturating_sub(1);
            let height_clamp = self.grid.height().saturating_sub(1);
            unclamped
                .with_column(unclamped.column().min(width_clamp))
                .with_row(unclamped.row().min(height_clamp))
        };
        Self::new_trusted(cell, self.grid)
    }
}

impl<G: Grid + VariableWidth> CellOnGrid<'_, G> {
    /// Moves `distance` in `direction`, clamping at the edges of a grid whose
    /// rows are each their own width.
    ///
    /// The row clamps first, since which row you land on decides how wide it is.
    #[must_use]
    pub fn saturating_moved_on_variable_width(self, direction: Direction, distance: usize) -> Self {
        let cell = {
            let unclamped = self.cell.saturating_moved(direction, distance);
            let height_clamp = self.grid.height().saturating_sub(1);
            let clamped_row = unclamped.row().min(height_clamp);
            let width_clamp = self
                .grid
                .width_of(clamped_row)
                .map_or(0, |n| n.saturating_sub(1));
            unclamped
                .with_column(unclamped.column().min(width_clamp))
                .with_row(clamped_row)
        };
        Self::new_trusted(cell, self.grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::solution::common::grid::jagged::Jagged;
    use crate::domain::solution::common::grid::rectangle::Rectangle;

    fn rectangle() -> Rectangle<char> {
        Rectangle::new(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]).unwrap()
    }

    #[test]
    fn cursor_new_rejects_a_cell_off_the_grid() {
        let grid = rectangle();
        assert!(CellOnGrid::new(Cell::new(0, 0), &grid).is_ok());
        assert!(CellOnGrid::new(Cell::new(3, 0), &grid).is_err());
        assert!(CellOnGrid::new(Cell::new(0, 2), &grid).is_err());
    }

    #[test]
    fn checked_moved_stays_on_the_grid() {
        let grid = rectangle();
        let cursor = CellOnGrid::new(Cell::new(0, 0), &grid).unwrap();
        assert_eq!(
            cursor
                .checked_moved(Direction::Right, 2)
                .unwrap()
                .cell()
                .column(),
            2
        );
        assert_eq!(
            cursor
                .checked_moved(Direction::Down, 1)
                .unwrap()
                .cell()
                .row(),
            1
        );
    }

    #[test]
    fn checked_moved_off_the_grid_is_none() {
        let grid = rectangle();
        let cursor = CellOnGrid::new(Cell::new(0, 0), &grid).unwrap();
        assert!(cursor.checked_moved(Direction::Left, 1).is_none());
        assert!(cursor.checked_moved(Direction::Up, 1).is_none());
        assert!(cursor.checked_moved(Direction::Right, 3).is_none());
    }

    #[test]
    fn saturating_moved_clamps_at_every_edge() {
        let grid = rectangle();
        let cursor = CellOnGrid::new(Cell::new(0, 0), &grid).unwrap();
        assert_eq!(
            cursor
                .saturating_moved_on_fixed_width(Direction::Left, 99)
                .cell()
                .column(),
            0
        );
        assert_eq!(
            cursor
                .saturating_moved_on_fixed_width(Direction::Up, 99)
                .cell()
                .row(),
            0
        );
        assert_eq!(
            cursor
                .saturating_moved_on_fixed_width(Direction::Right, 99)
                .cell()
                .column(),
            2
        );
        assert_eq!(
            cursor
                .saturating_moved_on_fixed_width(Direction::Down, 99)
                .cell()
                .row(),
            1
        );
    }

    /// The row has to clamp before the width lookup, or an overshoot reads the
    /// width of a row that is not there.
    #[test]
    fn saturating_moved_takes_its_width_from_the_row_it_lands_on() {
        let grid = Jagged::new(vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g'],
            vec!['h', 'i', 'j', 'k'],
        ])
        .unwrap();
        let cursor = CellOnGrid::new(Cell::new(4, 0), &grid).unwrap();

        let landed = cursor.saturating_moved_on_variable_width(Direction::Down, 99);
        assert_eq!(landed.cell().row(), 2);
        assert_eq!(landed.cell().column(), 3);
        assert!(grid.contains_cell(&landed.cell()));
    }

    #[test]
    fn saturating_moved_clamps_within_a_short_row() {
        let grid = Jagged::new(vec![vec!['a', 'b', 'c'], vec!['d']]).unwrap();
        let cursor = CellOnGrid::new(Cell::new(2, 0), &grid).unwrap();

        let landed = cursor.saturating_moved_on_variable_width(Direction::Down, 1);
        assert_eq!(landed.cell().row(), 1);
        assert_eq!(landed.cell().column(), 0);
    }
}
