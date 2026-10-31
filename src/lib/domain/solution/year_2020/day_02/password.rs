//! A password and the policy on its line, parsed from `1-3 a: abcde`.

use std::str::FromStr;

use thiserror::Error;

/// The `1-3 a` before the colon. Its two numbers mean a count range in part one and two positions in part two.
#[derive(Debug, Clone, Copy)]
struct Policy {
    /// The minimum count, or the first position counted from 1. Never more than `hi`.
    lo: usize,
    /// The maximum count, or the second position counted from 1.
    hi: usize,
    /// The one character the policy is about.
    letter: char,
}

/// Why a policy failed to parse.
#[derive(Debug, Error)]
pub(super) enum InvalidPolicy {
    /// The range or the letter is missing.
    #[error("expected a range and a letter separated by whitespace")]
    TooFewParts,
    /// No hyphen, or a side that isn't an unsigned number.
    #[error("expected a range like `1-3`")]
    MalformedRange,
    /// The low number is above the high one, like `3-1`.
    #[error("expected the range to run upward")]
    Backwards,
    /// The letter part is empty or more than one character.
    #[error("expected a single letter")]
    MalformedLetter,
}

impl FromStr for Policy {
    type Err = InvalidPolicy;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split_whitespace();
        let (Some(range), Some(letter)) = (parts.next(), parts.next()) else {
            return Err(InvalidPolicy::TooFewParts);
        };
        let Some((lo, hi)) = range.split_once('-') else {
            return Err(InvalidPolicy::MalformedRange);
        };
        let (Ok(lo), Ok(hi)) = (lo.parse::<usize>(), hi.parse::<usize>()) else {
            return Err(InvalidPolicy::MalformedRange);
        };
        if lo > hi {
            return Err(InvalidPolicy::Backwards);
        }
        let mut letters = letter.chars();
        let (Some(letter), None) = (letters.next(), letters.next()) else {
            return Err(InvalidPolicy::MalformedLetter);
        };
        Ok(Self { lo, hi, letter })
    }
}

/// Why a database line failed to parse.
#[derive(Debug, Error)]
pub(super) enum InvalidPassword {
    /// The line has no `:` to split on.
    #[error("expected a colon between the policy and the password")]
    MissingColon,
    /// The part before the colon isn't a valid [`Policy`].
    #[error(transparent)]
    Policy(#[from] InvalidPolicy),
}

/// One database line: a password and the policy it should meet.
#[derive(Debug, Clone)]
pub(super) struct Password {
    /// The rule written before the colon.
    policy: Policy,
    /// The password after the colon, trimmed of surrounding whitespace.
    value: String,
}

impl FromStr for Password {
    type Err = InvalidPassword;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (policy, password) = value.split_once(':').ok_or(InvalidPassword::MissingColon)?;
        let value = password.trim().to_owned();
        let policy = policy.parse()?;
        Ok(Self { policy, value })
    }
}

impl Password {
    /// Whether the letter appears between `lo` and `hi` times, both inclusive.
    pub(super) fn valid_count(&self) -> bool {
        let count = self
            .value
            .chars()
            .filter(|c| *c == self.policy.letter)
            .count();
        count >= self.policy.lo && count <= self.policy.hi
    }

    /// Whether exactly one of the two positions, counted from 1, holds the
    /// letter. Position 0 holds nothing.
    pub(super) fn valid_position(&self) -> bool {
        let at = |position: usize| {
            position
                .checked_sub(1)
                .and_then(|i| self.value.chars().nth(i))
        };
        let lo = at(self.policy.lo);
        let hi = at(self.policy.hi);
        (lo.is_some_and(|lo| lo == self.policy.letter)
            || hi.is_some_and(|hi| hi == self.policy.letter))
            && lo != hi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_from_str_ok() {
        let policy = "1-3 a".parse::<Policy>().unwrap();
        assert_eq!(policy.lo, 1);
        assert_eq!(policy.hi, 3);
        assert_eq!(policy.letter, 'a');
    }

    #[test]
    fn policy_from_str_err() {
        assert!(matches!(
            "1-3".parse::<Policy>(),
            Err(InvalidPolicy::TooFewParts)
        ));
        assert!(matches!(
            "13 a".parse::<Policy>(),
            Err(InvalidPolicy::MalformedRange)
        ));
        assert!(matches!(
            "1-3 abc".parse::<Policy>(),
            Err(InvalidPolicy::MalformedLetter)
        ));
    }

    /// A policy's range runs low to high, so `3-1` is a typo rather than a
    /// range to flip.
    #[test]
    fn a_backwards_range_is_rejected() {
        assert!(matches!(
            "3-1 a".parse::<Policy>(),
            Err(InvalidPolicy::Backwards)
        ));
    }

    #[test]
    fn password_from_str_ok() {
        let password = "1-3 a: abcd".parse::<Password>().unwrap();
        assert_eq!(password.value, "abcd");
    }

    #[test]
    fn password_from_str_err() {
        assert!(matches!(
            "1-3 a abcd".parse::<Password>(),
            Err(InvalidPassword::MissingColon)
        ));
    }

    #[test]
    fn password_valid_count() {
        assert!("1-3 a: abcd".parse::<Password>().unwrap().valid_count());
        assert!(!"1-3 a: aaaabcd".parse::<Password>().unwrap().valid_count());
    }

    #[test]
    fn password_valid_position() {
        assert!("1-3 a: abcd".parse::<Password>().unwrap().valid_position());
        assert!(!"1-3 a: abad".parse::<Password>().unwrap().valid_position());
    }

    /// Positions count from 1, so a 0 names no character rather than
    /// underflowing.
    #[test]
    fn position_zero_matches_nothing() {
        assert!("0-1 a: abcd".parse::<Password>().unwrap().valid_position());
        assert!(!"0-3 a: abcd".parse::<Password>().unwrap().valid_position());
    }
}
