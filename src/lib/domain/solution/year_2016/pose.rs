use crate::domain::solution::common::{direction::Direction, point::Point};

type Dir = Direction;

/// Somewhere on the plane, plus which way you are pointing.
///
/// A [`Point`] alone is not enough for a day whose instructions are relative:
/// `R2` means turn right from wherever you already point, so the heading has to
/// be carried alongside the position. Starts at the origin heading
/// [`Direction::Up`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Pose {
    pub heading: Direction,
    pub position: Point,
}

impl Pose {
    /// Walks `distance` along the current heading, clamping at the edges of
    /// [`i32`] rather than failing.
    pub fn saturating_moved(self, distance: i32) -> Self {
        Self {
            heading: self.heading,
            position: self.position.saturating_moved(self.heading, distance),
        }
    }

    /// Turns without moving.
    ///
    /// `direction` is read as a turn rather than a heading, so only `Left` and
    /// `Right` do anything. `Up` and `Down` are not turns you can make from a
    /// heading, and are ignored rather than rejected, since the input never
    /// contains them.
    pub fn saturating_turned(self, direction: Direction) -> Self {
        Self {
            heading: match direction {
                Dir::Left => self.heading.turn_left(),
                Dir::Right => self.heading.turn_right(),
                _ => self.heading,
            },
            position: self.position,
        }
    }
}
