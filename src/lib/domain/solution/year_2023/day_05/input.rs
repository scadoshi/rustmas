use std::str::FromStr;

use anyhow::anyhow;

use crate::domain::solution::year_2023::day_05::{RangeMaps, Seeds};

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Input {
    seeds: Seeds,
    maps: RangeMaps,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds, range_maps) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("seed section not found"))?;
        let seeds = Seeds::from_str(seeds)?;
        let maps = RangeMaps::from_str(range_maps)?;
        Ok(Self { seeds, maps })
    }
}

impl Input {
    pub(super) fn seeds(&self) -> &Seeds {
        &self.seeds
    }
    pub(super) fn maps(&self) -> &RangeMaps {
        &self.maps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input<'a>() -> &'a str {
        r"seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15

                fertilizer-to-water map:
                49 53 8
                0 11 42
                42 0 7
                57 7 4

                water-to-light map:
                88 18 7
                18 25 70

                light-to-temperature map:
                45 77 23
                81 45 19
                68 64 13

                temperature-to-humidity map:
                0 69 1
                1 0 69

                humidity-to-location map:
                60 56 37
                56 93 4"
    }

    #[test]
    fn input_from_str_ok() {
        assert!(Input::from_str(example_input()).is_ok());
    }

    #[test]
    fn input_from_str_err_invalid_range_mapping_too_few_parts() {
        assert!(
            Input::from_str(
                r"seeds: 79 14 55 13

                seed-to-soil map:
                50 98
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15"
            )
            .is_err()
        );
    }

    #[test]
    fn input_from_str_err_invalid_range_mapping_too_many_parts() {
        assert!(
            Input::from_str(
                r"seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2 0
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15"
            )
            .is_err()
        );
    }

    #[test]
    fn input_from_str_err_invalid_range_mapping_parse_int() {
        assert!(
            Input::from_str(
                r"seeds: 79 14 55 13

                seed-to-soil map:
                foo 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15"
            )
            .is_err()
        );
    }

    #[test]
    fn input_from_str_err_seed_parse_int() {
        assert!(
            Input::from_str(
                r"seeds: foo 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15"
            )
            .is_err()
        );
    }
}
