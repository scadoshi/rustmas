//! A position on a grid that moves around it without leaving it.

pub mod cell_on_grid;

use crate::domain::solution::common::{
    cell::Cell,
    direction::Direction,
    grid::{FixedWidth, Grid, VariableWidth},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("expected a cell on the grid")]
pub struct CellOutOfBounds;

/// A [`Cell`] paired with the grid it indexes, always in bounds.
pub trait Cursor<'a, G: Grid + 'a> {
    /// `Err` if `cell` is not on `grid`.
    fn new(cell: Cell, grid: &'a G) -> Result<Self, CellOutOfBounds>
    where
        Self: Sized;
    /// Skips the bounds check. The caller guarantees `cell` is on `grid`.
    fn new_unchecked(cell: Cell, grid: &'a G) -> Self;
    fn cell(&self) -> Cell;
    fn grid(&self) -> &'a G;

    /// Moves `distance` in `direction`, `None` if that would leave the grid.
    fn checked_moved(self, direction: Direction, distance: usize) -> Option<Self>
    where
        Self: Sized,
    {
        self.cell()
            .checked_moved(direction, distance)
            .filter(|c| self.grid().contains_cell(c))
            .map(|c| Self::new_unchecked(c, self.grid()))
    }

    /// The item under the cursor.
    fn value(&self) -> &'a G::Item {
        self.grid()
            .get_at(&self.cell())
            .expect("a cursor is always in bounds")
    }
}

/// Moving over a grid whose rows are all one width.
pub trait FixedWidthGridCursor<'a, F: FixedWidth + Grid + 'a>: Cursor<'a, F> {
    /// Moves `distance` in `direction`, clamping at the edges of the grid.
    fn saturating_moved(self, direction: Direction, distance: usize) -> Self
    where
        Self: Sized,
    {
        let cell = {
            let unclamped = self.cell().saturating_moved(direction, distance);
            let width_clamp = self.grid().width().saturating_sub(1);
            let height_clamp = self.grid().height().saturating_sub(1);
            unclamped
                .with_column(unclamped.column().min(width_clamp))
                .with_row(unclamped.row().min(height_clamp))
        };
        Self::new_unchecked(cell, self.grid())
    }
}

/// Moving over a grid whose rows are each their own width.
pub trait VariableWidthGridCursor<'a, V: VariableWidth + Grid + 'a>: Cursor<'a, V> {
    /// Moves `distance` in `direction`, clamping at the edges of the grid.
    ///
    /// The row clamps first, since which row you land on decides how wide it is.
    fn saturating_moved(self, direction: Direction, distance: usize) -> Self
    where
        Self: Sized,
    {
        let cell = {
            let unclamped = self.cell().saturating_moved(direction, distance);
            let height_clamp = self.grid().height().saturating_sub(1);
            let clamped_row = unclamped.row().min(height_clamp);
            let width_clamp = self
                .grid()
                .width_of(clamped_row)
                .map_or(0, |n| n.saturating_sub(1));
            unclamped
                .with_column(unclamped.column().min(width_clamp))
                .with_row(clamped_row)
        };
        Self::new_unchecked(cell, self.grid())
    }
}
