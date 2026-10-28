use std::collections::HashSet;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;

#[derive(Debug)]
pub struct Puzzle {
    nums: Vec<i32>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let nums = parse::lines(input.as_ref())?;
        Ok(Self { nums })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let total = self
            .nums
            .iter()
            .try_fold(0_i32, |total, &num| total.checked_add(num))
            .ok_or_else(|| anyhow!("the frequency passes the range of an i32"))?;
        Ok(Answer::solved(total.to_string()))
    }

    /// The first frequency reached twice, looping over the changes as often
    /// as it takes. [`Answer::None`] when no frequency ever repeats.
    fn part_two(&self) -> anyhow::Result<Answer> {
        if !self.ever_repeats()? {
            return Ok(Answer::None);
        }
        let mut current = 0_i32;
        let mut seen = HashSet::<i32>::from([current]);
        for num in self.nums.iter().cycle() {
            current = current
                .checked_add(*num)
                .ok_or_else(|| anyhow!("the frequency passes the range of an i32"))?;
            if !seen.insert(current) {
                return Ok(Answer::solved(current.to_string()));
            }
        }
        Ok(Answer::None)
    }
}

impl Puzzle {
    /// Whether looping over the changes ever reaches a frequency twice.
    ///
    /// Each pass shifts every frequency by the pass's total, so the frequency
    /// before change `r` on pass `k` is `prefix[r] + k * total`. With a total of
    /// zero the first pass ends back at 0. Otherwise two of those frequencies
    /// can only meet when their prefixes leave the same remainder after
    /// dividing by the total, and any such pair does meet on some later pass.
    ///
    /// # Errors
    ///
    /// When the prefix sums pass the range of an `i64`.
    fn ever_repeats(&self) -> anyhow::Result<bool> {
        let overflow = || anyhow!("the changes add up past the range of an i64");
        let mut prefixes = Vec::with_capacity(self.nums.len());
        let mut prefix = 0_i64;
        for &num in &self.nums {
            prefixes.push(prefix);
            prefix = prefix.checked_add(i64::from(num)).ok_or_else(overflow)?;
        }
        let total = prefix;
        if total == 0 {
            return Ok(true);
        }
        let mut remainders = HashSet::with_capacity(prefixes.len());
        for prefix in prefixes {
            let remainder = prefix.checked_rem_euclid(total).ok_or_else(overflow)?;
            if !remainders.insert(remainder) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn puzzle(changes: &str) -> Puzzle {
        Puzzle::new(changes.replace(", ", "\n")).unwrap()
    }

    #[test]
    fn part_one_examples() {
        assert_eq!(
            puzzle("+1, -2, +3, +1").part_one().unwrap(),
            Answer::solved("3")
        );
        assert_eq!(
            puzzle("-1, -2, -3").part_one().unwrap(),
            Answer::solved("-6")
        );
    }

    #[test]
    fn part_two_examples() {
        for (changes, first_repeat) in [
            ("+1, -2, +3, +1", "2"),
            ("+1, -1", "0"),
            ("+3, +3, +4, -2, -4", "10"),
            ("-6, +3, +8, +5, -6", "5"),
            ("+7, +7, -2, -7, -4", "14"),
        ] {
            assert_eq!(
                puzzle(changes).part_two().unwrap(),
                Answer::solved(first_repeat),
                "{changes}"
            );
        }
    }

    /// Climbing by one forever never comes back to a frequency.
    #[test]
    fn changes_that_never_repeat_have_no_answer() {
        assert_eq!(puzzle("+1").part_two().unwrap(), Answer::None);
        assert_eq!(puzzle("+2, +4").part_two().unwrap(), Answer::None);
    }

    #[test]
    fn a_frequency_past_i32_is_an_error() {
        assert!(puzzle("+2147483647, +1").part_one().is_err());
        assert!(puzzle("+2147483647, +1, -2147483648").part_two().is_err());
    }
}
