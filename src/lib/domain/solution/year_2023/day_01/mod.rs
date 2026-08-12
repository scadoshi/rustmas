use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    input: Vec<String>,
}

trait Calibration {
    fn calibration_value(&self) -> Option<u32>;
    fn calibration_value_with_words(&self) -> Option<u32>;
}

impl<S: AsRef<str>> Calibration for S {
    fn calibration_value(&self) -> Option<u32> {
        let mut digits = self.as_ref().chars().filter_map(|c| c.to_digit(10));
        let first = digits.next()?;
        Some(first * 10 + digits.next_back().unwrap_or(first))
    }

    fn calibration_value_with_words(&self) -> Option<u32> {
        todo!()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input
                .as_ref()
                .trim()
                .lines()
                .map(|s| s.to_owned())
                .collect(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.input
                .iter()
                .filter_map(|s| s.calibration_value())
                .sum::<u32>()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Unwritten)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_value() {
        assert_eq!("foo6bar7baz".calibration_value(), Some(67));
        assert_eq!("".calibration_value(), None);
    }
}
