use std::collections::HashSet;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

pub struct Puzzle {
    /// Box ids, each only `a-z` and all the same length, checked when parsed.
    input: Vec<String>,
}

/// How many times each lowercase letter appears in `line`.
///
/// Index `0` is `a`, `1` is `b`, and so on. Anything outside `a-z` is not counted.
fn letter_counts(line: &str) -> [usize; 26] {
    let mut counts = [0; 26];
    for b in line.bytes() {
        if let Some(count) = b
            .checked_sub(b'a')
            .and_then(|i| counts.get_mut(usize::from(i)))
        {
            *count += 1;
        }
    }
    counts
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let input: Vec<String> = input.as_ref().lines().map(str::to_owned).collect();
        let len = input.first().map_or(0, String::len);
        for line in &input {
            if let Some(other) = line.chars().find(|c| !c.is_ascii_lowercase()) {
                return Err(anyhow!("expected only lowercase letters, read {other:?}"));
            }
            if line.len() != len {
                return Err(anyhow!(
                    "expected every box id to be {len} letters, read {line:?}"
                ));
            }
        }
        Ok(Self { input })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let (doubles, triples) = self.input.iter().fold((0, 0), |(d, t), line| {
            let counts = letter_counts(line);
            (
                d + usize::from(counts.contains(&2)),
                t + usize::from(counts.contains(&3)),
            )
        });
        Ok(Answer::solved((doubles * triples).to_string()))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        let Some(len) = self.input.first().map(std::string::String::len) else {
            return Err(anyhow!("no input lines"));
        };
        for i in 0..len {
            let mut seen: HashSet<(&str, &str)> = HashSet::new();
            for line in &self.input {
                let (Some(head), Some(tail)) = (line.get(..i), line.get(i + 1..)) else {
                    return Err(anyhow!("box id {line:?} is shorter than the first"));
                };
                let halves = (head, tail);
                if !seen.insert(halves) {
                    return Ok(Answer::solved(format!("{}{}", halves.0, halves.1)));
                }
            }
        }
        Err(anyhow!("no two lines differ by one character"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let puzzle = Puzzle::new("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("12"));
    }

    #[test]
    fn part_two_example() {
        let puzzle = Puzzle::new("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz").unwrap();
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("fgij"));
    }

    /// Anything outside `a-z` would index past the letter counts.
    #[test]
    fn a_box_id_outside_lowercase_letters_is_rejected() {
        assert!(Puzzle::new("abc\naBc").is_err());
        assert!(Puzzle::new("abc\na\u{e9}").is_err());
    }

    /// Slicing a shorter line at a longer line's positions would panic.
    #[test]
    fn box_ids_of_different_lengths_are_rejected() {
        assert!(Puzzle::new("abcde\nab").is_err());
    }
}
