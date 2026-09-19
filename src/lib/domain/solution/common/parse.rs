//! The shapes every input takes, parsed once here rather than in each day.

use std::{error::Error, str::FromStr};

/// Every non-blank line as a `T`, stopping at the first that will not parse.
///
/// Blank lines are skipped, so a trailing newline or a stray gap never fails a
/// day whose records are one per line.
pub fn lines<T>(input: &str) -> anyhow::Result<Vec<T>>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().map_err(anyhow::Error::from))
        .collect()
}

/// Every blank-line-separated block as a `T`, stopping at the first that will
/// not parse. For inputs whose records span several lines.
pub fn blocks<T>(input: &str) -> anyhow::Result<Vec<T>>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    input
        .split("\n\n")
        .map(str::trim)
        .filter(|block| !block.is_empty())
        .map(|block| block.parse().map_err(anyhow::Error::from))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_skips_blanks_and_trims() {
        let parsed: Vec<u32> = lines(" 1 \n\n2\n3\n").unwrap();
        assert_eq!(parsed, [1, 2, 3]);
    }

    #[test]
    fn lines_reports_the_first_bad_line() {
        assert!(lines::<u32>("1\nx\n3").is_err());
    }

    #[test]
    fn blocks_splits_on_a_blank_line() {
        let parsed: Vec<String> = blocks("a\nb\n\nc\n\n\nd").unwrap();
        assert_eq!(parsed, ["a\nb", "c", "d"]);
    }
}
