pub mod spiral;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::point::Point,
    year_2017::day_03::spiral::{neighbors, spiral},
};
use std::collections::HashMap;

pub struct Puzzle {
    target: usize,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            target: input.as_ref().trim().parse()?,
        })
    }

    /// Walk the spiral to the target square and measure it back to the center.
    fn part_one(&self) -> anyhow::Result<Answer> {
        match spiral().nth(self.target.saturating_sub(1)) {
            Some(point) => Ok(Answer::solved(point.distance_from_origin().to_string())),
            None => Ok(Answer::None),
        }
    }

    /// Each square holds the sum of its filled neighbors, so walk until one
    /// passes the target.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut values: HashMap<Point, usize> = HashMap::from([(Point::default(), 1)]);
        for point in spiral().skip(1) {
            let value: usize = neighbors(point)
                .filter_map(|neighbor| values.get(&neighbor))
                .sum();
            if value > self.target {
                return Ok(Answer::solved(value.to_string()));
            }
            values.insert(point, value);
        }
        Ok(Answer::None)
    }
}
