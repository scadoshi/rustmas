//! A present's size, and the paper and ribbon it takes.

use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

/// Returned when a line is not three sizes joined by `x`.
#[derive(Debug, Error)]
pub(super) enum InvalidDimensions {
    /// Fewer or more than three sizes.
    #[error("expected three sizes separated by `x`, like `2x3x4`")]
    WrongCount,
    /// A size that is not a whole number that fits a `u32`.
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// A present's three sides in feet, in the order the input lists them.
///
/// Nothing depends on which side is which: every calculation either uses all three or sorts them first.
#[derive(Debug)]
pub(super) struct Dimensions {
    /// The first size on the line.
    length: u32,
    /// The second size on the line.
    width: u32,
    /// The third size on the line.
    height: u32,
}

impl Dimensions {
    /// A present measuring `length` by `width` by `height` feet.
    fn new(length: u32, width: u32, height: u32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    /// Twice every side's area plus the smallest side's, or `None` past
    /// [`u32::MAX`].
    pub(super) fn wrapping_paper_required(&self) -> Option<u32> {
        let base = self.length.checked_mul(self.width)?;
        let front = self.width.checked_mul(self.height)?;
        let side = self.length.checked_mul(self.height)?;
        let smallest = base.min(front).min(side);
        base.checked_add(front)?
            .checked_add(side)?
            .checked_mul(2)?
            .checked_add(smallest)
    }

    /// Length times width times height, or `None` past [`u32::MAX`].
    fn volume(&self) -> Option<u32> {
        self.length
            .checked_mul(self.width)?
            .checked_mul(self.height)
    }

    /// The smallest perimeter plus the volume, or `None` past [`u32::MAX`].
    pub(super) fn ribbon_required(&self) -> Option<u32> {
        let mut sizes = [self.length, self.width, self.height];
        sizes.sort_unstable();
        let [shortest, middle, _] = sizes;
        shortest
            .checked_add(middle)?
            .checked_mul(2)?
            .checked_add(self.volume()?)
    }
}

impl FromStr for Dimensions {
    type Err = InvalidDimensions;

    /// Parses `2x3x4` as length, width and height.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut sizes = value.trim().split('x');
        let (Some(length), Some(width), Some(height), None) =
            (sizes.next(), sizes.next(), sizes.next(), sizes.next())
        else {
            return Err(InvalidDimensions::WrongCount);
        };
        Ok(Self::new(length.parse()?, width.parse()?, height.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_paper_required() {
        assert_eq!(Dimensions::new(2, 3, 4).wrapping_paper_required(), Some(58));
        assert_eq!(
            Dimensions::new(1, 1, 10).wrapping_paper_required(),
            Some(43)
        );
    }

    #[test]
    fn from_str_ok() {
        let dimensions = "1x2x3".parse::<Dimensions>();
        assert!(dimensions.is_ok());
        let dimensions = dimensions.unwrap();
        assert_eq!(dimensions.length, 1);
        assert_eq!(dimensions.width, 2);
        assert_eq!(dimensions.height, 3);
    }

    #[test]
    fn from_str_err() {
        assert!("foo".parse::<Dimensions>().is_err());
        assert!("".parse::<Dimensions>().is_err());
        assert!("1x2xfoo".parse::<Dimensions>().is_err());
        assert!("1x2x3x4".parse::<Dimensions>().is_err());
    }

    #[test]
    fn ribbon_required() {
        assert_eq!(Dimensions::new(2, 3, 4).ribbon_required(), Some(34));
        assert_eq!(Dimensions::new(1, 1, 10).ribbon_required(), Some(14));
    }
}
