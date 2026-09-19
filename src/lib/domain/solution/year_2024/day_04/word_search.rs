use crate::domain::solution::common::{
    cell::Cell,
    grid::{
        FixedWidth, Grid,
        rectangle::{InvalidRectangle, Rectangle},
    },
};
use std::str::FromStr;

/// The eight directions a word can run in, as (column, row) steps.
const DIRECTIONS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

/// The grid of letters the words hide in.
pub struct WordSearch(Rectangle<char>);

impl FromStr for WordSearch {
    type Err = InvalidRectangle;

    /// Parses one row of letters per line, ignoring blank lines.
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

impl WordSearch {
    /// The letter `steps` cells from `cell` along `(dc, dr)`, if on the grid.
    pub fn letter(&self, cell: Cell, (dc, dr): (isize, isize), steps: usize) -> Option<char> {
        let steps = isize::try_from(steps).ok()?;
        let column = cell.column().checked_add_signed(dc * steps)?;
        let row = cell.row().checked_add_signed(dr * steps)?;
        self.0.get_at(&Cell::new(column, row)).copied()
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.0.height())
            .flat_map(move |row| (0..self.0.width()).map(move |column| Cell::new(column, row)))
    }

    /// How many times XMAS reads outward from `cell`, in any direction.
    pub fn xmas_from(&self, cell: Cell) -> usize {
        DIRECTIONS
            .iter()
            .filter(|&&dir| {
                "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(i, wanted)| self.letter(cell, dir, i) == Some(wanted))
            })
            .count()
    }

    /// Whether `cell` is the A of two crossing MAS, either way round.
    pub fn x_mas_at(&self, cell: Cell) -> bool {
        self.letter(cell, (0, 0), 0) == Some('A')
            && [((-1, -1), (1, 1)), ((1, -1), (-1, 1))]
                .iter()
                .all(|&(a, b)| {
                    matches!(
                        (self.letter(cell, a, 1), self.letter(cell, b, 1)),
                        (Some('M'), Some('S')) | (Some('S'), Some('M'))
                    )
                })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_stops_at_the_edge() {
        let grid: WordSearch = "AB\nCD".parse().unwrap();
        assert_eq!(grid.letter(Cell::new(0, 0), (1, 1), 1), Some('D'));
        assert_eq!(grid.letter(Cell::new(0, 0), (-1, 0), 1), None);
        assert_eq!(grid.letter(Cell::new(1, 1), (1, 0), 1), None);
    }
}
