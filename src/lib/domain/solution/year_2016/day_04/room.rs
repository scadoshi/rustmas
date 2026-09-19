use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidRoom {
    #[error("expected a checksum in brackets")]
    MissingChecksum,
    #[error("expected a sector id after the name")]
    MissingId,
    #[error("sector id must be a number")]
    IdNotNumber,
}

/// How many letters a checksum holds.
const CHECKSUM_LEN: usize = 5;

#[derive(Debug, Clone)]
pub struct Room {
    encrypted_name: String,
    pub id: u32,
    checksum: String,
}

impl TryFrom<&str> for Room {
    type Error = InvalidRoom;

    /// Parses `aaaaa-bbb-z-y-x-123[abxyz]`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (head, checksum) = value
            .trim()
            .strip_suffix(']')
            .and_then(|rest| rest.rsplit_once('['))
            .ok_or(InvalidRoom::MissingChecksum)?;
        let (name, id) = head.rsplit_once('-').ok_or(InvalidRoom::MissingId)?;
        Ok(Self {
            encrypted_name: name.to_owned(),
            id: id.parse().map_err(|_| InvalidRoom::IdNotNumber)?,
            checksum: checksum.to_owned(),
        })
    }
}

impl Room {
    /// Whether the checksum is the five commonest letters, ties by letter.
    pub fn is_real(&self) -> bool {
        let mut counts = [0usize; 26];
        for b in self.encrypted_name.bytes().filter(|b| *b != b'-') {
            counts[(b - b'a') as usize] += 1;
        }
        let mut letters: Vec<(usize, u8)> = (0..26)
            .map(|i| (counts[i], b'a' + i as u8))
            .filter(|(count, _)| *count > 0)
            .collect();
        letters.sort_unstable_by(|(c1, l1), (c2, l2)| c2.cmp(c1).then(l1.cmp(l2)));
        letters
            .iter()
            .take(CHECKSUM_LEN)
            .map(|(_, letter)| *letter as char)
            .eq(self.checksum.chars())
    }

    /// The name with each letter rotated forward by the sector id.
    ///
    /// Dashes become spaces, as the puzzle describes.
    pub fn decrypted_name(&self) -> String {
        let shift = (self.id % 26) as u8;
        self.encrypted_name
            .bytes()
            .map(|b| match b {
                b'-' => ' ',
                _ => ((b - b'a' + shift) % 26 + b'a') as char,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_try_from_str_ok() {
        let room = Room::try_from("aaaaa-bbb-z-y-x-123[abxyz]").unwrap();
        assert_eq!(room.encrypted_name, "aaaaa-bbb-z-y-x");
        assert_eq!(room.id, 123);
        assert_eq!(room.checksum, "abxyz");
    }

    #[test]
    fn room_try_from_str_err() {
        assert!(matches!(
            Room::try_from("aaaaa-bbb-123"),
            Err(InvalidRoom::MissingChecksum)
        ));
        assert!(matches!(
            Room::try_from("noname[abxyz]"),
            Err(InvalidRoom::MissingId)
        ));
        assert!(matches!(
            Room::try_from("aaaaa-bbb-xyz[abxyz]"),
            Err(InvalidRoom::IdNotNumber)
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
            assert!(Room::try_from(line).unwrap().is_real(), "{line}");
        }
        assert!(
            !Room::try_from("totally-real-room-200[decoy]")
                .unwrap()
                .is_real()
        );
    }

    #[test]
    fn decrypted_name_rotates_by_the_id() {
        assert_eq!(
            Room::try_from("qzmt-zixmtkozy-ivhz-343[zimth]")
                .unwrap()
                .decrypted_name(),
            "very encrypted name"
        );
    }
}
