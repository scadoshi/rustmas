/// Which bit value to keep when the readings are split on one position.
#[derive(Debug, Clone, Copy)]
pub enum Keep {
    /// The commoner bit, ones winning a tie.
    MostCommon,
    /// The rarer bit, zeros winning a tie.
    LeastCommon,
}

/// The bit `keep` picks at position `bit` across `readings`.
fn bit_to_keep(readings: &[u32], bit: u32, keep: Keep) -> u32 {
    let ones = readings.iter().filter(|r| *r >> bit & 1 == 1).count();
    let ones_lead = ones * 2 >= readings.len();
    match keep {
        Keep::MostCommon => u32::from(ones_lead),
        Keep::LeastCommon => u32::from(!ones_lead),
    }
}

/// A number built from the bit `keep` picks at every position.
pub fn rate(readings: &[u32], width: u32, keep: Keep) -> u32 {
    (0..width).fold(0, |rate, bit| {
        rate | bit_to_keep(readings, bit, keep) << bit
    })
}

/// Narrows the readings one bit at a time, top down, until one is left.
pub fn rating_by_filter(readings: &[u32], width: u32, keep: Keep) -> Option<u32> {
    let mut left = readings.to_vec();
    for bit in (0..width).rev() {
        if left.len() <= 1 {
            break;
        }
        let wanted = bit_to_keep(&left, bit, keep);
        left.retain(|r| r >> bit & 1 == wanted);
    }
    match left.as_slice() {
        [only] => Some(*only),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [u32; 12] = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    /// The puzzle's example, which exercises the tie rules in part two.
    #[test]
    fn the_example_rates() {
        assert_eq!(rate(&EXAMPLE, 5, Keep::MostCommon), 22);
        assert_eq!(rate(&EXAMPLE, 5, Keep::LeastCommon), 9);
        assert_eq!(rating_by_filter(&EXAMPLE, 5, Keep::MostCommon), Some(23));
        assert_eq!(rating_by_filter(&EXAMPLE, 5, Keep::LeastCommon), Some(10));
    }

    #[test]
    fn a_tie_goes_to_ones_for_most_and_zeros_for_least() {
        let tied = [0b1, 0b0];
        assert_eq!(bit_to_keep(&tied, 0, Keep::MostCommon), 1);
        assert_eq!(bit_to_keep(&tied, 0, Keep::LeastCommon), 0);
    }
}
