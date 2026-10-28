use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Policy {
    pub lo: usize,
    pub hi: usize,
    pub letter: char,
}

#[derive(Debug, Error)]
pub enum InvalidPolicy {
    #[error("expected a range and a letter separated by whitespace")]
    TooFewParts,
    #[error("expected a range like `1-3`")]
    MalformedRange,
    #[error("expected the range to run upward")]
    Backwards,
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

#[derive(Debug, Error)]
pub enum InvalidPassword {
    #[error("expected a colon between the policy and the password")]
    MissingColon,
    #[error(transparent)]
    Policy(#[from] InvalidPolicy),
}

#[derive(Debug, Clone)]
pub struct Password {
    policy: Policy,
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
    pub fn valid_count(&self) -> bool {
        let count = self
            .value
            .chars()
            .filter(|c| *c == self.policy.letter)
            .count();
        count >= self.policy.lo && count <= self.policy.hi
    }

    /// Whether exactly one of the two positions, counted from 1, holds the
    /// letter. Position 0 holds nothing.
    pub fn valid_position(&self) -> bool {
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
