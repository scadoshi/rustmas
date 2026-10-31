//! Day 5, How About a Nice Game of Chess?: builds a door's eight-character password from MD5 hashes of the door id plus a counter, keeping only hashes whose hex starts with five zeros. The hashing lives in [`hashing`].

mod hashing;

use crate::domain::solution::Solution;
use crate::domain::solution::answer::Answer;

/// The door to crack.
#[derive(Debug)]
pub(crate) struct Puzzle {
    /// The door id, trimmed, used as the prefix of every hash.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_digit_spells_lowercase() {
        for (nibble, want) in [(0, '0'), (9, '9'), (10, 'a'), (15, 'f')] {
            assert_eq!(hex_digit(nibble).unwrap(), want, "nibble {nibble}");
        }
    }

    #[test]
    fn hex_digit_rejects_more_than_a_nibble() {
        assert!(hex_digit(16).is_err());
    }

    #[test]
    #[ignore = "slow in debug; run with --release --ignored"]
    fn example() {
        let puzzle = Puzzle::new("abc").unwrap();
        assert_eq!(puzzle.part_one().unwrap(), Answer::solved("18f47a30"));
        assert_eq!(puzzle.part_two().unwrap(), Answer::solved("05ace8e3"));
    }
}
