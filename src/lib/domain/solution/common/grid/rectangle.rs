use crate::domain::solution::common::{
    cell::Cell,
    grid::{FixedWidth, Grid},
};
use thiserror::Error;

/// A grid with at least one cell, every row the same width.
#[derive(Debug, Clone)]
pub struct Rectangle<T> {
    inner: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Error)]
pub enum InvalidRectangle {
    #[error("width must be greater than 0")]
    WidthIsZero,
    #[error("height must be greater than 0")]
    HeightIsZero,
    #[error("all rows must have same width")]
    VariableWidth,
}

impl<T> Rectangle<T> {
    /// Checks that `inner` has a cell and that its rows are all one width.
    pub fn new(inner: Vec<Vec<T>>) -> Result<Self, InvalidRectangle> {
        let Some(first_row) = inner.first() else {
            return Err(InvalidRectangle::HeightIsZero);
        };
        if first_row.is_empty() {
            return Err(InvalidRectangle::WidthIsZero);
        }
        let width = first_row.len();
        let height = inner.len();
        if inner.iter().any(|r| r.len() != width) {
            return Err(InvalidRectangle::VariableWidth);
        }
        Ok(Self {
            inner,
            width,
            height,
        })
    }
}

impl<T> FixedWidth for Rectangle<T> {
    fn width(&self) -> usize {
        self.width
    }
}

impl<T> Grid for Rectangle<T> {
    type Item = T;

    fn get_at(&self, cell: &Cell) -> Option<&Self::Item> {
        self.inner
            .get(cell.row())
            .and_then(|r| r.get(cell.column()))
    }

    fn get_mut_at(&mut self, cell: &Cell) -> Option<&mut Self::Item> {
        self.inner
            .get_mut(cell.row())
            .and_then(|r| r.get_mut(cell.column()))
    }

    fn contains_cell(&self, cell: &Cell) -> bool {
        self.get_at(cell).is_some()
    }

    fn height(&self) -> usize {
        self.height
    }

    fn iter_rows(&self) -> impl Iterator<Item = &[Self::Item]> {
        self.inner.iter().map(|v| v.as_slice())
    }

    fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [Self::Item]> {
        self.inner.iter_mut().map(|v| v.as_mut_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Rectangle<char> {
        Rectangle::new(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]).unwrap()
    }

    #[test]
    fn rectangle_new_rejects_a_grid_with_no_rows() {
        assert!(matches!(
            Rectangle::<char>::new(vec![]),
            Err(InvalidRectangle::HeightIsZero)
        ));
    }

    #[test]
    fn rectangle_new_rejects_a_row_with_no_cells() {
        assert!(matches!(
            Rectangle::<char>::new(vec![vec![]]),
            Err(InvalidRectangle::WidthIsZero)
        ));
    }

    #[test]
    fn rectangle_new_rejects_rows_of_differing_width() {
        assert!(matches!(
            Rectangle::new(vec![vec!['a', 'b'], vec!['c']]),
            Err(InvalidRectangle::VariableWidth)
        ));
    }

    #[test]
    fn rectangle_reports_its_shape() {
        assert_eq!(grid().width(), 3);
        assert_eq!(grid().height(), 2);
    }

    #[test]
    fn rectangle_get_at_reads_column_then_row() {
        assert_eq!(grid().get_at(&Cell::new(2, 0)), Some(&'c'));
        assert_eq!(grid().get_at(&Cell::new(0, 1)), Some(&'d'));
    }

    #[test]
    fn rectangle_get_at_off_the_grid_is_none() {
        assert!(grid().get_at(&Cell::new(3, 0)).is_none());
        assert!(grid().get_at(&Cell::new(0, 2)).is_none());
        assert!(!grid().contains_cell(&Cell::new(3, 0)));
    }

    #[test]
    fn rectangle_get_mut_at_writes_through() {
        let mut grid = grid();
        *grid.get_mut_at(&Cell::new(1, 1)).unwrap() = 'z';
        assert_eq!(grid.get_at(&Cell::new(1, 1)), Some(&'z'));
    }

    #[test]
    fn rectangle_iterates_rows_top_to_bottom() {
        let grid = grid();
        let rows: Vec<&[char]> = grid.iter_rows().collect();
        assert_eq!(rows, vec![['a', 'b', 'c'].as_slice(), &['d', 'e', 'f']]);
    }

    #[test]
    fn rectangle_contains_scans_every_row() {
        assert!(grid().contains(&'f'));
        assert!(!grid().contains(&'z'));
    }
}
