//! The MD5 search for hashes starting with five zero hex digits.

use std::fmt::Write;

/// The sixth and seventh hex digits of every digest starting with five zeros.
///
/// Suffixes run from 0 upward, so the digests come in the order the puzzle
/// reads them. Working on the digest keeps this off the allocator, which
/// matters across the millions of hashes a door takes.
pub(super) fn interesting_digests(door: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    let mut candidate = door.to_owned();
    (0u64..).filter_map(move |suffix| {
        candidate.truncate(door.len());
        let _ = write!(candidate, "{suffix}");
        sixth_and_seventh(&md5::compute(&candidate))
    })
}

/// The sixth and seventh hex digits of `digest`, if its first five are zeros.
///
/// Five zero digits are two zero bytes and a third below `0x10`, so the sixth
/// digit is that byte's low nibble and the seventh is the next byte's high one.
fn sixth_and_seventh(digest: &md5::Digest) -> Option<(u8, u8)> {
    let md5::Digest([0, 0, third, fourth, ..]) = *digest else {
        return None;
    };
    (third < 0x10).then_some((third & 0x0f, fourth >> 4))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Indexes from the puzzle's example for door `abc`, and the two digits each
    /// digest gives.
    #[test]
    fn known_digests_give_their_sixth_and_seventh_digits() {
        for (suffix, want) in [
            (3_231_929, (0x1, 0x5)),
            (5_017_308, (0x8, 0xf)),
            (5_278_568, (0xf, 0x9)),
            (5_357_525, (0x4, 0xe)),
        ] {
            let digest = md5::compute(format!("abc{suffix}"));
            assert_eq!(sixth_and_seventh(&digest), Some(want), "suffix {suffix}");
        }
    }

    #[test]
    fn digests_without_five_leading_zeros_are_skipped() {
        for suffix in [0, 1, 3_231_928] {
            let digest = md5::compute(format!("abc{suffix}"));
            assert_eq!(sixth_and_seventh(&digest), None, "suffix {suffix}");
        }
    }

    #[test]
    fn four_zeros_then_a_high_nibble_is_not_five_zeros() {
        let mut bytes = [0_u8; 16];
        bytes[2] = 0x10;
        assert_eq!(sixth_and_seventh(&md5::Digest(bytes)), None);
        bytes[2] = 0x0a;
        bytes[3] = 0xb7;
        assert_eq!(sixth_and_seventh(&md5::Digest(bytes)), Some((0xa, 0xb)));
    }
}
