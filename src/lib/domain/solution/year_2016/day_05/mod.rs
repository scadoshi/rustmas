pub mod hashing;

use crate::domain::solution::{
    Solution,
    answer::Answer,
    year_2016::day_05::hashing::{HEX, interesting_digests},
};

pub struct Puzzle {
    door: String,
}

/// How many characters a password holds.
const PASSWORD_LEN: usize = 8;

impl Solution for Puzzle {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self> {
        Ok(Self {
            door: input.as_ref().trim().to_owned(),
        })
    }

    /// Each interesting digest gives the next character, in order.
    fn part_one(&self) -> anyhow::Result<Answer> {
        let password: Vec<u8> = interesting_digests(&self.door)
            .take(PASSWORD_LEN)
            .map(|(sixth, _)| HEX[sixth as usize])
            .collect();
        Ok(Answer::solved(String::from_utf8(password)?))
    }

    /// The sixth digit is now the position, the seventh the character, and only
    /// the first digest to name a position counts.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut password = [None; PASSWORD_LEN];
        for (position, character) in interesting_digests(&self.door) {
            let Some(slot) = password.get_mut(position as usize) else {
                continue;
            };
            slot.get_or_insert(HEX[character as usize]);
            if password.iter().all(Option::is_some) {
                break;
            }
        }
        let password: Vec<u8> = password.into_iter().flatten().collect();
        Ok(Answer::solved(String::from_utf8(password)?))
    }
}
