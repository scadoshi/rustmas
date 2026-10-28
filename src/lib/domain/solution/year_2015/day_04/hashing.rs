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
/// zeros in hex, or `None` if no [`usize`] does.
///
/// Reuses one buffer, since part two tries this several million times.
pub fn lowest_suffix(secret: &str, zeros: usize) -> Option<usize> {
    let mut candidate = String::with_capacity(secret.len() + 8);
    (0..=usize::MAX).find(|suffix| {
        candidate.clear();
        candidate.push_str(secret);
        let _ = write!(candidate, "{suffix}");
        leading_zeros(&md5::compute(&candidate)) >= zeros
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_zeros_counts_hex_digits() {
        let mut digest = [0xff; 16];
        assert_eq!(leading_zeros(&digest), 0);
        digest[0] = 0x0f;
        assert_eq!(leading_zeros(&digest), 1);
        digest[0] = 0;
        digest[1] = 0x0a;
        assert_eq!(leading_zeros(&digest), 3);
    }

    /// The puzzle's first example.
    #[test]
    fn lowest_suffix_finds_the_example() {
        assert_eq!(lowest_suffix("abcdef", 5), Some(609_043));
    }
}
