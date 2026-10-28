use std::fmt::Write;

/// The sixth and seventh hex digits of every digest starting with five zeros.
///
/// Five zero digits are two zero bytes and a third below `0x10`, so the sixth
/// digit is that byte's low nibble and the seventh is the next byte's high one.
/// Working on the digest keeps this off the allocator, which matters across the
/// millions of hashes a door takes.
pub fn interesting_digests(door: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    let mut candidate = door.to_owned();
    (0u64..).filter_map(move |suffix| {
        candidate.truncate(door.len());
        let _ = write!(candidate, "{suffix}");
        let md5::Digest([0, 0, third, fourth, ..]) = md5::compute(&candidate) else {
            return None;
        };
        (third < 0x10).then_some((third & 0x0f, fourth >> 4))
    })
}
