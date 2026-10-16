use crate::domain::solution::year_2023::day_05::range::mapping::{
    InvalidRangeMapping, RangeMapping,
};
use std::{
    ops::{Deref, Range},
    str::FromStr,
};

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

    pub(in super::super) fn map_range(
        &self,
        range: Range<u64>,
    ) -> impl Iterator<Item = Range<u64>> {
        let mut mapped = Vec::<Range<u64>>::new();
        let mut unmapped = Vec::<Range<u64>>::new();
        let mut queue = Vec::<Range<u64>>::from([range]);
        while let Some(range) = queue.pop() {
            if let Some((m, l)) = self
                .iter()
                .find_map(|mapping| mapping.map_range(range.clone()))
            {
                mapped.push(m);
                queue.extend(l);
            } else {
                unmapped.push(range);
            }
        }
        mapped.into_iter().chain(unmapped)
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

    #[test]
    fn range_map_map_range() {
        let map = RangeMap::from_str("50 98 2\n52 50 48").unwrap();
        let mapped = |range| {
            let mut ranges = map.map_range(range).collect::<Vec<_>>();
            ranges.sort_by_key(|r| r.start);
            ranges
        };
        // inside one mapping
        assert_eq!(mapped(79..93), vec![81..95]);
        // straddles both mappings, with unmapped crusts either side
        assert_eq!(mapped(40..105), vec![40..50, 50..52, 52..100, 100..105]);
        // touches no mapping
        assert_eq!(mapped(0..10), vec![0..10]);
    }
}
