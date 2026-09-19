use std::collections::HashSet;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::{direction::Direction, point::Point},
};

pub struct Puzzle {
    directions: Vec<Direction>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            directions: input
                .as_ref()
                .chars()
                .map(Direction::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.directions
                .iter()
                .fold(
                    (HashSet::<Point>::from([Point::default()]), Point::default()),
                    |(mut visited, point), direction| {
                        let new_point = point.saturating_moved(*direction, 1);
                        visited.insert(new_point);
                        (visited, new_point)
                    },
                )
                .0
                .len()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.directions
                .iter()
                .fold(
                    (
                        HashSet::<Point>::from([Point::default()]),
                        Point::default(),
                        Point::default(),
                        true,
                    ),
                    |(mut visited, santa, robot, turn), direction| {
                        let santa = if turn {
                            santa.saturating_moved(*direction, 1)
                        } else {
                            santa
                        };
                        visited.insert(santa);
                        let robot = if !turn {
                            robot.saturating_moved(*direction, 1)
                        } else {
                            robot
                        };
                        visited.insert(robot);
                        (visited, santa, robot, !turn)
                    },
                )
                .0
                .len()
                .to_string(),
        ))
    }
}
