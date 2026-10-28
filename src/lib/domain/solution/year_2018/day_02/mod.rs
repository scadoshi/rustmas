use std::collections::HashSet;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub struct Puzzle {
    /// Box ids, each only `a-z` and all the same length, checked when parsed.
    input: Vec<String>,
}

/// The letters a box id may hold, in order.
const ALPHABET: [u8; 26] = *b"abcdefghijklmnopqrstuvwxyz";

/// How many times each lowercase letter appears in `line`.
///
/// Index `0` is `a`, `1` is `b`, and so on. Anything outside `a-z` is not counted.
fn letter_counts(line: &str) -> [usize; 26] {
    ALPHABET.map(|letter| line.bytes().filter(|b| *b == letter).count())
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
        let counts: Vec<[usize; 26]> = self.input.iter().map(|line| letter_counts(line)).collect();
        let doubles = counts.iter().filter(|counts| counts.contains(&2)).count();
        let triples = counts.iter().filter(|counts| counts.contains(&3)).count();
        let checksum = doubles
            .checked_mul(triples)
            .ok_or_else(|| anyhow!("the checksum passes usize"))?;
        Ok(Answer::solved(checksum.to_string()))
    }

    /// The letters two ids share when they differ in exactly one place.
    /// [`Answer::None`] when no two ids do.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let len = self.input.first().map_or(0, String::len);
        for i in 0..len {
            let mut seen: HashSet<(&str, &str)> = HashSet::new();
            for line in &self.input {
                let (Some(head), Some(tail)) =
                    (line.get(..i), line.get(i..).and_then(|rest| rest.get(1..)))
                else {
                    return Err(anyhow!("box id {line:?} is shorter than the first"));
                };
                let halves = (head, tail);
                if !seen.insert(halves) {
                    return Ok(Answer::solved(format!("{}{}", halves.0, halves.1)));
                }
            }
        }
        Ok(Answer::None)
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

    #[test]
    fn ids_that_never_differ_by_one_letter_have_no_answer() {
        let puzzle = Puzzle::new("abc\nxyz").unwrap();
        assert_eq!(puzzle.part_two().unwrap(), Answer::None);
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
