//! Day 4: Ceres Search. Part one counts XMAS in all eight directions from every cell. Part two counts each A at the center of two crossing MAS.

mod word_search;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::year_2024::day_04::word_search::WordSearch;

/// The elf's word search.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One letter per cell, the input's lines as rows.
    grid: WordSearch,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            grid: input.as_ref().parse()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let total = self
            .grid
            .cells()
            .try_fold(0_usize, |total, cell| {
                total.checked_add(self.grid.xmas_from(cell))
            })
            .context("the XMAS count passes usize")?;
        Ok(Answer::solved(total.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.grid
                .cells()
                .filter(|&cell| self.grid.x_mas_at(cell))
                .count()
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    /// The puzzle's example: 18 XMAS and 9 X-MAS.
    #[test]
    fn the_example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap().to_string(), "18");
        assert_eq!(puzzle.part_two().unwrap().to_string(), "9");
    }
}
