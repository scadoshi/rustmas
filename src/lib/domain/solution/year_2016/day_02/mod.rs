use crate::domain::solution::{
    Solution,
    answer::Answer,
    common::{
        cell::Cell,
        cursor::{Cursor, FixedWidthGridCursor, cell_on_grid::CellOnGrid},
        direction::Direction,
        grid::rectangle::Rectangle,
    },
};

pub struct Puzzle {
    input: Vec<Vec<Direction>>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            input: input
                .as_ref()
                .lines()
                .map(|l| {
                    l.chars()
                        .filter(|c| !c.is_whitespace())
                        .map(Direction::try_from)
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let grid = Rectangle::new(vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ])?;
        let code: String = self
            .input
            .iter()
            .scan(
                CellOnGrid::new(Cell::new(1, 1), &grid)?,
                |cursor, directions| {
                    *cursor = directions.iter().fold(*cursor, |cursor, direction| {
                        cursor.saturating_moved(*direction, 1)
                    });
                    Some(*cursor.value())
                },
            )
            .collect();
        Ok(Answer::Value(code))
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        Ok(Answer::Unwritten)
    }
}
