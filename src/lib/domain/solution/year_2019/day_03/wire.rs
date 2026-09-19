use crate::domain::solution::common::{
    direction::{Direction, InvalidDirection},
    point::Point,
};
use std::{collections::HashMap, num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidWire {
    #[error("expected a direction letter followed by a distance")]
    EmptyStep,
    #[error(transparent)]
    Direction(#[from] InvalidDirection),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// Every point a wire runs through, with how many steps reached it first.
///
/// A wire can cross itself, and the puzzle counts the first visit, so a later
/// pass over the same point never overwrites the step count.
#[derive(Debug)]
pub struct Wire(HashMap<Point, usize>);

impl FromStr for Wire {
    type Err = InvalidWire;

    /// Parses `R8,U5,L5,D3`, walking it from the origin.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut visited = HashMap::new();
        let mut point = Point::default();
        let mut steps = 0;
        for segment in value.trim().split(',') {
            let mut chars = segment.trim().chars();
            let direction = Direction::try_from(chars.next().ok_or(InvalidWire::EmptyStep)?)?;
            let distance: usize = chars.as_str().parse()?;
            for _ in 0..distance {
                point = point.saturating_moved(direction, 1);
                steps += 1;
                visited.entry(point).or_insert(steps);
            }
        }
        Ok(Self(visited))
    }
}

impl Wire {
    /// Where this wire meets `other`, with the combined steps to get there.
    ///
    /// The origin is where both start, so it never counts as a crossing.
    pub fn crossings<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = (Point, usize)> + 'a {
        self.0
            .iter()
            .filter(|(point, _)| **point != Point::default())
            .filter_map(move |(point, steps)| {
                other
                    .0
                    .get(point)
                    .map(|other_steps| (*point, steps + other_steps))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_from_str_walks_each_segment() {
        let wire = "U3,R2".parse::<Wire>().unwrap();
        let mut visited: Vec<(Point, usize)> = wire.0.into_iter().collect();
        visited.sort_by_key(|(_, steps)| *steps);
        assert_eq!(
            visited,
            vec![
                (Point::new(0, 1), 1),
                (Point::new(0, 2), 2),
                (Point::new(0, 3), 3),
                (Point::new(1, 3), 4),
                (Point::new(2, 3), 5),
            ]
        );
    }

    #[test]
    fn wire_from_str_err() {
        assert!(matches!(
            "R2,,U1".parse::<Wire>(),
            Err(InvalidWire::EmptyStep)
        ));
        assert!(matches!(
            "X2".parse::<Wire>(),
            Err(InvalidWire::Direction(_))
        ));
        assert!(matches!(
            "Rx".parse::<Wire>(),
            Err(InvalidWire::ParseInt(_))
        ));
    }

    /// A wire that doubles back keeps the step count of its first visit.
    #[test]
    fn first_visit_wins() {
        let wire = "R2,L2,R1".parse::<Wire>().unwrap();
        assert_eq!(wire.0[&Point::new(1, 0)], 1);
    }

    /// The worked example from the puzzle: nearest crossing 6, fewest steps 30.
    #[test]
    fn the_example_crossings() {
        let a = "R8,U5,L5,D3".parse::<Wire>().unwrap();
        let b = "U7,R6,D4,L4".parse::<Wire>().unwrap();
        let nearest = a.crossings(&b).map(|(p, _)| p.distance_from_origin()).min();
        let fewest = a.crossings(&b).map(|(_, steps)| steps).min();
        assert_eq!((nearest, fewest), (Some(6), Some(30)));
    }
}
