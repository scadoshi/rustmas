use crate::domain::solution::{Solution, answer::Answer};
use std::fmt::Write;

pub struct Puzzle {
    door: String,
}

/// How many characters a password holds.
const PASSWORD_LEN: usize = 8;
/// The hex digits, indexed by the nibble they spell.
const HEX: [u8; 16] = *b"0123456789abcdef";

/// The sixth and seventh hex digits of every digest starting with five zeros.
///
/// Five zero digits are two zero bytes and a third below `0x10`, so the sixth
/// digit is that byte's low nibble and the seventh is the next byte's high one.
/// Working on the digest keeps this off the allocator, which matters across the
/// millions of hashes a door takes.
fn interesting_digests(door: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    let mut candidate = String::with_capacity(door.len() + 10);
    (0u64..).filter_map(move |suffix| {
        candidate.clear();
        candidate.push_str(door);
        let _ = write!(candidate, "{suffix}");
        let digest = md5::compute(&candidate);
        (digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10)
            .then(|| (digest[2] & 0x0f, digest[3] >> 4))
    })
}

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
