//! Rucksack contents as bit sets of item types.

use std::str::FromStr;

use thiserror::Error;

/// Why a rucksack line failed to parse.
#[derive(Debug, Error)]
pub(super) enum InvalidRucksack {
    /// A character outside `a` to `z` and `A` to `Z`. Carries the character.
    #[error("expected only letters, read {0:?}")]
    NotAnItem(char),
    /// The line has an odd number of bytes, so it can't split into two equal compartments.
    #[error("expected an even number of items, so the compartments match")]
    OddLength,
}

/// A set that should hold one shared item holds none or several.
#[derive(Debug, Error)]
#[error("expected exactly one item in common")]
pub(super) struct NoSingleCommonItem;

/// Every item type, in priority order.
const ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// An item's priority: `a`-`z` are 1-26, `A`-`Z` are 27-52.
fn priority(item: char) -> Option<u32> {
    ITEMS
        .chars()
        .zip(1..)
        .find_map(|(other, priority)| (other == item).then_some(priority))
}

/// A set of item types as bits, bit `p` standing for priority `p`.
///
/// Bit 0 stays clear. 52 item types fit a `u64`, and "in common" becomes `&`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Items(u64);

impl FromStr for Items {
    type Err = InvalidRucksack;
    fn from_str(items: &str) -> Result<Self, Self::Err> {
        items.chars().try_fold(Self(0), Self::with)
    }
}

impl Items {
    /// This set plus `item`.
    fn with(self, item: char) -> Result<Self, InvalidRucksack> {
        priority(item)
            .and_then(|priority| 1_u64.checked_shl(priority))
            .map(|bit| Self(self.0 | bit))
            .ok_or(InvalidRucksack::NotAnItem(item))
    }

    /// The items both sets hold.
    #[must_use]
    pub(super) fn common(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// The priority of the only item here.
    ///
    /// # Errors
    ///
    /// [`NoSingleCommonItem`] if there are no items or more than one.
    pub(super) fn single_priority(self) -> Result<u32, NoSingleCommonItem> {
        match self.0.count_ones() {
            1 => Ok(self.0.trailing_zeros()),
            _ => Err(NoSingleCommonItem),
        }
    }
}

/// One elf's rucksack, split into its two equal compartments.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Rucksack {
    /// The first half of the line.
    pub(super) left: Items,
    /// The second half of the line.
    pub(super) right: Items,
}

impl FromStr for Rucksack {
    type Err = InvalidRucksack;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        if !value.len().is_multiple_of(2) {
            return Err(InvalidRucksack::OddLength);
        }
        let middle = value.len() / 2;
        value
            .char_indices()
            .try_fold(Self::default(), |sack, (at, item)| {
                Ok(if at < middle {
                    Self {
                        left: sack.left.with(item)?,
                        ..sack
                    }
                } else {
                    Self {
                        right: sack.right.with(item)?,
                        ..sack
                    }
                })
            })
    }
}

impl Rucksack {
    /// Everything in either compartment.
    pub(super) fn all(self) -> Items {
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

    /// A two-byte letter straddles the byte midpoint of `a\u{e9}b`.
    #[test]
    fn a_non_ascii_item_is_rejected_not_split() {
        assert!(matches!(
            "a\u{e9}b".parse::<Rucksack>(),
            Err(InvalidRucksack::NotAnItem('\u{e9}'))
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
        assert!(Items(0b110).single_priority().is_err());
        assert_eq!(Items(0b1000).single_priority().unwrap(), 3);
    }
}
