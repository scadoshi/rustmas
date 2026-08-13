use std::{collections::HashMap, sync::LazyLock};

/// The spelled-out digits part two accepts. No `zero`, which never appears.
pub(super) static NUMBER_WORD_MAP: LazyLock<HashMap<&str, u32>> = LazyLock::new(|| {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
});

/// A line's calibration value: its first and last number as a two-digit one.
pub(super) trait Calibration {
    /// Digits only. [`None`] when the line has none.
    fn calibration_value(&self) -> Option<u32>;
    /// Digits and spelled-out words. [`None`] when the line has neither.
    fn calibration_value_with_words(&self) -> Option<u32>;
}

/// The number starting at the front of `s`, as a digit or a word.
fn number_at(s: &str) -> Option<u32> {
    s.chars().next()?.to_digit(10).or_else(|| {
        NUMBER_WORD_MAP
            .iter()
            .find_map(|(word, num)| s.starts_with(word).then_some(*num))
    })
}

impl Calibration for str {
    /// One digit is both the first and the last.
    fn calibration_value(&self) -> Option<u32> {
        let mut digits = self.chars().filter_map(|c| c.to_digit(10));
        let first = digits.next()?;
        Some(first * 10 + digits.next_back().unwrap_or(first))
    }

    /// Reads from every position rather than consuming a match, so overlapping
    /// words both count: `eightwo` is 8 and 2.
    fn calibration_value_with_words(&self) -> Option<u32> {
        let mut nums = self
            .char_indices()
            .filter_map(|(i, _)| number_at(&self[i..]));
        let first = nums.next()?;
        Some(first * 10 + nums.next_back().unwrap_or(first))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_value() {
        assert_eq!("foo6bar7baz".calibration_value(), Some(67));
        assert_eq!("".calibration_value(), None);
    }

    #[test]
    fn calibration_value_with_words() {
        assert_eq!("sixty7".calibration_value_with_words(), Some(67));
        assert_eq!("".calibration_value_with_words(), None);
        assert_eq!("eightwo".calibration_value_with_words(), Some(82));
    }
}
