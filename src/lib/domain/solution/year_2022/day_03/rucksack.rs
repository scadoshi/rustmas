use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidRucksack {
    #[error("expected only letters, read {0:?}")]
    NotAnItem(char),
    #[error("expected an even number of items, so the compartments match")]
    OddLength,
}

#[derive(Debug, Error)]
#[error("expected exactly one item in common")]
pub struct NoSingleCommonItem;

/// An item's priority: `a`-`z` are 1-26, `A`-`Z` are 27-52.
pub fn priority(item: char) -> Option<u32> {
    match item {
        'a'..='z' => Some(item as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(item as u32 - 'A' as u32 + 27),
        _ => None,
    }
}

/// A set of item types as bits, bit `p - 1` standing for priority `p`.
///
/// 52 item types fit a `u64`, and "in common" becomes `&`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Items(u64);

impl FromStr for Items {
    type Err = InvalidRucksack;
    fn from_str(items: &str) -> Result<Self, Self::Err> {
        items.chars().try_fold(Self(0), |set, item| {
            priority(item)
                .map(|p| Self(set.0 | 1 << (p - 1)))
                .ok_or(InvalidRucksack::NotAnItem(item))
        })
    }
}

impl Items {
    pub fn common(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// The priority of the only item here, if there is exactly one.
    pub fn single_priority(self) -> Result<u32, NoSingleCommonItem> {
        match self.0.count_ones() {
            1 => Ok(self.0.trailing_zeros() + 1),
            _ => Err(NoSingleCommonItem),
        }
    }
}

/// One elf's rucksack, split into its two equal compartments.
#[derive(Debug, Clone, Copy)]
pub struct Rucksack {
    pub left: Items,
    pub right: Items,
}

impl FromStr for Rucksack {
    type Err = InvalidRucksack;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        if !value.len().is_multiple_of(2) {
            return Err(InvalidRucksack::OddLength);
        }
        let (left, right) = value.split_at(value.len() / 2);
        Ok(Self {
            left: left.parse()?,
            right: right.parse()?,
        })
    }
}

impl Rucksack {
    /// Everything in either compartment.
    pub fn all(self) -> Items {
        Items(self.left.0 | self.right.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_runs_lowercase_then_uppercase() {
        assert_eq!(priority('a'), Some(1));
        assert_eq!(priority('z'), Some(26));
        assert_eq!(priority('A'), Some(27));
        assert_eq!(priority('Z'), Some(52));
        assert_eq!(priority('1'), None);
    }

    #[test]
    fn rucksack_from_str_err() {
        assert!(matches!(
            "abc".parse::<Rucksack>(),
            Err(InvalidRucksack::OddLength)
        ));
        assert!(matches!(
            "a1".parse::<Rucksack>(),
            Err(InvalidRucksack::NotAnItem('1'))
        ));
    }

    /// The puzzle's first example: `p` is in both halves, priority 16.
    #[test]
    fn the_shared_item_between_compartments() {
        let sack = "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Rucksack>().unwrap();
        assert_eq!(sack.left.common(sack.right).single_priority().unwrap(), 16);
    }

    #[test]
    fn single_priority_needs_exactly_one() {
        assert!(Items(0).single_priority().is_err());
        assert!(Items(0b11).single_priority().is_err());
        assert_eq!(Items(0b100).single_priority().unwrap(), 3);
    }
}
