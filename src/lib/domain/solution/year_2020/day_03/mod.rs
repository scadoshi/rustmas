use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::{
        cell::Cell,
        grid::{FixedWidth, Grid, rectangle::Rectangle},
    },
};

/// The slopes part two multiplies together, as (right, down).
const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

pub struct Puzzle {
    map: Rectangle<char>,
}

impl Puzzle {
    /// Trees hit going `right` and `down` from the top-left until the bottom.
    ///
    /// The map repeats to the right forever, so the column wraps.
    fn trees_on_slope(&self, right: usize, down: usize) -> usize {
        let width = self.map.width();
        (0..self.map.height())
            .step_by(down)
            .enumerate()
            .filter(|&(step, row)| {
                self.map.get_at(&Cell::new(step * right % width, row)) == Some(&'#')
            })
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
        Ok(Answer::solved(
            SLOPES
                .iter()
                .map(|&(right, down)| self.trees_on_slope(right, down) as u64)
                .product::<u64>()
                .to_string(),
        ))
    }
}
