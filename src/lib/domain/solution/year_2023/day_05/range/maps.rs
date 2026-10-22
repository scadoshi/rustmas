use std::ops::{Deref, Range};
use std::str::FromStr;

use crate::domain::solution::common::range::ext::RangeExt;
use crate::domain::solution::year_2023::day_05::range::map::RangeMap;
use crate::domain::solution::year_2023::day_05::range::mapping::InvalidRangeMapping;

#[derive(Debug, Clone, PartialEq)]
pub(in super::super) struct RangeMaps(Vec<RangeMap>);

impl Deref for RangeMaps {
    type Target = [RangeMap];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for RangeMaps {
    type Err = InvalidRangeMapping;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split("\n\n")
                .map(RangeMap::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl RangeMaps {
    pub(in super::super) fn map(&self, seed: u64) -> u64 {
        self.iter().fold(seed, |seed, r| r.map(seed))
    }

    pub(in super::super) fn map_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        self.iter().fold(vec![range], |ranges, map| {
            merged(ranges.into_iter().flat_map(|r| map.map_range(r)).collect())
        })
    }
}

/// Sorts the pieces and joins any that overlap or touch, so they stop fragmenting.
fn merged(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_by_key(|r| r.start);
    ranges.into_iter().fold(Vec::new(), |mut merged, r| {
        match merged.pop() {
            Some(last) => merged.extend(last.merge(r)),
            None => merged.push(r),
        }
        merged
    })
}

#[cfg(test)]
mod tests {
    use super::super::mapping::RangeMapping;
    use super::*;

    fn example() -> RangeMaps {
        RangeMaps(vec![
            RangeMap(vec![
                RangeMapping::new(52, 50, 48),
                RangeMapping::new(50, 98, 2),
            ]),
            RangeMap(vec![
                RangeMapping::new(39, 0, 15),
                RangeMapping::new(0, 15, 37),
                RangeMapping::new(37, 52, 2),
            ]),
        ])
    }

    #[test]
    fn range_maps_from_str_ok() {
        assert_eq!(
            RangeMaps::from_str(
                r"seed-to-soil-map:
                50 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15"
            )
            .unwrap(),
            example()
        );
        assert_eq!(
            RangeMaps::from_str(
                r"50 98 2
                52 50 48

                0 15 37
                37 52 2
                39 0 15"
            )
            .unwrap(),
            example()
        );
    }

    #[test]
    fn range_maps_from_str_err() {
        assert!(matches!(
            RangeMaps::from_str(
                r"50 98 2
                52 50 48

                0 15 37
                37 52
                39 0 15"
            ),
            Err(InvalidRangeMapping::TooFewParts)
        ));
        assert!(matches!(
            RangeMaps::from_str(
                r"50 98 2
                52 50 48

                0 15 37 0
                37 52 2
                39 0 15"
            ),
            Err(InvalidRangeMapping::TooManyParts)
        ));
        assert!(matches!(
            RangeMaps::from_str(
                r"50 98 2
                52 foo 48

                0 15 37
                37 52
                39 0 15"
            ),
            Err(InvalidRangeMapping::ParseInt(_))
        ));
    }

    #[test]
    fn range_maps_map() {
        assert_eq!(example().map(51), 38);
    }

    #[test]
    fn range_maps_map_range() {
        let mut ranges = example().map_range(40..60);
        ranges.sort_by_key(|r| r.start);
        assert_eq!(ranges, vec![25..35, 37..39, 54..62]);
        // agrees with mapping each seed on its own
        for seed in 40..60 {
            let location = example().map(seed);
            assert!(ranges.iter().any(|r| r.contains(&location)));
        }
    }

    #[test]
    fn range_maps_map_range_merges_touching_pieces() {
        let maps = RangeMaps(vec![RangeMap::from_str("50 98 2\n52 50 48").unwrap()]);
        // splits into 40..50, 50..52, 52..100 and 100..105, which all touch
        assert_eq!(maps.map_range(40..105), vec![40..105]);
    }
}
