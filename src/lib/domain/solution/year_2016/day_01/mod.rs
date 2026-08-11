use crate::domain::solution::{
    Solution,
    answer::Answer,
    year_2016::{instruction::Instructions, pose::Pose},
};

pub struct Puzzle {
    input: String,
    /// Parsed here so a bad input fails before either part runs.
    instructions: Instructions,
}

impl Solution for Puzzle {
    fn new(input: impl Into<String>) -> anyhow::Result<Self> {
        let input = input.into();
        let instructions = Instructions::try_from(input.as_str())?;
        Ok(Self {
            input,
            instructions,
        })
    }

    fn input(&self) -> &str {
        &self.input
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
        Ok(Answer::None)
    }
}
