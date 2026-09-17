//! Grids of cells, split by whether every row is the same width.

pub mod jagged;
pub mod rectangle;
pub mod square;

use crate::domain::solution::common::cell::Cell;

/// Reading and iterating a grid by [`Cell`], whatever shape it is.
pub trait Grid {
    type Item;

    /// The item at `cell`, `None` if it is off the grid.
    fn get_at(&self, cell: &Cell) -> Option<&Self::Item>;
    /// As [`get_at`](Grid::get_at), mutably.
    fn get_mut_at(&mut self, cell: &Cell) -> Option<&mut Self::Item>;

    /// Whether any cell holds `value`, scanning row by row.
    fn contains(&self, value: &Self::Item) -> bool
    where
        Self::Item: PartialEq,
    {
        self.iter_rows().flatten().any(|v| v == value)
    }

    /// Whether `cell` is on the grid.
    fn contains_cell(&self, cell: &Cell) -> bool;

    fn height(&self) -> usize;

    /// Each row as a slice, top to bottom.
    fn iter_rows(&self) -> impl Iterator<Item = &[Self::Item]>;
    fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [Self::Item]>;
}

/// A grid whose rows are all the same width.
pub trait FixedWidth {
    fn width(&self) -> usize;
}

/// A grid whose rows may each be a different width.
pub trait VariableWidth {
    /// The width of `row`, `None` if it is past the bottom of the grid.
    fn width_of(&self, row: usize) -> Option<usize>;
}
