use std::ops::{Deref, Range};
use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Seeds(Vec<u64>);

impl Deref for Seeds {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Seeds {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.strip_prefix("seeds: ")
                .ok_or_else(|| anyhow!("missing prefix"))?
                .split_whitespace()
                .map(FromStr::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Seeds {
    pub(super) fn to_ranges(&self) -> anyhow::Result<Vec<Range<u64>>> {
        self.chunks(2).try_fold(Vec::new(), |mut ranges, chunk| {
            let (Some(start), Some(len)) = (chunk.first(), chunk.get(1)) else {
                bail!("seed len is odd; to chunk into ranges it must be even");
            };
            let Some(end) = start.checked_add(*len) else {
                bail!("a seed range of {len} from {start} ends past {}", u64::MAX);
            };
            ranges.push(*start..end);
            Ok(ranges)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_from_str_ok() {
        assert!(Seeds::from_str("seeds: 79 14 55 13").is_ok());
    }

    #[test]
    fn seed_from_str_err() {
        assert!(Seeds::from_str("seeds: foo 79 14 55 13").is_err());
        assert!(Seeds::from_str("foo 79 14 55 13").is_err());
    }

    #[test]
    fn seed_to_ranges_ok() {
        assert_eq!(
            Seeds::from_str("seeds: 79 14 55 13")
                .unwrap()
                .to_ranges()
                .unwrap(),
            vec![(79..79 + 14), (55..55 + 13)]
        );
    }

    #[test]
    fn seed_to_ranges_err() {
        assert!(
            Seeds::from_str("seeds: 79 14 55")
                .unwrap()
                .to_ranges()
                .is_err()
        );
    }

    #[test]
    fn seed_to_ranges_rejects_a_range_past_u64() {
        assert!(
            Seeds::from_str("seeds: 18446744073709551615 2")
                .unwrap()
                .to_ranges()
                .is_err()
        );
    }
}
