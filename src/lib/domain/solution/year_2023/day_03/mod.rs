use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::{
        cell::Cell,
        grid::{Grid, rectangle::Rectangle},
    },
};

pub struct Puzzle {
    schematic: Rectangle<char>,
}

/// A run of digits in the schematic, and where it sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number {
    value: u32,
    row: usize,
    /// Columns of the first and last digit, inclusive.
    first: usize,
    last: usize,
}

impl Number {
    /// Whether `cell` touches this number, corners included.
    fn touches(&self, cell: Cell) -> bool {
        cell.row().abs_diff(self.row) <= 1
            && cell.column() + 1 >= self.first
            && cell.column() <= self.last + 1
    }
}

/// Anything in the schematic that is neither a digit nor empty space.
fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

impl Puzzle {
    /// Every number in the schematic, read left to right, top to bottom.
    fn numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        for (row, cells) in self.schematic.iter_rows().enumerate() {
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
    fn symbols(&self, wanted: impl Fn(char) -> bool) -> Vec<Cell> {
        let mut cells = Vec::new();
        for (row, chars) in self.schematic.iter_rows().enumerate() {
            for (column, &c) in chars.iter().enumerate() {
                if wanted(c) {
                    cells.push(Cell::new(column, row));
                }
            }
        }
        cells
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            schematic: Rectangle::new(
                input
                    .as_ref()
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.trim().chars().collect())
                    .collect(),
            )?,
        })
    }

    /// A number is a part number if any symbol touches it.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let symbols = self.symbols(is_symbol);
        Ok(Answer::solved(
            self.numbers()
                .iter()
                .filter(|number| symbols.iter().any(|&cell| number.touches(cell)))
                .map(|number| number.value)
                .sum::<u32>()
                .to_string(),
        ))
    }

    /// A `*` touching exactly two numbers is a gear; its ratio is their product.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let numbers = self.numbers();
        Ok(Answer::solved(
            self.symbols(|c| c == '*')
                .into_iter()
                .filter_map(|star| {
                    let touching: Vec<u64> = numbers
                        .iter()
                        .filter(|number| number.touches(star))
                        .map(|number| u64::from(number.value))
                        .collect();
                    match touching.as_slice() {
                        [a, b] => Some(a * b),
                        _ => None,
                    }
                })
                .sum::<u64>()
                .to_string(),
        ))
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
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        let numbers = puzzle.numbers();
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

    #[test]
    fn the_example_gives_4361_and_467835() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap().to_string(), "4361");
        assert_eq!(puzzle.part_two().unwrap().to_string(), "467835");
    }
}
