use crate::domain::solution::common::{
    cell::Cell,
    grid::{
        FixedWidth, Grid,
        rectangle::{InvalidRectangle, Rectangle},
    },
};
use std::ops::Deref;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidSquare {
    #[error("expected at least one row and one column")]
    DimensionIsZero,
    #[error("expected as many rows as columns")]
    WidthAndHeightInequal,
    #[error("expected every row the same width")]
    VariableWidth,
}

impl From<InvalidRectangle> for InvalidSquare {
    fn from(value: InvalidRectangle) -> Self {
        match value {
            InvalidRectangle::WidthIsZero | InvalidRectangle::HeightIsZero => Self::DimensionIsZero,
            InvalidRectangle::VariableWidth => Self::VariableWidth,
        }
    }
}

/// A [`Rectangle`] whose width and height are equal.
#[derive(Debug, Clone)]
pub struct Square<T>(Rectangle<T>);

impl<T> Deref for Square<T> {
    type Target = Rectangle<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Square<T> {
    /// Checks that `inner` is as wide as it is tall, then defers to
    /// [`Rectangle::new`] for the rest.
    pub fn new(inner: Vec<Vec<T>>) -> Result<Self, InvalidSquare> {
        if inner.len() != inner.first().ok_or(InvalidSquare::DimensionIsZero)?.len() {
            return Err(InvalidSquare::WidthAndHeightInequal);
        }
        Ok(Self(Rectangle::new(inner)?))
    }
}

impl<T> Grid for Square<T> {
    type Item = T;

    fn get_at(&self, cell: &Cell) -> Option<&Self::Item> {
        self.0.get_at(cell)
    }

    fn get_mut_at(&mut self, cell: &Cell) -> Option<&mut Self::Item> {
        self.0.get_mut_at(cell)
    }

    fn contains_cell(&self, cell: &Cell) -> bool {
        self.0.contains_cell(cell)
    }

    fn height(&self) -> usize {
        self.0.height()
    }

    fn iter_rows(&self) -> impl Iterator<Item = &[Self::Item]> {
        self.0.iter_rows()
    }

    fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [Self::Item]> {
        self.0.iter_rows_mut()
    }
}

impl<T> FixedWidth for Square<T> {
    fn width(&self) -> usize {
        self.0.width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Square<char> {
        Square::new(vec![vec!['a', 'b'], vec!['c', 'd']]).unwrap()
    }

    #[test]
    fn square_new_rejects_a_grid_with_no_rows() {
        assert!(matches!(
            Square::<char>::new(vec![]),
            Err(InvalidSquare::DimensionIsZero)
        ));
    }

    #[test]
    fn square_new_rejects_a_rectangle_that_is_not_square() {
        assert!(matches!(
            Square::new(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]),
            Err(InvalidSquare::WidthAndHeightInequal)
        ));
    }

    #[test]
    fn square_new_rejects_rows_of_differing_width() {
        assert!(matches!(
            Square::new(vec![vec!['a', 'b'], vec!['c']]),
            Err(InvalidSquare::VariableWidth)
        ));
    }

    #[test]
    fn square_reports_one_dimension_for_both() {
        assert_eq!(grid().width(), 2);
        assert_eq!(grid().height(), 2);
    }

    #[test]
    fn square_reads_through_to_the_rectangle() {
        assert_eq!(grid().get_at(&Cell::new(1, 0)), Some(&'b'));
        assert!(grid().get_at(&Cell::new(2, 0)).is_none());
    }
}
