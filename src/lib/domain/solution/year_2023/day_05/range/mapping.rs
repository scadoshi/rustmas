use crate::domain::solution::common::range::ext::RangeExt;
use std::{num::ParseIntError, ops::Range, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub(in super::super) enum InvalidRangeMapping {
    #[error("too few parts")]
    TooFewParts,
    #[error("too many parts")]
    TooManyParts,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in super::super) struct RangeMapping {
    destination: u64,
    source: u64,
    length: u64,
}

impl FromStr for RangeMapping {
    type Err = InvalidRangeMapping;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (Some(d), Some(s), Some(l)) = (parts.next(), parts.next(), parts.next()) else {
            return Err(InvalidRangeMapping::TooFewParts);
        };
        if parts.next().is_some() {
            return Err(InvalidRangeMapping::TooManyParts);
        }
        Ok(Self {
            destination: d.parse()?,
            source: s.parse()?,
            length: l.parse()?,
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
        }
    }

    pub(super) fn source_range(self) -> Range<u64> {
        self.source..self.source + self.length
    }

    pub(super) fn map(&self, seed: u64) -> Option<u64> {
        self.source_range()
            .contains(&seed)
            .then(|| self.destination + (seed - self.source))
    }

    pub(super) fn map_range(
        self,
        range: Range<u64>,
    ) -> Option<(Range<u64>, impl Iterator<Item = Range<u64>>)> {
        range.clone().intersection(self.source_range()).map(|i| {
            let start_offset = i.start - self.source;
            let end_offset = i.end - self.source;
            let mapped_range = self.destination + start_offset..self.destination + end_offset;
            (mapped_range, range.difference(self.source_range()))
        })
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
