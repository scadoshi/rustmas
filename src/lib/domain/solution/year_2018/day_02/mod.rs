use std::collections::HashSet;

use anyhow::anyhow;

use crate::domain::solution::{Solution, answer::Answer};

pub struct Puzzle {
    input: Vec<String>,
}

/// How many times each lowercase letter appears in `line`.
///
/// Index `0` is `a`, `1` is `b`, and so on. Panics on anything outside `a-z`,
/// which no puzzle line holds.
fn letter_counts(line: &str) -> [usize; 26] {
    let mut counts = [0; 26];
    for b in line.bytes() {
        counts[(b - b'a') as usize] += 1;
    }
    counts
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input.as_ref().lines().map(|l| l.to_string()).collect(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let (doubles, triples) = self.input.iter().fold((0, 0), |(d, t), line| {
            let counts = letter_counts(line);
            (
                d + counts.contains(&2) as usize,
                t + counts.contains(&3) as usize,
            )
        });
        Ok(Answer::solved((doubles * triples).to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let Some(len) = self.input.first().map(|l| l.len()) else {
            return Err(anyhow!("no input lines"));
        };
        for i in 0..len {
            let mut seen: HashSet<(&str, &str)> = HashSet::new();
            for line in self.input.iter() {
                let halves = (&line[..i], &line[i + 1..]);
                if !seen.insert(halves) {
                    return Ok(Answer::solved(format!("{}{}", halves.0, halves.1)));
                }
            }
        }
        Err(anyhow!("no two lines differ by one character"))
    }
}
