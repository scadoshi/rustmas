mod instruction;
mod pose;

use std::collections::HashSet;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::point::Point;
use crate::domain::solution::year_2016::day_01::instruction::Instructions;
use crate::domain::solution::year_2016::day_01::pose::Pose;

#[derive(Debug)]
pub(crate) struct Puzzle {
    /// Parsed here so a bad input fails before either part runs.
    instructions: Instructions,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            instructions: input.as_ref().parse()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let mut pose = Pose::default();
        for i in self.instructions.iter() {
            pose = pose.turned(i.turn).saturating_moved(i.distance);
        }
        Ok(Answer::solved(
            pose.position.distance_from_origin().to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut pose = Pose::default();
        let mut visited = HashSet::<Point>::new();
        for i in self.instructions.iter() {
            pose = pose.turned(i.turn);
            for _ in 0..i.distance {
                if !visited.insert(pose.position) {
                    return Ok(Answer::solved(
                        pose.position.distance_from_origin().to_string(),
                    ));
                }
                pose = pose.saturating_moved(1);
            }
        }
        Ok(Answer::None)
    }
}
