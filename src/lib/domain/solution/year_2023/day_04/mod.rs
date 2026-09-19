pub mod card;

use crate::domain::solution::{
    Solution, answer::Answer, common::parse, year_2023::day_04::card::Card,
};

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
        Ok(Answer::solved(
            self.cards.iter().map(Card::points).sum::<u32>().to_string(),
        ))
    }

    /// A card with `n` matches wins a copy of each of the next `n` cards, and
    /// copies win copies too. Counting forward means each card's copies are
    /// settled before it hands any out.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut copies = vec![1u32; self.cards.len()];
        for (i, card) in self.cards.iter().enumerate() {
            let won = copies[i];
            for later in copies.iter_mut().skip(i + 1).take(card.matches()) {
                *later += won;
            }
        }
        Ok(Answer::solved(copies.iter().sum::<u32>().to_string()))
    }
}
