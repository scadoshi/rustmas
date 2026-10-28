use std::str::FromStr;

use thiserror::Error;

use crate::domain::solution::common::cell::Cell;
use crate::domain::solution::common::grid::Grid;
use crate::domain::solution::common::grid::rectangle::{InvalidRectangle, Rectangle};

/// The engine schematic, one character per cell.
#[derive(Debug, Clone)]
pub(super) struct Schematic(Rectangle<char>);

impl FromStr for Schematic {
    type Err = InvalidRectangle;

    /// Parses one row per non-blank line.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(Rectangle::new(
            value
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().chars().collect())
                .collect(),
        )?))
    }
}

/// A run of digits too long for a [`u32`], counted from 0 at the top-left.
#[derive(Debug, Error)]
#[error(
    "expected every number to fit in a u32, the one reaching row {row} column {column} does not"
)]
pub(super) struct NumberTooLarge {
    row: usize,
    column: usize,
}

/// A run of digits in the schematic, and where it sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Number {
    pub(super) value: u32,
    row: usize,
    /// Columns of the first and last digit, inclusive.
    first: usize,
    last: usize,
}

impl Number {
    /// Whether `cell` touches this number, corners included.
    pub(super) fn touches(&self, cell: Cell) -> bool {
        let column = cell.column();
        let over = (self.first..=self.last).contains(&column);
        let beside = column.abs_diff(self.first) <= 1 || column.abs_diff(self.last) <= 1;
        cell.row().abs_diff(self.row) <= 1 && (over || beside)
    }
}

/// Anything in the schematic that is neither a digit nor empty space.
pub(super) fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

impl Schematic {
    /// Every number in the schematic, read left to right, top to bottom.
    ///
    /// # Errors
    ///
    /// [`NumberTooLarge`] if a run of digits passes [`u32`].
    pub(super) fn numbers(&self) -> Result<Vec<Number>, NumberTooLarge> {
        let mut numbers = Vec::new();
        for (row, cells) in self.0.iter_rows().enumerate() {
            let mut current: Option<Number> = None;
            for (column, c) in cells.iter().enumerate() {
                match (c.to_digit(10), current.as_mut()) {
                    (Some(digit), Some(number)) => {
                        number.value = number
                            .value
                            .checked_mul(10)
                            .and_then(|value| value.checked_add(digit))
                            .ok_or(NumberTooLarge { row, column })?;
                        number.last = column;
                    }
                    (Some(digit), None) => {
                        current = Some(Number {
                            value: digit,
                            row,
                            first: column,
                            last: column,
                        });
                    }
                    (None, _) => numbers.extend(current.take()),
                }
            }
            numbers.extend(current);
        }
        Ok(numbers)
    }

    /// Every cell holding a character that satisfies `wanted`.
    pub(super) fn symbols(&self, wanted: impl Fn(char) -> bool) -> Vec<Cell> {
        let mut cells = Vec::new();
        for (row, chars) in self.0.iter_rows().enumerate() {
            for (column, &c) in chars.iter().enumerate() {
                if wanted(c) {
                    cells.push(Cell::new(column, row));
                }
            }
        }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's example schematic.
    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn numbers_are_read_with_their_columns() {
        let schematic: Schematic = EXAMPLE.parse().unwrap();
        let numbers = schematic.numbers().unwrap();
        assert_eq!(numbers.len(), 10);
        assert_eq!(
            numbers[0],
            Number {
                value: 467,
                row: 0,
                first: 0,
                last: 2
            }
        );
        assert_eq!(
            numbers[1],
            Number {
                value: 114,
                row: 0,
                first: 5,
                last: 7
            }
        );
    }

    #[test]
    fn touching_includes_the_diagonal_and_the_ends() {
        let n = Number {
            value: 1,
            row: 5,
            first: 3,
            last: 4,
        };
        assert!(n.touches(Cell::new(2, 4)), "diagonal before");
        assert!(n.touches(Cell::new(5, 6)), "diagonal after");
        assert!(n.touches(Cell::new(2, 5)), "just left");
        assert!(!n.touches(Cell::new(1, 5)), "two left");
        assert!(!n.touches(Cell::new(3, 7)), "two rows down");
        assert!(n.touches(Cell::new(5, 5)), "just right");
        assert!(!n.touches(Cell::new(6, 5)), "two right");
        assert!(n.touches(Cell::new(3, 6)), "under");
    }

    #[test]
    fn a_number_past_u32_is_an_error() {
        let schematic: Schematic = "*4294967296".parse().unwrap();
        assert!(schematic.numbers().is_err());
        let schematic: Schematic = "*4294967295".parse().unwrap();
        assert_eq!(schematic.numbers().unwrap()[0].value, u32::MAX);
    }
}
