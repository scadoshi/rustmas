use anyhow::anyhow;

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

impl TryFrom<&str> for Dimensions {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.trim().split('x').map(|str| str.parse::<u32>());
        let err_str = "Invalid dimension input";
        let include_str = "include length, width and height)";
        let (Some(length), Some(width), Some(height)) = (iter.next(), iter.next(), iter.next())
        else {
            return Err(anyhow!(
                "{err_str} (not enough parts; must {include_str}: {value:?}"
            ));
        };
        if iter.next().is_some() {
            return Err(anyhow!(
                "{err_str} (too many parts; must only {include_str}: {value:?}"
            ));
        }
        Ok(Self {
            length: length?,
            width: width?,
            height: height?,
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
    fn try_from_str_ok() {
        let dimensions = Dimensions::try_from("1x2x3");
        assert!(dimensions.is_ok());
        let dimensions = dimensions.unwrap();
        assert_eq!(dimensions.length, 1);
        assert_eq!(dimensions.width, 2);
        assert_eq!(dimensions.height, 3);
    }

    #[test]
    fn try_from_str_err() {
        assert!(Dimensions::try_from("foo").is_err());
        assert!(Dimensions::try_from("").is_err());
        assert!(Dimensions::try_from("1x2xfoo").is_err());
        assert!(Dimensions::try_from("1x2x3x4").is_err());
    }

    #[test]
    fn ribbon_required() {
        assert_eq!(Dimensions::new(2, 3, 4).ribbon_required(), 34);
        assert_eq!(Dimensions::new(1, 1, 10).ribbon_required(), 14);
    }
}
