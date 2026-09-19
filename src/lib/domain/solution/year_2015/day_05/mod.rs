use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    lines: Vec<String>,
}

/// Pairs that disqualify a line under the first rules.
const NAUGHTY: [&[u8]; 4] = [b"ab", b"cd", b"pq", b"xy"];

/// Three vowels, a doubled letter, and none of [`NAUGHTY`].
fn is_nice(line: &[u8]) -> bool {
    let vowels = line.iter().filter(|b| b"aeiou".contains(b)).count();
    let doubled = line.windows(2).any(|w| w[0] == w[1]);
    let clean = !line.windows(2).any(|w| NAUGHTY.contains(&w));
    vowels >= 3 && doubled && clean
}

/// A pair repeated later in the line, and a letter repeated one apart.
///
/// Searching from `i + 2` is what keeps the two pairs from overlapping, so
/// `aaa` holds one pair rather than two.
fn is_nicer(line: &[u8]) -> bool {
    let repeated_pair = line
        .windows(2)
        .enumerate()
        .any(|(i, pair)| line[i + 2..].windows(2).any(|other| other == pair));
    let sandwiched = line.windows(3).any(|w| w[0] == w[2]);
    repeated_pair && sandwiched
}

impl Puzzle {
    /// How many lines `rule` calls nice.
    fn count(&self, rule: impl Fn(&[u8]) -> bool) -> String {
        self.lines
            .iter()
            .filter(|line| rule(line.as_bytes()))
            .count()
            .to_string()
    }
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            lines: input.as_ref().lines().map(|l| l.to_owned()).collect(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(is_nice)))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(self.count(is_nicer)))
    }
}
