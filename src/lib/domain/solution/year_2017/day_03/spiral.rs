use crate::domain::solution::common::{direction::Direction, point::Point};

/// The spiral's squares in order, starting at the origin.
///
/// Runs go right, up, left, down, and every second turn lengthens the run by
/// one, which is what makes the path spiral outward rather than circle.
pub fn spiral() -> impl Iterator<Item = Point> {
    let mut point = Point::default();
    let mut direction = Direction::Right;
    let mut run = 1;
    let mut left_in_run = 1;
    let mut turns = 0;
    std::iter::once(point).chain(std::iter::from_fn(move || {
        point = point.saturating_moved(direction, 1);
        left_in_run -= 1;
        if left_in_run == 0 {
            direction = direction.turn_left();
            turns += 1;
            if turns % 2 == 0 {
                run += 1;
            }
            left_in_run = run;
        }
        Some(point)
    }))
}

/// The eight squares touching `point`, corners included.
pub fn neighbors(point: Point) -> impl Iterator<Item = Point> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1)
            .filter(move |dy| dx != 0 || *dy != 0)
            .map(move |dy| Point::new(point.x + dx, point.y + dy))
    })
}
