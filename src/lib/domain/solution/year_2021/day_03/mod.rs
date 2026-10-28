pub mod rating;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::year_2021::day_03::rating::{Keep, rate, rating_by_filter};

#[derive(Debug)]
pub struct Puzzle {
    readings: Vec<u32>,
    /// How many bits each reading has, taken from the input rather than
    /// assumed, and never more than a `u32` holds, so every shift fits.
    width: u32,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let lines: Vec<&str> = input.as_ref().lines().collect();
        let width = lines.first().map_or(0, |line| line.len());
        if lines.iter().any(|line| line.len() != width) {
            return Err(anyhow!("expected every line the same width"));
        }
        let width = u32::try_from(width)?;
        if width > u32::BITS {
            return Err(anyhow!(
                "expected at most {} bits per reading, read {width}",
                u32::BITS
            ));
        }
        Ok(Self {
            readings: lines
                .iter()
                .map(|line| u32::from_str_radix(line, 2))
                .collect::<Result<_, _>>()?,
            width,
        })
    }

    /// Gamma is the commonest bit everywhere, epsilon the rarest.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let gamma = rate(&self.readings, self.width, Keep::MostCommon);
        let epsilon = rate(&self.readings, self.width, Keep::LeastCommon);
        let power = u64::from(gamma)
            .checked_mul(u64::from(epsilon))
            .ok_or_else(|| anyhow!("the power consumption passes u64"))?;
        Ok(Answer::solved(power.to_string()))
    }

    /// Oxygen keeps the commonest bit at each step, CO2 the rarest.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let oxygen = rating_by_filter(&self.readings, self.width, Keep::MostCommon);
        let co2 = rating_by_filter(&self.readings, self.width, Keep::LeastCommon);
        match (oxygen, co2) {
            (Some(oxygen), Some(co2)) => {
                let life_support = u64::from(oxygen)
                    .checked_mul(u64::from(co2))
                    .ok_or_else(|| anyhow!("the life support rating passes u64"))?;
                Ok(Answer::solved(life_support.to_string()))
            }
            _ => Ok(Answer::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 33 digits still parse into a `u32` with leading zeros, but shifting by
    /// the top position would overflow.
    #[test]
    fn readings_wider_than_a_u32_are_rejected() {
        let wide = format!("{}1", "0".repeat(32));
        assert!(Puzzle::new(format!("{wide}\n{wide}")).is_err());
    }
}
