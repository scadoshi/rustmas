use crate::domain::solution::year_2023::day_05::range::mapping::{
    InvalidRangeMapping, RangeMapping,
};
use std::{ops::Deref, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub(in super::super) struct RangeMap(pub(super) Vec<RangeMapping>);

impl Deref for RangeMap {
    type Target = [RangeMapping];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for RangeMap {
    type Err = InvalidRangeMapping;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter(|l| !l.contains("map"))
                .map(RangeMapping::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl RangeMap {
    pub(in super::super) fn map(&self, seed: u64) -> u64 {
        self.iter().find_map(|r| r.map(seed)).unwrap_or(seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_map_from_str_ok() {
        assert_eq!(
            RangeMap::from_str("50 98 2\n52 50 48").unwrap(),
            RangeMap(vec![
                RangeMapping::new(50, 98, 2),
                RangeMapping::new(52, 50, 48)
            ])
        );
    }

    #[test]
    fn range_map_from_str_err() {
        assert!(matches!(
            RangeMap::from_str("50 98\n52 50 48"),
            Err(InvalidRangeMapping::TooFewParts)
        ));
        assert!(matches!(
            RangeMap::from_str("50 98 2\n52 50 48 0"),
            Err(InvalidRangeMapping::TooManyParts)
        ));
        assert!(matches!(
            RangeMap::from_str("50 98 foo\n52 50 48"),
            Err(InvalidRangeMapping::ParseInt(_))
        ));
    }

    #[test]
    fn range_map_map() {
        assert_eq!(RangeMap::from_str("50 98 2\n52 50 48").unwrap().map(55), 57);
    }
}
