use std::{collections::HashSet, hash::Hash};

/// How many passphrases hold no two words with the same `key`.
///
/// `HashSet::insert` reports a duplicate, and `all` stops at the first, so a
/// repeated word ends the line rather than the scan.
pub fn count_valid<'a, K: Eq + Hash>(
    passphrases: &'a [String],
    key: impl Fn(&'a str) -> K,
) -> usize {
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
pub fn sorted_letters(word: &str) -> Vec<u8> {
    let mut letters = word.as_bytes().to_vec();
    letters.sort_unstable();
    letters
}
