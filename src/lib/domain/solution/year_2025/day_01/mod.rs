pub mod instructions;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::turn::Turn;
use crate::domain::solution::year_2025::day_01::instructions::Instructions;

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
/// The multiples of [`DIAL_SIZE`] in the swept interval. [`i64::div_euclid`]
/// rather than `/`, since that interval runs negative and `-1 / 100` truncates
/// to zero.
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
            instructions: input.as_ref().parse()?,
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
        assert_eq!(zeros_crossed(50, Turn::Right, 50), 1);
        assert_eq!(zeros_crossed(50, Turn::Left, 50), 1);
        assert_eq!(zeros_crossed(0, Turn::Left, 5), 0);
        assert_eq!(zeros_crossed(0, Turn::Right, 5), 0);
        assert_eq!(zeros_crossed(0, Turn::Left, 100), 1);
        assert_eq!(zeros_crossed(50, Turn::Right, 1000), 10);
        assert_eq!(zeros_crossed(50, Turn::Left, 150), 2);
        assert_eq!(zeros_crossed(50, Turn::Left, 0), 0);
    }

    /// A negative distance would turn the other way and count zeros backwards.
    #[test]
    fn a_negative_distance_is_rejected() {
        assert!(Puzzle::new("R-55").is_err());
        assert!(Puzzle::new("L-5").is_err());
    }
}
