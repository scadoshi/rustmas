use std::collections::HashSet;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::direction::Direction;
use crate::domain::solution::common::point::Point;

#[derive(Debug)]
pub(crate) struct Puzzle {
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
                        let robot = if turn {
                            robot
                        } else {
                            robot.saturating_moved(*direction, 1)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        for (input, want) in [(">", "2"), ("^>v<", "4"), ("^v^v^v^v^v", "2")] {
            let got = Puzzle::new(input).unwrap().part_one().unwrap();
            assert_eq!(got, Answer::solved(want), "input {input:?}");
        }
    }

    #[test]
    fn part_two_examples() {
        for (input, want) in [("^v", "3"), ("^>v<", "3"), ("^v^v^v^v^v", "11")] {
            let got = Puzzle::new(input).unwrap().part_two().unwrap();
            assert_eq!(got, Answer::solved(want), "input {input:?}");
        }
    }

    #[test]
    fn anything_but_arrows_is_rejected() {
        assert!(Puzzle::new("^>x").is_err());
    }
}
