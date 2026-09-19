use crate::domain::solution::common::{
    cell::Cell,
    grid::{
        FixedWidth, Grid,
        rectangle::{InvalidRectangle, Rectangle},
    },
};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidBoard {
    #[error(transparent)]
    Shape(#[from] InvalidRectangle),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// A bingo board, where a marked number is simply gone.
///
/// Keeping `Option<u8>` rather than a parallel grid of flags means a row or
/// column is complete exactly when it holds no numbers.
#[derive(Debug, Clone)]
pub struct Board(Rectangle<Option<u8>>);

impl TryFrom<&str> for Board {
    type Error = InvalidBoard;

    /// Parses a block of whitespace-separated numbers, one row per line.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rows = value
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().map(Some))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(Rectangle::new(rows)?))
    }
}

impl Board {
    /// Crosses `number` off wherever it appears.
    pub fn mark(&mut self, number: u8) {
        for slot in self.0.iter_rows_mut().flatten() {
            if *slot == Some(number) {
                *slot = None;
            }
        }
    }

    /// Whether any row or column has been marked through.
    pub fn has_bingo(&self) -> bool {
        let row_done = self
            .0
            .iter_rows()
            .any(|row| row.iter().all(Option::is_none));
        let column_done = (0..self.0.width()).any(|column| {
            (0..self.0.height()).all(|row| self.0.get_at(&Cell::new(column, row)) == Some(&None))
        });
        row_done || column_done
    }

    /// The sum of everything not yet marked.
    pub fn unmarked_sum(&self) -> u32 {
        self.0
            .iter_rows()
            .flatten()
            .flatten()
            .map(|&n| u32::from(n))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board() -> Board {
        Board::try_from("1 2 3\n4 5 6\n7 8 9").unwrap()
    }

    #[test]
    fn board_try_from_str_err() {
        assert!(matches!(
            Board::try_from("1 2\n3"),
            Err(InvalidBoard::Shape(_))
        ));
        assert!(matches!(
            Board::try_from("1 x\n3 4"),
            Err(InvalidBoard::ParseInt(_))
        ));
    }

    #[test]
    fn a_full_row_or_column_is_bingo() {
        let mut across = board();
        for n in [4, 5, 6] {
            across.mark(n);
        }
        assert!(across.has_bingo());

        let mut down = board();
        for n in [2, 5, 8] {
            down.mark(n);
        }
        assert!(down.has_bingo());

        let mut diagonal = board();
        for n in [1, 5, 9] {
            diagonal.mark(n);
        }
        assert!(!diagonal.has_bingo(), "diagonals do not count");
    }

    #[test]
    fn unmarked_sum_drops_what_was_marked() {
        let mut b = board();
        assert_eq!(b.unmarked_sum(), 45);
        b.mark(5);
        b.mark(9);
        assert_eq!(b.unmarked_sum(), 31);
    }
}
