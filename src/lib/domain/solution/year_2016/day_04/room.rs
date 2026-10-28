use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidRoom {
    #[error("expected a checksum in brackets")]
    MissingChecksum,
    #[error("expected a sector id after the name")]
    MissingId,
    /// The name holds something other than `a-z` and dashes.
    #[error("expected only lowercase letters and dashes in the name, read {0:?}")]
    NotALetter(char),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// The letters a room name may hold besides dashes, in order.
const ALPHABET: &[u8; 26] = b"abcdefghijklmnopqrstuvwxyz";

/// How many letters a checksum holds.
const CHECKSUM_LEN: usize = 5;

#[derive(Debug, Clone)]
pub struct Room {
    /// Only `a-z` and dashes, checked when parsed.
    encrypted_name: String,
    pub id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = InvalidRoom;

    /// Parses `aaaaa-bbb-z-y-x-123[abxyz]`.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (head, checksum) = value
            .trim()
            .strip_suffix(']')
            .and_then(|rest| rest.rsplit_once('['))
            .ok_or(InvalidRoom::MissingChecksum)?;
        let (name, id) = head.rsplit_once('-').ok_or(InvalidRoom::MissingId)?;
        if let Some(other) = name.chars().find(|c| *c != '-' && !c.is_ascii_lowercase()) {
            return Err(InvalidRoom::NotALetter(other));
        }
        Ok(Self {
            encrypted_name: name.to_owned(),
            id: id.parse()?,
            checksum: checksum.to_owned(),
        })
    }
}

impl Room {
    /// Whether the checksum is the five commonest letters, ties by letter.
    pub fn is_real(&self) -> bool {
        let mut letters: Vec<(usize, u8)> = ALPHABET
            .iter()
            .map(|letter| {
                let count = self.encrypted_name.bytes().filter(|b| b == letter).count();
                (count, *letter)
            })
            .filter(|(count, _)| *count > 0)
            .collect();
        letters.sort_unstable_by(|(c1, l1), (c2, l2)| c2.cmp(c1).then(l1.cmp(l2)));
        letters
            .iter()
            .take(CHECKSUM_LEN)
            .map(|(_, letter)| char::from(*letter))
            .eq(self.checksum.chars())
    }

    /// The name with each letter rotated forward by the sector id.
    ///
    /// Dashes become spaces, as the puzzle describes.
    pub fn decrypted_name(&self) -> String {
        let shift = self.id % 26;
        self.encrypted_name
            .bytes()
            .map(|b| match b {
                b'-' => ' ',
                _ => char::from(rotated(b, shift)),
            })
            .collect()
    }
}

/// `letter` moved `shift` places on through the alphabet, wrapping past `z`.
///
/// Anything outside `a-z` comes back unchanged.
fn rotated(letter: u8, shift: u32) -> u8 {
    let Some(start) = ALPHABET.iter().position(|l| *l == letter) else {
        return letter;
    };
    ALPHABET
        .iter()
        .cycle()
        .skip(start)
        .zip(0..=shift)
        .last()
        .map_or(letter, |(rotated, _)| *rotated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_from_str_ok() {
        let room = "aaaaa-bbb-z-y-x-123[abxyz]".parse::<Room>().unwrap();
        assert_eq!(room.encrypted_name, "aaaaa-bbb-z-y-x");
        assert_eq!(room.id, 123);
        assert_eq!(room.checksum, "abxyz");
    }

    #[test]
    fn room_from_str_err() {
        assert!(matches!(
            "aaaaa-bbb-123".parse::<Room>(),
            Err(InvalidRoom::MissingChecksum)
        ));
        assert!(matches!(
            "noname[abxyz]".parse::<Room>(),
            Err(InvalidRoom::MissingId)
        ));
        assert!(matches!(
            "aaaaa-bbb-xyz[abxyz]".parse::<Room>(),
            Err(InvalidRoom::ParseInt(_))
        ));
    }

    /// The examples from the puzzle, the last of which is the decoy.
    #[test]
    fn is_real_follows_the_examples() {
        for line in [
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
        ] {
            assert!(line.parse::<Room>().unwrap().is_real(), "{line}");
        }
        assert!(
            !"totally-real-room-200[decoy]"
                .parse::<Room>()
                .unwrap()
                .is_real()
        );
    }

    /// Anything outside `a-z` and dashes would index past the letter counts.
    #[test]
    fn a_name_outside_lowercase_letters_is_rejected() {
        for line in [
            "Abc-123[abc]",
            "a{b-123[ab]",
            "a.b-123[ab]",
            "\u{e9}t-123[et]",
        ] {
            assert!(line.parse::<Room>().is_err(), "{line}");
        }
    }

    #[test]
    fn decrypted_name_rotates_by_the_id() {
        assert_eq!(
            "qzmt-zixmtkozy-ivhz-343[zimth]"
                .parse::<Room>()
                .unwrap()
                .decrypted_name(),
            "very encrypted name"
        );
    }
}
