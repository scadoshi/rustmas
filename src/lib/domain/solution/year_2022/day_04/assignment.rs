use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidAssignment {
    #[error("expected a range like `2-4`")]
    MissingDash,
    #[error("expected the range to run upward")]
    Backwards,
    #[error("expected two ranges separated by a comma")]
    MissingComma,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// The sections one elf is assigned, both ends included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Assignment {
    start: u32,
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
    pub fn contains(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Whether the two share at least one section.
    pub fn overlaps(self, other: Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

/// Two elves' assignments, as one line of the input.
#[derive(Debug, Clone, Copy)]
pub struct Pair(pub Assignment, pub Assignment);

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
    pub fn one_contains_the_other(self) -> bool {
        self.0.contains(self.1) || self.1.contains(self.0)
    }

    pub fn overlap(self) -> bool {
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
