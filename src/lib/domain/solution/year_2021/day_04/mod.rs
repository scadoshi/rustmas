pub mod board;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::year_2021::day_04::board::Board;

#[derive(Debug)]
pub struct Puzzle {
    draws: Vec<u8>,
    boards: Vec<Board>,
}

impl Puzzle {
    /// Each board's score in the order they win, as the draws are called.
    ///
    /// A board's score is its unmarked sum times the draw that completed it.
    /// Boards leave the game when they win, so nothing scores twice.
    fn scores_in_winning_order(&self) -> anyhow::Result<Vec<u32>> {
        let mut boards = self.boards.clone();
        let mut scores = Vec::new();
        for &draw in &self.draws {
            for board in &mut boards {
                board.mark(draw);
            }
            for board in boards.extract_if(.., |board| board.has_bingo()) {
                let score = board
                    .unmarked_sum()
                    .and_then(|sum| sum.checked_mul(u32::from(draw)))
                    .ok_or_else(|| anyhow!("a board's score passes u32"))?;
                scores.push(score);
            }
        }
        Ok(scores)
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
        match self.scores_in_winning_order()?.first() {
            Some(score) => Ok(Answer::solved(score.to_string())),
            None => Ok(Answer::None),
        }
    }

    fn part_two(&self) -> anyhow::Result<Answer> {
        match self.scores_in_winning_order()?.last() {
            Some(score) => Ok(Answer::solved(score.to_string())),
            None => Ok(Answer::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    /// The puzzle's example for both parts.
    #[test]
    fn example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("4512"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("1924"));
    }
}
