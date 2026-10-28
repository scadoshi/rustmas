use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::cell::Cell;
use crate::domain::solution::common::grid::rectangle::Rectangle;
use crate::domain::solution::common::grid::{FixedWidth, Grid};

/// The slopes part two multiplies together, as (right, down).
const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

#[derive(Debug)]
pub struct Puzzle {
    map: Rectangle<char>,
}

impl Puzzle {
    /// Trees hit going `right` and `down` from the top-left until the bottom.
    ///
    /// The map repeats to the right forever, so the column wraps.
    fn trees_on_slope(&self, right: usize, down: usize) -> usize {
        let rows = (0..self.map.height()).step_by(down);
        let columns = (0..self.map.width()).cycle().step_by(right);
        rows.zip(columns)
            .filter(|&(row, column)| self.map.at(&Cell::new(column, row)) == Some(&'#'))
            .count()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            map: Rectangle::new(
                input
                    .as_ref()
                    .lines()
                    .map(|line| line.chars().collect())
                    .collect(),
            )?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.trees_on_slope(3, 1).to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let product = SLOPES.iter().try_fold(1_u64, |product, &(right, down)| {
            let trees = u64::try_from(self.trees_on_slope(right, down))?;
            product
                .checked_mul(trees)
                .context("the product of the trees passes u64")
        })?;
        Ok(Answer::solved(product.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    /// The puzzle's example for both parts.
    #[test]
    fn example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("7"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("336"));
    }
}
