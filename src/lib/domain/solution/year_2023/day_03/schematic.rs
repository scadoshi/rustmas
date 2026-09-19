use crate::domain::solution::common::{
    cell::Cell,
    grid::{
        Grid,
        rectangle::{InvalidRectangle, Rectangle},
    },
};
use std::str::FromStr;

/// The engine schematic, one character per cell.
#[derive(Debug, Clone)]
pub struct Schematic(Rectangle<char>);

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

/// A run of digits in the schematic, and where it sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    pub value: u32,
    pub row: usize,
    /// Columns of the first and last digit, inclusive.
    pub first: usize,
    pub last: usize,
}

impl Number {
    /// Whether `cell` touches this number, corners included.
    pub fn touches(&self, cell: Cell) -> bool {
        cell.row().abs_diff(self.row) <= 1
            && cell.column() + 1 >= self.first
            && cell.column() <= self.last + 1
    }
}

/// Anything in the schematic that is neither a digit nor empty space.
pub fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

impl Schematic {
    /// Every number in the schematic, read left to right, top to bottom.
    pub fn numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        for (row, cells) in self.0.iter_rows().enumerate() {
            let mut column = 0;
            while column < cells.len() {
                if !cells[column].is_ascii_digit() {
                    column += 1;
                    continue;
                }
                let first = column;
                let mut value = 0;
                while column < cells.len() && cells[column].is_ascii_digit() {
                    value = value * 10 + cells[column].to_digit(10).unwrap_or(0);
                    column += 1;
                }
                numbers.push(Number {
                    value,
                    row,
                    first,
                    last: column - 1,
                });
            }
        }
        numbers
    }

    /// Every cell holding a character that satisfies `wanted`.
    pub fn symbols(&self, wanted: impl Fn(char) -> bool) -> Vec<Cell> {
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
        let numbers = schematic.numbers();
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
    }
}
