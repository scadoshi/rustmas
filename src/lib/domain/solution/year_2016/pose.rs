use crate::domain::solution::common::{direction::Direction, point::Point, turn::Turn};

/// A position plus a heading, starting at the origin facing [`Direction::Up`].
///
/// A [`Point`] alone will not do, since `R2` turns relative to wherever the
/// last instruction left you pointing.
#[derive(Debug, Clone, Copy, Default)]
pub struct Pose {
    pub heading: Direction,
    pub position: Point,
}

impl Pose {
    /// Walks `distance` along the current heading, clamping at the [`i32`] edges.
    pub fn saturating_moved(self, distance: i32) -> Self {
        Self {
            heading: self.heading,
            position: self.position.saturating_moved(self.heading, distance),
        }
    }

    /// Turns without moving.
    pub fn turned(self, turn: Turn) -> Self {
        Self {
            heading: self.heading.turned(turn),
            position: self.position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`Direction`] has no `PartialEq`, and the tests should not dictate that.
    fn heading_is(pose: &Pose, expected: Direction) -> bool {
        use Direction::*;
        matches!(
            (pose.heading, expected),
            (Up, Up) | (Right, Right) | (Down, Down) | (Left, Left)
        )
    }

    fn at(pose: &Pose) -> (i32, i32) {
        (pose.position.x, pose.position.y)
    }

    #[test]
    fn starts_at_the_origin_heading_up() {
        let pose = Pose::default();
        assert_eq!(at(&pose), (0, 0));
        assert!(heading_is(&pose, Direction::Up));
    }

    #[test]
    fn turning_changes_the_heading_and_nothing_else() {
        let pose = Pose::default().turned(Turn::Right);
        assert!(heading_is(&pose, Direction::Right));
        assert_eq!(at(&pose), (0, 0));

        let pose = Pose::default().turned(Turn::Left);
        assert!(heading_is(&pose, Direction::Left));
        assert_eq!(at(&pose), (0, 0));
    }

    #[test]
    fn moving_follows_the_heading() {
        assert_eq!(at(&Pose::default().saturating_moved(3)), (0, 3));
        assert_eq!(
            at(&Pose::default().turned(Turn::Right).saturating_moved(3)),
            (3, 0)
        );
        assert_eq!(
            at(&Pose::default().turned(Turn::Left).saturating_moved(3)),
            (-3, 0)
        );
    }

    /// The puzzle's own example: `R2, L3` lands 2 east and 3 north, 5 away.
    #[test]
    fn walks_the_puzzle_example() {
        let pose = Pose::default()
            .turned(Turn::Right)
            .saturating_moved(2)
            .turned(Turn::Left)
            .saturating_moved(3);
        assert_eq!(at(&pose), (2, 3));
        assert_eq!(pose.position.distance_from_origin(), 5);
    }

    #[test]
    fn four_right_turns_return_to_the_start() {
        let mut pose = Pose::default();
        for _ in 0..4 {
            pose = pose.turned(Turn::Right).saturating_moved(2);
        }
        assert_eq!(at(&pose), (0, 0));
        assert!(heading_is(&pose, Direction::Up));
    }

    #[test]
    fn moving_past_the_edge_clamps() {
        let pose = Pose::default()
            .saturating_moved(i32::MAX)
            .saturating_moved(1);
        assert_eq!(at(&pose), (0, i32::MAX));
    }
}
