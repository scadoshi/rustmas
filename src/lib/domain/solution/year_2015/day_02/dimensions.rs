use std::{num::ParseIntError, str::FromStr};
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

    pub fn wrapping_paper_required(&self) -> u32 {
        let s1 = self.length * self.width;
        let s2 = self.width * self.height;
        let s3 = self.length * self.height;
        let smallest = s1.min(s2).min(s3);
        (s1 * 2) + (s2 * 2) + (s3 * 2) + smallest
    }

    pub fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    pub fn ribbon_required(&self) -> u32 {
        let dimensions = [self.length, self.width, self.height];
        let (min_i, min) = dimensions
            .into_iter()
            .enumerate()
            .min_by_key(|(_, x)| *x)
            .expect("Array should never be empty");
        let (_, next_min) = dimensions
            .into_iter()
            .enumerate()
            .filter(|(i, x)| *i != min_i && *x >= min)
            .min_by_key(|(_, x)| *x)
            .expect("Array should never be empty");
        (min * 2) + (next_min * 2) + self.volume()
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
        assert_eq!(Dimensions::new(2, 3, 4).wrapping_paper_required(), 58);
        assert_eq!(Dimensions::new(1, 1, 10).wrapping_paper_required(), 43);
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
        assert_eq!(Dimensions::new(2, 3, 4).ribbon_required(), 34);
        assert_eq!(Dimensions::new(1, 1, 10).ribbon_required(), 14);
    }
}
