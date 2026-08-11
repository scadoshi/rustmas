use crate::domain::solution::common::direction::Direction;

type Dir = Direction;

/// A position on the cartesian plane.
///
/// Axes run the usual way: `x` grows to the right and `y` grows upward, so
/// `Up` increases `y` and `Down` decreases it. Both fields are signed and the
/// plane is unbounded in every direction. For row and column indices into a
/// grid, use [`Cell`](super::cell::Cell).
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Builds a point at the given x and y.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Moves `distance` units in `direction`, returning `None` if that would
    /// overflow past [`i32::MIN`] or [`i32::MAX`].
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

    /// Moves `distance` units in `direction`, clamping at [`i32::MIN`] or
    /// [`i32::MAX`] rather than failing.
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
