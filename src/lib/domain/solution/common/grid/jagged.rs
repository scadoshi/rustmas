use crate::domain::solution::common::{
    cell::Cell,
    grid::{Grid, VariableWidth},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidJagged {
    #[error("expected at least one row")]
    HeightIsZero,
    #[error("expected at least one cell in every row")]
    RowIsEmpty,
}

/// A grid with at least one row, each row free to be its own width.
#[derive(Debug, Clone)]
pub struct Jagged<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Jagged<T> {
    /// Checks that `inner` has a row and that no row is empty.
    pub fn new(inner: Vec<Vec<T>>) -> Result<Self, InvalidJagged> {
        if inner.is_empty() {
            return Err(InvalidJagged::HeightIsZero);
        }
        if inner.iter().any(std::vec::Vec::is_empty) {
            return Err(InvalidJagged::RowIsEmpty);
        }
        Ok(Self { inner })
    }
}

impl<T> Grid for Jagged<T> {
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
        self.inner.len()
    }

    fn iter_rows(&self) -> impl Iterator<Item = &[Self::Item]> {
        self.inner.iter().map(std::vec::Vec::as_slice)
    }

    fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [Self::Item]> {
        self.inner.iter_mut().map(std::vec::Vec::as_mut_slice)
    }
}

impl<T> VariableWidth for Jagged<T> {
    fn width_of(&self, row: usize) -> Option<usize> {
        self.inner.get(row).map(std::vec::Vec::len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Jagged<char> {
        Jagged::new(vec![vec!['a', 'b', 'c'], vec!['d'], vec!['e', 'f']]).unwrap()
    }

    #[test]
    fn jagged_new_rejects_a_grid_with_no_rows() {
        assert!(matches!(
            Jagged::<char>::new(vec![]),
            Err(InvalidJagged::HeightIsZero)
        ));
    }

    #[test]
    fn jagged_new_rejects_a_row_with_no_cells() {
        assert!(matches!(
            Jagged::new(vec![vec!['a'], vec![]]),
            Err(InvalidJagged::RowIsEmpty)
        ));
    }

    #[test]
    fn jagged_new_keeps_rows_of_differing_width() {
        let grid = grid();
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.width_of(0), Some(3));
        assert_eq!(grid.width_of(1), Some(1));
    }

    #[test]
    fn jagged_width_of_past_the_bottom_is_none() {
        assert_eq!(grid().width_of(3), None);
    }

    #[test]
    fn jagged_get_at_respects_each_row_width() {
        assert_eq!(grid().get_at(&Cell::new(2, 0)), Some(&'c'));
        assert!(grid().get_at(&Cell::new(2, 1)).is_none());
        assert!(!grid().contains_cell(&Cell::new(2, 1)));
    }
}
