use crate::domain::solution::common::direction::Direction;

type Dir = Direction;

/// A signed position on the cartesian plane, `y` growing upward.
///
/// For row and column indices into a grid, use [`Cell`](super::cell::Cell).
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Moves `distance` in `direction`, `None` if that would overflow [`i32`].
    pub fn checked_moved(self, direction: Direction, distance: i32) -> Option<Self> {
        Some(Self {
            x: match direction {
                Dir::Left => self.x.checked_sub(distance)?,
                Dir::Right => self.x.checked_add(distance)?,
                _ => self.x,
            },
            y: match direction {
                Dir::Down => self.y.checked_sub(distance)?,
                Dir::Up => self.y.checked_add(distance)?,
                _ => self.y,
            },
        })
    }

    /// Moves `distance` in `direction`, clamping at the edges of [`i32`].
    pub fn saturating_moved(self, direction: Direction, distance: i32) -> Self {
        Self {
            x: match direction {
                Dir::Left => self.x.saturating_sub(distance),
                Dir::Right => self.x.saturating_add(distance),
                _ => self.x,
            },
            y: match direction {
                Dir::Down => self.y.saturating_sub(distance),
                Dir::Up => self.y.saturating_add(distance),
                _ => self.y,
            },
        }
    }

    /// Manhattan distance back to `(0, 0)`, not the straight line.
    ///
    /// `unsigned_abs` rather than `abs`, which panics on [`i32::MIN`] in debug.
    pub fn distance_from_origin(&self) -> u32 {
        self.x.unsigned_abs().saturating_add(self.y.unsigned_abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_checked_moved_in_bounds() {
        assert_eq!(Point::default().checked_moved(Dir::Left, 1).unwrap().x, -1);
        assert_eq!(Point::default().checked_moved(Dir::Right, 1).unwrap().x, 1);
        assert_eq!(Point::default().checked_moved(Dir::Up, 1).unwrap().y, 1);
        assert_eq!(Point::default().checked_moved(Dir::Down, 1).unwrap().y, -1);
    }

    #[test]
    fn point_checked_moved_out_of_bounds() {
        assert!(
            Point::new(i32::MIN, 0)
                .checked_moved(Dir::Left, 1)
                .is_none()
        );
        assert!(
            Point::new(i32::MAX, 0)
                .checked_moved(Dir::Right, 1)
                .is_none()
        );
        assert!(Point::new(0, i32::MAX).checked_moved(Dir::Up, 1).is_none());
        assert!(
            Point::new(0, i32::MIN)
                .checked_moved(Dir::Down, 1)
                .is_none()
        );
    }

    #[test]
    fn point_saturated_moved_in_bounds() {
        assert_eq!(Point::default().saturating_moved(Dir::Left, 1).x, -1);
        assert_eq!(Point::default().saturating_moved(Dir::Right, 1).x, 1);
        assert_eq!(Point::default().saturating_moved(Dir::Up, 1).y, 1);
        assert_eq!(Point::default().saturating_moved(Dir::Down, 1).y, -1);
    }

    #[test]
    fn distance_from_origin_is_manhattan() {
        assert_eq!(Point::default().distance_from_origin(), 0);
        assert_eq!(Point::new(3, 4).distance_from_origin(), 7);
        assert_eq!(Point::new(-3, -4).distance_from_origin(), 7);
    }

    /// `abs` would panic on [`i32::MIN`] in debug builds.
    #[test]
    fn distance_from_origin_survives_the_extremes() {
        assert_eq!(
            Point::new(i32::MIN, 0).distance_from_origin(),
            2_147_483_648
        );
        assert_eq!(
            Point::new(i32::MIN, i32::MIN).distance_from_origin(),
            u32::MAX
        );
    }

    #[test]
    fn point_saturated_moved_left_out_of_bounds() {
        assert_eq!(
            Point::new(i32::MIN, 0).saturating_moved(Dir::Left, 1).x,
            i32::MIN
        );
        assert_eq!(
            Point::new(i32::MAX, 0).saturating_moved(Dir::Right, 1).x,
            i32::MAX
        );
        assert_eq!(
            Point::new(0, i32::MAX).saturating_moved(Dir::Up, 1).y,
            i32::MAX
        );
        assert_eq!(
            Point::new(0, i32::MIN).saturating_moved(Dir::Down, 1).y,
            i32::MIN
        );
    }
}
