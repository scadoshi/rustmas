//! One bingo board and the marking and scoring on it.

use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::domain::solution::common::cell::Cell;
use crate::domain::solution::common::grid::rectangle::{InvalidRectangle, Rectangle};
use crate::domain::solution::common::grid::{FixedWidth, Grid};

/// Why a board block failed to parse.
#[derive(Debug, Error)]
pub(super) enum InvalidBoard {
    /// The rows aren't all the same length, or there are none.
    #[error(transparent)]
    Shape(#[from] InvalidRectangle),
    /// A cell isn't a number from 0 to 255.
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// A bingo board, where a marked number is simply gone.
///
/// Keeping `Option<u8>` rather than a parallel grid of flags means a row or
/// column is complete exactly when it holds no numbers.
#[derive(Debug, Clone)]
pub(super) struct Board(Rectangle<Option<u8>>);

impl FromStr for Board {
    type Err = InvalidBoard;

    /// Parses a block of whitespace-separated numbers, one row per line.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
    pub(super) fn mark(&mut self, number: u8) {
        for slot in self.0.iter_rows_mut().flatten() {
            if *slot == Some(number) {
                *slot = None;
            }
        }
    }

    /// Whether any row or column has been marked through.
    pub(super) fn has_bingo(&self) -> bool {
        let row_done = self
            .0
            .iter_rows()
            .any(|row| row.iter().all(Option::is_none));
        let column_done = (0..self.0.width()).any(|column| {
            (0..self.0.height()).all(|row| self.0.at(&Cell::new(column, row)) == Some(&None))
        });
        row_done || column_done
    }

    /// The sum of everything not yet marked, or `None` past [`u32::MAX`].
    pub(super) fn unmarked_sum(&self) -> Option<u32> {
        self.0
            .iter_rows()
            .flatten()
            .flatten()
            .try_fold(0_u32, |total, &n| total.checked_add(u32::from(n)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board() -> Board {
        "1 2 3\n4 5 6\n7 8 9".parse::<Board>().unwrap()
    }

    #[test]
    fn board_from_str_err() {
        assert!(matches!(
            "1 2\n3".parse::<Board>(),
            Err(InvalidBoard::Shape(_))
        ));
        assert!(matches!(
            "1 x\n3 4".parse::<Board>(),
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
        assert_eq!(b.unmarked_sum(), Some(45));
        b.mark(5);
        b.mark(9);
        assert_eq!(b.unmarked_sum(), Some(31));
    }
}
