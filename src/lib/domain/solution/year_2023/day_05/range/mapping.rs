use std::{num::ParseIntError, ops::Range, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub(in crate::domain::solution::year_2023::day_05) enum InvalidRangeMapping {
    #[error("too few parts")]
    TooFewParts,
    #[error("too many parts")]
    TooManyParts,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::domain::solution::year_2023::day_05) struct RangeMapping {
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

    pub(super) fn map(&self, seed: u64) -> Option<u64> {
        (self.source..self.source + self.length)
            .contains(&seed)
            .then(|| self.destination + self.source.abs_diff(seed))
    }

    pub(super) fn map_range(&self, _range: Range<u64>) -> (Option<Range<u64>>, Vec<Range<u64>>) {
        let _source_range = self.source..self.source + self.length;
        let _destination_range = self.destination..self.destination + self.length;
        todo!();
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
}
