use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("expected lowercase words separated by spaces, read {0:?}")]
pub(super) struct InvalidPassphrase(char);

/// One line of the input: its words, each only `a-z`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Passphrase(Vec<String>);

impl FromStr for Passphrase {
    type Err = InvalidPassphrase;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(other) = value
            .chars()
            .find(|c| !c.is_ascii_lowercase() && !c.is_whitespace())
        {
            return Err(InvalidPassphrase(other));
        }
        Ok(Self(value.split_whitespace().map(str::to_owned).collect()))
    }
}

/// How many passphrases hold no two words with the same `key`.
///
/// `HashSet::insert` reports a duplicate, and `all` stops at the first, so a
/// repeated word ends the line rather than the scan.
pub(super) fn count_valid<'a, K: Eq + Hash>(
    passphrases: &'a [Passphrase],
    key: impl Fn(&'a str) -> K,
) -> usize {
    passphrases
        .iter()
        .filter(|passphrase| {
            let mut seen = HashSet::new();
            passphrase.0.iter().all(|word| seen.insert(key(word)))
        })
        .count()
}

/// A word's letters in order, which two anagrams share and nothing else does.
pub(super) fn sorted_letters(word: &str) -> Vec<u8> {
    let mut letters = word.as_bytes().to_vec();
    letters.sort_unstable();
    letters
}
