pub mod board;

use crate::domain::solution::{Solution, answer::Answer, year_2021::day_04::board::Board};
use anyhow::anyhow;

pub struct Puzzle {
    draws: Vec<u8>,
    boards: Vec<Board>,
}

impl Puzzle {
    /// Each board's score in the order they win, as the draws are called.
    ///
    /// A board's score is its unmarked sum times the draw that completed it.
    /// Boards leave the game when they win, so nothing scores twice.
    fn scores_in_winning_order(&self) -> Vec<u32> {
        let mut boards = self.boards.clone();
        let mut scores = Vec::new();
        for &draw in &self.draws {
            for board in &mut boards {
                board.mark(draw);
            }
            scores.extend(
                boards
                    .extract_if(.., |board| board.has_bingo())
                    .map(|board| board.unmarked_sum() * u32::from(draw)),
            );
        }
        scores
    }
}

impl Solution for Puzzle {
    /// The draws come first, then boards separated by blank lines.
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        let mut blocks = input.as_ref().split("\n\n");
        let draws = blocks
            .next()
            .ok_or_else(|| anyhow!("expected a line of draws before the boards"))?
            .split(',')
            .map(|n| n.trim().parse())
            .collect::<Result<_, _>>()?;
        let boards = blocks
            .filter(|block| !block.trim().is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self { draws, boards })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        match self.scores_in_winning_order().first() {
            Some(score) => Ok(Answer::solved(score.to_string())),
            None => Ok(Answer::None),
        }
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        match self.scores_in_winning_order().last() {
            Some(score) => Ok(Answer::solved(score.to_string())),
            None => Ok(Answer::None),
        }
    }
}
