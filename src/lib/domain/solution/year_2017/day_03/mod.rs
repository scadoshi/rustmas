use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::{direction::Direction, point::Point},
};
use std::collections::HashMap;

pub struct Puzzle {
    target: usize,
}

/// The spiral's squares in order, starting at the origin.
///
/// Runs go right, up, left, down, and every second turn lengthens the run by
/// one, which is what makes the path spiral outward rather than circle.
fn spiral() -> impl Iterator<Item = Point> {
    let mut point = Point::default();
    let mut direction = Direction::Right;
    let mut run = 1;
    let mut left_in_run = 1;
    let mut turns = 0;
    std::iter::once(point).chain(std::iter::from_fn(move || {
        point = point.saturating_moved(direction, 1);
        left_in_run -= 1;
        if left_in_run == 0 {
            direction = direction.turn_left();
            turns += 1;
            if turns % 2 == 0 {
                run += 1;
            }
            left_in_run = run;
        }
        Some(point)
    }))
}

/// The eight squares touching `point`, corners included.
fn neighbours(point: Point) -> impl Iterator<Item = Point> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1)
            .filter(move |dy| dx != 0 || *dy != 0)
            .map(move |dy| Point::new(point.x + dx, point.y + dy))
    })
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            target: input.as_ref().trim().parse()?,
        })
    }

    /// Walk the spiral to the target square and measure it back to the centre.
    fn part_one(&self) -> anyhow::Result<Answer> {
        match spiral().nth(self.target.saturating_sub(1)) {
            Some(point) => Ok(Answer::solved(point.distance_from_origin().to_string())),
            None => Ok(Answer::None),
        }
    }

    /// Each square holds the sum of its filled neighbours, so walk until one
    /// passes the target.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut values: HashMap<Point, usize> = HashMap::from([(Point::default(), 1)]);
        for point in spiral().skip(1) {
            let value: usize = neighbours(point)
                .filter_map(|neighbour| values.get(&neighbour))
                .sum();
            if value > self.target {
                return Ok(Answer::solved(value.to_string()));
            }
            values.insert(point, value);
        }
        Ok(Answer::None)
    }
}
