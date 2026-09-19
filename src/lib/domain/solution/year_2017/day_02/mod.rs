use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    input: Vec<Vec<u32>>,
}

/// `a` over `b` or `b` over `a`, whichever divides evenly, `None` if neither.
///
/// Panics on a zero, which no row holds.
fn even_quotient(a: u32, b: u32) -> Option<u32> {
    if a.is_multiple_of(b) {
        Some(a / b)
    } else if b.is_multiple_of(a) {
        Some(b / a)
    } else {
        None
    }
}

impl Puzzle {
    /// Sums what each row reduces to, skipping rows that reduce to nothing.
    fn checksum(&self, row: impl Fn(&[u32]) -> Option<u32>) -> String {
        self.input
            .iter()
            .filter_map(|nums| row(nums))
            .sum::<u32>()
            .to_string()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input
                .as_ref()
                .trim()
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .map(|str| str.trim().parse::<u32>())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.checksum(|nums| {
            nums.iter()
                .fold(None::<(u32, u32)>, |acc, &n| {
                    Some(match acc {
                        None => (n, n),
                        Some((lo, hi)) => (lo.min(n), hi.max(n)),
                    })
                })
                .map(|(lo, hi)| hi - lo)
        })))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.checksum(|nums| {
            nums.iter()
                .enumerate()
                .find_map(|(i, &n1)| nums[i + 1..].iter().find_map(|&n2| even_quotient(n1, n2)))
        })))
    }
}
