pub mod instructions;

use crate::domain::solution::{
    Solution, answer::Answer, common::turn::Turn, year_2025::day_01::instructions::Instructions,
};

pub struct Puzzle {
    instructions: Instructions,
}

const DIAL_SIZE: i64 = 100;
const START: i32 = 50;

fn wrapped(raw: i64) -> i32 {
    let rem = raw % DIAL_SIZE;
    (if rem < 0 { DIAL_SIZE + rem } else { rem }) as i32
}

fn moved(position: i32, turn: Turn, distance: i32) -> i32 {
    let distance = i64::from(distance);
    let position = i64::from(position);
    wrapped(match turn {
        Turn::Left => position - distance,
        Turn::Right => position + distance,
    })
}

/// How many times a turn goes over zero, counted rather than walked.
///
/// The multiples of [`DIAL_SIZE`] in the swept interval. Turning left sweeps
/// `position - distance ..= position - 1`, since the position it starts on is
/// not one it passes. [`i64::div_euclid`] rather than `/` because that interval
/// runs negative and `-1 / 100` truncates to zero.
fn zeros_crossed(position: i32, turn: Turn, distance: i32) -> i64 {
    let distance = i64::from(distance);
    let position = i64::from(position);
    match turn {
        Turn::Right => (position + distance).div_euclid(DIAL_SIZE) - position.div_euclid(DIAL_SIZE),
        Turn::Left => {
            (position - 1).div_euclid(DIAL_SIZE) - (position - distance - 1).div_euclid(DIAL_SIZE)
        }
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            instructions: Instructions::try_from(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.instructions
                .iter()
                .fold((START, 0), |(position, count), instruction| {
                    let moved = moved(position, instruction.turn, instruction.distance);
                    if moved == 0 {
                        (moved, count + 1)
                    } else {
                        (moved, count)
                    }
                })
                .1
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.instructions
                .iter()
                .fold((START, 0_i64), |(position, count), instruction| {
                    (
                        moved(position, instruction.turn, instruction.distance),
                        count + zeros_crossed(position, instruction.turn, instruction.distance),
                    )
                })
                .1
                .to_string(),
        ))
    }
}
