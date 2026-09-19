use crate::domain::solution::{Solution, answer::Answer};
use anyhow::anyhow;

pub struct Puzzle {
    readings: Vec<u32>,
    /// How many bits each reading has, taken from the input rather than assumed.
    width: u32,
}

/// Which bit value to keep when the readings are split on one position.
#[derive(Debug, Clone, Copy)]
enum Keep {
    /// The commoner bit, ones winning a tie.
    MostCommon,
    /// The rarer bit, zeros winning a tie.
    LeastCommon,
}

/// The bit `keep` picks at position `bit` across `readings`.
fn bit_to_keep(readings: &[u32], bit: u32, keep: Keep) -> u32 {
    let ones = readings.iter().filter(|r| *r >> bit & 1 == 1).count();
    let ones_lead = ones * 2 >= readings.len();
    match keep {
        Keep::MostCommon => u32::from(ones_lead),
        Keep::LeastCommon => u32::from(!ones_lead),
    }
}

/// A number built from the bit `keep` picks at every position.
fn rate(readings: &[u32], width: u32, keep: Keep) -> u32 {
    (0..width).fold(0, |rate, bit| {
        rate | bit_to_keep(readings, bit, keep) << bit
    })
}

/// Narrows the readings one bit at a time, top down, until one is left.
fn rating_by_filter(readings: &[u32], width: u32, keep: Keep) -> Option<u32> {
    let mut left = readings.to_vec();
    for bit in (0..width).rev() {
        if left.len() <= 1 {
            break;
        }
        let wanted = bit_to_keep(&left, bit, keep);
        left.retain(|r| r >> bit & 1 == wanted);
    }
    match left.as_slice() {
        [only] => Some(*only),
        _ => None,
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let lines: Vec<&str> = input.as_ref().lines().collect();
        let width = lines.first().map_or(0, |line| line.len());
        if lines.iter().any(|line| line.len() != width) {
            return Err(anyhow!("expected every line the same width"));
        }
        Ok(Self {
            readings: lines
                .iter()
                .map(|line| u32::from_str_radix(line, 2))
                .collect::<Result<_, _>>()?,
            width: u32::try_from(width)?,
        })
    }

    /// Gamma is the commonest bit everywhere, epsilon the rarest.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let gamma = rate(&self.readings, self.width, Keep::MostCommon);
        let epsilon = rate(&self.readings, self.width, Keep::LeastCommon);
        Ok(Answer::solved(
            (u64::from(gamma) * u64::from(epsilon)).to_string(),
        ))
    }

    /// Oxygen keeps the commonest bit at each step, CO2 the rarest.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let oxygen = rating_by_filter(&self.readings, self.width, Keep::MostCommon);
        let co2 = rating_by_filter(&self.readings, self.width, Keep::LeastCommon);
        match (oxygen, co2) {
            (Some(o), Some(c)) => Ok(Answer::solved((u64::from(o) * u64::from(c)).to_string())),
            _ => Ok(Answer::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [u32; 12] = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    /// The puzzle's example, which exercises the tie rules in part two.
    #[test]
    fn the_example_rates() {
        assert_eq!(rate(&EXAMPLE, 5, Keep::MostCommon), 22);
        assert_eq!(rate(&EXAMPLE, 5, Keep::LeastCommon), 9);
        assert_eq!(rating_by_filter(&EXAMPLE, 5, Keep::MostCommon), Some(23));
        assert_eq!(rating_by_filter(&EXAMPLE, 5, Keep::LeastCommon), Some(10));
    }

    #[test]
    fn a_tie_goes_to_ones_for_most_and_zeros_for_least() {
        let tied = [0b1, 0b0];
        assert_eq!(bit_to_keep(&tied, 0, Keep::MostCommon), 1);
        assert_eq!(bit_to_keep(&tied, 0, Keep::LeastCommon), 0);
    }
}
