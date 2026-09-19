use crate::domain::solution::common::cell::Cell;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidClaim {
    #[error("expected `#id @ left,top: widthxheight`")]
    Malformed,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy)]
pub struct Claim {
    pub id: u32,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl TryFrom<&str> for Claim {
    type Error = InvalidClaim;

    /// Parses `#1 @ 55,885: 22x10`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (id, rest) = value
            .trim()
            .strip_prefix('#')
            .and_then(|rest| rest.split_once(" @ "))
            .ok_or(InvalidClaim::Malformed)?;
        let (corner, size) = rest.split_once(": ").ok_or(InvalidClaim::Malformed)?;
        let (left, top) = corner.split_once(',').ok_or(InvalidClaim::Malformed)?;
        let (width, height) = size.split_once('x').ok_or(InvalidClaim::Malformed)?;
        Ok(Self {
            id: id.parse()?,
            left: left.parse()?,
            top: top.parse()?,
            width: width.parse()?,
            height: height.parse()?,
        })
    }
}

impl Claim {
    /// Every square inch this claim covers.
    pub fn cells(self) -> impl Iterator<Item = Cell> {
        (0..self.height).flat_map(move |row| {
            (0..self.width).map(move |column| Cell::new(self.left + column, self.top + row))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_try_from_str_ok() {
        let claim = Claim::try_from("#1 @ 55,885: 22x10").unwrap();
        assert_eq!(claim.id, 1);
        assert_eq!((claim.left, claim.top), (55, 885));
        assert_eq!((claim.width, claim.height), (22, 10));
    }

    #[test]
    fn claim_try_from_str_err() {
        for line in ["1 @ 55,885: 22x10", "#1 55,885: 22x10", "#1 @ 55,885 22x10"] {
            assert!(
                matches!(Claim::try_from(line), Err(InvalidClaim::Malformed)),
                "{line}"
            );
        }
        assert!(matches!(
            Claim::try_from("#x @ 55,885: 22x10"),
            Err(InvalidClaim::ParseInt(_))
        ));
    }

    #[test]
    fn cells_cover_the_rectangle_from_its_corner() {
        let cells: Vec<(usize, usize)> = Claim::try_from("#1 @ 1,3: 3x2")
            .unwrap()
            .cells()
            .map(|cell| (cell.column(), cell.row()))
            .collect();
        assert_eq!(cells, vec![(1, 3), (2, 3), (3, 3), (1, 4), (2, 4), (3, 4)]);
    }
}
