use anyhow::Context;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub struct Puzzle {
    digits: Vec<u32>,
}

impl Puzzle {
    /// The sum of every digit that matches the one `step` places ahead,
    /// wrapping around the end.
    fn matching(&self, step: usize) -> anyhow::Result<u32> {
        let ahead = self.digits.iter().cycle().skip(step);
        self.digits
            .iter()
            .zip(ahead)
            .filter(|(digit, other)| digit == other)
            .try_fold(0_u32, |total, (&digit, _)| total.checked_add(digit))
            .context("the matching digits add up past u32")
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let digits = input
            .as_ref()
            .trim()
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .with_context(|| format!("expected only digits, read {c:?}"))
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(Self { digits })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.matching(1)?.to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.matching(self.digits.len() / 2)?.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn answers(input: &str) -> (Answer, Answer) {
        let puzzle = Puzzle::new(input).unwrap();
        (puzzle.part_one().unwrap(), puzzle.part_two().unwrap())
    }

    /// The puzzle's examples for both parts.
    #[test]
    fn examples() {
        assert_eq!(answers("1122").0, Answer::solved("3"));
        assert_eq!(answers("1111").0, Answer::solved("4"));
        assert_eq!(answers("1234").0, Answer::solved("0"));
        assert_eq!(answers("91212129").0, Answer::solved("9"));
        assert_eq!(answers("1212").1, Answer::solved("6"));
        assert_eq!(answers("1221").1, Answer::solved("0"));
        assert_eq!(answers("123425").1, Answer::solved("4"));
        assert_eq!(answers("123123").1, Answer::solved("12"));
        assert_eq!(answers("12131415").1, Answer::solved("4"));
    }

    #[test]
    fn a_trailing_newline_is_trimmed() {
        assert_eq!(answers("1122\n").0, Answer::solved("3"));
    }

    #[test]
    fn anything_but_digits_is_rejected() {
        assert!(Puzzle::new("11a22").is_err());
        assert!(Puzzle::new("11 22").is_err());
    }
}
