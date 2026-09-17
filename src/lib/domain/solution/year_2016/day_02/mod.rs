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
        let grid = Rectangle::new(vec![
            vec![None, None, Some('1'), None, None],
            vec![None, Some('2'), Some('3'), Some('4'), None],
            vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
            vec![None, Some('A'), Some('B'), Some('C'), None],
            vec![None, None, Some('D'), None, None],
        ])?;
        let code: String = self
            .input
            .iter()
            .scan(
                CellOnGrid::new(Cell::new(1, 1), &grid)?,
                |cursor, directions| {
                    *cursor = directions.iter().fold(*cursor, |cursor, direction| {
                        cursor
                            .checked_moved(*direction, 1)
                            .filter(|c| c.value().is_some())
                            .unwrap_or(cursor)
                    });
                    Some(cursor.value().unwrap_or_default())
                },
            )
            .collect();
        Ok(Answer::Value(code))
    }
}
