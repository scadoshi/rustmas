use crate::domain::solution::{Solution, answer::Answer};
use std::{collections::HashSet, hash::Hash};

pub struct Puzzle {
    passphrases: Vec<String>,
}

/// How many passphrases hold no two words with the same `key`.
///
/// `HashSet::insert` reports a duplicate, and `all` stops at the first, so a
/// repeated word ends the line rather than the scan.
fn count_valid<'a, K: Eq + Hash>(passphrases: &'a [String], key: impl Fn(&'a str) -> K) -> usize {
    passphrases
        .iter()
        .filter(|passphrase| {
            let mut seen = HashSet::new();
            passphrase
                .split_whitespace()
                .all(|word| seen.insert(key(word)))
        })
        .count()
}

/// A word's letters in order, which two anagrams share and nothing else does.
fn sorted_letters(word: &str) -> Vec<u8> {
    let mut letters = word.as_bytes().to_vec();
    letters.sort_unstable();
    letters
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            passphrases: input.as_ref().lines().map(str::to_owned).collect(),
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            count_valid(&self.passphrases, |word| word).to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            count_valid(&self.passphrases, sorted_letters).to_string(),
        ))
    }
}
