pub mod word_search;

use crate::domain::solution::{
    Solution, answer::Answer, year_2024::day_04::word_search::WordSearch,
};

pub struct Puzzle {
    grid: WordSearch,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            grid: input.as_ref().parse()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.grid
                .cells()
                .map(|cell| self.grid.xmas_from(cell))
                .sum::<usize>()
                .to_string(),
        ))
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
