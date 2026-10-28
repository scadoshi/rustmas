pub mod instructions;

use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::turn::Turn;
use crate::domain::solution::year_2025::day_01::instructions::Instructions;

#[derive(Debug)]
pub struct Puzzle {
    instructions: Instructions,
}

const DIAL_SIZE: i64 = 100;
const START: i32 = 50;

/// Where the dial points after `turn` by `distance` from `position`, always
/// in `0..DIAL_SIZE`. `None` only if the arithmetic leaves [`i64`], which two
/// widened [`i32`] values cannot.
fn moved(position: i32, turn: Turn, distance: i32) -> Option<i32> {
    let distance = i64::from(distance);
    let position = i64::from(position);
    let raw = match turn {
        Turn::Left => position.checked_sub(distance)?,
        Turn::Right => position.checked_add(distance)?,
    };
    i32::try_from(raw.rem_euclid(DIAL_SIZE)).ok()
}

/// How many times a turn goes over zero, counted rather than walked.
///
/// The multiples of [`DIAL_SIZE`] in the swept interval. [`i64::div_euclid`]
/// rather than `/`, since that interval runs negative and `-1 / 100` truncates
/// to zero.
fn zeros_crossed(position: i32, turn: Turn, distance: i32) -> Option<i64> {
    let distance = i64::from(distance);
    let position = i64::from(position);
    let (high, low) = match turn {
        Turn::Right => (position.checked_add(distance)?, position),
        Turn::Left => {
            let high = position.checked_sub(1)?;
            (high, high.checked_sub(distance)?)
        }
    };
    high.div_euclid(DIAL_SIZE)
        .checked_sub(low.div_euclid(DIAL_SIZE))
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            instructions: input.as_ref().parse()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let (_, count) = self
            .instructions
            .iter()
            .try_fold((START, 0_usize), |(position, count), instruction| {
                let moved = moved(position, instruction.turn, instruction.distance)?;
                let count = if moved == 0 {
                    count.checked_add(1)?
                } else {
                    count
                };
                Some((moved, count))
            })
            .context("the dial count passes its range")?;
        Ok(Answer::solved(count.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let (_, count) = self
            .instructions
            .iter()
            .try_fold((START, 0_i64), |(position, count), instruction| {
                let crossed = zeros_crossed(position, instruction.turn, instruction.distance)?;
                Some((
                    moved(position, instruction.turn, instruction.distance)?,
                    count.checked_add(crossed)?,
                ))
            })
            .context("the zero count passes its range")?;
        Ok(Answer::solved(count.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn the_example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("3"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("6"));
    }

    /// Landing on zero counts, leaving it does not, and every full turn
    /// passes it once.
    #[test]
    fn zeros_crossed_counts_each_click_on_zero() {
        assert_eq!(zeros_crossed(50, Turn::Right, 50), Some(1));
        assert_eq!(zeros_crossed(50, Turn::Left, 50), Some(1));
        assert_eq!(zeros_crossed(0, Turn::Left, 5), Some(0));
        assert_eq!(zeros_crossed(0, Turn::Right, 5), Some(0));
        assert_eq!(zeros_crossed(0, Turn::Left, 100), Some(1));
        assert_eq!(zeros_crossed(50, Turn::Right, 1000), Some(10));
        assert_eq!(zeros_crossed(50, Turn::Left, 150), Some(2));
        assert_eq!(zeros_crossed(50, Turn::Left, 0), Some(0));
    }

    /// A negative distance would turn the other way and count zeros backwards.
    #[test]
    fn a_negative_distance_is_rejected() {
        assert!(Puzzle::new("R-55").is_err());
        assert!(Puzzle::new("L-5").is_err());
    }
}
