use anyhow::{Context, anyhow};

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub struct Puzzle {
    rows: Vec<Vec<u32>>,
}

/// `a` over `b` or `b` over `a`, whichever divides evenly, `None` if neither.
///
/// A zero is evenly divided by anything else, giving `0`.
///
/// # Errors
///
/// When both are zero, which has no quotient.
fn even_quotient(a: u32, b: u32) -> anyhow::Result<Option<u32>> {
    if a == 0 && b == 0 {
        return Err(anyhow!("two zeros in one row have no quotient"));
    }
    Ok(if a.is_multiple_of(b) {
        a.checked_div(b)
    } else if b.is_multiple_of(a) {
        b.checked_div(a)
    } else {
        None
    })
}

impl Puzzle {
    /// Sums what each row reduces to, skipping rows that reduce to nothing.
    ///
    /// # Errors
    ///
    /// When a row fails to reduce, or the sum passes [`u32::MAX`].
    fn checksum(
        &self,
        row: impl Fn(&[u32]) -> anyhow::Result<Option<u32>>,
    ) -> anyhow::Result<String> {
        let mut total = 0_u32;
        for numbers in &self.rows {
            if let Some(value) = row(numbers)? {
                total = total
                    .checked_add(value)
                    .ok_or_else(|| anyhow!("the checksum passes {}", u32::MAX))?;
            }
        }
        Ok(total.to_string())
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            rows: input
                .as_ref()
                .trim()
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|number| number.trim().parse::<u32>())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.checksum(|numbers| {
            numbers
                .iter()
                .fold(None::<(u32, u32)>, |span, &n| {
                    Some(match span {
                        None => (n, n),
                        Some((lo, hi)) => (lo.min(n), hi.max(n)),
                    })
                })
                .map(|(lo, hi)| {
                    hi.checked_sub(lo)
                        .context("expected the high to be at least the low")
                })
                .transpose()
        })?))
    }

    /// The first pair in each row where one divides the other evenly.
    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.checksum(|numbers| {
            let mut rest = numbers;
            while let Some((&a, tail)) = rest.split_first() {
                for &b in tail {
                    if let Some(quotient) = even_quotient(a, b)? {
                        return Ok(Some(quotient));
                    }
                }
                rest = tail;
            }
            Ok(None)
        })?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let puzzle = Puzzle::new("5 1 9 5\n7 5 3\n2 4 6 8").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("18"));
    }

    #[test]
    fn part_two_example() {
        let puzzle = Puzzle::new("5 9 2 8\n9 4 7 3\n3 8 6 5").unwrap();
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("9"));
    }

    /// Zero over zero has no quotient to add.
    #[test]
    fn two_zeros_in_a_row_is_an_error() {
        let puzzle = Puzzle::new("0 0").unwrap();
        assert!(puzzle.part_two().is_err());
    }

    /// A single zero divides into nothing but is divided evenly by anything.
    #[test]
    fn one_zero_is_an_even_quotient_of_zero() {
        let puzzle = Puzzle::new("0 7").unwrap();
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("0"));
    }

    #[test]
    fn a_checksum_past_u32_is_an_error() {
        let puzzle = Puzzle::new(format!("0 {max}\n0 {max}", max = u32::MAX)).unwrap();
        assert!(puzzle.part_one().is_err());
    }
}
