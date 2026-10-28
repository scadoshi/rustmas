//! The shapes every input takes, parsed once here rather than in each day.

use std::error::Error;
use std::str::FromStr;

use anyhow::Context;

/// Every non-blank line as a `T`, stopping at the first that will not parse.
///
/// The error names that line by its 1-based number in `input`. Blank lines are
/// skipped, so a trailing newline or a stray gap never fails a
/// day whose records are one per line.
pub fn lines<T>(input: &str) -> anyhow::Result<Vec<T>>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    (1..)
        .zip(input.lines())
        .map(|(number, line)| (number, line.trim()))
        .filter(|(_, line)| !line.is_empty())
        .map(|(number, line)| {
            line.parse()
                .map_err(anyhow::Error::from)
                .with_context(|| format!("failed to parse line {number}"))
        })
        .collect()
}

/// Every blank-line-separated block as a `T`, stopping at the first that will
/// not parse. For inputs whose records span several lines.
///
/// The error names that block by its 1-based position in `input`.
pub fn blocks<T>(input: &str) -> anyhow::Result<Vec<T>>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    (1..)
        .zip(input.split("\n\n"))
        .map(|(number, block)| (number, block.trim()))
        .filter(|(_, block)| !block.is_empty())
        .map(|(number, block)| {
            block
                .parse()
                .map_err(anyhow::Error::from)
                .with_context(|| format!("failed to parse block {number}"))
        })
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
        let error = lines::<u32>("1\n\nx\n3").unwrap_err();
        assert_eq!(error.to_string(), "failed to parse line 3");
    }

    #[test]
    fn blocks_reports_the_first_bad_block() {
        let error = blocks::<u32>("1\n\nx\n\n3").unwrap_err();
        assert_eq!(error.to_string(), "failed to parse block 2");
    }

    #[test]
    fn blocks_splits_on_a_blank_line() {
        let parsed: Vec<String> = blocks("a\nb\n\nc\n\n\nd").unwrap();
        assert_eq!(parsed, ["a\nb", "c", "d"]);
    }
}
