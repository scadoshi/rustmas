//! Report Repair: find the expense entries that sum to 2020 and multiply them, with a hash lookup for the pair and two pointers over a sorted list for the triple.

use std::cmp::Ordering;
use std::collections::HashMap;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;

/// The sum both parts look for.
const TARGET: i32 = 2020;

/// Two entries at different positions that sum to `target`, the first found in input order.
///
/// The difference is taken in [`i64`], so an entry near [`i32::MIN`] or [`i32::MAX`] can't overflow it. Equal values at two positions count as a pair.
fn two_sum(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let map = nums
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect::<HashMap<i32, usize>>();
    for (i, &x) in nums.iter().enumerate() {
        let Some(y) = i64::from(target)
            .checked_sub(i64::from(x))
            .and_then(|y| i32::try_from(y).ok())
        else {
            continue;
        };
        if map.get(&y).is_some_and(|j| i != *j) {
            return Some((x, y));
        }
    }
    None
}

/// Three entries at different positions that sum to `target`, or `None`.
///
/// Sorts a copy, then for each entry walks two pointers inward over the rest, so it runs in O(n²) without a set. The sum is taken in [`i64`].
fn three_sum(nums: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    let mut nums = nums.to_owned();
    nums.sort_unstable();
    let target = i64::from(target);
    let mut rest = nums.as_slice();
    while let [x, tail @ ..] = rest {
        let mut window = tail;
        while let [lo, .., hi] = window {
            let sum: i64 = [*x, *lo, *hi].into_iter().map(i64::from).sum();
            window = match sum.cmp(&target) {
                Ordering::Equal => return Some((*x, *lo, *hi)),
                Ordering::Less => window.split_first().map_or(&[], |(_, rest)| rest),
                Ordering::Greater => window.split_last().map_or(&[], |(_, rest)| rest),
            };
        }
        rest = tail;
    }
    None
}

/// The expense report.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// One entry per line, in input order, duplicates kept.
    nums: Vec<i32>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let nums: Vec<i32> = parse::lines(input.as_ref())?;
        Ok(Self { nums })
    }

    /// The product of the two entries that sum to 2020, taken in [`i64`],
    /// where two [`i32`] entries always fit.
    fn part_one(&self) -> anyhow::Result<Answer> {
        match two_sum(&self.nums, TARGET) {
            Some((x, y)) => {
                let product = i64::from(x)
                    .checked_mul(i64::from(y))
                    .ok_or_else(|| anyhow!("the product passes the range of an i64"))?;
                Ok(Answer::solved(product.to_string()))
            }
            None => Ok(Answer::None),
        }
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        match three_sum(&self.nums, TARGET) {
            Some((x, y, z)) => {
                let product = i64::from(x)
                    .checked_mul(i64::from(y))
                    .and_then(|product| product.checked_mul(i64::from(z)))
                    .ok_or_else(|| anyhow!("the product passes the range of an i64"))?;
                Ok(Answer::solved(product.to_string()))
            }
            None => Ok(Answer::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        let puzzle = Puzzle::new("1721\n979\n366\n299\n675\n1456").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("514579"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("241861950"));
    }

    /// `2020 - i32::MIN` and sums of large entries leave the `i32` range.
    #[test]
    fn entries_at_the_i32_extremes_do_not_overflow() {
        let puzzle = Puzzle::new("-2147483648\n2147483647\n2147483647\n1\n2019").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("2019"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::None);
    }

    /// The pair sums to 2020 but its product is far past an `i32`.
    #[test]
    fn a_large_product_is_exact() {
        let puzzle = Puzzle::new("-1000000000\n1000002020").unwrap();
        assert_eq!(
            puzzle.part_one().unwrap(),
            Answer::solved("-1000002020000000000")
        );
    }

    /// The three sum to 2020 but their product is past an `i64`.
    #[test]
    fn a_product_past_i64_is_an_error() {
        let puzzle = Puzzle::new("-2000000000\n2000000000\n2020").unwrap();
        assert!(puzzle.part_two().is_err());
    }
}
