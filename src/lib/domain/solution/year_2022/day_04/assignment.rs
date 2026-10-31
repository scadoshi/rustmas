//! Section assignments and the pairs of elves they come in.

use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

/// Why a pair line or one of its ranges failed to parse.
#[derive(Debug, Error)]
pub(super) enum InvalidAssignment {
    /// A range has no `-`.
    #[error("expected a range like `2-4`")]
    MissingDash,
    /// A range starts after it ends, like `4-2`.
    #[error("expected the range to run upward")]
    Backwards,
    /// The line has no `,` between the two ranges.
    #[error("expected two ranges separated by a comma")]
    MissingComma,
    /// A section ID isn't an unsigned number.
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// The sections one elf is assigned, both ends included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Assignment {
    /// The first section ID. Never above `end`.
    start: u32,
    /// The last section ID.
    end: u32,
}

impl FromStr for Assignment {
    type Err = InvalidAssignment;

    /// Parses `2-4`.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (start, end) = value
            .trim()
            .split_once('-')
            .ok_or(InvalidAssignment::MissingDash)?;
        let (start, end) = (start.parse()?, end.parse()?);
        if start > end {
            return Err(InvalidAssignment::Backwards);
        }
        Ok(Self { start, end })
    }
}

impl Assignment {
    /// Whether every section of `other` is also in this one.
    fn contains(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Whether the two share at least one section.
    fn overlaps(self, other: Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

/// Two elves' assignments, as one line of the input.
#[derive(Debug, Clone, Copy)]
pub(super) struct Pair(Assignment, Assignment);

impl FromStr for Pair {
    type Err = InvalidAssignment;

    /// Parses `2-4,6-8`.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (first, second) = value
            .trim()
            .split_once(',')
            .ok_or(InvalidAssignment::MissingComma)?;
        Ok(Self(first.parse()?, second.parse()?))
    }
}

impl Pair {
    /// Whether one elf's assignment is entirely inside the other's.
    pub(super) fn one_contains_the_other(self) -> bool {
        self.0.contains(self.1) || self.1.contains(self.0)
    }

    /// Whether the two assignments share at least one section, a single shared end included.
    pub(super) fn overlap(self) -> bool {
        self.0.overlaps(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(line: &str) -> Pair {
        line.parse().unwrap()
    }

    #[test]
    fn pair_from_str_err() {
        assert!(matches!(
            "2-4 6-8".parse::<Pair>(),
            Err(InvalidAssignment::MissingComma)
        ));
        assert!(matches!(
            "24,6-8".parse::<Pair>(),
            Err(InvalidAssignment::MissingDash)
        ));
        assert!(matches!(
            "4-2,6-8".parse::<Pair>(),
            Err(InvalidAssignment::Backwards)
        ));
        assert!(matches!(
            "2-x,6-8".parse::<Pair>(),
            Err(InvalidAssignment::ParseInt(_))
        ));
    }

    #[test]
    fn containment_works_in_either_direction() {
        assert!(pair("2-8,3-7").one_contains_the_other());
        assert!(pair("3-7,2-8").one_contains_the_other());
        assert!(pair("6-6,4-6").one_contains_the_other());
        assert!(!pair("2-4,6-8").one_contains_the_other());
    }

    /// Ranges that only touch at one section still overlap.
    #[test]
    fn overlap_includes_a_shared_edge() {
        assert!(pair("5-7,7-9").overlap());
        assert!(pair("2-6,4-8").overlap());
        assert!(!pair("2-4,6-8").overlap());
        assert!(!pair("2-3,4-5").overlap());
    }
}
