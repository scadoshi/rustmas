mod hashing;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

#[derive(Debug)]
pub(crate) struct Puzzle {
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
        let password = hashing::interesting_digests(&self.door)
            .take(PASSWORD_LEN)
            .map(|(sixth, _)| hex_digit(sixth))
            .collect::<anyhow::Result<String>>()?;
        Ok(Answer::solved(password))
    }

    /// The sixth digit names the position and the seventh the character, and
    /// only the first digest to name a position counts.
    fn part_two(&self) -> anyhow::Result<Answer> {
        let mut password = [None; PASSWORD_LEN];
        for (position, character) in hashing::interesting_digests(&self.door) {
            let Some(slot) = password.get_mut(usize::from(position)) else {
                continue;
            };
            if slot.is_none() {
                *slot = Some(hex_digit(character)?);
            }
            if password.iter().all(Option::is_some) {
                break;
            }
        }
        let password: String = password.into_iter().flatten().collect();
        Ok(Answer::solved(password))
    }
}

/// The lowercase hex digit spelling `nibble`.
fn hex_digit(nibble: u8) -> anyhow::Result<char> {
    char::from_digit(u32::from(nibble), 16)
        .ok_or_else(|| anyhow::anyhow!("expected a nibble below 16, got {nibble}"))
}
