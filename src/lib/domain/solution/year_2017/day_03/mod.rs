mod spiral;

use std::collections::HashMap;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::point::Point;

#[derive(Debug)]
pub(crate) struct Puzzle {
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
        let index = self.target.checked_sub(1).context("squares start at 1")?;
        let point = spiral::spiral()
            .nth(index)
            .context("the spiral leaves the i32 grid before that square")?;
        Ok(Answer::solved(point.distance_from_origin().to_string()))
    }

    /// Each square holds the sum of its filled neighbors, so walk until one
    /// passes the target.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut values: HashMap<Point, usize> = HashMap::from([(Point::default(), 1)]);
        for point in spiral::spiral().skip(1) {
            let value = spiral::neighbors(point)
                .filter_map(|neighbor| values.get(&neighbor))
                .try_fold(0_usize, |total, &value| total.checked_add(value))
                .context("a square's neighbors add up past usize")?;
            if value > self.target {
                return Ok(Answer::solved(value.to_string()));
            }
            values.insert(point, value);
        }
        Ok(Answer::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The puzzle's examples for part one.
    #[test]
    fn part_one_examples() {
        for (square, steps) in [("1", "0"), ("12", "3"), ("23", "2"), ("1024", "31")] {
            let puzzle = Puzzle::new(square).unwrap();
            assert_eq!(puzzle.part_one().unwrap(), Answer::solved(steps));
        }
    }

    /// The first values written past the example sequence's targets.
    #[test]
    fn part_two_examples() {
        for (target, value) in [("1", "2"), ("5", "10"), ("747", "806")] {
            let puzzle = Puzzle::new(target).unwrap();
            assert_eq!(puzzle.part_two().unwrap(), Answer::solved(value));
        }
    }

    #[test]
    fn square_zero_is_an_error() {
        assert!(Puzzle::new("0").unwrap().part_one().is_err());
    }
}
