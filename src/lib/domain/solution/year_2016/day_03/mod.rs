pub mod triangle;

use crate::domain::solution::{Solution, answer::Answer, year_2016::day_03::triangle::Triangle};

pub struct Puzzle {
    triangles: Vec<Triangle>,
}

/// The triangles read down the columns instead of across the rows.
///
/// Every three rows give three triangles, one per column. A trailing partial
/// group has no column to complete, so it is dropped.
fn by_column(rows: &[Triangle]) -> impl Iterator<Item = Triangle> + '_ {
    rows.chunks_exact(3).flat_map(|rows| {
        (0..3).map(move |side| Triangle([rows[0].0[side], rows[1].0[side], rows[2].0[side]]))
    })
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            triangles: input
                .as_ref()
                .lines()
                .map(Triangle::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.triangles
                .iter()
                .filter(|t| t.is_valid())
                .count()
                .to_string(),
        ))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            by_column(&self.triangles)
                .filter(Triangle::is_valid)
                .count()
                .to_string(),
        ))
    }
}
