//! The almanac's `seeds:` line, read as single seeds or as ranges of them.

use std::num::ParseIntError;
use std::ops::{Deref, Range};
use std::str::FromStr;

use thiserror::Error;

/// Why the first line does not read as seeds.
#[derive(Debug, Error)]
pub(super) enum InvalidSeeds {
    /// The line does not start with `seeds: `.
    #[error("expected the almanac to open with `seeds: `")]
    MissingPrefix,
    /// `seeds:` with no numbers after it.
    #[error("expected at least one seed")]
    Empty,
    /// A seed that is not whole or does not fit a [`u64`].
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// Why the seeds do not pair up into ranges for part two.
#[derive(Debug, Error)]
pub(super) enum InvalidSeedRanges {
    /// The last start has no length after it.
    #[error("expected seeds in start and length pairs, read an odd count")]
    OddCount,
    /// A range ends past [`u64::MAX`].
    #[error("a seed range of {len} from {start} ends past {}", u64::MAX)]
    PastU64 {
        /// The range's first seed.
        start: u64,
        /// How many seeds the range holds.
        len: u64,
    },
}

/// The numbers on the `seeds:` line, in order. Never empty.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Seeds(Vec<u64>);

impl Deref for Seeds {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Seeds {
    type Err = InvalidSeeds;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds = s
            .strip_prefix("seeds: ")
            .ok_or(InvalidSeeds::MissingPrefix)?
            .split_whitespace()
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        if seeds.is_empty() {
            return Err(InvalidSeeds::Empty);
        }
        Ok(Self(seeds))
    }
}

impl Seeds {
    /// The seeds read as `start length` pairs, for part two.
    ///
    /// # Errors
    ///
    /// [`InvalidSeedRanges::OddCount`] when the seeds do not pair up, and [`InvalidSeedRanges::PastU64`] when a range ends past [`u64::MAX`].
    pub(super) fn to_ranges(&self) -> Result<Vec<Range<u64>>, InvalidSeedRanges> {
        self.chunks(2).try_fold(Vec::new(), |mut ranges, chunk| {
            let &[start, len] = chunk else {
                return Err(InvalidSeedRanges::OddCount);
            };
            let end = start
                .checked_add(len)
                .ok_or(InvalidSeedRanges::PastU64 { start, len })?;
            ranges.push(start..end);
            Ok(ranges)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_from_str_ok() {
        assert!(Seeds::from_str("seeds: 79 14 55 13").is_ok());
    }

    #[test]
    fn seed_from_str_err() {
        assert!(Seeds::from_str("seeds: foo 79 14 55 13").is_err());
        assert!(Seeds::from_str("foo 79 14 55 13").is_err());
        assert!(matches!(
            Seeds::from_str("seeds: "),
            Err(InvalidSeeds::Empty)
        ));
    }

    #[test]
    fn seed_to_ranges_ok() {
        assert_eq!(
            Seeds::from_str("seeds: 79 14 55 13")
                .unwrap()
                .to_ranges()
                .unwrap(),
            vec![(79..79 + 14), (55..55 + 13)]
        );
    }

    #[test]
    fn seed_to_ranges_err() {
        assert!(
            Seeds::from_str("seeds: 79 14 55")
                .unwrap()
                .to_ranges()
                .is_err()
        );
    }

    #[test]
    fn seed_to_ranges_rejects_a_range_past_u64() {
        assert!(
            Seeds::from_str("seeds: 18446744073709551615 2")
                .unwrap()
                .to_ranges()
                .is_err()
        );
    }
}
