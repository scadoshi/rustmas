use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidDimensions {
    #[error("expected three sizes separated by `x`, like `2x3x4`")]
    WrongCount,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimensions {
    pub fn new(length: u32, width: u32, height: u32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    /// Twice every side's area plus the smallest side's, or `None` past
    /// [`u32::MAX`].
    pub fn wrapping_paper_required(&self) -> Option<u32> {
        let s1 = self.length.checked_mul(self.width)?;
        let s2 = self.width.checked_mul(self.height)?;
        let s3 = self.length.checked_mul(self.height)?;
        let smallest = s1.min(s2).min(s3);
        s1.checked_add(s2)?
            .checked_add(s3)?
            .checked_mul(2)?
            .checked_add(smallest)
    }

    /// Length times width times height, or `None` past [`u32::MAX`].
    pub fn volume(&self) -> Option<u32> {
        self.length
            .checked_mul(self.width)?
            .checked_mul(self.height)
    }

    /// The smallest perimeter plus the volume, or `None` past [`u32::MAX`].
    pub fn ribbon_required(&self) -> Option<u32> {
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
        Ok(Self {
            length: length.parse()?,
            width: width.parse()?,
            height: height.parse()?,
        })
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
