use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidTriangle {
    #[error("expected three sides")]
    SideCount,
    #[error("each side must be a number")]
    SideNotNumber,
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle(pub [u32; 3]);

impl TryFrom<&str> for Triangle {
    type Error = InvalidTriangle;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sides: Vec<u32> = value
            .split_whitespace()
            .map(|side| side.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| InvalidTriangle::SideNotNumber)?;
        Ok(Self(
            sides.try_into().map_err(|_| InvalidTriangle::SideCount)?,
        ))
    }
}

impl Triangle {
    /// Whether the two shorter sides together beat the longest.
    pub fn is_valid(&self) -> bool {
        let mut sides = self.0;
        sides.sort_unstable();
        sides[0] + sides[1] > sides[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_try_from_str_ok() {
        assert_eq!(Triangle::try_from(" 5  10  25 ").unwrap().0, [5, 10, 25]);
    }

    #[test]
    fn triangle_try_from_str_err() {
        assert!(matches!(
            Triangle::try_from("5 10"),
            Err(InvalidTriangle::SideCount)
        ));
        assert!(matches!(
            Triangle::try_from("5 10 25 30"),
            Err(InvalidTriangle::SideCount)
        ));
        assert!(matches!(
            Triangle::try_from("5 10 x"),
            Err(InvalidTriangle::SideNotNumber)
        ));
    }

    /// The longest side decides, wherever it sits in the line.
    #[test]
    fn is_valid_ignores_the_order_of_the_sides() {
        assert!(!Triangle([5, 10, 25]).is_valid());
        assert!(!Triangle([25, 10, 5]).is_valid());
        assert!(Triangle([3, 4, 5]).is_valid());
        assert!(!Triangle([1, 2, 3]).is_valid(), "equal is not greater");
    }
}
