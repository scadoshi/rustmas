use crate::domain::solution::year_2023::day_05::range::{
    map::RangeMap, mapping::InvalidRangeMapping,
};
use std::{ops::Deref, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub(in crate::domain::solution::year_2023::day_05) struct RangeMaps(Vec<RangeMap>);

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
    pub(in crate::domain::solution::year_2023::day_05) fn map(&self, seed: u64) -> u64 {
        self.iter().fold(seed, |seed, r| r.map(seed))
    }
}

#[cfg(test)]
mod tests {
    use super::super::mapping::RangeMapping;
    use super::*;

    fn example() -> RangeMaps {
        RangeMaps(vec![
            RangeMap(vec![
                RangeMapping::new(50, 98, 2),
                RangeMapping::new(52, 50, 48),
            ]),
            RangeMap(vec![
                RangeMapping::new(0, 15, 37),
                RangeMapping::new(37, 52, 2),
                RangeMapping::new(39, 0, 15),
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
}
