use std::fmt::Write;

/// The hex digits, indexed by the nibble they spell.
pub const HEX: [u8; 16] = *b"0123456789abcdef";

/// The sixth and seventh hex digits of every digest starting with five zeros.
///
/// Five zero digits are two zero bytes and a third below `0x10`, so the sixth
/// digit is that byte's low nibble and the seventh is the next byte's high one.
/// Working on the digest keeps this off the allocator, which matters across the
/// millions of hashes a door takes.
pub fn interesting_digests(door: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
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
