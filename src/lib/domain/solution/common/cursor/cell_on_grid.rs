use crate::domain::solution::common::{
    cell::Cell,
    cursor::{CellOutOfBounds, Cursor, FixedWidthGridCursor, VariableWidthGridCursor},
    grid::{FixedWidth, Grid, VariableWidth},
};

/// A [`Cursor`] holding a cell and a borrow of the grid it sits on.
#[derive(Debug)]
pub struct CellOnGrid<'a, G: Grid> {
    cell: Cell,
    grid: &'a G,
}

// Written out rather than derived: `derive` would demand `G: Copy`, which a
// `&G` field never needs, and no grid type is `Copy`.
impl<G: Grid> Copy for CellOnGrid<'_, G> {}
impl<G: Grid> Clone for CellOnGrid<'_, G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, G: Grid> Cursor<'a, G> for CellOnGrid<'a, G> {
    fn new(cell: Cell, grid: &'a G) -> Result<Self, CellOutOfBounds>
    where
        Self: Sized,
    {
        if !grid.contains_cell(&cell) {
            return Err(CellOutOfBounds);
        }
        Ok(Self { cell, grid })
    }

    fn new_unchecked(cell: Cell, grid: &'a G) -> Self {
        Self { cell, grid }
    }

    fn cell(&self) -> Cell {
        self.cell
    }

    fn grid(&self) -> &'a G {
        self.grid
    }
}

impl<'a, G: Grid + FixedWidth> FixedWidthGridCursor<'a, G> for CellOnGrid<'a, G> {}

impl<'a, G: Grid + VariableWidth> VariableWidthGridCursor<'a, G> for CellOnGrid<'a, G> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::solution::common::{
        direction::Direction as Dir,
        grid::{jagged::Jagged, rectangle::Rectangle},
    };

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
            cursor.checked_moved(Dir::Right, 2).unwrap().cell().column(),
            2
        );
        assert_eq!(cursor.checked_moved(Dir::Down, 1).unwrap().cell().row(), 1);
    }

    #[test]
    fn checked_moved_off_the_grid_is_none() {
        let grid = rectangle();
        let cursor = CellOnGrid::new(Cell::new(0, 0), &grid).unwrap();
        assert!(cursor.checked_moved(Dir::Left, 1).is_none());
        assert!(cursor.checked_moved(Dir::Up, 1).is_none());
        assert!(cursor.checked_moved(Dir::Right, 3).is_none());
    }

    #[test]
    fn saturating_moved_clamps_at_every_edge() {
        let grid = rectangle();
        let cursor = CellOnGrid::new(Cell::new(0, 0), &grid).unwrap();
        assert_eq!(cursor.saturating_moved(Dir::Left, 99).cell().column(), 0);
        assert_eq!(cursor.saturating_moved(Dir::Up, 99).cell().row(), 0);
        assert_eq!(cursor.saturating_moved(Dir::Right, 99).cell().column(), 2);
        assert_eq!(cursor.saturating_moved(Dir::Down, 99).cell().row(), 1);
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

        let landed = cursor.saturating_moved(Dir::Down, 99);
        assert_eq!(landed.cell().row(), 2);
        assert_eq!(landed.cell().column(), 3);
        assert!(grid.contains_cell(&landed.cell()));
    }

    #[test]
    fn saturating_moved_clamps_within_a_short_row() {
        let grid = Jagged::new(vec![vec!['a', 'b', 'c'], vec!['d']]).unwrap();
        let cursor = CellOnGrid::new(Cell::new(2, 0), &grid).unwrap();

        let landed = cursor.saturating_moved(Dir::Down, 1);
        assert_eq!(landed.cell().row(), 1);
        assert_eq!(landed.cell().column(), 0);
    }
}
