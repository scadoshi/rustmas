mod triangle;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2016::day_03::triangle::Triangle;

#[derive(Debug)]
pub(crate) struct Puzzle {
    triangles: Vec<Triangle>,
}

/// The triangles read down the columns instead of across the rows.
///
/// Every three rows give three triangles, one per column. A trailing partial
/// group has no column to complete, so it is dropped.
fn by_column(rows: &[Triangle]) -> impl Iterator<Item = Triangle> + '_ {
    rows.as_chunks::<3>().0.iter().flat_map(
        |[Triangle(first), Triangle(second), Triangle(third)]| {
            first
                .iter()
                .zip(second)
                .zip(third)
                .map(|((&a, &b), &c)| Triangle([a, b, c]))
        },
    )
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            triangles: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        Ok(Answer::solved(
            self.triangles
                .iter()
                .filter(|triangle| triangle.is_valid())
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Rows `[1 2 3]`, `[4 5 6]`, `[7 8 9]` read down as `[1 4 7]` and so on.
    #[test]
    fn by_column_reads_down_each_group_of_three() {
        let rows = [
            Triangle([1, 2, 3]),
            Triangle([4, 5, 6]),
            Triangle([7, 8, 9]),
            Triangle([1, 1, 1]),
        ];
        let columns: Vec<[u32; 3]> = by_column(&rows).map(|triangle| triangle.0).collect();
        assert_eq!(columns, vec![[1, 4, 7], [2, 5, 8], [3, 6, 9]]);
    }
}
