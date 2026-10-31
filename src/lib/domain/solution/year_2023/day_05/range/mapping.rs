//! One mapping line: a source range and the destination it shifts to.

use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

use thiserror::Error;

use crate::domain::solution::common::range::ext::RangeExt;

/// Why a mapping line does not read as a destination, a source and a length.
#[derive(Debug, Error)]
pub(in super::super) enum InvalidRangeMapping {
    /// Fewer than three numbers on the line.
    #[error("too few parts")]
    TooFewParts,
    /// More than three numbers on the line.
    #[error("too many parts")]
    TooManyParts,
    /// A number that is not whole or does not fit a [`u64`].
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    /// The source or destination range ends past [`u64::MAX`].
    #[error("a range of {length} from {start} ends past {}", u64::MAX)]
    PastU64 {
        /// Start of the range that overflows, source or destination.
        start: u64,
        /// The mapping's length.
        length: u64,
    },
}

/// Both ranges end at or before [`u64::MAX`], checked when parsed, so every
/// offset inside them adds without overflow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in super::super) struct RangeMapping {
    /// Where the first source number lands.
    destination: u64,
    /// The first number the mapping covers.
    source: u64,
    /// How many numbers the mapping covers, the same for both ranges.
    length: u64,
    /// `source + length`, kept so the source range needs no addition.
    source_end: u64,
}

impl FromStr for RangeMapping {
    type Err = InvalidRangeMapping;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (Some(destination), Some(source), Some(length)) =
            (parts.next(), parts.next(), parts.next())
        else {
            return Err(InvalidRangeMapping::TooFewParts);
        };
        if parts.next().is_some() {
            return Err(InvalidRangeMapping::TooManyParts);
        }
        let (destination, source, length): (u64, u64, u64) =
            (destination.parse()?, source.parse()?, length.parse()?);
        let past_u64 = |start: u64| InvalidRangeMapping::PastU64 { start, length };
        let source_end = source.checked_add(length).ok_or_else(|| past_u64(source))?;
        destination
            .checked_add(length)
            .ok_or_else(|| past_u64(destination))?;
        Ok(Self {
            destination,
            source,
            length,
            source_end,
        })
    }
}

impl RangeMapping {
    #[cfg(test)]
    pub(super) fn new(destination: u64, source: u64, length: u64) -> Self {
        Self {
            destination,
            source,
            length,
            source_end: source.checked_add(length).unwrap(),
        }
    }

    /// The numbers this mapping covers, end exclusive.
    pub(super) fn source_range(self) -> Range<u64> {
        self.source..self.source_end
    }

    /// Where `seed` lands, or `None` if this mapping does not cover it.
    pub(super) fn map(&self, seed: u64) -> Option<u64> {
        if self.source_range().contains(&seed) {
            self.shifted(seed)
        } else {
            None
        }
    }

    /// Where a point from the start to the end of the source range lands.
    ///
    /// The parse check keeps this `Some` for every such point.
    fn shifted(&self, point: u64) -> Option<u64> {
        self.destination
            .checked_add(point.checked_sub(self.source)?)
    }

    /// The part of `range` this mapping covers, shifted to the destination, and the pieces of `range` outside it. `None` when they do not overlap.
    pub(super) fn map_range(
        self,
        range: Range<u64>,
    ) -> Option<(Range<u64>, impl Iterator<Item = Range<u64>>)> {
        let overlap = range.clone().intersection(self.source_range())?;
        let mapped = self.shifted(overlap.start)?..self.shifted(overlap.end)?;
        Some((mapped, range.difference(self.source_range())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_new() {
        let rm = RangeMapping::new(50, 98, 2);
        assert_eq!(rm.destination, 50);
        assert_eq!(rm.source, 98);
        assert_eq!(rm.length, 2);
    }

    #[test]
    fn range_mapping_from_str_ok() {
        assert_eq!(
            RangeMapping::from_str("50 98 2").unwrap(),
            RangeMapping::new(50, 98, 2)
        );
    }

    #[test]
    fn range_mapping_from_str_err() {
        assert!(matches!(
            RangeMapping::from_str("50 98"),
            Err(InvalidRangeMapping::TooFewParts)
        ));
        assert!(matches!(
            RangeMapping::from_str("50 98 2 0"),
            Err(InvalidRangeMapping::TooManyParts)
        ));
        assert!(matches!(
            RangeMapping::from_str("foo 98 2"),
            Err(InvalidRangeMapping::ParseInt(_))
        ));
    }

    /// A range that ends past [`u64::MAX`] cannot be added up to.
    #[test]
    fn range_mapping_from_str_rejects_a_range_past_u64() {
        assert!(RangeMapping::from_str("0 18446744073709551615 2").is_err());
        assert!(RangeMapping::from_str("18446744073709551615 0 2").is_err());
        assert!(RangeMapping::from_str("18446744073709551613 18446744073709551613 2").is_ok());
    }

    #[test]
    fn range_mapping_map() {
        assert_eq!(RangeMapping::new(50, 98, 2).map(99).unwrap(), 51);
        assert_eq!(RangeMapping::new(50, 98, 2).map(100), None);
    }

    #[test]
    fn range_mapping_map_range() {
        let result = RangeMapping::new(50, 98, 2).map_range(90..110);
        assert!(result.is_some());
        let Some((mapped, leftovers)) = result else {
            panic!();
        };
        assert_eq!(mapped, 50..52);
        assert_eq!(
            leftovers.collect::<Vec<Range<u64>>>(),
            vec![90..98, 100..110]
        );
    }
}
