use std::fmt::Write;

/// How many leading hex digits of `digest` are zero.
///
/// A zero byte is two zero digits, a byte under `0x10` is one.
pub fn leading_zeros(digest: &[u8; 16]) -> usize {
    let mut zeros = 0;
    for byte in digest {
        match byte {
            0 => zeros += 2,
            b if *b < 0x10 => {
                zeros += 1;
                break;
            }
            _ => break,
        }
    }
    zeros
}

/// The lowest number that, appended to `secret`, hashes with `zeros` leading
/// zeros in hex.
///
/// Reuses one buffer, since part two tries this several million times.
pub fn lowest_suffix(secret: &str, zeros: usize) -> usize {
    let mut candidate = String::with_capacity(secret.len() + 8);
    for suffix in 0.. {
        candidate.clear();
        candidate.push_str(secret);
        let _ = write!(candidate, "{suffix}");
        if leading_zeros(&md5::compute(&candidate)) >= zeros {
            return suffix;
        }
    }
    unreachable!("the range is unbounded")
}
