use std::iter;

use crate::domain::solution::common::direction::Direction;
use crate::domain::solution::common::point::Point;

/// The spiral's squares in order, starting at the origin, ending if the
/// spiral leaves the [`i32`] grid.
///
/// Runs go right, up, left, down, and every second turn lengthens the run by
/// one, which is what makes the path spiral outward rather than circle.
pub(super) fn spiral() -> impl Iterator<Item = Point> {
    let mut point = Point::default();
    let mut direction = Direction::Right;
    let mut run: u32 = 1;
    let mut left_in_run: u32 = 1;
    let mut lengthen = false;
    iter::once(point).chain(iter::from_fn(move || {
        point = point.checked_moved(direction, 1)?;
        left_in_run = left_in_run.checked_sub(1)?;
        if left_in_run == 0 {
            direction = direction.turn_left();
            if lengthen {
                run = run.checked_add(1)?;
            }
            lengthen = !lengthen;
            left_in_run = run;
        }
        Some(point)
    }))
}

/// The eight squares touching `point`, corners included, skipping any off
/// the [`i32`] grid.
pub(super) fn neighbors(point: Point) -> impl Iterator<Item = Point> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1)
            .filter(move |dy| dx != 0 || *dy != 0)
            .filter_map(move |dy| {
                Some(Point::new(
                    point.x.checked_add(dx)?,
                    point.y.checked_add(dy)?,
                ))
            })
    })
}
