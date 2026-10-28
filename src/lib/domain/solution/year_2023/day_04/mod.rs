pub mod card;

use anyhow::anyhow;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;
use crate::domain::solution::common::parse;
use crate::domain::solution::year_2023::day_04::card::Card;

#[derive(Debug)]
pub struct Puzzle {
    cards: Vec<Card>,
}

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            cards: parse::lines(input.as_ref())?,
        })
    }

    fn part_one(&self) -> anyhow::Result<Answer> {
        let total = self.cards.iter().try_fold(0_u32, |total, card| {
            card.points()
                .and_then(|points| total.checked_add(points))
                .ok_or_else(|| anyhow!("the points add up past {}", u32::MAX))
        })?;
        Ok(Answer::solved(total.to_string()))
    }

    /// A card with `n` matches wins a copy of each of the next `n` cards, and
    /// copies win copies too. Counting forward means each card's copies are
    /// settled before it hands any out.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let too_many = || anyhow!("the copies add up past {}", u32::MAX);
        let mut copies = vec![1_u32; self.cards.len()];
        let mut rest = copies.as_mut_slice();
        for card in &self.cards {
            let Some((won, later)) = rest.split_first_mut() else {
                break;
            };
            for copy in later.iter_mut().take(card.matches()) {
                *copy = copy.checked_add(*won).ok_or_else(too_many)?;
            }
            rest = later;
        }
        let total = copies
            .iter()
            .try_fold(0_u32, |total, &copy| total.checked_add(copy))
            .ok_or_else(too_many)?;
        Ok(Answer::solved(total.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    /// `count` cards that each match `count` numbers.
    fn cards_matching(count: u32) -> String {
        let numbers = (1..=count)
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let mut cards = String::new();
        for id in 1..=count {
            writeln!(cards, "Card {id}: {numbers} | {numbers}").unwrap();
        }
        cards
    }

    #[test]
    fn the_example() {
        let puzzle = Puzzle::new(EXAMPLE).unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("13"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("30"));
    }

    /// 33 matches is worth 2^32 points, past a `u32`.
    #[test]
    fn too_many_points_is_an_error() {
        let puzzle = Puzzle::new(cards_matching(33)).unwrap();
        assert!(puzzle.part_one().is_err());
    }

    /// Each card doubles the copies of every one after it.
    #[test]
    fn too_many_copies_is_an_error() {
        let puzzle = Puzzle::new(cards_matching(40)).unwrap();
        assert!(puzzle.part_two().is_err());
    }
}
